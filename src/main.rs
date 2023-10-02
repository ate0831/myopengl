use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;


fn main() {
    //SDL初期化
    let sdl_context = sdl2::init().unwrap();
    //VideoSubsystem構造体取得（ウィンドウやディスプレイの機能を担当）
    let video_subsystem = sdl_context.video().unwrap();

    //ウィンドウ初期化
    let window = match video_subsystem
        .window("SDL",640,480)
        .position_centered()    //ウィンドウをディスプレイ中央に配置する
        .build()
        {
            Ok(window) => window,
            Err(err) => panic!("failed to build window: {:?}",err),
        };

    //キャンバスの取得
    let mut canvas = window.into_canvas().build().unwrap();
    //色の設定
    canvas.set_draw_color( Color::RGB( 255, 255, 255 ) );
    //塗りつぶして消去
    canvas.clear();
    //レンダリングを画面に反映
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    //閉じるまたはEscapeキー押下で終了
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown {
                    keycode:Some( Keycode::Escape ),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        //画面の更新（現段階では必要なし）
        canvas.present();

        //描画を60fpsに調整
        ::std::thread::sleep( Duration::new( 0, 1_000_000_000u32/60 ) );
    }
}
