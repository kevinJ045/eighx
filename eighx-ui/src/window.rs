use crate::widgets::*;

pub fn launch() -> anyhow::Result<()> {
  let app = AppWindow::new()?;
  crate::state::init_state(&app);

  app.run()?;

  Ok(())
}
