use super::Theme;
use gpui::{App, Application, Context, Corner, Div, Hsla, Stateful, Window, WindowOptions, anchored, deferred, div, prelude::*, px, rgb, Global, Rgba, rgba};
use gpui::private::serde_json;
use serde::Serialize;

struct AppTheme {
  pub inner: Theme,
}

impl AppTheme {
  pub fn new() -> Self {
    Self { inner: Theme::default() }
  }

  pub fn with(theme: Theme) -> Self {
    Self { inner: theme }
  }

}

impl std::ops::Deref for AppTheme {
  type Target = Theme;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl Global for AppTheme {}

/// An example show use deferred to create a floating layers.
struct HelloWorld {
  open: bool,
  secondary_open: bool,
}

fn line(color: impl Into<Hsla>) -> Div {
  div().w(px(480.)).h_2().bg(color.into())
}

impl Render for HelloWorld {
  fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
    let theme = cx.global::<AppTheme>();
    println!("{:?}", theme.primary);
    div()
      .flex()
      .flex_col()
      .gap_3()
      .size_full()
      .bg(theme.base)
      .text_color(theme.text)
      .justify_center()
      .items_center()
      .child(
        "Here is an example text rendered, \
                A few colors as well to test the global theme.",
      )
      .children([
        line(theme.primary),
        line(theme.secondary),
        line(theme.tertiary),
      ])
  }
}

pub fn render() {
  Application::new().run(|cx: &mut App| {
    cx.set_global(AppTheme::new());
    cx.open_window(WindowOptions::default(), |_, cx| {
      cx.new(|_| HelloWorld {
        open: false,
        secondary_open: false,
      })
    })
    .unwrap();
    cx.activate(true);
  });
}
