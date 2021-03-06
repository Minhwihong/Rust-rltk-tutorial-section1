use rltk::{VirtualKeyCode, Rltk};
use specs::prelude::*;
use std::cmp::{max, min};
use super::{Position, Player, ViewShed, TileType, State, Map};
//use crate::ViewShed;


pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World){

    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<ViewShed>();
    let map = ecs.fetch::<Map>();



    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let destination_idx = Map::xy_idx(&map, pos.x + delta_x, pos.y + delta_y);

        if map.tiles[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));

            viewshed.dirty = true;
        }

    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk){

    // Player movement
    match ctx.key {
        None => {}
        Some(Key) => match Key {
            VirtualKeyCode::Left |
            VirtualKeyCode::Numpad4 |
            VirtualKeyCode::A => try_move_player(-1, 0, &mut gs.ecs),

            VirtualKeyCode::Right |
            VirtualKeyCode::Numpad6 |
            VirtualKeyCode::D => try_move_player(1, 0, &mut gs.ecs),

            VirtualKeyCode::Up |
            VirtualKeyCode::Numpad8 |
            VirtualKeyCode::W => try_move_player(0, -1, &mut gs.ecs),

            VirtualKeyCode::Down |
            VirtualKeyCode::Numpad2 |
            VirtualKeyCode::S => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }

}