use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;

use std::num::NonZeroU32;
use std::ops::Deref;

use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoopBuilder;
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::{Window, WindowBuilder};

use raw_window_handle::HasRawWindowHandle;

use glutin::config::{Config, ConfigTemplateBuilder};
use glutin::context::{ContextApi, ContextAttributesBuilder};
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::{Surface, SurfaceAttributesBuilder, SwapInterval, WindowSurface};

use glutin_winit::{self, DisplayBuilder};
use lazy_static::lazy_static;

pub mod gl {
    #![allow(clippy::all)]
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

fn get_fragment_shader() -> Vec<u8> {
    let mut fragment_file = File::open("src/shaders/fragment.glsl").unwrap();

    let mut fragment_content = Vec::new();

    fragment_file.read_to_end(&mut fragment_content).unwrap();

    fragment_content.push(0);

    return fragment_content;
}

fn get_vertex_shaders() -> Vec<u8> {
    let mut vertex_file = File::open("src/shaders/vertex.glsl").unwrap();

    let mut vertex_contents = Vec::new();

    vertex_file.read_to_end(&mut vertex_contents).unwrap();

    vertex_contents.push(0);

    return vertex_contents;
}

lazy_static! {
    static ref VERTEX_SHADER: Vec<u8> = get_vertex_shaders();
    static ref FRAGMENT_SHADER: Vec<u8> = get_fragment_shader();
}

pub fn init<'a>(scene: &'a crate::scene::Scene) -> () {
    let mut event_loop = EventLoopBuilder::new().build();

    let window_builder = Some(
        WindowBuilder::new()
            .with_title("Biopix")
            .with_transparent(false),
    );

    let template = ConfigTemplateBuilder::new();

    let display_builder = DisplayBuilder::new().with_window_builder(window_builder);

    let (mut window, gl_config) = display_builder
        .build(&event_loop, template, |configs| {
            configs
                .reduce(|accum, config| {
                    let transparency_check = config.supports_transparency().unwrap_or(false)
                        & !accum.supports_transparency().unwrap_or(false);

                    if transparency_check || config.num_samples() > accum.num_samples() {
                        config
                    } else {
                        accum
                    }
                })
                .unwrap()
        })
        .unwrap();

    let raw_window_handle = window.as_ref().map(|window| window.raw_window_handle());

    let gl_display = gl_config.display();

    let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);

    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(raw_window_handle);

    let mut not_current_gl_context = Some(unsafe {
        gl_display
            .create_context(&gl_config, &context_attributes)
            .unwrap_or_else(|_| {
                gl_display
                    .create_context(&gl_config, &fallback_context_attributes)
                    .expect("failed to create context")
            })
    });

    let mut state = None;
    let mut renderer = None;
    let mut mouse_hold = false;

    let mut prev_x = 0.0;
    let mut prev_y = 0.0;

    let mut x_diff = 0.0;
    let mut y_diff = 0.0;

    let event_loop_closure = {
        move |event: Event<()>,
              window_target: &winit::event_loop::EventLoopWindowTarget<()>,
              control_flow: &mut winit::event_loop::ControlFlow| {
            control_flow.set_wait();
            match event {
                Event::Resumed => {
                    let window = window.take().unwrap_or_else(|| {
                        let window_builder = WindowBuilder::new().with_transparent(true);
                        glutin_winit::finalize_window(window_target, window_builder, &gl_config)
                            .unwrap()
                    });

                    let gl_window = GlWindow::new(window, &gl_config);

                    let gl_context = not_current_gl_context
                        .take()
                        .unwrap()
                        .make_current(&gl_window.surface)
                        .unwrap();

                    renderer.get_or_insert_with(|| Renderer::new(&gl_display, scene));

                    if let Err(res) = gl_window.surface.set_swap_interval(
                        &gl_context,
                        SwapInterval::Wait(NonZeroU32::new(1).unwrap()),
                    ) {
                        eprintln!("Error setting vsync: {:?}", res);
                    }

                    assert!(state.replace((gl_context, gl_window)).is_none());
                }
                Event::Suspended => {
                    let (gl_context, _) = state.take().unwrap();
                    assert!(not_current_gl_context
                        .replace(gl_context.make_not_current().unwrap())
                        .is_none());
                }
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(size) => {
                        if size.width != 0 && size.height != 0 {
                            if let Some((gl_context, gl_window)) = &state {
                                gl_window.surface.resize(
                                    gl_context,
                                    NonZeroU32::new(size.width).unwrap(),
                                    NonZeroU32::new(size.height).unwrap(),
                                );
                                let renderer = renderer.as_ref().unwrap();
                                renderer.resize(size.width as i32, size.height as i32);
                            }
                        }
                    }
                    WindowEvent::CloseRequested => {
                        control_flow.set_exit();
                    }
                    WindowEvent::MouseWheel { delta, .. } => match delta {
                        winit::event::MouseScrollDelta::LineDelta(_, dirn) => {
                            if dirn < 0.0 {
                                let current_scale = renderer.as_ref().unwrap().scale;
                                renderer.as_mut().unwrap().scale = current_scale - 0.002;
                            } else {
                                let current_scale = renderer.as_ref().unwrap().scale;
                                renderer.as_mut().unwrap().scale = current_scale + 0.002;
                            }
                        }
                        _ => {}
                    },
                    WindowEvent::CursorMoved { position, .. } => {
                        if mouse_hold {
                            x_diff += prev_x - position.x;

                            renderer.as_mut().unwrap().x_rotate = Some(x_diff as f32 / 200.0);

                            y_diff += prev_y - position.y;

                            renderer.as_mut().unwrap().y_rotate = Some(y_diff as f32 / 200.0);
                        }
                        prev_x = position.x;
                        prev_y = position.y;
                    }

                    WindowEvent::MouseInput { state, .. } => {
                        mouse_hold = match state {
                            winit::event::ElementState::Pressed => true,
                            winit::event::ElementState::Released => false,
                        }
                    }
                    _ => (),
                },
                Event::RedrawEventsCleared => {
                    if let Some((gl_context, gl_window)) = &state {
                        let renderer = renderer.as_mut().unwrap();
                        renderer.draw();
                        gl_window.window.request_redraw();

                        gl_window.surface.swap_buffers(gl_context).unwrap();
                    }
                }
                _ => (),
            }
        }
    };

    event_loop.run_return(event_loop_closure);
}

