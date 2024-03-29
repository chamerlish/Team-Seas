extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{Events, EventSettings};
use piston::input::{Button, Key, PressEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::EventLoop;
use glam;

#[derive(Clone, PartialEq)]
enum Direction {
    Right, Left, Up, Down
}
pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    fish: Fish,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            // Clear the screen.
            clear(BLUE, gl);
        });
        self.fish.render(&mut self.gl, args);
    }

    fn update(&mut self, _args: &UpdateArgs) {
        self.fish.update();
    }
    fn press(&mut self, _args: &Button) {
        let last_direction = self.fish.dir.clone();
        self.fish.dir = match _args {
            &Button::Keyboard(Key::Up) 
                if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down) 
                if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Right) 
                if last_direction != Direction::Left => Direction::Right,
            &Button::Keyboard(Key::Left) 
                if last_direction != Direction::Right => Direction::Left,
            _ => last_direction
        };
    }
}

struct Fish {
    pos: glam::Vec2,
    dir: Direction,
}

impl Fish {
    fn render(&self, gl: &mut GlGraphics, _args: &RenderArgs) {

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = graphics::rectangle::square((self.pos.x * 20.0) as f64,
                                                 (self.pos.y * 20.0) as f64,
                                                 20_f64);
        gl.draw(_args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(RED, square, transform, gl);
        });
    }
    fn update(&mut self) {
        match self.dir {
            Direction::Up => self.pos.y -= 1.0,
            Direction::Down => self.pos.y += 1.0,
            Direction::Right => self.pos.x += 1.0,
            Direction::Left => self.pos.x -= 1.0,

        }
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Snake Game", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),    
        fish : Fish {pos: glam::Vec2::new(0.0, 0.0), dir: Direction::Right},
    };

    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(k) = e.press_args() {
            app.press(&k);
        }
    }
}
