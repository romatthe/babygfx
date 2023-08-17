use sdl2::{self, video::{Window}, event::Event, keyboard::Keycode, render::Canvas, pixels::Color, Sdl};

fn main() {
    // Setup
    let (context, mut canvas) = setup();
    let mut event_pump = context.event_pump().unwrap();

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
        render(&mut canvas)
    }
}

fn setup() -> (Sdl, Canvas<Window>) {
    // Initialize the SDL context and video subsystem
    let sdl_context = sdl2::init().unwrap_or_else(|err| panic!("Failed to initialize SDL: {}", err));
    let video_subsystem = sdl_context.video().unwrap_or_else(|err| panic!("SDL video subsystem failed to initialize: {}", err));

    // Create an SDL window
    let window = video_subsystem.window("BabyGFX", 800, 600)
        .position_centered()
        // .borderless()
        .build()
        .unwrap_or_else(|err| panic!("Error creating an SDL window: {}", err));

    // Create a canvas with a renderer
    let mut canvas = window.into_canvas()    
        .build()
        .unwrap();

    (sdl_context, canvas)
}

fn process_input(event: Event) {

}

fn render(canvas: &mut Canvas<Window>) {
    // Draw something to the Window and present it
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.clear();
    canvas.present();
}
