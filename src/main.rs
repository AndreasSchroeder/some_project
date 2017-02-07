// Main Module
extern crate piston_window;
extern crate find_folder;
extern crate gfx_device_gl;
extern crate gfx_graphics;
extern crate gfx;
extern crate vecmath;
extern crate image as im;
extern crate time;
extern crate rand;
extern crate xml;

use piston_window::*;
use time::PreciseTime;

// modules
mod player;
mod io;
mod level;
mod inventory;
mod item;
mod actor;
mod field;
mod interactable;
mod coord;
mod camera;
mod bot;
mod renderable;
mod effect;

// own uses
use camera::Cam;
use player::{Player, LastKey, Direction};
use bot::Bot;
use io::render_tile;
use io::tileset::Tileset;
use io::xml::load_xml;
use level::Level;
use effect::{EffectHandler, EffectOption};
use interactable::InteractableType;
use renderable::Renderable;
use io::all_sprites::SpriteMap;

//EINGABEN
const TWO_PLAYER: bool = true;
const SIZE_PER_TILE: u64 = 64;
const BORDER_BETWEEN_TILES: u64 = 1;
const CAMERA_BUF_X: u64 = 4;
const CAMERA_BUF_Y: u64 = 4;
const WIDTH: i64 = 584;
const HEIGHT: i64 = 584;
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

/// struct for Settings
pub struct Settings {
    pub sprite_map: SpriteMap,
}

impl Settings {
    /// Constructor
    pub fn new(mut w: &mut PistonWindow) -> Self {
        let sprite_map = SpriteMap::init(w);
        Settings { sprite_map: sprite_map }
    }
}

/// Struct for the app
/// player_one: the first Player
/// Player_two: teh seconde Player if available
/// bots: Vector of all bots
/// cam: the Camera
pub struct App<'a> {
    player_one: Player<'a>,
    player_two: Option<Player<'a>>,
    bots: Vec<Bot<'a>>,
    cam: Cam,
}

impl<'a> App<'a> {
    /// Constructor
    fn new(mut players: Vec<Player<'a>>, mut bots: Vec<Bot<'a>>) -> Self {
        let mut p1 = match players.pop() {
            Some(p) => p,
            None => panic!("No player found!"),
        };

        let mut p2 = None;

        match players.pop() {
            Some(p) => {
                p2 = Some(p1);
                p1 = p;
            },
            None => (),
        };

        App {
            player_one: p1,
            player_two: p2,
            bots: bots,
            cam: Cam::new(CAMERA_BUF_X, CAMERA_BUF_Y),
        }
    }

