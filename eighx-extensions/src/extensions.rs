pub enum Capabilities {
  Lang(&'static str),
  Ui,
  Native,
  Assets,
}

pub enum Permissions {
  Mic,
  Camera,
  Files,
  Dylib,
  Networking,
}

pub struct Extension {}
