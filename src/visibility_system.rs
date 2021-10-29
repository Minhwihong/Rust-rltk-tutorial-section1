use specs::prelude::*;
use super::{ViewShed, Position};
use specs::{RunningTime, AccessorCow};

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {

    type SystemData = (WriteStorage<'a, ViewShed>, WriteStorage<'a, Position>);

    fn run(&mut self, (mut viewshed, pos): Self::SystemData) {

        for (viewshed, pos) in (&mut viewshed, &pos).join(){

        }
    }
}