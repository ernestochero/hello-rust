use std::ffi::{CString, NulError};
use std::string::FromUtf8Error;
use glutin::{Api, ContextBuilder, GlRequest};
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use gl::types::*;
use std::ptr;
use thiserror::Error;

// SHADERS

#[derive(Debug, Error)]
pub enum ShaderError {
    #[error("Error while compiling shader: {0}")]
    CompilationError(String),
    #[error("Error while linking shaders: {0}")]
    LinkingError(String),
    #[error{"{0}"}]
    Utf8Error(#[from] FromUtf8Error),
    #[error{"{0}"}]
    NulError(#[from] NulError),
}

pub struct Shader {
    pub id: GLuint,
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

impl Shader {
    pub unsafe fn new(source_code:&str, shader_type: GLenum) -> Result<Self, ShaderError> {
        let source_code = CString::new(source_code)?;
        let shader = Self {
            id: gl::CreateShader(shader_type),
        };
        gl::ShaderSource(shader.id, 1, &source_code.as_ptr(), ptr::null());
        gl::CompileShader(shader.id);
        // check for shader compilation errors
        let mut success:GLint = 0;
        gl::GetShaderiv(shader.id, gl::COMPILE_STATUS, &mut success);
        if success == 1 { Ok(shader) }
        else {
            let mut error_log_size: GLint = 0;
            gl::GetShaderiv(shader.id, gl::INFO_LOG_LENGTH, &mut error_log_size);
            let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
            gl::GetShaderInfoLog(
                shader.id,
                error_log_size,
                &mut error_log_size,
                error_log.as_mut_ptr() as *mut _,
            );
            error_log.set_len(error_log_size as usize);
            let log = String::from_utf8(error_log)?;
            Err(ShaderError::CompilationError(log))
        }
    }
}

// SHADER PROGRAM
pub struct ShaderProgram {
    pub id: GLuint,
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

impl ShaderProgram {
    pub unsafe fn new(shaders: &[Shader]) -> Result<Self, ShaderError> {
        let program = Self {
            id: gl::CreateProgram(),
        };

        for shader in shaders {
            gl::AttachShader(program.id, shader.id);
        }

        gl::LinkProgram(program.id);

        let mut success: GLint = 0;
        gl::GetProgramiv(program.id, gl::LINK_STATUS, &mut success);

        if success == 1 {
            Ok(program)
        } else {
            let mut error_log_size: GLint = 0;
            gl::GetProgramiv(program.id, gl::INFO_LOG_LENGTH, &mut error_log_size);
            let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
            gl::GetProgramInfoLog(
                program.id,
                error_log_size,
                &mut error_log_size,
                error_log.as_mut_ptr() as *mut _,
            );

            error_log.set_len(error_log_size as usize);
            let log = String::from_utf8(error_log)?;
            Err(ShaderError::LinkingError(log))
        }
    }

    pub unsafe fn apply(&self) {
        gl::UseProgram(self.id);
    }

    pub unsafe fn get_attrib_location(&self, attrib: &str) -> Result<GLuint, NulError> {
        let attrib = CString::new(attrib)?;
        Ok(gl::GetAttribLocation(self.id, attrib.as_ptr()) as GLuint)
    }
}


// BUFFER
pub struct Buffer {
    pub id: GLuint,
    target: GLuint,
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, [self.id].as_ptr());
        }
    }
}

impl Buffer {
    pub unsafe fn new(target: GLuint) -> Self {
        let mut id: GLuint = 0;
        gl::GenBuffers(1, &mut id);
        Self { id, target }
    }

    pub unsafe fn set_data<D>(&self, data: &[D], usage: GLuint) {
        self.bind();
        let (_, data_bytes, _) = data.align_to::<u8>();
        gl::BufferData(
            self.target,
            data_bytes.len() as GLsizeiptr,
            data_bytes.as_ptr() as *const _,
            usage,
        );
    }

    pub unsafe fn bind(&self) {
        gl::BindBuffer(self.target, self.id);
    }
}

// VERTX Array
pub struct VertexArray {
    pub id: GLuint,
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, [self.id].as_ptr());
        }
    }
}

impl VertexArray {
    pub unsafe fn new() -> Self {
        let mut id: GLuint = 0;
        gl::GenVertexArrays(1, &mut id);
        Self { id }
    }