pub struct GlWindow {
    pub surface: Surface<WindowSurface>,
    pub window: Window,
}

impl GlWindow {
    pub fn new(window: Window, config: &Config) -> Self {
        let (width, height): (u32, u32) = window.inner_size().into();
        let raw_window_handle = window.raw_window_handle();
        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_window_handle,
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );

        let surface = unsafe {
            config
                .display()
                .create_window_surface(config, &attrs)
                .unwrap()
        };

        Self { window, surface }
    }
}

pub struct Renderer<'a> {
    pub vao: gl::types::GLuint,
    pub vbo: gl::types::GLuint,
    pub ibo: gl::types::GLuint,
    pub program: Option<gl::types::GLuint>,
    pub gl: gl::Gl,
    pub scale: f32,
    pub x_rotate: Option<f32>,
    pub y_rotate: Option<f32>,
    pub scene: &'a crate::scene::Scene,
}

impl<'a> Renderer<'a> {
    pub fn new<D>(gl_display: &D, scene: &'a crate::scene::Scene) -> Self
    where
        D: GlDisplay,
    {
        unsafe {
            let gl = gl::Gl::load_with(|symbol| {
                let symbol = CString::new(symbol).unwrap();
                gl_display.get_proc_address(symbol.as_c_str()).cast()
            });

            if let Some(renderer) = get_gl_string(&gl, gl::RENDERER) {
                println!("Running on {}", renderer.to_string_lossy());
            }
            if let Some(version) = get_gl_string(&gl, gl::VERSION) {
                println!("OpenGL Version {}", version.to_string_lossy());
            }

            if let Some(shaders_version) = get_gl_string(&gl, gl::SHADING_LANGUAGE_VERSION) {
                println!("Shaders version on {}", shaders_version.to_string_lossy());
            }

            Self {
                vao: std::mem::zeroed(),
                vbo: std::mem::zeroed(),
                ibo: std::mem::zeroed(),
                program: None,
                gl,
                scale: 0.1,
                x_rotate: None,
                y_rotate: None,
                scene,
            }
        }
    }

    pub fn draw(&mut self) {
        unsafe {
            let vertex_shader = create_shader(&self.gl, gl::VERTEX_SHADER, &VERTEX_SHADER);
            let fragment_shader = create_shader(&self.gl, gl::FRAGMENT_SHADER, &FRAGMENT_SHADER);

            self.program = Some(self.gl.CreateProgram());

            self.gl.AttachShader(self.program.unwrap(), vertex_shader);

            self.gl.AttachShader(self.program.unwrap(), fragment_shader);

            self.gl.LinkProgram(self.program.unwrap());

            self.gl.UseProgram(self.program.unwrap());

            self.gl.GenVertexArrays(1, &mut self.vao);
            self.gl.BindVertexArray(self.vao);

            self.gl.GenBuffers(1, &mut self.vbo);
            self.gl.BindBuffer(gl::ARRAY_BUFFER, self.vbo);

            self.gl.GenBuffers(1, &mut self.ibo);
            self.gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ibo);

            self.gl.Enable(gl::DEPTH_TEST);
            self.gl.DepthFunc(gl::LESS);

            self.gl.Clear(gl::COLOR_BUFFER_BIT);
            self.gl.Clear(gl::DEPTH_BUFFER_BIT);
            self.gl.ClearColor(0.1, 0.1, 0.1, 1.0);

            self.scene.render(self);
        }
    }

    pub fn resize(&self, width: i32, height: i32) {
        unsafe {
            self.gl.Viewport(0, 0, width, height);
        }
    }

    pub fn scale(&self, width: i32, height: i32) {
        unsafe {
            self.gl.Viewport(0, 0, width, height);
        }
    }
}

impl Deref for Renderer<'_> {
    type Target = gl::Gl;

    fn deref(&self) -> &Self::Target {
        &self.gl
    }
}

impl Drop for Renderer<'_> {
    fn drop(&mut self) {
        unsafe {
            if let Some(program) = self.program {
                self.gl.DeleteProgram(program);
            }
            self.gl.DeleteBuffers(1, &self.vbo);
            self.gl.DeleteBuffers(1, &self.ibo);
            self.gl.DeleteVertexArrays(1, &self.vao);
        }
    }
}

fn get_gl_string(gl: &gl::Gl, variant: gl::types::GLenum) -> Option<&'static CStr> {
    unsafe {
        let s = gl.GetString(variant);
        (!s.is_null()).then(|| CStr::from_ptr(s.cast()))
    }
}
pub unsafe fn create_shader(
    gl: &gl::Gl,
    shader: gl::types::GLenum,
    source: &[u8],
) -> gl::types::GLuint {
    let shader = gl.CreateShader(shader);
    gl.ShaderSource(
        shader,
        1,
        [source.as_ptr().cast()].as_ptr(),
        std::ptr::null(),
    );
    gl.CompileShader(shader);
    shader
}