    /// Handle the rendering of all Objects
    fn on_draw(&mut self,
               mut w: &mut PistonWindow,
               e: &Event,
               tileset: &Tileset,
               level: &mut Level,
               state: usize,
               effects: &EffectHandler) {
        // Range of the camera
        let range = self.cam.get_range();

        // draw in 2D
        w.draw_2d(e, |c, gl| {
            let player_one = &self.player_one;
            let player_two = &self.player_two;

            // Clear the screen.
            clear(BLACK, gl);

            let center_lv = c.transform.trans(0.0, 0.0);

            // render all Tiles if in Camera range
            for (h, j) in (range.x_min..range.x_max).enumerate() {
                for (w, i) in (range.y_min..range.y_max).enumerate() {
                    // get tile
                    let tile = match tileset.get_texture(level.get_data()[i as usize][j as usize].get_id()) {
                    Some(x) => x,
                    None => panic!("No texture found."),
                    };
                    // render tile
                    render_tile(&tile, gl, center_lv,  h as u32 * tileset.get_tile_height(),
                            w as u32 * tileset.get_tile_width(),
                            w as u32,
                            h as u32);
                }
            }
            // position of Player one in Pixel coordinates
            let center_p1 = c.transform.trans(((self.player_one.coord.get_x() - range.x_min )* (SIZE_PER_TILE+ BORDER_BETWEEN_TILES)) as f64,
                                          ((self.player_one.coord.get_y() - range.y_min)*  (SIZE_PER_TILE+ BORDER_BETWEEN_TILES)) as f64);

            // render player one
            player_one.render(gl, center_p1);

            // Render Player two
            if let Some(ref p2) = *player_two {


                let center_p2 = c.transform.trans(((p2.coord.get_x() - range.x_min) *  (SIZE_PER_TILE+ BORDER_BETWEEN_TILES)) as f64,

                                              ((p2.coord.get_y() - range.y_min )*  (SIZE_PER_TILE+ BORDER_BETWEEN_TILES)) as f64);
                 p2.render(gl, center_p2);


            }
            // Render all bots
            for b in &mut self.bots {
                    if b.coord.get_x() >= range.x_min &&  b.coord.get_x() < range.x_max &&
                        b.coord.get_y() >= range.y_min && b.coord.get_y() < range.y_max {

                        let center_b1 = c.transform.trans(((b.coord.get_x() - range.x_min )*  (SIZE_PER_TILE+ BORDER_BETWEEN_TILES)) as f64,
                                                          ((b.coord.get_y() - range.y_min)*  (SIZE_PER_TILE+ BORDER_BETWEEN_TILES)) as f64);
                        b.render(gl, center_b1);
                }
            }

            // Render all active effects
            for e in &effects.effects {
                let center = c.transform.trans(((e.coord.get_x() - range.x_min )*  (SIZE_PER_TILE+ BORDER_BETWEEN_TILES)) as f64,
                                              ((e.coord.get_y() - range.y_min)*  (SIZE_PER_TILE+ BORDER_BETWEEN_TILES)) as f64);
                e.render(gl, center);
            }
        });
    }
    /// Updates all Players, Bots, effects and camera
    fn on_update(&mut self,
                 args: &UpdateArgs,
                 level: &mut Level,
                 state: usize,
                 effects: &mut EffectHandler) {
        // Update Coordinates
        let coord1 = self.player_one.coord.clone();
        let mut coord2 = coord1.clone();
        if let Some(ref p2) = self.player_two {
            coord2 = p2.coord.clone();
        }
        // Update range with coordinates
        let range = self.cam.get_range_update();
        // Update Player one
        self.player_one.on_update(args, range, level, InteractableType::Player(1));
        // Update Player two
        if let Some(ref mut x) = self.player_two {
            x.on_update(args, range, level, InteractableType::Player(2));
        }
        // Updates bots
        for b in &mut self.bots {
            b.on_update(args, range, level, state);
        }
        // Update Camera
        self.cam.calc_coordinates(coord1, coord2, level);

        // FOR TESTING KEY Q TO ACTIVATE ANIMATION
        if self.player_one.dead {
            effects.handle(coord1, EffectOption::Sword, Direction::Down);
        }
        // END OB TESTING

        // Update Effects
        effects.on_update(args)
    }

    /// Handles Input
    fn on_input(&mut self, inp: Button, pressed: bool) {

        match inp {
            // BUTTON Q FOR TESTING
            Button::Keyboard(Key::Q) => {

                self.player_one.dead = pressed;
            }
            // ENF OF TESTING
            Button::Keyboard(Key::Up) => {
                if pressed {
                    self.player_one.last = LastKey::Up;
                }
                self.player_one.pressed = pressed;
            }
            Button::Keyboard(Key::Down) => {
                if pressed {
                    self.player_one.last = LastKey::Down;
                }
                self.player_one.pressed = pressed;
            }
            Button::Keyboard(Key::Left) => {
                if pressed {
                    self.player_one.last = LastKey::Left;
                }
                self.player_one.pressed = pressed;
            }
            Button::Keyboard(Key::Right) => {
                if pressed {
                    self.player_one.last = LastKey::Right;
                }
                self.player_one.pressed = pressed;
            }
            Button::Keyboard(Key::W) => {
                if let Some(ref mut x) = self.player_two {
                    if pressed {
                        x.last = LastKey::Up;
                    }
                    x.pressed = pressed;
                }
            }
            Button::Keyboard(Key::S) => {
                if let Some(ref mut x) = self.player_two {
                    if pressed {
                        x.last = LastKey::Down;
                    }
                    x.pressed = pressed;
                }
            }
            Button::Keyboard(Key::A) => {
                if let Some(ref mut x) = self.player_two {
                    if pressed {
                        x.last = LastKey::Left;
                    }
                    x.pressed = pressed;
                }
            }
            Button::Keyboard(Key::D) => {
                if let Some(ref mut x) = self.player_two {
                    if pressed {
                        x.last = LastKey::Right;
                    }
                    x.pressed = pressed;
                }
            }
            Button::Keyboard(Key::Space) => {
                if pressed {
                    println!("Space!!!");
                }
            }
            _ => {}

        }
    }
}

