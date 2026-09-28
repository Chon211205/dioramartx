use raylib::prelude::*;

const WIDTH: i32 = 960;
const HEIGHT: i32 = 640;
const COURSE_LENGTH: f32 = 6400.0;
const PLAYER_Y: f32 = 475.0;
const HORIZON_Y: f32 = 145.0;

#[derive(Clone, Copy, PartialEq)]
enum GameState {
    Title,
    Countdown,
    Racing,
    Finished,
}

#[derive(Clone, Copy)]
struct Gate {
    y: f32,
    x: f32,
    width: f32,
    passed: bool,
}

#[derive(Clone, Copy)]
struct Rock {
    y: f32,
    x: f32,
    radius: f32,
}

#[derive(Clone, Copy)]
struct WakeParticle {
    position: Vector2,
    velocity: Vector2,
    life: f32,
    size: f32,
}

struct Game {
    state: GameState,
    player_x: f32,
    speed: f32,
    distance: f32,
    elapsed: f32,
    countdown: f32,
    boost: f32,
    health: i32,
    gates: Vec<Gate>,
    rocks: Vec<Rock>,
    wake: Vec<WakeParticle>,
    missed: u32,
    shake: f32,
}

impl Game {
    fn new() -> Self {
        let mut game = Self {
            state: GameState::Title,
            player_x: WIDTH as f32 * 0.5,
            speed: 0.0,
            distance: 0.0,
            elapsed: 0.0,
            countdown: 3.0,
            boost: 1.0,
            health: 3,
            gates: Vec::new(),
            rocks: Vec::new(),
            wake: Vec::new(),
            missed: 0,
            shake: 0.0,
        };
        game.build_course();
        game
    }

    fn build_course(&mut self) {
        self.gates.clear();
        self.rocks.clear();

        for i in 0..18 {
            let y = 420.0 + i as f32 * 320.0;
            let wave = (i as f32 * 1.37).sin();
            self.gates.push(Gate {
                y,
                x: WIDTH as f32 * 0.5 + wave * 245.0,
                width: if i % 5 == 4 { 190.0 } else { 235.0 },
                passed: false,
            });
        }

        for i in 0..26 {
            let y = 650.0 + i as f32 * 215.0;
            let x = 115.0 + ((i * 173) % 730) as f32;
            self.rocks.push(Rock {
                y,
                x,
                radius: 24.0 + (i % 3) as f32 * 7.0,
            });
        }
    }

    fn start(&mut self) {
        self.state = GameState::Countdown;
        self.player_x = WIDTH as f32 * 0.5;
        self.speed = 0.0;
        self.distance = 0.0;
        self.elapsed = 0.0;
        self.countdown = 3.0;
        self.boost = 1.0;
        self.health = 3;
        self.missed = 0;
        self.shake = 0.0;
        self.wake.clear();
        self.build_course();
    }

    fn update(&mut self, rl: &mut RaylibHandle, dt: f32) {
        if self.state == GameState::Title {
            if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                self.start();
            }
            return;
        }

