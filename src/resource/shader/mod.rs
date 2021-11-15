use std::{collections::HashMap, ffi::CString, ops::Deref, rc::Rc};

use gl::types::GLuint;

pub enum ShaderDataType{
    Int,
    Float,
    Vec2F,
    Vec3F,
    Vec4F,
    Mat2F,
    Mat3F,
    Mat4F
}

pub struct ShaderInput{
    id: i32,
    data_type: ShaderDataType,
}

struct RawGlShader{
    id: gl::types::GLuint,
    uniforms: HashMap<String,ShaderInput>,
    attributes: HashMap<String,ShaderInput>,
}

impl RawGlShader{
    fn from_source(vertex: &str, fragment: &str) -> Result<Self,String>{
        let v = Self::create_shader(vertex, gl::VERTEX_SHADER)?;
        let f = Self::create_shader(fragment, gl::FRAGMENT_SHADER)?;



        Ok(Self{
            id: 0,
            uniforms: HashMap::new(),
            attributes: HashMap::new(),
        })
    }

    fn create_program(vertex: GLuint, fragment: GLuint) -> Result<GLuint,String>{
        unsafe{
            let program = gl::CreateProgram();

            gl::AttachShader(program, vertex);
            gl::AttachShader(program,fragment);
            gl::LinkProgram(program);
            Self::validate_program(program, gl::LINK_STATUS);
            

            
            let uniform_count = {
                let mut out = 0;
                gl::GetProgramiv(program, gl::ACTIVE_UNIFORMS, &mut out);
                out
            } as u32;

            for i in 0..uniform_count{
                let name = {
                    let mut len = 0;
                    gl::GetActiveUniformBlockiv(program, i, gl::UNIFORM_NAME_LENGTH, &mut len);
                    let mut buf = vec![0;len as usize];
                    let mut _outlen = 0;
                    gl::GetActiveUniformName(program,i,len,&mut _outlen, buf.as_mut_ptr());
                    String::from(CString::from_raw(buf.as_mut_ptr()).to_str().unwrap())
                };
                
            }
            

            Ok(program)
        }
    }

    fn create_shader(source: &str, shader_type: gl::types::GLenum) -> Result<gl::types::GLuint,String>{
        unsafe{
            let shader = gl::CreateShader(shader_type);
            gl::ShaderSource(shader,1,source.as_ptr() as *const *const i8,&(source.len() as i32));
            gl::CompileShader(shader);
            Self::validate_shader(shader, gl::COMPILE_STATUS)?;
            Ok(shader)
        }
    }

    fn validate_shader(shader: gl::types::GLuint, target: gl::types::GLenum) -> Result<(),String>{
        
        unsafe{
            let mut ok = 0;
            gl::GetShaderiv(shader, target, &mut ok);
            if ok != gl::TRUE as i32{
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = vec![0;len as usize];
                let mut out = 0;
                gl::GetShaderInfoLog(shader,len,&mut out,buf.as_mut_ptr());
                let cstring = std::ffi::CString::from_raw(buf.as_mut_ptr());
                Err(String::from(cstring.to_string_lossy()))
            }   
            else{                
                Ok(())
            }     
        }
    }

    fn validate_program(program: gl::types::GLuint, target: gl::types::GLenum) -> Result<(),String>{
        
        unsafe{
            let mut ok = 0;
            gl::GetProgramiv(program, target, &mut ok);
            if ok != gl::TRUE as i32{
                let mut len = 0;
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = vec![0;len as usize];
                let mut out = 0;
                gl::GetProgramInfoLog(program,len,&mut out,buf.as_mut_ptr());
                let cstring = std::ffi::CString::from_raw(buf.as_mut_ptr());
                Err(String::from(cstring.to_string_lossy()))
            }   
            else{                
                Ok(())
            }     
        }
    }
}

pub struct GlShader{
    raw: Rc<RawGlShader>
}


