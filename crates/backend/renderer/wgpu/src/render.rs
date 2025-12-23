use core_hal::render::RenderHAL;
use wgpu::{Device, Instance, Queue, Surface, SurfaceConfiguration};
use wgpu::*;
use raw_window_handle::{HasWindowHandle, HasDisplayHandle};

pub struct RenderWGPU {
    instance: Instance,
    surface: Option<Surface<'static>>,
    device: Option<Device>,
    queue: Option<Queue>,
    config: Option<SurfaceConfiguration>,
    size: (u32, u32),
}

impl RenderHAL for RenderWGPU {
    fn new() -> Self {
        Self {
            instance: Instance::new(&InstanceDescriptor {
                backends: Backends::all(),
                ..Default::default()
            }),
            surface: None,
            device: None,
            queue: None,
            config: None,
            size: (0, 0),
        }
    }

    fn init_surface<W: HasWindowHandle, D: HasDisplayHandle>(&mut self, window_handle: &W, display_handle: &D) {
        // SAFETY: A janela deve ser válida e sobreviver à Surface, o que é garantido pela sua arquitetura.
        self.surface = Some(unsafe { 
            self.instance.create_surface_unsafe(
                SurfaceTargetUnsafe::RawHandle {
                    raw_display_handle: display_handle.display_handle().expect("No display handle").into(),
                    raw_window_handle: window_handle.window_handle().expect("No window handle").into(),
                }
            )
        }.expect("Falha ao criar surface"));

        // pollster::block_on é usado para chamar código async em main síncrona
        pollster::block_on(self.setup_async())
    }

    fn destroy_surface(&mut self) {
        // Limpeza de recursos, se necessário
        self.surface = None;
        self.device = None;
        self.queue = None;
        self.config = None;
    }

    fn render(&mut self) {
        // 1. Obter a próxima textura de superfície a ser renderizada.
        let surface = self.surface.as_ref().expect("Surface not set");
        let device = self.device.as_ref().expect("Device not set");
        let queue = self.queue.as_ref().expect("Queue not set");
        let config = self.config.as_ref().expect("Config not set");

        let frame = match surface.get_current_texture() {
            Ok(frame) => frame,
            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                // Se a superfície foi perdida ou está desatualizada, reconfiguramos.
                // O evento Resized deveria fazer isso, mas é bom ter uma fallback aqui.
                self.resize(self.size);
                return;
            }
            Err(wgpu::SurfaceError::Timeout) => {
                log::warn!("Surface texture timed out!");
                return;
            }
            Err(e) => panic!("Failed to get current surface texture: {:?}", e),
        };
        
        // 2. Criar a view da textura e o Command Encoder.
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        // 3. Iniciar o Render Pass (definir a cor de fundo).
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1, // Cor de fundo: Cinza claro/Azulado
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            // AQUI você adicionaria chamadas de desenho futuras:
            // render_pass.set_pipeline(...);
            // render_pass.set_bind_group(...);
            // render_pass.set_vertex_buffer(...);
            // render_pass.draw(0..num_vertices, 0..1); 
        } // O render_pass termina aqui (fim do escopo)

        // 4. Enviar o buffer de comandos para a GPU.
        queue.submit(std::iter::once(encoder.finish()));
        
        // 5. Apresentar o frame finalizado.
        frame.present();
    }

    fn resize(&mut self, size: (u32, u32)) {
        self.size = size;
        if let (Some(surface), Some(device)) = (&self.surface, &self.device) {
            let config = self.config.as_mut().unwrap();
            (config.width, config.height) = size;
            surface.configure(device, &config);
        }
    }
}

// Lógica Async do WGPU (padrão em 2025)
impl RenderWGPU {
    async fn setup_async(&mut self) {
        let adapter = self.instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: self.surface.as_ref(),
                force_fallback_adapter: false,
            })
            .await
            .expect("Falha ao encontrar adapter compatível");

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor::default())
            .await
            .expect("Falha ao criar device e queue");

        let caps = self.surface.as_ref().unwrap().get_capabilities(&adapter);
        let format = caps.formats[0]; // Usar o primeiro formato suportado

        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: format,
            width: self.size.0,
            height: self.size.1,
            present_mode: PresentMode::Fifo,
            alpha_mode: CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        self.device = Some(device);
        self.queue = Some(queue);
        self.config = Some(config);
    }
}