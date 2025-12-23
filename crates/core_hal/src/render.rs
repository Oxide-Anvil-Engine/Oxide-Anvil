use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

pub trait RenderHAL {
    fn new() -> Self;

    fn init_surface<W: HasWindowHandle, D: HasDisplayHandle>(&mut self, window_handle: &W, display_handle: &D);

    fn destroy_surface(&mut self);

    fn render(&mut self);

    fn resize(&mut self, size: (u32, u32));
}

