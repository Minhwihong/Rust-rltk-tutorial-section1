use specs_derive::Component;
use specs::prelude::*;
//use rltk::{GameState, Rltk, RGB, VirtualKeyCode};




#[derive(Component)]
pub struct ViewShed {
    pub visible_tiles : Vec<rltk::Point>,
    pub range: i32,
    pub dirty : bool,
}

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}