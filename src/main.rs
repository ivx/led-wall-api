// #![feature(proc_macro_hygiene, decl_macro)]
#[allow(clippy::all)]
use redis::Commands;

use ggez::{
    conf::FullscreenType,
    event,
    event::{KeyCode, KeyMods},
    graphics, Context, GameResult,
};
//--------------------------------------------

const APP_NAME: &str = "led-wall";

struct MainState {
    frames: usize,
    stop: bool,
    color: graphics::Color,
    con: redis::Connection,
    debug_rect: graphics::Rect,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let color = [0.1, 0.2, 0.3, 1.0].into();
        let client = redis::Client::open("redis://127.0.0.1/").unwrap();
        let con = client.get_connection().unwrap();

        let debug_rect = graphics::Rect::new_i32(0, 0, 0, 0);

        Ok(MainState {
            frames: 0,
            stop: false,
            color,
            con,
            debug_rect,
        })
    }

    fn strobe_colors(&mut self) {
        match self.frames % 6 {
            0 => self.color = [1.0, 0.0, 0.0, 1.0].into(),
            2 => self.color = [0.0, 1.0, 0.0, 1.0].into(),
            4 => self.color = [0.0, 0.0, 1.0, 1.0].into(),
            _ => (),
        }
    }

    fn read_color_from_redis(&mut self) {
        let result = self.con.get(&[
            "background:color:red",
            "background:color:green",
            "background:color:blue",
        ]);

        // if result.is_ok() {
        //     let (r, g, b): (u8, u8, u8) = result.unwrap();
        //     self.color = (r, g, b, 0).into();
        // }

        if let Ok(res) = result {
            let (r, g, b): (u8, u8, u8) = res;
            self.color = (r, g, b, 0).into();
        }

        // let result = self.con.hgetall("debug_rect");

        // if result.is_ok() {
        //     let map: HashMap<String, i32> = result.unwrap();

        //     let to_find =
        //         ["x", "y", "width", "height"];
        //     println!("{:?}", map);

        //     for &field in &to_find {
        //         match map.get(field) {
        //             Some(value) => println!("{}: {}", field, value),
        //             None => println!("{} is unreviewed.", field),
        //         }
        //     }
        // }
    }

    fn draw_debug_rect(&mut self, ctx: &mut Context) -> GameResult {
        let rect_color = [1.0, 0.33, 0.23, 1.0].into();

        println!("{:?}", self.debug_rect);

        let debug_rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.debug_rect,
            rect_color,
        )?;

        graphics::draw(
            ctx,
            &debug_rect,
            (ggez::mint::Point2 { x: 0.0, y: 0.0 },),
        )
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
        if self.stop {
            event::quit(ctx);
        }
        self.update_frames(ctx);
        self.read_color_from_redis();
        // self.strobe_colors();

        graphics::clear(ctx, self.color);

        self.draw_debug_rect(ctx)?;

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
            KeyCode::W => self.debug_rect.y -= 1.0,
            KeyCode::A => self.debug_rect.x -= 1.0,
            KeyCode::S => self.debug_rect.y += 1.0,
            KeyCode::D => self.debug_rect.x += 1.0,
            KeyCode::Q => {
                self.debug_rect.w += 1.0;
                self.debug_rect.h += 1.0;
            }
            KeyCode::E => {
                self.debug_rect.w -= 1.0;
                self.debug_rect.h -= 1.0;
            }
            KeyCode::R => self.color = [1.0, 0.0, 0.0, 1.0].into(),
            KeyCode::G => self.color = [0.0, 1.0, 0.0, 1.0].into(),
            KeyCode::B => self.color = [0.0, 0.0, 1.0, 1.0].into(),
            KeyCode::Escape => self.stop = true,
            _ => (),
        }
    }
}

pub fn main() -> GameResult {
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new(APP_NAME, "ggez")
        .backend(ggez::conf::Backend::OpenGLES{major: 3, minor: 1})
        // .backend(ggez::conf::Backend::OpenGL { major: 3, minor: 2 })
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title(APP_NAME)
                .vsync(false),
        )
        .window_mode(
            ggez::conf::WindowMode {
                width: 1920.0,
                height: 1080.0,
                // width: 600.0,
                // height: 600.0,
                maximized: true,
                fullscreen_type: FullscreenType::Windowed,
                borderless: true,
                min_width: 0.0,
                max_width: 0.0,
                min_height: 0.0,
                max_height: 0.0,
                resizable: true,
            }
            .borderless(true)
            .resizable(true),
        )
        .build()?;

    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
