use std::{rc::Rc, num::NonZeroU32};
use once_cell::sync::OnceCell;
use softbuffer::Surface;
use winit::{
    event::{Event, WindowEvent, KeyEvent}, keyboard::{Key, NamedKey},
    event_loop::{EventLoop, ControlFlow}, 
    window::{WindowBuilder, Window},
};

static WIN_WIDTH: OnceCell<u32> = OnceCell::new();
static WIN_HEIGHT: OnceCell<u32> = OnceCell::new();


struct PixelBuffer {
    buffer: Vec<u32>,
}

impl PixelBuffer {
    pub fn new() -> Self {
        PixelBuffer {
            buffer: vec![0; (WIN_WIDTH.get().unwrap()  * WIN_HEIGHT.get().unwrap()) as usize],
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0);
    }

    pub fn fill(&mut self, color: u32) {
        self.buffer.fill(color);
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if (x < *WIN_WIDTH.get().unwrap() as usize) && (y < *WIN_HEIGHT.get().unwrap() as usize) {
            self.buffer[(*WIN_WIDTH.get().unwrap() as usize) * y + x] = color;
        }
    }
}

impl Default for PixelBuffer {
    fn default() -> Self {
        Self::new()
    }
}

struct Renderer {
    surface: Surface<Rc<Window>, Rc<Window>>,
    buffer: PixelBuffer,
}

impl Renderer {
    pub fn new(window: Rc<Window>) -> Self {
        let buffer = PixelBuffer::new(); 

        let context = softbuffer::Context::new(window.clone()).unwrap();
        let size = window.inner_size();
        let mut surface = softbuffer::Surface::new(&context, window.clone()).unwrap();
        surface.resize(NonZeroU32::new(size.width).unwrap(), NonZeroU32::new(size.height).unwrap()).unwrap();

        Renderer { surface, buffer }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    fn draw_grid(&mut self, multiple: usize) {
        for x in (0..*WIN_WIDTH.get().unwrap()).step_by(multiple as usize) {
            for y in (0..*WIN_HEIGHT.get().unwrap()).step_by(multiple as usize) {
                self.buffer.set_pixel(x as usize, y as usize, 0xFF333333);
            }
        }
    }

    fn draw_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: u32) {
        for i in x..(x + width) {
            for j in y..(y + height) {
                self.buffer.set_pixel(i as usize, j as usize, color);
            }
        }
    }

    pub fn render(&mut self) {
        if let Ok(mut buffer) = self.surface.buffer_mut() {
            buffer.clone_from_slice(self.buffer.buffer.as_slice());
            buffer.present().unwrap();
        }

        // Clear the pixel buffer
        self.clear();
    }
}

fn main() {
    WIN_WIDTH.set(1280).unwrap();
    WIN_HEIGHT.set(720).unwrap();

    let event_loop = EventLoop::new().unwrap();
    let window = Rc::new(WindowBuilder::new()
        .with_inner_size(winit::dpi::PhysicalSize::new(*WIN_WIDTH.get().unwrap(), *WIN_HEIGHT.get().unwrap()))
        .build(&event_loop).unwrap()
    );

    let mut renderer = Renderer::new(window.clone());

    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);

        match event {
            Event::WindowEvent { event: WindowEvent::CloseRequested 
                | WindowEvent::KeyboardInput { 
                    event: KeyEvent { logical_key: Key::Named(NamedKey::Escape), .. }, ..
                }, window_id,
            } if window_id == window.id() => {
                // Close the window
                println!("The close button was pressed; stopping");
                elwt.exit();
            }
            Event::AboutToWait => {
                // Update and redraw here
                renderer.draw_grid(10);        
                renderer.draw_rect(200, 300, 400, 400, 0x00FFC0CB);
                renderer.render();
            },
            _ => {
                // What else?
            }
        };
    }).unwrap();
}