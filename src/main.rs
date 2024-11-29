pub mod reducers;
pub mod renderers;

use femtovg::{renderer::OpenGl, Canvas};
use raw_window_handle::HasRawWindowHandle;
use std::num::NonZeroU32;
use std::thread;
use std::time::Duration;

use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextApi, ContextAttributesBuilder},
    display::GetGlDisplay,
    prelude::*,
    surface::{SurfaceAttributesBuilder, WindowSurface},
};

use glutin_winit::DisplayBuilder;

use crate::reducers::{GameState, PacMan, INITIAL_MAP};
use crate::renderers::render_scene;
use femtovg::Color;
use winit::event::ElementState;
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};
use winit::{event_loop::EventLoop, window::WindowBuilder};

const WINDOW_SIZE: u32 = 1124;
const PX_PER_CELL: f32 = 8.0;

fn main() {
    start(WINDOW_SIZE, WINDOW_SIZE, "Functional PacMan", false);
}

fn run(
    mut canvas: Canvas<OpenGl>,
    el: EventLoop<()>,
    context: glutin::context::PossiblyCurrentContext,
    surface: glutin::surface::Surface<WindowSurface>,
    window: Window,
) {
    let mut state = GameState {
        pacman: PacMan {
            pos: [104, 160],
            vel: [0, 0],
        },
        time: 0,
        map: INITIAL_MAP,
    };
    let mut last_key = None;

    let dpi_factor = window.scale_factor();
    let window_size = window.inner_size();
    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(1000 / 30));
        window.request_redraw();
    });

    el.run(move |event, _window, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => *control_flow = ControlFlow::Exit,
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    surface.resize(
                        &context,
                        physical_size.width.try_into().unwrap(),
                        physical_size.height.try_into().unwrap(),
                    );
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => {
                    if input.state == ElementState::Pressed {
                        last_key = input.virtual_keycode;
                    }
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                canvas.set_size(window_size.width, window_size.height, dpi_factor as f32);
                canvas.clear_rect(
                    0,
                    0,
                    window_size.width,
                    window_size.height,
                    Color::rgbf(0.2, 0.2, 0.2),
                );
                canvas.save();
                canvas.reset();
                canvas.scale(5.0, 5.0);

                state = reducers::tick(&state, last_key);
                render_scene(&mut canvas, &state);

                canvas.restore();
                canvas.flush();
                surface.swap_buffers(&context).unwrap();
            }
            _ => (),
        }
    });
}

pub fn start(width: u32, height: u32, title: &'static str, resizeable: bool) {
    let event_loop = EventLoop::new();

    let (canvas, window, context, surface) = {
        let window_builder = WindowBuilder::new()
            .with_inner_size(winit::dpi::PhysicalSize::new(width, height))
            .with_resizable(resizeable)
            .with_title(title);

        let template = ConfigTemplateBuilder::new().with_alpha_size(8);

        let display_builder = DisplayBuilder::new().with_window_builder(Some(window_builder));

        let (window, gl_config) = display_builder
            .build(&event_loop, template, |configs| {
                configs
                    .reduce(|accum, config| {
                        let transparency_check = config.supports_transparency().unwrap_or(false)
                            & !accum.supports_transparency().unwrap_or(false);

                        if transparency_check || config.num_samples() < accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            })
            .unwrap();

        let window = window.unwrap();

        let raw_window_handle = Some(window.raw_window_handle());

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

        let (width, height): (u32, u32) = window.inner_size().into();
        let raw_window_handle = window.raw_window_handle();
        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_window_handle,
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );

        let surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &attrs)
                .unwrap()
        };

        let gl_context = not_current_gl_context
            .take()
            .unwrap()
            .make_current(&surface)
            .unwrap();

        let renderer = unsafe {
            OpenGl::new_from_function_cstr(|s| gl_display.get_proc_address(s) as *const _)
        }
        .expect("Cannot create renderer");

        let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");
        canvas.set_size(width, height, window.scale_factor() as f32);

        (canvas, window, gl_context, surface)
    };

    run(canvas, event_loop, context, surface, window);
}
