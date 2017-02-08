extern crate find_folder;

use super::super::bot::Bot;
use super::super::player::Player;
use super::super::io::tileset::Tileset;
use super::super::io::read_tileset;
use super::super::io::read_level;
use super::super::level::Level;
use super::super::io::all_sprites::SpriteMap;
use std::str::FromStr;
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};
use piston_window::*;

pub fn load_xml<'a>(path: &str, map: &'a SpriteMap, mut w: &mut PistonWindow) -> (Level, Tileset, Vec<Bot<'a>>, Vec<Player<'a>>) {
    let mut bots : Vec<Bot> = Vec::new();
    let mut tileset = Tileset::new(1, 1, 1, 1);
    let mut level = Level::with_size(0, 0);
    let mut players = Vec::new();
    let mut last = String::new();
    let mut i = 0;

    let file = File::open(path).unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                last = name.local_name.clone();
                match &last[..] {
                    "tileset" => {
                        let mut it = attributes.iter();

                        let path = match it.next() {
                            Some(s) => s,
                            None => panic!("Wrong xml format!"),
                        };
                        let mut tileset_path = path.value.clone();

                        let tile_height = match it.next() {
                            Some(s) => match u32::from_str(&s.value) {
                                Ok(n) => n,
                                Err(_) => panic!("{:?} is not a number!", s.value),
                            },
                            None => panic!("Wrong xml format!"),
                        };

                        let tile_width = match it.next() {
                            Some(s) => match u32::from_str(&s.value) {
                                Ok(n) => n,
                                Err(_) => panic!("{:?} is not a number!", s.value),
                            },
                            None => panic!("Wrong xml format!"),
                        };

                        let tileset_height = match it.next() {
                            Some(s) => match u32::from_str(&s.value) {
                                Ok(n) => n,
                                Err(_) => panic!("{:?} is not a number!", s.value),
                            },
                            None => panic!("Wrong xml format!"),
                        };

                        let tileset_width = match it.next() {
                            Some(s) => match u32::from_str(&s.value) {
                                Ok(n) => n,
                                Err(_) => panic!("{:?} is not a number!", s.value),
                            },
                            None => panic!("Wrong xml format!"),
                        };

                        let tiles = match find_folder::Search::Kids(1).for_folder("tiles") {
                            Ok(res) => res.join(tileset_path),
                            Err(_) => panic!("Folder 'tiles' not found!"),
                        };

                        let file_path = match tiles.to_str() {
                            Some(res) => res,
                            None => panic!("Tileset not found!"),
                        };

                        tileset = read_tileset(file_path, &mut w, tile_height, tile_width, tileset_height, tileset_width);
                    },
                    "file" => {
                        let path = match attributes.first() {
                            Some(s) => s,
                            None => panic!("Wrong xml format!aaa"),
                        };
                        let lvl_path = path.value.clone();

                        let folder_level = match find_folder::Search::Kids(0).for_folder("src") {
                            Ok(res) => res.join(lvl_path),
                            Err(_) => panic!("Folder 'src' not found!"),
                        };

                        let level_path = match folder_level.to_str() {
                            Some(res) => res,
                            None => panic!("Level not found!"),
                        };

                        level = read_level(level_path);
                    },
                    "player1" => {
                        let mut it = attributes.iter();

                        let x = match it.next() {
                            Some(s) => match u64::from_str(&s.value) {
                                Ok(n) => n,
                                Err(_) => panic!("{:?} is not a number!", s.value),
                            },
                            None => panic!("Wrong xml format!"),
                        };
                        let y = match it.next() {
                            Some(s) => match u64::from_str(&s.value) {
                                Ok(n) => n,
                                Err(_) => panic!("{:?} is not a number!", s.value),
                            },
                            None => panic!("Wrong xml format!"),
                        };

                        let mut p1 = Player::new(x, y, 1);
                        let p = match it.next() {
                            Some(s) => s,
                            None => panic!("Wrong xml format!"),
                        };
                        let sprite = p.value.clone();
                        p1.set_sprite(map.get_sprite(sprite));
                        players.push(p1);
                    },
                    "player2" => {
                        let mut it = attributes.iter();

                        let x = match it.next() {
                            Some(s) => match u64::from_str(&s.value) {
                                Ok(n) => n,
                                Err(_) => panic!("{:?} is not a number!", s.value),
                            },
                            None => panic!("Wrong xml format!"),
                        };
                        let y = match it.next() {
                            Some(s) => match u64::from_str(&s.value) {
                                Ok(n) => n,
                                Err(_) => panic!("{:?} is not a number!", s.value),
                            },
                            None => panic!("Wrong xml format!"),
                        };

                        let mut p2 = Player::new(x, y, 2);
                        let p = match it.next() {
                            Some(s) => s,
                            None => panic!("Wrong xml format!"),
                        };
                        let sprite = p.value.clone();
                        p2.set_sprite(map.get_sprite(sprite));
                        players.push(p2);
                    },
                    "bot" => {
                        i += 1;
                        let mut it = attributes.iter();

                        let x = match it.next() {
                            Some(s) => match u64::from_str(&s.value) {
                                Ok(n) => n,
                                Err(_) => panic!("{:?} is not a number!", s.value),
                            },
                            None => panic!("Wrong xml format!"),
                        };
                        let y = match it.next() {
                            Some(s) => match u64::from_str(&s.value) {
                                Ok(n) => n,
                                Err(_) => panic!("{:?} is not a number!", s.value),
                            },
                            None => panic!("Wrong xml format!"),
                        };
                        let p = match it.next() {
                            Some(s) => s,
                            None => panic!("Wrong xml format!"),
                        };
                        let sprite = p.value.clone();

                        let mut b = Bot::new(x, y, i);
                        b.set_sprite(map.get_sprite(sprite));

                        bots.push(b);
                    },
                    _ => (),
                }
            }
            /* Für zukünftige Eigenschaften */
            Ok(XmlEvent::Characters(s)) => {
                match &last[..] {
                    "player1" | "player2" => (), // Setze irgendwelche Eigenschaften über players.last_mut().
                    "bot" => (), // Setze irgendwelche Eigenschaften über bots.last_mut().
                    _ => (),
                }
            }
            Err(e) => {
                panic!("Wrong xml format!");
            }
            _ => {}
        }
    }


    (level, tileset, bots, players)
}
