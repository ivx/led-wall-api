#![feature(proc_macro_hygiene, decl_macro)]

// #[macro_use] extern crate rocket;

use redis;
use redis::Commands;

use ggez;

use ggez::{
    event,
    event::{KeyCode, KeyMods},
    graphics,
    Context,
    GameResult,
};
//--------------------------------------------

const APP_NAME: &str = "led-wall";

struct MainState {
    frames: usize,
    color: graphics::Color,
    con: redis::Connection,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let color = [0.1, 0.2, 0.3, 1.0].into();
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let con =  client.get_connection().unwrap();

        Ok(MainState {
            frames: 0,
            color,
            con,
        })
    }

    // fn strobe_colors(&mut self) {
    //     match self.frames % 6 {
    //         0 => self.color = [1.0, 0.0, 0.0, 1.0].into(),
    //         2 => self.color = [0.0, 1.0, 0.0, 1.0].into(),
    //         4 => self.color = [0.0, 0.0, 1.0, 1.0].into(),
    //         _ => (),
    //     }
    // }

    fn read_color_from_redis(&mut self) {
        let (r, g, b) : (String, String, String) = self.con.get(&["bg:r", "bg:b", "bg:b"]).unwrap();

        let (red, green, blue) = (r.parse::<f32>().unwrap(), g.parse::<f32>().unwrap(), b.parse::<f32>().unwrap());

        self.color = [red, green, blue, 1.0].into();
    }

    fn update_frames(&mut self, ctx: &mut Context) {
        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::fps(ctx));
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.update_frames(ctx);
        self.read_color_from_redis();
        // self.strobe_colors();

        graphics::clear(ctx, self.color);

        graphics::present(ctx)?;

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::R => self.color = [1.0, 0.0, 0.0, 1.0].into(),
            KeyCode::G => self.color = [0.0, 1.0, 0.0, 1.0].into(),
            KeyCode::B => self.color = [0.0, 0.0, 1.0, 1.0].into(),
            _ => (),
        }
    }
}

pub fn main() -> GameResult {
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new(APP_NAME, "ggez")
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title(APP_NAME)
                .vsync(false),
        )
        .window_mode(
            ggez::conf::WindowMode::default()
                // .dimensions(600.0, 600.0)
                .borderless(true)
                .resizable(true),
        )
        .build()?;

    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