        if self.state == GameState::Finished {
            if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                self.start();
            }
            if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                self.state = GameState::Title;
            }
            return;
        }

        if self.state == GameState::Countdown {
            self.countdown -= dt;
            if self.countdown <= 0.0 {
                self.state = GameState::Racing;
                self.speed = 280.0;
            }
            return;
        }

        self.elapsed += dt;
        self.shake = (self.shake - dt).max(0.0);

        let mut steering = 0.0;
        if rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_A) {
            steering -= 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) || rl.is_key_down(KeyboardKey::KEY_D) {
            steering += 1.0;
        }

        let boosting = (rl.is_key_down(KeyboardKey::KEY_SPACE)
            || rl.is_key_down(KeyboardKey::KEY_UP))
            && self.boost > 0.02;
        let target_speed = if boosting { 555.0 } else { 360.0 };
        self.speed += (target_speed - self.speed) * (2.7 * dt).min(1.0);
        self.player_x += steering * (315.0 + self.speed * 0.18) * dt;
        self.player_x = self.player_x.clamp(70.0, WIDTH as f32 - 70.0);

        if boosting {
            self.boost = (self.boost - 0.28 * dt).max(0.0);
        } else {
            self.boost = (self.boost + 0.10 * dt).min(1.0);
        }

        let previous_distance = self.distance;
        self.distance += self.speed * dt;

        for gate in &mut self.gates {
            if !gate.passed && previous_distance < gate.y && self.distance >= gate.y {
                gate.passed = true;
                if (self.player_x - gate.x).abs() <= gate.width * 0.5 {
                    self.boost = (self.boost + 0.24).min(1.0);
                } else {
                    self.missed += 1;
                    self.speed *= 0.68;
                    self.shake = 0.28;
                }
            }
        }

        for rock in &self.rocks {
            if previous_distance < rock.y + 18.0 && self.distance >= rock.y + 18.0 {
                if (self.player_x - rock.x).abs() < rock.radius + 31.0 {
                    self.health -= 1;
                    self.speed *= 0.48;
                    self.shake = 0.45;
                    if self.health <= 0 {
                        self.health = 3;
                        self.distance = (self.distance - 280.0).max(0.0);
                        self.player_x = WIDTH as f32 * 0.5;
                    }
                }
            }
        }

        self.spawn_wake(steering, boosting);
        for particle in &mut self.wake {
            particle.position.x += particle.velocity.x * dt;
            particle.position.y += particle.velocity.y * dt;
            particle.life -= dt;
        }
        self.wake.retain(|particle| particle.life > 0.0);

        if self.distance >= COURSE_LENGTH {
            self.distance = COURSE_LENGTH;
            self.state = GameState::Finished;
        }
    }

    fn spawn_wake(&mut self, steering: f32, boosting: bool) {
        if self.wake.len() > 120 {
            return;
        }
        let phase = self.distance * 0.09;
        for side in [-1.0_f32, 1.0] {
            self.wake.push(WakeParticle {
                position: Vector2::new(
                    self.player_x + side * (30.0 + phase.sin() * 3.0),
                    PLAYER_Y + 20.0,
                ),
                velocity: Vector2::new(side * (18.0 + steering * 12.0), 85.0),
                life: if boosting { 0.75 } else { 0.48 },
                size: if boosting { 7.0 } else { 4.5 },
            });
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Tide Dash - Carrera 2.5D")
        .resizable()
        .build();
    rl.set_target_fps(60);

    let manta_sheet = rl
        .load_texture(&thread, "assets/sprites/manta_ray_sheet.png")
        .expect("No se pudo cargar assets/sprites/manta_ray_sheet.png");

    let mut game = Game::new();
    while !rl.window_should_close() {
        let dt = rl.get_frame_time().min(0.033);
        game.update(&mut rl, dt);
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(6, 40, 76, 255));
        draw_game(&mut d, &game, &manta_sheet);
    }
}

fn draw_game(d: &mut RaylibDrawHandle, game: &Game, manta_sheet: &Texture2D) {
    let shake_x = if game.shake > 0.0 {
        (game.elapsed * 91.0).sin() * 7.0
    } else {
        0.0
    };
    let shake_y = if game.shake > 0.0 {
        (game.elapsed * 73.0).cos() * 5.0
    } else {
        0.0
    };

    draw_water(d, game.distance, shake_x, shake_y);
    draw_course(d, game, shake_x, shake_y);

    for particle in &game.wake {
        let alpha = (particle.life * 300.0).clamp(0.0, 180.0) as u8;
        d.draw_circle_v(
            particle.position,
            particle.size,
            Color::new(190, 245, 255, alpha),
        );
    }

    draw_rider(
        d,
        manta_sheet,
        game.player_x + shake_x,
        PLAYER_Y + shake_y,
        game.elapsed,
    );
    draw_hud(d, game);

    match game.state {
        GameState::Title => draw_title(d),
        GameState::Countdown => draw_countdown(d, game.countdown),
        GameState::Finished => draw_finish(d, game),
        GameState::Racing => {}
    }
}

