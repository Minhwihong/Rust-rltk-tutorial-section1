use specs::prelude::*;
use super::{ViewShed, Position, Map, Player};
use specs::{RunningTime, AccessorCow};
use rltk::{field_of_view, Point};

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {

    type SystemData = ( WriteExpect<'a, Map>,
                        Entities<'a>,
                        WriteStorage<'a, ViewShed>,
                        WriteStorage<'a, Position>,
                        ReadStorage<'a, Player>);

    fn run(&mut self, data: Self::SystemData) {

        let (mut map, entities, mut viewshed, pos, player) = data;

        for (ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join(){
            viewshed.dirty = false;
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
            //viewshed.visible_tiles.retain(|p| p.x >= map.width && p.y >= 0 && p.y < map.height);
            viewshed.visible_tiles.retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height );

            // If this is the player, reveal what they can see
            let _p : Option<&Player> = player.get(ent);

            if let Some(_p) = _p {

                for t in map.visible_tiles.iter_mut() {
                    *t = false
                };

                for vis in viewshed.visible_tiles.iter() {
                    let idx = map.xy_idx(vis.x, vis.y);
                    map.revealed_tiles[idx] = true;
                    map.visible_tiles[idx] = true;
                }
            }
            // let p : Option<&Player> = player.get(ent);
            //
            // if let Some(p) = p {
            //     for vis in viewshed.visible_tiles.iter(){
            //         let idx = map.xy_idx(vis.x, vis.y);
            //         map.revealed_tiles[idx] = true;
            //         map.visible_tiles[idx] = true;
            //     }
            // }

        }
    }
}