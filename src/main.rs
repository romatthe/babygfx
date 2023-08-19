use sdl2::{self, video::{Window, WindowContext}, event::Event, keyboard::Keycode, render::{Canvas, Texture, TextureAccess, TextureCreator}, pixels::{Color, PixelFormat, PixelFormatEnum}, Sdl};

const WIN_WIDTH: u32 = 800;
const WIN_HEIGHT: u32 = 600;

struct PixelBuffer {
    // buffer: [[u32; WIN_HEIGHT as usize]; WIN_WIDTH as usize],
    buffer: [u32; (WIN_WIDTH * WIN_HEIGHT) as usize],
}

impl PixelBuffer {
    pub fn new() -> Self {
        PixelBuffer {
            buffer: [0; (WIN_WIDTH * WIN_HEIGHT) as usize],
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0);
    }

    pub fn fill(&mut self, color: u32) {
        self.buffer.fill(color);
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if (x < WIN_WIDTH as usize) && (y < WIN_HEIGHT as usize) {
            // self.buffer[x][y] = color;
            self.buffer[(WIN_WIDTH as usize) * y + x] = color;
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
            .create_texture(PixelFormatEnum::ARGB8888, TextureAccess::Streaming, WIN_WIDTH, WIN_HEIGHT)
            .unwrap();

        // Renderer { pixbuf: PixelBuffer::default(), canvas, texture_creator: creator, texture }
        Renderer { pixbuf: PixelBuffer::default(), canvas, texture }
    }

    pub fn clear_canvas(&mut self, color: u32) {
        self.canvas.set_draw_color(Color::from_u32(&PixelFormat::try_from(PixelFormatEnum::ARGB8888).unwrap(), color));
        self.canvas.clear();
    }

    
    pub fn render(&mut self) {
        // Clear the renderer
        self.clear_canvas(0x00000000);

        // Update the texture with the pixel buffer data and copy it to the canvas renderer
        let data = self.pixbuf.buffer.iter().flat_map(|b| b.to_le_bytes()).collect::<Vec<u8>>();
        self.texture.update(None, data.as_slice(), (WIN_WIDTH * 4) as usize).unwrap();
        self.canvas.copy(&self.texture, None, None).unwrap();

        // Clear the pixel buffer
        self.clear_buffer(0xFFFFFF00);

        // Present the renderer
        self.canvas.present();
    }
    
    fn clear_buffer(&mut self, color: u32) {
        self.pixbuf.fill(color);
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
        renderer.render();
    }
}

fn setup() -> (Sdl, Canvas<Window>) {
    // Initialize the SDL context and video subsystem
    let sdl_context = sdl2::init().unwrap_or_else(|err| panic!("Failed to initialize SDL: {}", err));
    let video_subsystem = sdl_context.video().unwrap_or_else(|err| panic!("SDL video subsystem failed to initialize: {}", err));

    // Create an SDL window
    let window = video_subsystem.window("BabyGFX", WIN_WIDTH, WIN_HEIGHT)
        .position_centered()
        // .borderless()
        .build()
        .unwrap_or_else(|err| panic!("Error creating an SDL window: {}", err));

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
