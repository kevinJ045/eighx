use gpui::{Div, Rgba, Size, Styled, rgba};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
  pub base: Rgba,
  pub crust: Rgba,
  pub mantle: Rgba,
  pub surface: Rgba,
  pub surface_lighten: Rgba,
  pub surface_darken: Rgba,
  pub overlay: Rgba,
  pub overlay_lighten: Rgba,
  pub overlay_darken: Rgba,
  pub text: Rgba,
  pub text_darken: Rgba,
  pub text_dark: Rgba,
  pub text_inverse: Rgba,
  pub text_primary: Rgba,
  pub primary: Rgba,
  pub secondary: Rgba,
  pub tertiary: Rgba,
  pub error: Rgba,
  pub success: Rgba,
  pub warn: Rgba,
  pub info: Rgba,
}

impl Default for ThemeColors {
  fn default() -> Self {
    Self {
      base: rgba(0x1e1e2eff),
      crust: rgba(0x181825ff),
      mantle: rgba(0x11111bff),
      surface: rgba(0x45475aff),
      surface_lighten: rgba(0x585b70ff),
      surface_darken: rgba(0x313244ff),
      overlay: rgba(0x7f849cff),
      overlay_lighten: rgba(0x9399b2ff),
      overlay_darken: rgba(0x6c7086ff),
      text: rgba(0xcdd6f4ff),
      text_darken: rgba(0xbac2deff),
      text_dark: rgba(0xa6adc8ff),
      text_inverse: rgba(0xcba6f7ff),
      text_primary: rgba(0x585b70ff),
      primary: rgba(0xcba6f7ff),
      secondary: rgba(0xeba0acff),
      tertiary: rgba(0x89dcebff),
      error: rgba(0xf38ba8ff),
      success: rgba(0xa6e3a1ff),
      warn: rgba(0xf9e2afff),
      info: rgba(0x89b4faff),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockStyles {
  pub rounding: f32,
  pub border: f32,
}

impl Default for BlockStyles {
  fn default() -> Self {
    Self {
      rounding: 0.0,
      border: 0.0,
    }
  }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ThemeStyles {
  pub block: BlockStyles,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Theme {
  pub colors: ThemeColors,
  pub styles: ThemeStyles,
}

impl<'a> Theme {
  pub fn stylize<T>(elt: T, theme: &'a Theme) -> Stylizer<'a, T>
  where
    T: gpui::Element + gpui::Styled,
  {
    Stylizer { elt, theme }
  }
}

impl Theme {
  pub fn get_color(color: ThemeColor, theme: &Theme) -> Rgba {
    use ThemeColor::*;
    match color {
      Crust => theme.colors.crust,
      Base => theme.colors.base,
      Mantle => theme.colors.mantle,
      Primary => theme.colors.primary,
      Secondary => theme.colors.secondary,
      Tertiary => theme.colors.tertiary,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThemeColor {
  Crust,
  Base,
  Mantle,
  Primary,
  Secondary,
  Tertiary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThemeBorder {
  None,
  Colored(ThemeColor),
  Sized(ThemeColor, f32),
}

pub struct Stylizer<'a, T>
where
  T: gpui::Element + gpui::Styled,
{
  elt: T,
  theme: &'a Theme,
}

impl<'a, T> Stylizer<'a, T>
where
  T: gpui::Element + gpui::Styled,
{
  pub fn bg(mut self, bg: ThemeColor) -> Self {
    let color = Theme::get_color(bg, self.theme);
    self.elt = self.elt.bg(color);

    self
  }

  pub fn border(mut self, border: ThemeBorder, size: Option<f32>) -> Self {
    use ThemeBorder::*;
    let size = match border {
      Sized(_, size) => size,
      _ => size.unwrap_or(1.0),
    };

    match border {
      None => {}
      Colored(color) | Sized(color, _) => {
        self.elt = self.elt.border(gpui::px(size));
        let color = Theme::get_color(color, self.theme);
        self.elt = self.elt.border_color(color);
      }
    }

    self
  }

  pub fn into(self) -> T {
    self.elt
  }
}