fn draw_water(d: &mut RaylibDrawHandle, distance: f32, shake_x: f32, shake_y: f32) {
    d.draw_rectangle_gradient_v(
        0,
        0,
        WIDTH,
        HEIGHT,
        Color::new(6, 72, 125, 255),
        Color::new(4, 35, 78, 255),
    );
    d.draw_circle_gradient(
        WIDTH / 2,
        HORIZON_Y as i32 - 18,
        86.0,
        Color::new(255, 226, 139, 190),
        Color::new(255, 226, 139, 0),
    );

    // Límites de la pista convergen hacia el horizonte y producen profundidad 3D.
    let vanishing = Vector2::new(WIDTH as f32 * 0.5 + shake_x, HORIZON_Y + shake_y);
    d.draw_triangle(
        vanishing,
        Vector2::new(28.0 + shake_x, HEIGHT as f32),
        Vector2::new(95.0 + shake_x, HEIGHT as f32),
        Color::new(3, 26, 49, 225),
    );
    d.draw_triangle(
        vanishing,
        Vector2::new(WIDTH as f32 - 28.0 + shake_x, HEIGHT as f32),
        Vector2::new(WIDTH as f32 - 95.0 + shake_x, HEIGHT as f32),
        Color::new(3, 26, 49, 225),
    );

    for lane in -4..=4 {
        let bottom_x = WIDTH as f32 * 0.5 + lane as f32 * 112.0 + shake_x;
        d.draw_line_ex(
            vanishing,
            Vector2::new(bottom_x, HEIGHT as f32),
            1.5,
            Color::new(100, 220, 234, 42),
        );
    }

    // Franjas que se acercan a cámara; el espaciado aumenta con la perspectiva.
    for row in 0..18 {
        let depth = ((row as f32 * 115.0 - distance * 0.55).rem_euclid(2070.0)) + 25.0;
        let (_, y, scale) = project_point(WIDTH as f32 * 0.5, depth + distance, distance);
        let half_width = 430.0 * scale;
        d.draw_line_ex(
            Vector2::new(WIDTH as f32 * 0.5 - half_width + shake_x, y + shake_y),
            Vector2::new(WIDTH as f32 * 0.5 + half_width + shake_x, y + shake_y),
            (1.0 + 3.0 * scale).max(1.0),
            Color::new(120, 230, 240, (35.0 + 75.0 * scale) as u8),
        );
    }
}

fn project_point(world_x: f32, world_y: f32, distance: f32) -> (f32, f32, f32) {
    let depth = world_y - distance;
    let scale = if depth >= 0.0 {
        (1.0 / (1.0 + depth / 620.0)).clamp(0.12, 1.0)
    } else {
        (1.0 - depth / 210.0).clamp(1.0, 1.7)
    };
    let screen_x = WIDTH as f32 * 0.5 + (world_x - WIDTH as f32 * 0.5) * scale;
    let screen_y = if depth >= 0.0 {
        HORIZON_Y + (PLAYER_Y - HORIZON_Y) * scale
    } else {
        PLAYER_Y - depth * 1.15
    };
    (screen_x, screen_y, scale)
}

