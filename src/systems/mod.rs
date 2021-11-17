use ecs_core::{components::Transform, data::storage::Storage, join::{create_iterator, create_iterator_2}, shred::{Read, System}};

use crate::{components::SpriteFilter, resource::Resources};

pub struct SpriteDrawSystem;


impl<'a> System<'a> for SpriteDrawSystem{
    type SystemData = (Read<'a, Storage<SpriteFilter>>, Read<'a, Storage<Transform>>, Read<'a, Resources>);

    fn run(&mut self, (sprites, transforms, render_resources): Self::SystemData) {
        
        

        for x in create_iterator_2(&sprites,&transforms){
            
        }
    }
}