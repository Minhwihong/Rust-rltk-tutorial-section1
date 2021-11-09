use rltk::{ Rltk, VirtualKeyCode, Point};
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;


use super::{ Position, Map, TileType, State, RunState};
use crate::ViewShed;

#[derive(Component, Debug)]
pub struct Player {}



pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World){

    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();
    let mut viewsheds = ecs.write_storage::<ViewShed>();
    let mut ppos = ecs.write_resource::<Point>();


    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let destination_idx = Map::xy_idx(&map, pos.x + delta_x, pos.y + delta_y);

        //if map.tiles[destination_idx] != TileType::Wall {
        if !map.blocked[destination_idx]  {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));

            viewshed.dirty = true;

            ppos.x = pos.x;
            ppos.y = pos.y;
        }

    }
}


pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {

    // Player movement
    match ctx.key {
        None => { return RunState::Paused }
        Some(key) => match key {
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

            // Diagonals
            VirtualKeyCode::Numpad9 |
            VirtualKeyCode::Y => try_move_player(1, -1, &mut gs.ecs),

            VirtualKeyCode::Numpad7 |
            VirtualKeyCode::U => try_move_player(-1, -1, &mut gs.ecs),

            VirtualKeyCode::Numpad3 |
            VirtualKeyCode::N => try_move_player(1, 1, &mut gs.ecs),

            VirtualKeyCode::Numpad1 |
            VirtualKeyCode::B => try_move_player(-1, 1, &mut gs.ecs),


            _ => {}
        },
    }

    RunState::Running

}
