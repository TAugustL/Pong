use sdl2::{
    event::Event,
    keyboard::{Keycode, Scancode},
    pixels::Color,
    rect::Rect,
    render::WindowCanvas,
    video::Window,
};
use std::{io, time::Duration};

pub struct Renderer {
    canvas: WindowCanvas,
}

pub struct Player {
    texture: Rect,
    score: u32,
}

pub struct Ball {
    texture: Rect,
    velocity: (i32, i32),
}

impl Renderer {
    pub fn new(window: Window) -> Result<Renderer, String> {
        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        Ok(Renderer { canvas })
    }

    fn draw_shape(&mut self, texture: Rect) -> Result<(), String> {
        self.canvas.fill_rect(Rect::new(
            texture.x,
            texture.y,
            texture.w as u32,
            texture.h as u32,
        ))?;

        Ok(())
    }

    pub fn draw(&mut self, player: &[Player; 2], ball: &[&Ball]) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        self.canvas.set_draw_color(Color::WHITE);
        for i in 0..player.len() {
            self.draw_shape(player[i].texture)?;
        }
        for i in 0..ball.len() {
            self.draw_shape(ball[i].texture)?;
        }

        self.draw_shape(Rect::new(399, 0, 1, 800))?;

        self.canvas.present();

        Ok(())
    }
}

impl Ball {
    pub fn mv(&mut self, player: &[Player; 2]) {
        self.texture.offset(self.velocity.0, self.velocity.1);

        if self.texture.x <= 0 || self.texture.x >= 800 - self.texture.w {
            self.velocity.0 *= -1;
        }
        if self.texture.y <= 0 || self.texture.y >= 800 - self.texture.h {
            self.velocity.1 *= -1;
        }

        if self.velocity.0 == 0 || self.velocity.1 == 0 {
            self.velocity = (10, 10);
        }

        for i in 0..player.len() {
            let pl_text = player[i].texture;
            if self.texture.has_intersection(pl_text) {
                let intersect = self.texture.intersection(pl_text).unwrap();
                let rel_pos = self.check_rel_pos(pl_text);

                self.texture.x += intersect.width() as i32 * rel_pos.0;
                self.texture.y += intersect.height() as i32 * rel_pos.1;

                if self.texture.y + self.texture.h > pl_text.y + pl_text.h
                    || self.texture.y < pl_text.y
                {
                    self.velocity.1 *= -1;
                } else {
                    self.velocity.0 *= -1;
                }
            }
        }
    }

    pub fn check_rel_pos(&mut self, other: Rect) -> (i32, i32) {
        let oth_center_point: (i32, i32) = (other.x + (other.w / 2), other.y + (other.h / 2));
        let bal_center_point: (i32, i32) = (
            self.texture.x + (self.texture.w / 2),
            self.texture.y + (self.texture.h / 2),
        );

        let mut rel_pos: (i32, i32) = (0, 0);
        if bal_center_point.0 > oth_center_point.0 {
            rel_pos.0 = 1;
        } else if bal_center_point.0 < oth_center_point.0 {
            rel_pos.0 = -1;
        }
        if self.texture.y + self.texture.h > other.y + other.h || self.texture.y < other.y {
            if bal_center_point.1 > oth_center_point.1 {
                rel_pos.1 = -1;
            } else if bal_center_point.1 < oth_center_point.1 {
                rel_pos.1 = 1;
            }
        }
        return rel_pos;
    }
}

impl Player {
    pub fn mv(&mut self, direction: i32) {
        self.texture.offset(0, direction);
        if self.texture.y <= 0 + direction || self.texture.y >= 800 - self.texture.h + direction {
            self.texture.y = self.texture.y.clamp(0, 800 - self.texture.h);
        }
    }
}

