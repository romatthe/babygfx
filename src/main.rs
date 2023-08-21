use once_cell::sync::OnceCell;
use sdl2::{self, Sdl};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormat, PixelFormatEnum};
use sdl2::render::{Canvas, Texture, TextureAccess, TextureCreator};
use sdl2::video::{Window, WindowContext};

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
            // self.buffer[x][y] = color;
            self.buffer[(*WIN_WIDTH.get().unwrap() as usize) * y + x] = color;
        }
    }
}

impl Default for PixelBuffer {
    fn default() -> Self {
        Self::new()
    }
}

struct Renderer<'a> {
    pixbuf: PixelBuffer,
    canvas: Canvas<Window>,
    // texture_creator: &'a TextureCreator<WindowContext>,
    texture: Texture<'a>,
}

impl <'a> Renderer<'a> {
    pub fn new(canvas: Canvas<Window>, creator: &'a TextureCreator<WindowContext>) -> Self {
        let texture = creator
            .create_texture(PixelFormatEnum::ARGB8888, TextureAccess::Streaming, *WIN_WIDTH.get().unwrap(), *WIN_HEIGHT.get().unwrap())
            .unwrap();

        Renderer { pixbuf: PixelBuffer::default(), canvas, texture }
    }

    pub fn clear_canvas(&mut self, color: u32) {
        self.canvas.set_draw_color(Color::from_u32(&PixelFormat::try_from(PixelFormatEnum::ARGB8888).unwrap(), color));
        self.canvas.clear();
    }
    
    pub fn render(&mut self) {
        // Clear the renderer
        self.clear_canvas(0x00000000);

        // We use bytemuck to cast &[u32] to &[u8] here.
        self.texture.update(None, bytemuck::cast_slice(&self.pixbuf.buffer), (WIN_WIDTH.get().unwrap() * 4) as usize).unwrap();
        self.canvas.copy(&self.texture, None, None).unwrap();

        // Clear the pixel buffer
        self.clear_buffer(0xFF000000);

        // Present the renderer
        self.canvas.present();
    }
    
    fn clear_buffer(&mut self, color: u32) {
        self.pixbuf.fill(color);
    }

    fn draw_grid(&mut self, multiple: usize) {
        for x in (0..*WIN_WIDTH.get().unwrap()).step_by(multiple as usize) {
            for y in (0..*WIN_HEIGHT.get().unwrap()).step_by(multiple as usize) {
                self.pixbuf.set_pixel(x as usize, y as usize, 0xFF333333);
            }
        }
    }

    fn draw_rect(&mut self, x: u32, y: u32, width: u32, height: u32, color: u32) {
        for i in x..(x + width) {
            for j in y..(y + height) {
                self.pixbuf.set_pixel(i as usize, j as usize, color);
            }
        }
    }
}

fn main() {
    // Setup
    let (context, canvas) = setup();
    let creator = canvas.texture_creator();
    let mut event_pump = context.event_pump().unwrap();

    let mut renderer = Renderer::new(canvas, &creator);

    // Main "game" loop
    'running: loop {
        // Process input
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { .. } => {
                    process_input(event)
                },
                _ => {
                    // What else?
                }
            }
        }
        // Update

        // Render
        // Draw the nice little grid
        renderer.draw_grid(10);        
        renderer.draw_rect(200, 300, 400, 400, 0x00FFC0CB);
        renderer.render();
    }
}

fn setup() -> (Sdl, Canvas<Window>) {
    // Initialize the SDL context and video subsystem
    let sdl_context = sdl2::init().unwrap_or_else(|err| panic!("Failed to initialize SDL: {}", err));
    let video_subsystem = sdl_context.video().unwrap_or_else(|err| panic!("SDL video subsystem failed to initialize: {}", err));

    // Get the main monitor dimensions and set the global variables
    let display_mode = video_subsystem.current_display_mode(0).expect("SDL video subsytem failed to query monitor 0 display mode.");
    WIN_WIDTH.set(display_mode.w as u32).unwrap();
    WIN_HEIGHT.set(display_mode.h as u32).unwrap();

    // Create an SDL window
    let mut window = video_subsystem.window("BabyGFX", *WIN_WIDTH.get().unwrap(), *WIN_HEIGHT.get().unwrap())
        .position_centered()
        .build()
        .unwrap_or_else(|err| panic!("Error creating an SDL window: {}", err));

    window.set_fullscreen(sdl2::video::FullscreenType::Desktop).unwrap();

    // Create a canvas with a renderer
    let canvas = window.into_canvas()
        .build()
        .unwrap();

    (sdl_context, canvas)
}

fn process_input(event: Event) {
    match event {
        _ => { }
    }
}
