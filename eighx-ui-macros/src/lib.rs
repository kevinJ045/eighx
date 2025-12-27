use proc_macro::TokenStream;
use proc_macro2::{self, TokenStream as TokenStream2};
use quote::quote;
use syn::{
  Attribute, Expr, FnArg, Ident, ItemFn, Meta, MetaList, Pat, Path, Token, Type,
  parse::{Parse, ParseStream, Result},
  parse_macro_input,
  punctuated::Punctuated,
};

#[derive(Clone)]
enum FieldKind {
  Prop { default: Option<Expr> },
  Signal { name: Ident },
  Global { value: Option<Expr> },
  Resource { loader: Path },
}

#[derive(Clone)]
struct Field {
  pub name: Ident,
  pub ty: Type,
  pub kind: FieldKind,
}

mod config {
  use super::*;

  pub struct MacroConfig {
    pub derive_traits: Option<Punctuated<Path, Token![,]>>,
    pub is_parent: bool,
    pub is_once: bool,
    pub is_element: bool,
    pub is_mut: bool,
    pub is_owned: bool,
  }

  impl Default for MacroConfig {
    fn default() -> Self {
      Self {
        derive_traits: None,
        is_parent: false,
        is_once: false,
        is_element: false,
        is_mut: false,
        is_owned: false,
      }
    }
  }

  impl Parse for MacroConfig {
    fn parse(input: ParseStream) -> Result<Self> {
      let mut config = MacroConfig::default();

      if input.is_empty() {
        return Ok(config);
      }

      let metas = Punctuated::<Meta, Token![,]>::parse_terminated(input)?;
      for meta in metas {
        if meta.path().is_ident("derive") {
          if let Meta::List(list) = meta {
            let traits: Punctuated<Path, Token![,]> =
              list.parse_args_with(Punctuated::parse_terminated)?;
            config.derive_traits = Some(traits);
          } else {
            return Err(syn::Error::new_spanned(
              meta,
              "expected derive(...) with a parenthesized list of traits",
            ));
          }
        } else if meta.path().is_ident("parent") {
          config.is_parent = true;
        } else if meta.path().is_ident("once") {
          config.is_once = true;
        } else if meta.path().is_ident("element") {
          config.is_element = true;
        } else if meta.path().is_ident("owned") {
          config.is_owned = true;
        } else if meta.path().is_ident("mutable") {
          config.is_mut = true;
        } else {
          return Err(syn::Error::new_spanned(
            meta.path(),
            "unknown attribute option",
          ));
        }
      }

      Ok(config)
    }
  }
}

mod parse {
  use super::*;

  fn parse_prop(attr: &Attribute) -> Result<FieldKind> {
    match &attr.meta {
      Meta::Path(_) => Ok(FieldKind::Prop { default: None }),

      Meta::List(MetaList { tokens, .. }) => {
        // #[prop(default)]
        if let Ok(ident) = syn::parse2::<Ident>(tokens.clone()) {
          if ident == "default" {
            return Ok(FieldKind::Prop {
              default: Some(syn::parse_quote!(Default::default())),
            });
          }
        }

        // #[prop(some_expr)]
        let expr: Expr = syn::parse2(tokens.clone())?;
        Ok(FieldKind::Prop {
          default: Some(expr),
        })
      }

      _ => Err(syn::Error::new_spanned(attr, "Invalid #[prop] attribute")),
    }
  }

  fn parse_global(attr: &Attribute) -> Result<FieldKind> {
    match &attr.meta {
      // Meta::Path(_) => Ok(FieldKind::Global { value: None }),
      Meta::List(MetaList { tokens, .. }) => {
        if let Ok(ident) = syn::parse2::<Ident>(tokens.clone()) {
          if ident == "default" {
            return Ok(FieldKind::Global {
              value: Some(syn::parse_quote!(Default::default())),
            });
          }
        }

        let expr: Expr = syn::parse2(tokens.clone())?;
        Ok(FieldKind::Global { value: Some(expr) })
      }

      _ => Err(syn::Error::new_spanned(attr, "Invalid #[global] attribute")),
    }
  }

  pub fn extract_fields(func: &ItemFn) -> Result<Vec<Field>> {
    let mut fields = Vec::new();

    for arg in &func.sig.inputs {
      let FnArg::Typed(pat) = arg else {
        continue;
      };

      let Pat::Ident(pat_ident) = &*pat.pat else {
        return Err(syn::Error::new_spanned(
          &pat.pat,
          "Unsupported parameter pattern – only simple identifiers allowed",
        ));
      };

      let name = pat_ident.ident.clone();
      let ty = (*pat.ty).clone();

      let mut kind = None;

      for attr in &pat.attrs {
        if attr.path().is_ident("prop") {
          kind = Some(parse_prop(attr)?);
          break;
        }
        if attr.path().is_ident("signal") {
          let ident: Ident = attr.parse_args()?;
          kind = Some(FieldKind::Signal { name: ident });
          break;
        }
        if attr.path().is_ident("global") {
          kind = Some(parse_global(attr)?);
          break;
        }
        if attr.path().is_ident("resource") {
          let path: Path = attr.parse_args()?;
          kind = Some(FieldKind::Resource { loader: path });
          break;
        }
      }

      let Some(kind) = kind else {
        continue;
      };

      fields.push(Field { name, ty, kind });
    }

    Ok(fields)
  }
}

