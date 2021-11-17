use std::env::current_dir;

use render_2d::{loader, resource::Resources};

fn main(){
    let mut resources = Resources::new();
    let root = current_dir().unwrap();
    let root = root.join("assets/textures");
    loader::texture::load_as_atlas(&root, &mut resources);
}