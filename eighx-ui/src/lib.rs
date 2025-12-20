pub mod simple;
mod theme;

use theme::Theme;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    simple::render();
  }
}