fn draw_course(d: &mut RaylibDrawHandle, game: &Game, sx: f32, sy: f32) {
    for gate in &game.gates {
        let (center_x, y, scale) = project_point(gate.x, gate.y, game.distance);
        let y = y + sy;
        if y < -80.0 || y > HEIGHT as f32 + 80.0 {
            continue;
        }
        let color = if gate.passed {
            Color::new(70, 120, 125, 120)
        } else {
            Color::new(255, 206, 54, 255)
        };
        let left = center_x - gate.width * 0.5 * scale + sx;
        let right = center_x + gate.width * 0.5 * scale + sx;
        let post_radius = 17.0 * scale;
        d.draw_circle_v(Vector2::new(left, y), post_radius, color);
        d.draw_circle_v(Vector2::new(right, y), post_radius, color);
        d.draw_line_ex(
            Vector2::new(left, y),
            Vector2::new(right, y),
            (5.0 * scale).max(1.0),
            Color::new(color.r, color.g, color.b, 105),
        );
        d.draw_triangle(
            Vector2::new(left - 13.0 * scale, y - 42.0 * scale),
            Vector2::new(left + 13.0 * scale, y - 42.0 * scale),
            Vector2::new(left, y - 5.0 * scale),
            color,
        );
        d.draw_triangle(
            Vector2::new(right - 13.0 * scale, y - 42.0 * scale),
            Vector2::new(right + 13.0 * scale, y - 42.0 * scale),
            Vector2::new(right, y - 5.0 * scale),
            color,
        );
    }

    for rock in &game.rocks {
        let (x, y, scale) = project_point(rock.x, rock.y, game.distance);
        let y = y + sy;
        if y < -70.0 || y > HEIGHT as f32 + 70.0 {
            continue;
        }
        let x = x + sx;
        let radius = rock.radius * scale;
        d.draw_ellipse(
            x as i32,
            (y + radius * 0.75) as i32,
            radius * 1.35,
            radius * 0.42,
            Color::new(4, 24, 39, 120),
        );
        d.draw_circle_v(
            Vector2::new(x + 5.0 * scale, y + 7.0 * scale),
            radius,
            Color::new(15, 34, 50, 135),
        );
        d.draw_circle_v(Vector2::new(x, y), radius, Color::new(62, 77, 85, 255));
        d.draw_circle_v(
            Vector2::new(x - radius * 0.25, y - radius * 0.3),
            radius * 0.35,
            Color::new(99, 117, 119, 255),
        );
    }

    let (_, finish_y, finish_scale) =
        project_point(WIDTH as f32 * 0.5, COURSE_LENGTH, game.distance);
    let finish_y = finish_y + sy;
    if finish_y > -100.0 && finish_y < HEIGHT as f32 + 100.0 {
        for i in 0..12 {
            let tile_width = 68.0 * finish_scale;
            let x = WIDTH as f32 * 0.5 - tile_width * 6.0 + i as f32 * tile_width;
            let color = if i % 2 == 0 {
                Color::WHITE
            } else {
                Color::new(20, 25, 40, 255)
            };
            d.draw_rectangle(
                x as i32,
                finish_y as i32,
                tile_width.ceil() as i32,
                (22.0 * finish_scale).max(2.0) as i32,
                color,
            );
        }
        let font_size = (30.0 * finish_scale).max(10.0) as i32;
        let label_width = d.measure_text("META", font_size);
        d.draw_text(
            "META",
            (WIDTH - label_width) / 2,
            finish_y as i32 - font_size - 7,
            font_size,
            Color::WHITE,
        );
    }
}

fn draw_rider(d: &mut RaylibDrawHandle, manta_sheet: &Texture2D, x: f32, y: f32, time: f32) {
    const COLUMNS: i32 = 4;
    const ROWS: i32 = 2;
    const FRAME_COUNT: i32 = 8;

    let frame = ((time * 9.0) as i32).rem_euclid(FRAME_COUNT);
    let frame_width = manta_sheet.width as f32 / COLUMNS as f32;
    let frame_height = manta_sheet.height as f32 / ROWS as f32;
    let source = Rectangle::new(
        (frame % COLUMNS) as f32 * frame_width,
        (frame / COLUMNS) as f32 * frame_height,
        frame_width,
        frame_height,
    );

    // La sombra sigue siendo 2D, mientras el sprite aporta volumen pintado.
    d.draw_ellipse(
        x as i32,
        (y + 43.0) as i32,
        62.0,
        15.0,
        Color::new(2, 22, 39, 105),
    );
    d.draw_texture_pro(
        manta_sheet,
        source,
        Rectangle::new(x, y, 150.0, 150.0),
        Vector2::new(75.0, 75.0),
        0.0,
        Color::WHITE,
    );
}