pub fn main() -> Result<(), String> {
    println!("[S]ingleplayer\n[M]ultiplayer\n[B]ot fight");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut mode: &str = input.trim();

    if ["s", "1", "single", "singleplayer"].contains(&mode.to_lowercase().as_str()) {
        mode = "single";
    } else if ["m", "2", "multi", "multiplayer"].contains(&mode.to_lowercase().as_str()) {
        mode = "multi";
    } else if ["b", "0", "botfight"].contains(&mode.to_lowercase().as_str()) {
        mode = "bots";
    } else {
        return Err(String::from("Invalid input!"));
    }

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("PONG", 800, 800)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let (window_scale_x, window_scale_y) = (&window.size().0, &window.size().1);

    let mut renderer = Renderer::new(window)?;

    let (player1, player2) = (
        Player {
            texture: Rect::new(100, 350, 15, 100),
            score: 0,
        },
        Player {
            texture: Rect::new(700, 350, 15, 100),
            score: 0,
        },
    );
    let mut player: [Player; 2] = [player1, player2];

    let mut ball = Ball {
        texture: Rect::new(
            (window_scale_x / 2) as i32,
            (window_scale_y / 2) as i32,
            15,
            15,
        ),
        velocity: (10, 10),
    };

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for keypress in event_pump.keyboard_state().pressed_scancodes() {
            if mode == "multi" {
                match keypress {
                    Scancode::W => player[0].mv(-15),
                    Scancode::S => player[0].mv(15),
                    Scancode::Up => player[1].mv(-15),
                    Scancode::Down => player[1].mv(15),
                    _ => {}
                }
            } else if mode == "single" {
                match keypress {
                    Scancode::W => player[0].mv(-15),
                    Scancode::S => player[0].mv(15),
                    _ => {}
                }
            }
        }
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        if mode == "single"
            && (ball.texture.x > (window_scale_x / 2) as i32 || player[1].score < player[0].score)
        {
            let pl2_text = player[1].texture;
            let in_range: bool = pl2_text.y + pl2_text.h / 2 + pl2_text.h / 4
                < ball.texture.y + ball.texture.h / 2
                || pl2_text.y + pl2_text.h / 2 - pl2_text.h / 4
                    > ball.texture.y + ball.texture.h / 2;

            if in_range && pl2_text.y + pl2_text.h / 2 > ball.texture.y + ball.texture.h / 2 {
                player[1].mv(-15);
            } else if in_range && pl2_text.y + pl2_text.h / 2 < ball.texture.y + ball.texture.h / 2
            {
                player[1].mv(15);
            }
        }

        if mode == "bots" {
            for i in 0..player.len() {
                let pli_text = player[i].texture;
                let in_range: bool = pli_text.y + pli_text.h / 2 + pli_text.h / 4
                    < ball.texture.y + ball.texture.h / 2
                    || pli_text.y + pli_text.h / 2 - pli_text.h / 4
                        > ball.texture.y + ball.texture.h / 2;

                if i == 0
                    && (ball.texture.x < (window_scale_x / 2) as i32
                        || player[0].score < player[1].score)
                {
                    if in_range && pli_text.y + pli_text.h / 2 > ball.texture.y + ball.texture.h / 2
                    {
                        player[i].mv(-15);
                    } else if in_range
                        && pli_text.y + pli_text.h / 2 < ball.texture.y + ball.texture.h / 2
                    {
                        player[i].mv(15);
                    }
                } else if i == 1
                    && (ball.texture.x + ball.texture.w > (window_scale_x / 2) as i32
                        || player[1].score < player[0].score)
                {
                    if in_range && pli_text.y + pli_text.h / 2 > ball.texture.y + ball.texture.h / 2
                    {
                        player[i].mv(-15);
                    } else if in_range
                        && pli_text.y + pli_text.h / 2 < ball.texture.y + ball.texture.h / 2
                    {
                        player[i].mv(15);
                    }
                }
            }
        }

        if ball.texture.x <= 0 {
            player[1].score += 1;
            println!("Score! P1: {}, P2: {}(+)", player[0].score, player[1].score);
            ball.texture.x = (window_scale_x / 2) as i32;
            ball.texture.y = 0;
            ball.velocity = (10, 10);
        } else if ball.texture.x >= 800 - ball.texture.w {
            player[0].score += 1;
            println!("Score! P1: {}(+), P2: {}", player[0].score, player[1].score);
            ball.texture.x = (window_scale_x / 2) as i32;
            ball.texture.y = *window_scale_y as i32 - ball.texture.h;
            ball.velocity = (-10, -10);
        }

        ball.mv(&player);
        renderer.draw(&player, &[&ball])?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    println!(
        "\n***Results:***\nPlayer 1: {}\nPlayer 2: {}\n",
        player[0].score, player[1].score
    );
    if player[0].score > player[1].score {
        println!("PLAYER 1 WINS!");
    } else if player[1].score > player[0].score {
        println!("PLAYER 2 WINS!");
    } else {
        println!("TIE!");
    }

    Ok(())
}
