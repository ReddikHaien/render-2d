use std::rc::Rc;

struct RawGlTexture{
    id: gl::types::GLuint,
}

impl RawGlTexture {
    fn from_data(width: u32, height: u32, data: Vec<u8>) -> Self{
        let id = unsafe {
            let mut out = 0;
            gl::CreateTextures(gl::TEXTURE_2D,1,&mut out);
            out
        };

        unsafe{
            gl::BindTexture(gl::TEXTURE_2D,id);
            gl::TexImage2D(gl::TEXTURE_2D,0,gl::RGBA as i32, width as i32, height as i32,0,gl::RGBA,gl::FLOAT,data.as_ptr() as *const std::ffi::c_void);
            
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        }

        Self{
            id
        }
    }

    fn empty() -> Self{
        let id = unsafe{
            let mut out = 0;
            gl::CreateTextures(gl::TEXTURE_2D, 1, &mut out);
            out
        };

        Self{
            id
        }
    }

    pub fn set_data(&self, width: u32, height: u32, data: Vec<u8>){
        unsafe{
            gl::BindTexture(gl::TEXTURE_2D,self.id);
            gl::TexImage2D(gl::TEXTURE_2D,0,gl::RGBA as i32, width as i32, height as i32,0,gl::RGBA,gl::FLOAT,data.as_ptr() as *const std::ffi::c_void);
            
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        }
    }
}

impl Drop for RawGlTexture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &mut self.id);
        }
    }
}


///
/// Represents a gl texture
/// It is safe to clone this texture due to the underlying data being ref_counted
#[derive(Clone)]
pub struct GlTexture{
    raw: Rc<RawGlTexture>
}

impl GlTexture{
    fn from_raw(raw: RawGlTexture) -> Self{
        Self{
            raw: Rc::new(raw)
        }
    }

    pub fn create_empty() -> Self{
        Self::from_raw(RawGlTexture::empty())   
    }

    pub fn from_data(width: u32, height: u32, data: Vec<u8>) -> Self {
        Self::from_raw(RawGlTexture::from_data(width, height, data))
    }

    pub fn set_data(&mut self, width: u32,height: u32,data: Vec<u8>) {
        self.raw.set_data(width, height, data);
    }

    pub fn bind_texture(&self, slot: u32){
        #[cfg(debug_assertions)]{
            if slot > 31{
                panic!("Out of range texture location, must be between 0 - 31(inclusive)")
            }
        }
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + slot);
            gl::BindTexture(gl::TEXTURE_2D, self.raw.id);
        }
    }

    
}