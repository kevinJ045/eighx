pub mod state;
pub mod window;

pub mod widgets;
pub use widgets::{Colors as ThemeColors, Styles as ThemeStyles, Theme};

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    window::launch();
  }
}
