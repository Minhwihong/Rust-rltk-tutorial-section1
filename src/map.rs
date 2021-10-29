use rltk::{ Rltk, RGB, BaseMap, Algorithm2D, RandomNumberGenerator, Point};
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;

use super::{Rect};


/*
Clone : add a .clone() method to the type
Copy : Allow a copy to be made programmatically ( tile1 = tile2 ) <- this is copy
PartialEq : allows us to use == to see if two tile types match
             if tile_type == TileType::Wall would fail to compile!
 */
#[derive(PartialEq, Copy, Clone)]
pub enum TileType{
    Wall, Floor
}


pub struct Map{
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32
}



impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    fn apply_room_to_map(&mut self, room : &Rect) {
        for y in room.y1 +1 ..= room.y2 {
            for x in room.x1 + 1 ..= room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1 : i32, x2: i32, y: i32) {
        for x in min(x1, x2) ..= max(x1,x2){
            let idx = self.xy_idx(x,y);

            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1:i32, y2:i32, x:i32) {
        for y in min(y1,y2) ..= max(y1,y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    /// Makes a new map using the algorithm from http://rogueliketutorials.com/tutorials/tcod/part-3/
    /// This gives a handful of random rooms and corridors joining them together.
    pub fn new_map_rooms_and_corridors() -> Map {
        let mut map = Map{
            tiles : vec![TileType::Wall; 80*50],
            rooms : Vec::new(),
            width : 80,
            height : 50
        };

        const MAX_ROOMS : i32 = 30;
        const MIN_SIZE : i32 = 6;
        const MAX_SIZE : i32 = 10;


        let mut rng = RandomNumberGenerator::new();


        for i in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, map.width - w - 1) - 1;
            let y = rng.roll_dice(1, map.height - h - 1) - 1;
            let new_room = Rect::new(x,y,w,h);

            let mut ok = true;

            for other_room in map.rooms.iter() {

                if new_room.intersect(&other_room){
                    ok = false;
                }
            }

            if ok == true {

                println!("Room position - x1: {}, x2: {}, y1: {}, y2: {}",new_room.x1, new_room.x2, new_room.y1, new_room.y2 );

                map.apply_room_to_map(&new_room);

                if map.rooms.is_empty() == false {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len()-1].center();

                    if rng.range(0,2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    }
                    else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        map
    }
}


impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] == TileType::Wall
    }
}




pub fn draw_map(map: &Map, ctx: &mut Rltk){
    let mut y = 0;
    let mut x = 0;

    for tile in map.tiles.iter() {
        //Render a tile depending upon the tile type
        match tile {
            TileType::Floor => {
                ctx.set(x,y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), rltk::to_cp437('.'));
            },
            TileType::Wall => {
                ctx.set(x,y, RGB::from_f32(0.5, 1.0, 0.5), RGB::from_f32(0., 0., 0.), rltk::to_cp437('#'));
            }
        }

        // Move the coordinates
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}










