use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;
//use specs_derive::Component;



mod map;
mod rect;
mod components;
mod visibility_system;
mod player;
mod monster_ai_system;


pub use map::*;
pub use rect::*;
pub use components::{Position, ViewShed, Renderable, Monster};
pub use player::*;
pub use visibility_system::VisibilitySystem;
pub use monster_ai_system::{MonsterAI};


#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Running,
}


pub struct State {
    ecs: World,
    pub runstate: RunState,
}



impl State {
    fn run_systems(&mut self){
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);

        let mut mob = MonsterAI{};
        mob.run_now(&self.ecs);

        self.ecs.maintain();
    }
}




impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        // monsters only think about what to do when you move. That's a basic turn-based tick loop!
        if self.runstate == RunState::Running {

            self.run_systems();
            self.runstate = RunState::Paused;
        }
        else {
            self.runstate = player_input(self, ctx);
        }


        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);

            if map.visible_tiles[idx]{
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }

        }


    }
}


fn main() -> rltk::BError  {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("test game")
        .build()?;

    let mut gs = State {
        ecs: World::new(),
        runstate : RunState::Running,
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<ViewShed>();
    gs.ecs.register::<Monster>();


    let game_map= Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = game_map.rooms[0].center();

    let mut rng = rltk::RandomNumberGenerator::new();

    for room in game_map.rooms.iter().skip(1){
        let (x,y) = room.center();

        let glyph : rltk::FontCharType;
        let roll = rng.roll_dice(1,2);

        match roll {
            1 => {glyph = rltk::to_cp437('g')}
            _ => {glyph = rltk::to_cp437('m')}
        }

        gs.ecs.create_entity()
            .with(Position{x,y})
            .with(Renderable{
                glyph : glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(ViewShed{visible_tiles: Vec::new(), range: 8, dirty: true})
            .with(Monster{})
            .build();
    }

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
            dirty : true,
        })
        .build();


    rltk::main_loop(context, gs)


}















