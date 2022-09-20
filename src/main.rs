use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};
use ggez::input::keyboard::{self, KeyCode};
use ggez::mint;
use ggez::{Context, GameResult};
use rand::{self, thread_rng, Rng};

const WINDOW_WIDTH: f32 = 1000.0;
const WINDOW_HEIGHT: f32 = 800.0;
const PADDING: f32 = 40.0;
const MIDDLE_LINE_W: f32 = 2.0;
const RACKET_HEIGHT: f32 = 100.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT * 0.5;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5;
const BALL_SIZE: f32 = 30.0;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;
const PLAYER_SPEED: f32 = 500.0;
const BALL_SPEED: f32 = 300.0;

fn clamp(value: &mut f32, low: f32, high: f32) {
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}
fn move_racket(pos: &mut mint::Point2<f32>, key_code: KeyCode, y_dir: f32, ctx: &mut Context) {
    let dt = ggez::timer::delta(ctx).as_secs_f32();
    let screen_h = graphics::drawable_size(ctx).1;

    if keyboard::is_key_pressed(ctx, key_code) {
        pos.y += y_dir * PLAYER_SPEED * dt;
    }

    clamp(
        &mut pos.y,
        RACKET_HEIGHT_HALF,
        screen_h - RACKET_HEIGHT_HALF,
    );
}

fn randomize_vec(vec: &mut mint::Vector2<f32>, x: f32, y: f32) {
    let mut rng = thread_rng();
    vec.x = match rng.gen_bool(0.5) {
        true => x,
        false => -x,
    };
    vec.y = match rng.gen_bool(0.5) {
        true => y,
        false => -y,
    };
}

struct MainState {
    player_1_position: mint::Point2<f32>,
    player_2_position: mint::Point2<f32>,
    ball_position: mint::Point2<f32>,
    ball_velocity: mint::Vector2<f32>,
    player_1_score: i32,
    player_2_score: i32,
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        let (screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);
        let mut ball_velocity = mint::Vector2::from([10.0, 5.0]);
        randomize_vec(&mut ball_velocity, BALL_SPEED, BALL_SPEED);

        MainState {
            player_1_position: mint::Point2::from([RACKET_WIDTH_HALF + PADDING, screen_h_half]),
            player_2_position: mint::Point2::from([
                screen_w - RACKET_WIDTH_HALF - PADDING,
                screen_h_half,
            ]),
            ball_position: mint::Point2::from([screen_w_half, screen_h_half]),
            ball_velocity,
            player_1_score: 0,
            player_2_score: 0,
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        // Player 1
        move_racket(&mut self.player_1_position, KeyCode::W, -1.0, ctx);
        move_racket(&mut self.player_1_position, KeyCode::S, 1.0, ctx);
        // Player 2
        move_racket(&mut self.player_2_position, KeyCode::Up, -1.0, ctx);
        move_racket(&mut self.player_2_position, KeyCode::Down, 1.0, ctx);

        // Ball movement
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        self.ball_position.x += self.ball_velocity.x * dt;
        self.ball_position.y += self.ball_velocity.y * dt;

        if self.ball_position.x < 0.0 {
            self.ball_position.x = screen_w * 0.5;
            self.ball_position.y = screen_h * 0.5;
            randomize_vec(&mut self.ball_velocity, BALL_SPEED, BALL_SPEED);
            self.player_2_score += 1;
        }

        if self.ball_position.x > screen_w {
            self.ball_position.x = screen_w * 0.5;
            self.ball_position.y = screen_h * 0.5;
            randomize_vec(&mut self.ball_velocity, BALL_SPEED, BALL_SPEED);
            self.player_1_score += 1;
        }

        // Bounce Ball
        if self.ball_position.y < BALL_SIZE_HALF {
            self.ball_position.y = BALL_SIZE_HALF;
            self.ball_velocity.y = self.ball_velocity.y.abs();
        } else if self.ball_position.y > screen_h - BALL_SIZE_HALF {
            self.ball_position.y = screen_h - BALL_SIZE_HALF;
            self.ball_velocity.y = -self.ball_velocity.y.abs();
        }

        let ball_intersects_player_1 = self.ball_position.x - BALL_SIZE_HALF
            < self.player_1_position.x + RACKET_WIDTH_HALF
            && self.ball_position.x + BALL_SIZE_HALF > self.player_1_position.x - RACKET_WIDTH_HALF
            && self.ball_position.y - BALL_SIZE_HALF
                < self.player_1_position.y + RACKET_HEIGHT_HALF
            && self.ball_position.y + BALL_SIZE_HALF
                > self.player_1_position.y - RACKET_HEIGHT_HALF;

        if ball_intersects_player_1 {
            self.ball_velocity.x = self.ball_velocity.x.abs();
        }

        let ball_intersects_player_2 = self.ball_position.x - BALL_SIZE_HALF
            < self.player_2_position.x + RACKET_WIDTH_HALF
            && self.ball_position.x + BALL_SIZE_HALF > self.player_2_position.x - RACKET_WIDTH_HALF
            && self.ball_position.y - BALL_SIZE_HALF
                < self.player_2_position.y + RACKET_HEIGHT_HALF
            && self.ball_position.y + BALL_SIZE_HALF
                > self.player_2_position.y - RACKET_HEIGHT_HALF;

        if ball_intersects_player_2 {
            self.ball_velocity.x = -self.ball_velocity.x.abs();
        }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);

