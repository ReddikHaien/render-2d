use crate::resource::{Resources, sprite::Sprite};

pub fn register_sprite(resources: &mut Resources){
    resources.register_type::<dyn Sprite>();
}