use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use specs_derive::Component;



mod map;
mod rect;
mod components;
mod visibility_system;
mod player;


pub use map::*;
pub use rect::*;
pub use components::{Position, ViewShed};
pub use player::*;
pub use visibility_system::VisibilitySystem;




// impl Component for Position {
//     type Storage = VecStorage<Self>;
// }

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}



pub struct State {
    ecs: World
}



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
    gs.ecs.register::<Player>();
    gs.ecs.register::<ViewShed>();


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
            range : 8,
            dirty : false,
        })
        .build();


    rltk::main_loop(context, gs)


}