        // Middle Line
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        let middle_line = graphics::Rect::new(MIDDLE_LINE_W * 0.5, 0.0, MIDDLE_LINE_W, screen_h);
        let middle_line_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            middle_line,
            Color::WHITE,
        )?;
        let draw_param =
            graphics::DrawParam::default().dest(mint::Point2::from([screen_w * 0.5, 0.0]));
        let _ = graphics::draw(ctx, &middle_line_mesh, draw_param);

        // Ball
        let ball_rect = graphics::Rect::new(-BALL_SIZE_HALF, -BALL_SIZE_HALF, BALL_SIZE, BALL_SIZE);
        let ball_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            ball_rect,
            Color::WHITE,
        )?;

        let draw_param = graphics::DrawParam::default().dest(self.ball_position);
        let _ = graphics::draw(ctx, &ball_mesh, draw_param);

        // Racket
        let racket_rect = graphics::Rect::new(
            -RACKET_WIDTH_HALF,
            -RACKET_HEIGHT_HALF,
            RACKET_WIDTH,
            RACKET_HEIGHT,
        );
        let racket_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            racket_rect,
            Color::WHITE,
        )?;

        // Player 1 display
        let draw_param = graphics::DrawParam::default().dest(self.player_1_position);
        let _ = graphics::draw(ctx, &racket_mesh, draw_param);

        // Player 2 display
        let draw_param = graphics::DrawParam::default().dest(self.player_2_position);
        let _ = graphics::draw(ctx, &racket_mesh, draw_param);

        // Score Display
        let mut score_text = graphics::Text::new(format!(
            "{}    {}",
            self.player_1_score, self.player_2_score
        ));
        score_text.set_font(graphics::Font::default(), graphics::PxScale::from(64.0));
        let score_text_w = score_text.width(ctx);
        let screen_w = graphics::drawable_size(ctx).0;
        let screen_w_half = screen_w * 0.5;
        let score_pos = mint::Point2::from([screen_w_half - (score_text_w * 0.5), 40.0]);
        let draw_param = graphics::DrawParam::default().dest(score_pos);
        let _ = graphics::draw(ctx, &score_text, draw_param);

        graphics::present(ctx)
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Pong", "Hann0t")
        .window_mode(ggez::conf::WindowMode::default().dimensions(WINDOW_WIDTH, WINDOW_HEIGHT));
    let (mut ctx, event_loop) = cb.build()?;
    graphics::set_window_title(&ctx, "PONG");
    let state = MainState::new(&mut ctx);
    event::run(ctx, event_loop, state);
}
