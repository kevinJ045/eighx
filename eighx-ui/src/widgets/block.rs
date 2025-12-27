use crate::{renderable, state::ThemeState, theme::*};
use gpui::{Styled, div, px};

#[renderable(element, owned, once)]
pub fn Block(
  #[prop(ThemeColor::Base)] bg: ThemeColor,
  #[prop(ThemeBorder::None)] border: ThemeBorder,
  #[prop(None)] round: Option<f32>,
) {
  let theme = cx.global::<ThemeState>();

  Theme::stylize(div(), theme)
    .bg(self.bg)
    .border(self.border, Some(theme.styles.block.border))
    .into()
    .border_r(px(self.round.unwrap_or(theme.styles.block.rounding)))
}

impl Block {
  pub fn bg(bg: ThemeColor) -> Self {
    let mut block = Self::new();
    block.bg = bg;
    block
  }
}
