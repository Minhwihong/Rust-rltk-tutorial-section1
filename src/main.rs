use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;


mod map;
mod rect;
mod components;
mod visibility_system;


pub use map::*;
pub use rect::*;
pub use components::ViewShed;
pub use visibility_system::VisibilitySystem;


#[derive(Component)]
pub struct Position {
    x: i32,
    y: i32,
}

// impl Component for Position {
//     type Storage = VecStorage<Self>;
// }

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}


#[derive(Component)]
struct LeftMover {}


#[derive(Component, Debug)]
pub struct Player {}





struct LeftWalker {}

impl<'a> System<'a> for LeftWalker {
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

    fn run(&mut self, (lefty, mut pos) : Self::SystemData){
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 { pos.x = 79; }
        }
    }
}



struct State {
    ecs: World
}


// impl State {
//     fn run_systems(&mut self){
//         let mut lw = LeftWalker{};
//         lw.run_now(&self.ecs);
//         self.ecs.maintain();
//     }
// }

impl State {
    fn run_systems(&mut self){
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);

        self.ecs.maintain();
    }
}




impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        self.run_systems();
        player_input(self, ctx);


        //let map = self.ecs.fetch::<Vec<TileType>>();
        let map = self.ecs.fetch::<Map>();
        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }



        //self.run_systems();
    }
}


fn main() -> rltk::BError  {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("test game")
        .build()?;

    let mut gs = State {
        ecs: World::new()
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<ViewShed>();

    // let (rooms, map) = new_map_rooms_and_corridors();
    // gs.ecs.insert(map);
    // let (player_x, player_y) = rooms[0].center();

    let game_map= Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = game_map.rooms[0].center();

    gs.ecs.insert(game_map);


    // builder pattern (Combining functions in this fashion is called method chaining)
    gs.ecs
        .create_entity()
        .with(Position{x:player_x, y:player_y})
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg : RGB::named(rltk::YELLOW),
            bg : RGB::named(rltk::BLACK),
        })
        .with(Player{})
        .with(ViewShed{
            visible_tiles : Vec::new(),
            range : 8
        })
        .build();


    rltk::main_loop(context, gs)


}


fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World){

    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Map>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = Map::xy_idx(&map, pos.x + delta_x, pos.y + delta_y);

        if map.tiles[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }

    }
}


fn player_input(gs: &mut State, ctx: &mut Rltk){

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