/// Main
fn main() {
    let mut window: PistonWindow = WindowSettings::new("Chicken Fight 3000 Ultimate Tournament",
                                                       [WIDTH as u32, HEIGHT as u32])
        .exit_on_esc(true)
        .fullscreen(false)
        .resizable(false)
        .build()
        .unwrap();


    // Create map for sprites and load all sprites
    let map = Settings::new(&mut window).sprite_map;

    // Create EffectHandler
    let mut effects = EffectHandler::new(&map);

    // Lade XML und erstelle daraus das Level, das Tileset, die Player und die Bots
    let folder_level = match find_folder::Search::Kids(0).for_folder("src") {
        Ok(res) => res.join("level1.xml"),
        Err(_) => panic!("Folder 'src' not found!"),
    };

    let level_path = match folder_level.to_str() {
        Some(res) => res,
        None => panic!("Level not found!"),
    };

    let (lv, ts, bots, players) = load_xml(level_path, &map, &mut window);

    let tileset = ts;

    let mut level = lv;

    // Create new app with one or two players
    let mut app = App::new(players, bots);

    // Create window
    let mut events = window.events();

    // insert players in level
    level.get_data()[app.player_one.coord.get_x() as usize][app.player_one.coord.get_y() as usize]
        .set_fieldstatus(InteractableType::Player(1));
    if let Some(ref p2) = app.player_two {
        level.get_data()[p2.coord.get_x() as usize][p2.coord.get_y() as usize]
            .set_fieldstatus(InteractableType::Player(2));
    }

    // Start counter
    let mut start = PreciseTime::now();

    // set Level-borders to camera
    app.cam.set_borders((level.get_width() as u64, level.get_height() as u64));

    // sets border for player one
    app.player_one.set_borders((level.get_width() as u64, level.get_height() as u64));

    // load sprite for player two and sets border
    if let Some(ref mut p2) = app.player_two {
        p2.set_borders((level.get_width() as u64, level.get_height() as u64));
    }

    // Load sprite for each bot and set borders
    for b in &mut app.bots {
        b.set_borders((level.get_width() as u64, level.get_height() as u64));
    }

    // End of Loading start game
    while let Some(e) = events.next(&mut window) {
        // Calculate Milliseconds
        let now = start.to(PreciseTime::now()).num_milliseconds();

        // Calculate state
        let state = if now <= 500 { 0 } else { 1 };

        // If Render-Event
        if let Some(_) = e.render_args() {
            app.on_draw(&mut window,
                        &e,
                        &tileset,
                        &mut level,
                        now as usize,
                        &effects);
        }

        // If Key-Press-Event
        if let Some(i) = e.release_args() {
            app.on_input(i, false);
        }
        // If Key-releas-Event
        if let Some(i) = e.press_args() {
            app.on_input(i, true);
        }
        {
            // if update
            if let Some(u) = e.update_args() {
                app.on_update(&u, &mut level, state, &mut effects);
            }
        }

        // restart time if 1 second over
        if now > 1000 {
            start = PreciseTime::now();
        }
    }
}
