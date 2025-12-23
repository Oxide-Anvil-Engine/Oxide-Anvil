use crate::types::callback::UpdateCallback;
use crate::render::RenderHAL;
use crate::window::WindowDesc;

pub trait EventLoopHAL<R: RenderHAL> {
    fn new(desc: EventLoopDesc) -> Self;

    fn run(&mut self);

    fn set_render(&mut self, render: R);
}

pub struct EventLoopDesc {
    pub target_fps: Option<f64>,
    pub update_callback: Option<UpdateCallback>,
    pub window_desc: WindowDesc,
}