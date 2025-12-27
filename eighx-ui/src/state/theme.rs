use crate::Theme;

#[derive(Clone, Default)]
pub struct ThemeState {
  pub inner: Theme,
}

impl ThemeState {
  pub fn new() -> Self {
    Self {
      inner: Theme::default(),
    }
  }

  pub fn with(theme: Theme) -> Self {
    Self { inner: theme }
  }
}

impl std::ops::Deref for ThemeState {
  type Target = Theme;

  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl gpui::Global for ThemeState {}