    pub unsafe fn set_attribute<V: Sized>(
        &self,
        attrib_pos: GLuint,
        components: GLint,
        offset: GLint,
    ) {
        self.bind();
        gl::VertexAttribPointer(
            attrib_pos,
            components,
            gl::FLOAT,
            gl::FALSE,
            std::mem::size_of::<V>() as GLint,
            offset as *const _,
        );
        gl::EnableVertexAttribArray(attrib_pos);
    }

    pub unsafe fn bind(&self) {
        gl::BindVertexArray(self.id);
    }
}

#[macro_export]
macro_rules! set_attribute {
    ($vbo:ident, $pos:tt, $t:ident :: $field:tt) => {{
        let dummy = core::mem::MaybeUninit::<$t>::uninit();
        let dummy_ptr = dummy.as_ptr();
        let member_ptr = core::ptr::addr_of!((*dummy_ptr).$field);
        const fn size_of_raw<T>(_: *const T) -> usize {
            core::mem::size_of::<T>()
        }
        let member_offset = member_ptr as i32 - dummy_ptr as i32;
        $vbo.set_attribute::<$t>(
            $pos,
            (size_of_raw(member_ptr) / core::mem::size_of::<f32>()) as i32,
            member_offset,
        )
    }};
}

// Renderer
const VERTEX_SHADER_SOURCE: &str = r#"
#version 330
in vec2 position;
in vec3 color;
out vec3 vertexColor;
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    vertexColor = color;
}
"#;

const FRAGMENT_SHADER_SOURCE: &str = r#"
#version 330
out vec4 FragColor;
in vec3 vertexColor;
void main() {
    FragColor = vec4(vertexColor, 1.0);
}
"#;

type Pos = [f32; 2];
type Color = [f32; 3];

#[repr(C, packed)]
struct Vertex(Pos, Color);

#[rustfmt::skip]
const VERTICES: [Vertex; 3] = [
    Vertex([-0.5, -0.5], [1.0, 0.0, 0.0]),
    Vertex([0.5,  -0.5], [0.0, 1.0, 0.0]),
    Vertex([0.0,   0.5], [0.0, 1.0, 1.0])
];

pub struct Renderer {
    program: ShaderProgram,
    _vertex_buffer: Buffer,
    vertex_array: VertexArray,
}

impl Renderer {
    pub fn new() -> Result<Self, ShaderError> {
        unsafe {
            let vertex_shader = Shader::new(VERTEX_SHADER_SOURCE, gl::VERTEX_SHADER)?;
            let fragment_shader = Shader::new(FRAGMENT_SHADER_SOURCE, gl::FRAGMENT_SHADER)?;
            let program = ShaderProgram::new(&[vertex_shader, fragment_shader])?;
            let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER);
            vertex_buffer.set_data(&VERTICES, gl::STATIC_DRAW);
            let vertex_array = VertexArray::new();
            let pos_attrib = program.get_attrib_location("position")?;
            set_attribute!(vertex_array, pos_attrib, Vertex::0);
            let color_attrib = program.get_attrib_location("color")?;
            set_attribute!(vertex_array, color_attrib, Vertex::1);
            Ok(Self {
                program,
                _vertex_buffer: vertex_buffer,
                vertex_array,
            })
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            self.program.apply();
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}



pub fn build_window() -> () {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("OpenGl");

    let gl_context =
        ContextBuilder::new()
            .with_gl(
                GlRequest::Specific(
                    Api::OpenGl,
                    (3,3)
                )
            )
            .build_windowed(window, &event_loop)
            .expect("Cannot create windowed context");

    let gl_context = unsafe {
        gl_context
            .make_current()
            .expect("Failed to make context current")
    };

    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    let renderer = Renderer::new().expect("Cannot create renderer");
    event_loop.run(
        move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            match event {
                Event::LoopDestroyed => (),
                Event::WindowEvent { event, ..} =>
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => gl_context.resize(physical_size),
                        _ => (),
                    },
                Event::RedrawRequested(_) => {
                    /*unsafe {
                        gl::ClearColor(1.0, 6.0, 0.0, 1.0);
                        gl::Clear(gl::COLOR_BUFFER_BIT);
                    }*/
                    renderer.draw();
                    gl_context.swap_buffers().unwrap();
                }
                _ => (),
            }
        });
}