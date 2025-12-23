use core_hal::types::callback::{UpdateCallback};
use core_hal::window::{WindowHAL};
use core_hal::window::WindowDesc;
use core_hal::render::RenderHAL;
use winit::event_loop::{EventLoop};
use winit::window::{Window};

pub struct WindowWINIT<R: RenderHAL> {
    pub desc: WindowDesc,
    pub update_callback: Option<UpdateCallback>,
    pub window: Option<Window>,
    pub event_loop: Option<EventLoop<()>>,
    pub render: Option<R>,
}

impl<R: RenderHAL> WindowHAL<R> for WindowWINIT<R> {
    fn new(desc: WindowDesc) -> Self {
        let event_loop = EventLoop::new().expect("Failed to create event loop");
        Self {
            desc,
            update_callback: None,
            window: None,
            event_loop: Some(event_loop),
            render: None,
        }
    }
}