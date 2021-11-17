use std::{collections::HashMap, env::current_dir, mem::forget, os::windows::process, path::Path};

use lodepng::{Bitmap, RGBA};

use crate::resource::{ResourceKey, Resources, sprite::uv_sprite::UvSprite, texture::GlTexture};
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

    create_atlas(images);
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
    images.push((name,lodepng::decode32_file(path).unwrap()));
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
            root.grow_width(size);
            root.grow_height(size);
        }
    }
    let mut atlas_data = vec![RGBA{..Default::default()};(size*size) as usize];
    let mut sprites = HashMap::new();
    let mut output_texture = GlTexture::create_empty();
    root.write_image(0, 0, &mut atlas_data, &mut sprites, &output_texture, size);

    atlas_data.shrink_to_fit();
    let len = atlas_data.len()*4;
    let cap = atlas_data.capacity()*4;
    let buf = atlas_data.as_mut_ptr() as *mut u8;
    forget(atlas_data);

    let raw_data = unsafe{Vec::from_raw_parts(buf,len,cap)};
    
    output_texture.set_data(size, size, raw_data);
    //lodepng::encode32_file(current_dir().unwrap().join("atlas.png"), &atlas_data, size as usize, size as usize).unwrap();
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
            ImageNode::Occupied { image:_, left, down } => {
                let a = left.try_insert(target);
                if a.is_some(){
                    down.try_insert(a.unwrap())
                }
                else {
                    None
                }
            }
            ImageNode::Empty { width, height } => {
                if *width >= target.1.width as u32 && *height >= target.1.height as u32{
                    let remaining_width = *width - target.1.width as u32;
                    let remaining_height = *height - target.1.height as u32;

                    *self = ImageNode::Occupied{
                        down: Box::new(ImageNode::Empty{width: target.1.width as u32, height: remaining_height}),
                        left: Box::new(ImageNode::Empty{width: remaining_width, height: target.1.height as u32}),
                        image: target
                    };

                    None
                }
                else{
                    Some(target)        
                }
            }
        }
    }

    fn grow_height(&mut self, amt: u32){
        match self {
            ImageNode::Occupied { image, left, down } => {
                let nh = amt - image.1.height as u32;
                left.grow_width(nh);
                down.grow_width(nh);
            }
            ImageNode::Empty { width:_, height } => {
                *height = (*height).max(amt);
            }
        }
    }

    fn grow_width(&mut self, amt: u32){
        match self {
            ImageNode::Occupied { image, left, down } => {
                let nw = amt - image.1.width as u32;
                left.grow_width(nw);
                down.grow_width(nw);
            }
            ImageNode::Empty { width, height:_ } => {
                *width = (*width).max(amt);
            }
        }
    }

    fn write_image(self, x: u32, y: u32, out: &mut Vec<RGBA>, sprites: &mut HashMap<String,UvSprite>, texture: &GlTexture, size: u32){
        match self{
            ImageNode::Occupied { image, left, down } => {

                for px in 0..image.1.width as u32{
                    for py in 0..image.1.height as u32{
                        out[((px + x) + (py + y) * size) as usize] = image.1.buffer[(px + py * image.1.width as u32) as usize].clone();
                    }
                }

                
                let fsize = size as f32;
                
                let half_pixel = 1.0/(fsize*16.0);

                let min_x = (x as f32)/fsize;
                let min_y = (y as f32)/fsize;
                let w = (image.1.width as f32)/fsize;
                let h = (image.1.height as f32)/fsize;
                let max_x = min_x + w;
                let max_y = min_y + h;


                sprites.insert(image.0, UvSprite::new(min_x + half_pixel, min_y + half_pixel, max_x - half_pixel, max_y - half_pixel, texture.clone()));

                left.write_image(x + image.1.width as u32, y, out, sprites, texture, size);
                down.write_image(x, y + image.1.height as u32, out, sprites, texture, size);
            },
            ImageNode::Empty { width:_, height:_ } => {
                
            },
        }
    }
}