use crate::state::ThemeState;
use crate::{ThemeColor, renderable, widgets::Block};
use gpui::{Context, IntoElement, ParentElement, Render, Window, div};

#[renderable(parent)]
pub fn UIRoot(
  /// Stuff
  #[global(default)]
  _theme: ThemeState,
) {
  div().children(self.children)
}
