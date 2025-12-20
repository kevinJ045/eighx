use serde::{Serialize, Deserialize};
use gpui::{Rgba, rgba};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
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
impl Default for Theme {
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