fn draw_hud(d: &mut RaylibDrawHandle, game: &Game) {
    d.draw_rectangle_rounded(
        Rectangle::new(22.0, 20.0, 270.0, 92.0),
        0.18,
        8,
        Color::new(2, 20, 42, 210),
    );
    d.draw_text(
        &format!("TIEMPO  {:.2}", game.elapsed),
        40,
        34,
        25,
        Color::WHITE,
    );
    d.draw_text(
        &format!(
            "ANILLOS  {}/{}",
            game.gates.iter().filter(|g| g.passed).count() - game.missed as usize,
            game.gates.len()
        ),
        40,
        72,
        19,
        Color::new(255, 220, 82, 255),
    );

    d.draw_text("IMPULSO", WIDTH - 258, 28, 18, Color::WHITE);
    d.draw_rectangle(WIDTH - 258, 54, 220, 20, Color::new(2, 20, 42, 220));
    d.draw_rectangle(
        WIDTH - 254,
        58,
        (212.0 * game.boost) as i32,
        12,
        Color::new(72, 235, 242, 255),
    );
    d.draw_text(
        &format!("VIDAS  {}", "◆".repeat(game.health as usize)),
        WIDTH - 258,
        88,
        18,
        Color::new(255, 116, 105, 255),
    );

    let progress = game.distance / COURSE_LENGTH;
    d.draw_rectangle(22, HEIGHT - 31, WIDTH - 44, 11, Color::new(2, 20, 42, 210));
    d.draw_rectangle(
        22,
        HEIGHT - 31,
        ((WIDTH - 44) as f32 * progress) as i32,
        11,
        Color::new(255, 208, 63, 255),
    );
}

fn draw_title(d: &mut RaylibDrawHandle) {
    d.draw_rectangle(0, 0, WIDTH, HEIGHT, Color::new(1, 12, 30, 165));
    centered_text(d, "TIDE DASH", 150, 64, Color::new(91, 240, 237, 255));
    centered_text(d, "CONTRARRELOJ DEL ARRECIFE 2.5D", 225, 25, Color::WHITE);
    centered_text(
        d,
        "A/D o flechas para girar",
        335,
        23,
        Color::new(210, 235, 242, 255),
    );
    centered_text(
        d,
        "ESPACIO para usar impulso",
        373,
        23,
        Color::new(210, 235, 242, 255),
    );
    centered_text(
        d,
        "Cruza los portales y evita las rocas",
        411,
        23,
        Color::new(255, 218, 96, 255),
    );
    centered_text(d, "PRESIONA ENTER", 505, 29, Color::WHITE);
}

fn draw_countdown(d: &mut RaylibDrawHandle, countdown: f32) {
    d.draw_rectangle(0, 0, WIDTH, HEIGHT, Color::new(1, 12, 30, 90));
    let label = if countdown > 2.0 {
        "3"
    } else if countdown > 1.0 {
        "2"
    } else if countdown > 0.0 {
        "1"
    } else {
        "¡YA!"
    };
    centered_text(d, label, 250, 100, Color::WHITE);
}

fn draw_finish(d: &mut RaylibDrawHandle, game: &Game) {
    d.draw_rectangle(0, 0, WIDTH, HEIGHT, Color::new(1, 12, 30, 185));
    centered_text(
        d,
        "¡RECORRIDO COMPLETO!",
        165,
        43,
        Color::new(255, 218, 67, 255),
    );
    centered_text(
        d,
        &format!("Tiempo: {:.2} s", game.elapsed),
        260,
        35,
        Color::WHITE,
    );
    centered_text(
        d,
        &format!("Portales fallados: {}", game.missed),
        310,
        25,
        Color::new(185, 230, 240, 255),
    );
    let rank = if game.elapsed < 14.0 && game.missed == 0 {
        "RANGO S"
    } else if game.elapsed < 18.0 {
        "RANGO A"
    } else {
        "RANGO B"
    };
    centered_text(d, rank, 375, 38, Color::new(91, 240, 237, 255));
    centered_text(d, "ENTER: otra carrera   ESC: menú", 500, 23, Color::WHITE);
}

fn centered_text(d: &mut RaylibDrawHandle, text: &str, y: i32, size: i32, color: Color) {
    let width = d.measure_text(text, size);
    d.draw_text(text, (WIDTH - width) / 2, y, size, color);
}
