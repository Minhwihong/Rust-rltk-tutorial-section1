use specs::prelude::*;
use super::{ViewShed, Position, Map, Monster, Name};
use rltk::{field_of_view, Point, console};

pub struct MonsterAI{
    //pub cnt : u32,
}



impl<'a> System<'a> for MonsterAI {

    type SystemData = ( ReadExpect<'a, Point>,
        ReadStorage<'a, ViewShed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>);

    fn run(&mut self, data: Self::SystemData){

        let (player_pos, viewshed, monster, name) = data;

        for(viewshed, _monster, name) in (&viewshed, &monster, &name).join(){

            if viewshed.visible_tiles.contains(&*player_pos){
                console::log(format!("{} Monster shouts insults!", name.name));
            }
            //console::log("Monster considers their own existence");
            //println!("Monster considers their own existence - {}", self.cnt);
            //self.cnt = self.cnt + 1;
        }
    }
}