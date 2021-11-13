use std::{env::current_dir, os::windows::process, path::Path};

use lodepng::{Bitmap, RGBA};

use crate::resource::{ResourceKey, Resources, texture::GlTexture};
///
/// loads a png into resources, naming it to the relative path to the current working dir
pub fn load_texture(file_path: &Path, resources: &mut Resources) -> Result<ResourceKey,String>{
    if file_path.is_dir(){
        return Err(String::from("Invalid Path, Path is pointing to a directory"));
    }

   let result = lodepng::decode32_file(file_path);
   
   if result.is_err(){
       let err = result.unwrap_err();
       return Err(format!("Failed to load file {}",err));
   }

   let data = result.unwrap();
   let width = data.width as u32;
   let height = data.height as u32;
   let mut buffer = data.buffer;
   
   let len = buffer.len()*4;
   let capasity = buffer.capacity()*4;
   let buffer = buffer.as_mut_ptr() as *mut u8;

   std::mem::forget(buffer);

   let new = unsafe {Vec::from_raw_parts(buffer,len,capasity)};

   let texture = GlTexture::from_data(width,height, new);
   Ok(resources.add_resource(texture,create_sprite_name(file_path, current_dir().unwrap().as_path())))
}

pub fn register_textures(resources: &mut Resources){
    resources.register_type::<GlTexture>();
}


///
/// Loads every image from the root and constructs a texture atlas with additional sprites
/// The atlas will be named 'spritesheet', and the sprites will be named in correlation to the filepath of each subsequent image relative to the current working dir
pub fn load_as_atlas(root: &Path, resources: &mut Resources){
    let mut images = Vec::new();
    load_dir(root, root, &mut images);

}

fn load_dir(root: &Path, path: &Path, images: &mut Vec<(String,Bitmap<RGBA>)>){
    let entries = path.read_dir().unwrap();
    for entry in entries{
        let entry = entry.unwrap();
        if entry.path().is_dir(){
            load_dir(root, entry.path().as_path(), images);
        }
        else{
            load_image(root, entry.path().as_path(), images);
        }
    }
}

fn load_image(root: &Path, path: &Path, images: &mut Vec<(String,Bitmap<RGBA>)>){
    let name = create_sprite_name(path, root);
    images.push((name,lodepng::decode32_file(root).unwrap()));
}


fn create_atlas(mut images: Vec<(String,Bitmap<RGBA>)>){
    images.sort_by(|a,b|{
        match b.1.height.partial_cmp(&a.1.width){
            Some(x) => match x{
                std::cmp::Ordering::Equal => b.1.width.partial_cmp(&a.1.height).unwrap(),
                x => x
            },
            _ => todo!(),
        }
    });

    let mut size = get_minimum_containing_square(images[0].1.width.max(images[1].1.height) as u32);
    let mut root = ImageNode::Empty{width: size, height: size};
    
    for x in images{
        let mut accum = x;
        while let Some(x) = root.try_insert(accum) {
            accum = x;
            
            size <<= 1;
        }
    }
}

fn get_minimum_containing_square(size: u32) -> u32{
    if size.count_ones() == 1{
        size
    }
    else{
        let z = size.leading_zeros();
        1u32 << (32u32 - z)
    }
}

fn create_sprite_name(path: &Path,root: &Path) -> String{
    String::from(path.to_str().unwrap())
    .replace(root.to_str().unwrap(), "")
    .replace('\\', "/")
}

pub enum ImageNode {
    Occupied{
        image: (String,Bitmap<RGBA>),
        left: Box<ImageNode>,
        down: Box<ImageNode>
    },
    Empty{
        width: u32,
        height: u32,
    }
}

impl ImageNode{
    fn try_insert(&mut self, target: (String, Bitmap<RGBA>)) -> Option<(String,Bitmap<RGBA>)>{
        match self {
            ImageNode::Occupied { image, left, down } => {
                let a = left.try_insert(target);
                if a.is_some(){
                    return down.try_insert(a.unwrap());
                }
                else {
                    return None;
                }
            }
            ImageNode::Empty { width, height } => {
                if *width == (target.1.width as u32) && *height == target.1.height as u32{
                    *self = ImageNode::Occupied{
                        down: Box::new(ImageNode::Empty{width: 0, height: 0}),
                        left: Box::new(ImageNode::Empty{width: 0, height: 0}),
                        image: target
                    };
                    None
                }
                else {
                    Some(target)        
                }
            }
        }
    }
}