extern crate rand;
#[macro_use]
extern crate ecs;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

use ecs::{World, System, Process, DataHelper, BuildData};
use ecs::system::{EntitySystem, EntityProcess};
use ecs::entity::EntityIter;

use rand::Rng;

/////////
// ecs //
/////////

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}


components! {
    struct MyComponents {
        #[hot] pos: Position,
        #[hot] vel: Velocity,
    }
}

// Process
pub struct MotionProcess;

impl System for MotionProcess {
    type Components = MyComponents;
    type Services = ();
}

impl EntityProcess for MotionProcess {
    fn process(&mut self, entities: EntityIter<MyComponents>, components: &mut DataHelper<MyComponents, ()>) {
        for e in entities {
            components.pos[e].x += components.vel[e].dx;
            components.pos[e].y += components.vel[e].dy;
            println!("Process entity");
        }
    }
}

// System
pub struct Particles;

impl System for Particles {
    type Components = MyComponents;
    type Services = ();
}

impl Process for Particles {
    fn process(&mut self, _: &mut DataHelper<MyComponents, ()>) {
        println!("Processing particles?");
    }
}

systems! {
    struct MySystems<MyComponents, ()> {
        active: {
            particles: Particles = Particles,
            motion: EntitySystem<MotionProcess> = EntitySystem::new( MotionProcess, aspect!(<MyComponents> all: [pos, vel]) ),
        },
        passive: {}
    }
}



fn main() {

    let opengl = OpenGL::V3_2;
    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "spinning-square",
            [200, 200]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new(opengl);
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64,   // Rotation for the square.
    world: World<MySystems>,
}

impl App {
    fn new(opengl: OpenGL) -> App {
        let mut world = World::<MySystems>::new();
        let mut rng = rand::thread_rng();
        for _ in 0..10 {
            let vel = Velocity { dx: rng.gen(), dy: rng.gen() };
            world.create_entity(
                |entity: BuildData<MyComponents>, components: &mut MyComponents| {

                    components.pos.add(&entity, Position { x: 0.0, y: 0.0 });
                    components.vel.add(&entity, vel);
                }
            );
        }
        App {
            gl: GlGraphics::new(opengl),
            rotation: 0.0,
            world: world,
        }
    }
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = ((args.width / 2) as f64,
                      (args.height / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform.trans(x, y)
                                       .rot_rad(rotation)
                                       .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;
        self.world.update();
    }
}
