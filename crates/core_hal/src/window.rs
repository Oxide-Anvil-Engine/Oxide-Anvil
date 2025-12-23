use crate::render::RenderHAL;

pub trait WindowHAL<R: RenderHAL> {
    fn new(desc: WindowDesc) -> Self;
}

#[derive(Clone, Debug)]
pub struct WindowDesc {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    pub vsync: bool,
}

impl Default for WindowDesc {
    fn default() -> Self {
        Self {
            title: "Oxide Anvil Window".to_string(),
            width: 800,
            height: 600,
            fullscreen: false,
            vsync: true,
        }
    }
}