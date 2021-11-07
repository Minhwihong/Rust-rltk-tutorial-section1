use specs::prelude::*;
use super::{ViewShed, Position, Map, Monster};
use rltk::{field_of_view, Point, console};

pub struct MonsterAI{
    //pub cnt : u32,
}



impl<'a> System<'a> for MonsterAI {

    type SystemData = (ReadStorage<'a, ViewShed>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Monster>);

    fn run(&mut self, data: Self::SystemData){

        let (viewshed, pos, monster) = data;

        for(viewshed, pos, _monster) in (&viewshed, &pos, &monster).join(){
            console::log("Monster considers their own existence");
            //println!("Monster considers their own existence - {}", self.cnt);
            //self.cnt = self.cnt + 1;
        }
    }
}