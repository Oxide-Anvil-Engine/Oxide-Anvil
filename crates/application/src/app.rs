use core_hal::event_loop::{EventLoopDesc, EventLoopHAL};
use core_hal::types::callback::{UpdateCallback};
use core_hal::window::{self};
use core_hal::window::WindowDesc;
use core_hal::render::RenderHAL;

use wgpu::render::RenderWGPU;
use winit::event_loop::EventLoopWINIT;

pub type App = Application<EventLoop, Render>;

#[cfg(feature = "winit")]
type EventLoop = EventLoopWINIT<Render>;

#[cfg(feature = "wgpu")]
type Render = RenderWGPU;

pub struct Application<E, R>
where 
    E: EventLoopHAL<R>,
    R: RenderHAL,
{
    desc: AppDesc,
    event_loop: E,
    render: Option<R>,
}

impl<E, R> Application<E, R>
where 
    E: EventLoopHAL<R>,
    R: RenderHAL,
{
    pub fn new(callback_update: UpdateCallback) -> Self {
        let desc = AppDesc::default();
        let event_loop = E::new(EventLoopDesc {
            target_fps: desc.target_fps,
            update_callback: Some(callback_update),
            window_desc: desc.window.clone(),
        });
        
        Self {
            desc,
            event_loop,
            render: None,
        }
    }

    pub fn run(&mut self) {
        let render = R::new();
        self.event_loop.set_render(render);
        self.event_loop.run();
    }

    pub fn with_window_desc(mut self, window_desc: WindowDesc) -> Self {
        self.desc.window = window_desc;
        self
    }

    pub fn with_target_fps(mut self, fps: f64) -> Self {
        self.desc.target_fps = Some(fps);
        self
    }

    pub fn with_update_callback(mut self, callback: UpdateCallback) -> Self {
        self.desc.update_callback = callback;
        self
    }
}

pub struct AppDesc {
    pub window: WindowDesc,
    pub target_fps: Option<f64>,
    pub update_callback: UpdateCallback,
}

impl Default for AppDesc {
    fn default() -> Self {
        Self {
            window: window::WindowDesc::default(),
            target_fps: None,
            update_callback: |_delta: f64| {},
        }
    }
}