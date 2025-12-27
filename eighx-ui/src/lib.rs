pub use eighx_ui_macros::renderable;

pub mod simple;
pub mod state;
pub mod theme;

pub mod widgets;
pub use theme::*;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    simple::render();
  }
}