mod generate {

  use syn::parse_quote;

  use super::*;

  fn generate_constructor(
    name: &Ident,
    fields: &[Field],
    config: &crate::config::MacroConfig,
  ) -> (TokenStream2, TokenStream2, TokenStream2) {
    let args = fields.iter().filter_map(|f| match &f.kind {
      FieldKind::Prop { default: None } => {
        let name = &f.name;
        let ty = &f.ty;
        Some(quote! { #name: #ty })
      }
      _ => None,
    });

    let inits = fields
      .iter()
      .map(|f| {
        let name = &f.name;
        match &f.kind {
          FieldKind::Prop {
            default: Some(expr),
          } => Some(quote! { #name: #expr }),
          FieldKind::Prop { default: None } => Some(quote! { #name }),
          _ => None, // FieldKind::Signal { name: signal } => {
                     //   quote! { #name: gpui::Signal::new(stringify!(#signal)) }
                     // }
        }
      })
      .filter(|x| x.is_some())
      .map(|x| x.unwrap());

    let globals = fields.iter().map(|f| {
      let ty = &f.ty;

      match &f.kind {
        FieldKind::Global { value: Some(value) } => quote! { cx.set_global::<#ty>(#value); },
        FieldKind::Resource { loader } => quote! { cx.set_global(#loader::load(cx)); },
        _ => quote! {},
      }
    });

    let globals_get = fields.iter().map(|f| {
      let name = &f.name;
      let ty = &f.ty;

      match &f.kind {
        FieldKind::Global { value: _ } => quote! { let #name = cx.global::<#ty>(); },
        FieldKind::Resource { loader: _ } => quote! { let #name = cx.global::<#ty>(); },
        _ => quote! {},
      }
    });

    let children = if config.is_parent {
      quote! { children: Vec::new(), }
    } else {
      quote! {}
    };

    (
      quote! {
          impl #name {
              pub fn new(#(#args),*) -> Self {
                  Self {
                      #children
                      #(#inits,)*
                  }
              }

              pub fn globals(&self, cx: &mut gpui::App) {
                #(#globals)*
              }
          }
      },
      quote! {
        #(#globals_get)*
      },
      if config.is_parent {
        quote! {
          impl gpui::ParentElement for #name {
              fn extend(&mut self, elements: impl IntoIterator<Item = gpui::AnyElement>) {
                  self.children
                      .extend(elements.into_iter())
              }
          }
        }
      } else {
        quote! {}
      },
    )
  }

  pub fn expand_renderable(
    func: ItemFn,
    config: crate::config::MacroConfig,
  ) -> syn::Result<TokenStream2> {
    let name = &func.sig.ident;
    let body = &func.block;

    let fields = crate::parse::extract_fields(&func)?;

    let struct_fields = fields
      .iter()
      .filter(|f| match f.kind {
        FieldKind::Prop { default: _ } => true,
        _ => false,
      })
      .map(|f| {
        let name = &f.name;
        let ty = &f.ty;
        quote! { pub #name: #ty }
      });

    let children = if config.is_parent {
      quote! { children: Vec<gpui::AnyElement>, }
    } else {
      quote! {}
    };

    let (ctor, pre_render, additional) = generate_constructor(name, &fields, &config);

    let derives = if config.is_element {
      config.derive_traits.map_or_else(
        || quote! { #[derive(gpui::IntoElement)] },
        |traits| quote! { #[derive(gpui::IntoElement, #traits)] },
      )
    } else {
      config
        .derive_traits
        .map_or_else(|| quote! {}, |traits| quote! { #[derive(#traits)] })
    };

    let render_impl = if config.is_once {
      quote! {
        gpui::RenderOnce
      }
    } else {
      quote! {
        gpui::Render
      }
    };

    let render_cx = if config.is_once {
      quote! {
        gpui::App
      }
    } else {
      quote! {
        gpui::Context<Self>
      }
    };

    let self_owned = if config.is_once {
      quote! { mut }
    } else {
      quote! { &mut }
    };

    Ok(quote! {
        #derives
        pub struct #name {
            #(#struct_fields,)*
            #children
        }

        #ctor

        impl #render_impl for #name {
            fn render(
                #self_owned self,
                window: &mut gpui::Window,
                cx: &mut #render_cx,
            ) -> impl gpui::IntoElement {
                #pre_render
                #body
            }
        }

        #additional
    })
  }
}

mod validate {
  use super::*;

  pub fn validate_signature(_func: &ItemFn) -> Result<()> {
    // Add stricter checks here in the future if needed
    Ok(())
  }
}

#[proc_macro_attribute]
pub fn renderable(attr: TokenStream, item: TokenStream) -> TokenStream {
  let config = if attr.is_empty() {
    config::MacroConfig::default()
  } else {
    match syn::parse2(attr.into()) {
      Ok(c) => c,
      Err(e) => return e.to_compile_error().into(),
    }
  };

  let func = parse_macro_input!(item as ItemFn);

  if let Err(e) = validate::validate_signature(&func) {
    return e.to_compile_error().into();
  }

  match generate::expand_renderable(func, config) {
    Ok(ts) => ts.into(),
    Err(e) => e.to_compile_error().into(),
  }
}
