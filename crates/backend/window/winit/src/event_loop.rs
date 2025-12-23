use core_hal::{event_loop::EventLoopHAL, render::RenderHAL};
use winit::{application::ApplicationHandler, event_loop::ActiveEventLoop};
use winit::event::{WindowEvent};
use winit::window::{Window, WindowId};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use core_hal::event_loop::EventLoopDesc;
use winit::event_loop::EventLoop;
use winit::dpi::PhysicalSize;

pub struct EventLoopWINIT<R>
where
    R: RenderHAL,
{
    desc: EventLoopDesc,
    event_loop: Option<EventLoop<()>>,
    window: Option<Window>,
    render: Option<R>,
}

impl<R: RenderHAL> EventLoopHAL<R> for EventLoopWINIT<R> {
    fn new(desc: EventLoopDesc) -> Self {
        let event_loop = match EventLoop::new() {
            Ok(el) => el,
            Err(e) => panic!("Failed to create event loop: {}", e),
        };
        Self {
            desc,
            event_loop: Some(event_loop),
            window: None,
            render: None,
        }
    }

    fn run(&mut self) {
        let _ = self.event_loop.take().unwrap().run_app(self);
    }

    fn set_render(&mut self, render: R) {
        self.render = Some(render);
    }
}

impl<R: RenderHAL> ApplicationHandler for EventLoopWINIT<R> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(event_loop.create_window(Window::default_attributes()).unwrap());
        if let Some(window) = &self.window {
            window.set_title(&self.desc.window_desc.title);
            let _ = window.request_inner_size(PhysicalSize::new(self.desc.window_desc.width, self.desc.window_desc.height));
            let window_handle = window.window_handle().expect("Failed to get window handle");
            let display_handle = window.display_handle().expect("Failed to get display handle");
            if let Some(render) = &mut self.render {
                render.init_surface(&window_handle, &display_handle);
            } else {
                panic!("Render not set");
            }
        } else {
            panic!("Failed to create window");
        }
    }
    
    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::Resized(physical_size) => {
                // ESTE É O LUGAR CORRETO
                if physical_size.width > 0 && physical_size.height > 0 {
                    // Chame o método resize na sua RenderHAL aqui
                    if let Some(render) = &mut self.render {
                        render.resize((physical_size.width, physical_size.height));
                    } else {
                        panic!("Render not set");
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(render) = &mut self.render {
                    render.render();
                } else {
                    panic!("Render not set");
                }
            }
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(callback) = &mut self.desc.update_callback {
            callback(0.0);
        }
        self.window.as_ref().unwrap().request_redraw();
    }
}