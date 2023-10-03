use std::mem;
use std::os::raw::c_void;
use std::time::Duration;

use c_str_macro::c_str;
use cgmath::perspective;
use cgmath::prelude::SquareMatrix;
use gl::types::{GLfloat,GLsizei,GLsizeiptr};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

mod shader;
mod vertex;

use shader::Shader;
use vertex::Vertex;

//im_str!マクロを使用するために追加
#[macro_use]
extern crate imgui;

#[allow(dead_code)]
type Point3 = cgmath::Point3<f32>;
#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

const WINODW_WIDTH :u32 = 640;
const WINODW_HEIGHT :u32 = 480;
const FLOAT_NUM :usize = 3;
const VERTEX_NUM :usize = 3;
const BUF_LEN: usize = FLOAT_NUM * VERTEX_NUM;

fn main() {
    //SDL初期化
    let sdl_context = sdl2::init().unwrap();
    //VideoSubsystem構造体取得（ウィンドウやディスプレイの機能を担当）
    let video_subsystem = sdl_context.video().unwrap();

    //OpenGLのバージョンを指定する
    //gl_attrはこれ以降使用しないので、クロージャで囲む
    {
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3,1);
        let (major,minor) = gl_attr.context_version();
        println!("OK: init OpenGL: version = {}.{}",major,minor);
    }

    //ウィンドウ初期化
    let window = match video_subsystem
        .window("SDL",WINODW_WIDTH,WINODW_HEIGHT)
        .opengl()
        .position_centered()    //ウィンドウをディスプレイ中央に配置する
        .build()
        {
            Ok(window) => window,
            Err(err) => panic!("failed to build window: {:?}",err),
        };

    //GLContext構造体の作成とAPIの読み込み
    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);

    let shader = Shader::new("rsc/shader/shader.vs","rsc/shader/shader.fs");

    //set buffer
    #[rustfmt::skip]
    let buffer_array:[f32;BUF_LEN] = [
        -1.0, -1.0, 0.0,
        1.0, -1.0, 0.0,
        0.0, 1.0, 0.0,
    ];
    let vertex = Vertex::new(
        (BUF_LEN * mem::size_of::<GLfloat>()) as GLsizeiptr,
        buffer_array.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
        vec![gl::FLOAT],
        vec![FLOAT_NUM as i32],
        FLOAT_NUM as i32 * mem::size_of::<GLfloat>() as GLsizei,
        VERTEX_NUM as i32,
    );

    //init imgui
    let mut imgui_context = imgui::Context::create();
    imgui_context.set_ini_filename(None);               //DearImGuiにはソフト終了時に自動でウィジェットの位置などを保存する機能があるため、それを無効化する。

    //init imgui sdl2
    let mut imgui_sdl2_context = imgui_sdl2::ImguiSdl2::new(&mut imgui_context, &window);
    let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui_context,|s|{
        video_subsystem.gl_get_proc_address(s) as _
    });

    let mut event_pump = sdl_context.event_pump().unwrap();

    //閉じるまたはEscapeキー押下で終了
    'running: loop {
        for event in event_pump.poll_iter() {
            //Imgui由来のイベントはImgui側で処理するため無視する
            imgui_sdl2_context.handle_event(&mut imgui_context, &event);
            if imgui_sdl2_context.ignore_event(&event){
                continue;
            }

            match event {
                Event::Quit {..}
                | Event::KeyDown {
                    keycode:Some( Keycode::Escape ),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        unsafe {
            gl::Viewport(0,0,WINODW_WIDTH as i32, WINODW_HEIGHT as i32);

            //clear screen
            gl::ClearColor(1.0,1.0,1.0,1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            //init matrice for model, view and projection
            let model_matrix = Matrix4::identity();
            let view_matrix = Matrix4::look_at(
                Point3{
                    x:0.0,
                    y:0.0,
                    z:5.0,
                },
                Point3{
                    x:0.0,
                    y:0.0,
                    z:0.0,
                },
                Vector3{
                    x:0.0,
                    y:1.0,
                    z:0.0,
                },
            );
            let projection_matrix:Matrix4 = perspective(
                cgmath::Deg(45.0f32),
                WINODW_WIDTH as f32 / WINODW_HEIGHT as f32,
                0.1,
                100.0
            );

            //shader use matrices
            shader.use_program();
            shader.set_mat4(c_str!("uModel"), &model_matrix);
            shader.set_mat4(c_str!("uView"), &view_matrix);
            shader.set_mat4(c_str!("uProjection"), &projection_matrix);

            vertex.draw();
            //SDL2における画面サイズやマウス、キーボードの状態などをImguiと調整する
            imgui_sdl2_context.prepare_frame(
                imgui_context.io_mut(),
                &window,
                &event_pump.mouse_state(),
            );

            //ウィジェットの生成を助けるUI構造体を生成する
            let ui = imgui_context.frame();
            imgui::Window::new(im_str!("Imformation"))
                .size([300.0,200.0],imgui::Condition::FirstUseEver)
                .build(&ui,||{});

            //SDL2とImGuiの間でマウスの位置やマウスのカーソルの情報を調整する
            imgui_sdl2_context.prepare_render(&ui, &window);
            //SDL2ウィンドウに描画する
            renderer.render(ui);

            window.gl_swap_window();
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
