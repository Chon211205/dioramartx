use raylib::prelude::*;

const WIDTH: i32 = 960;
const HEIGHT: i32 = 640;
const COURSE_LENGTH: f32 = 12800.0;
const CIRCUIT_SECTION_LENGTH: f32 = 6400.0;
const PLAYER_Y: f32 = 475.0;
const HORIZON_Y: f32 = 145.0;
const ROAD_HALF_WIDTH: f32 = 238.0;

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
    wake: Vec<WakeParticle>,
    missed: u32,
    shake: f32,
    fall_timer: f32,
    lean: f32,
}

impl Game {
    fn new() -> Self {
        let mut game = Self {
            state: GameState::Title,
            player_x: course_center(0.0),
            speed: 0.0,
            distance: 0.0,
            elapsed: 0.0,
            countdown: 3.0,
            boost: 1.0,
            health: 3,
            gates: Vec::new(),
            wake: Vec::new(),
            missed: 0,
            shake: 0.0,
            fall_timer: 0.0,
            lean: 0.0,
        };
        game.build_course();
        game
    }

    fn build_course(&mut self) {
        self.gates.clear();

        for i in 0..36 {
            let y = 420.0 + i as f32 * 340.0;
            self.gates.push(Gate {
                y,
                x: course_center(y),
                width: if i % 5 == 4 { 190.0 } else { 235.0 },
                passed: false,
            });
        }
    }

    fn start(&mut self) {
        self.state = GameState::Countdown;
        self.player_x = course_center(0.0);
        self.speed = 0.0;
        self.distance = 0.0;
        self.elapsed = 0.0;
        self.countdown = 3.0;
        self.boost = 1.0;
        self.health = 3;
        self.missed = 0;
        self.shake = 0.0;
        self.fall_timer = 0.0;
        self.lean = 0.0;
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

        if self.fall_timer > 0.0 {
            self.fall_timer -= dt;
            self.lean *= (1.0 - 2.5 * dt).max(0.0);
            self.speed = (self.speed - 520.0 * dt).max(0.0);
            if self.fall_timer <= 0.0 {
                self.health -= 1;
                self.distance = (self.distance - 150.0).max(0.0);
                self.player_x = course_center(self.distance);
                self.speed = 230.0;
                self.boost *= 0.55;
                if self.health <= 0 {
                    self.health = 3;
                }
            }
            return;
        }

        let mut steering = 0.0;
        if rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_A) {
            steering -= 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) || rl.is_key_down(KeyboardKey::KEY_D) {
            steering += 1.0;
        }
        self.lean += (steering - self.lean) * (8.0 * dt).min(1.0);

        let boosting = (rl.is_key_down(KeyboardKey::KEY_SPACE)
            || rl.is_key_down(KeyboardKey::KEY_UP))
            && self.boost > 0.02;
        let target_speed = if boosting { 555.0 } else { 360.0 };
        self.speed += (target_speed - self.speed) * (2.7 * dt).min(1.0);
        self.player_x += steering * (345.0 + self.speed * 0.24) * dt;
        self.player_x = self.player_x.clamp(70.0, WIDTH as f32 - 70.0);

        if boosting {
            self.boost = (self.boost - 0.28 * dt).max(0.0);
        } else {
            self.boost = (self.boost + 0.10 * dt).min(1.0);
        }

        let previous_distance = self.distance;
        self.distance += self.speed * dt;

        if (self.player_x - course_center(self.distance)).abs() > ROAD_HALF_WIDTH - 28.0 {
            self.fall_timer = 0.85;
            self.shake = 0.40;
            self.wake.clear();
            return;
        }

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
        let screen_x = WIDTH as f32 * 0.5 + self.player_x - course_center(self.distance);
        for side in [-1.0_f32, 1.0] {
            self.wake.push(WakeParticle {
                position: Vector2::new(
                    screen_x + side * (30.0 + phase.sin() * 3.0),
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
    let water_texture = rl
        .load_texture(&thread, "assets/textures/water_circuit/water_flow.png")
        .expect("No se pudo cargar assets/textures/water_circuit/water_flow.png");
    manta_sheet.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_POINT);
    water_texture.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_POINT);

    let mut game = Game::new();
    while !rl.window_should_close() {
        let dt = rl.get_frame_time().min(0.033);
        game.update(&mut rl, dt);
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(6, 40, 76, 255));
        draw_game(&mut d, &game, &manta_sheet, &water_texture);
    }
}

fn draw_game(
    d: &mut RaylibDrawHandle,
    game: &Game,
    manta_sheet: &Texture2D,
    water_texture: &Texture2D,
) {
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

    draw_water(d, water_texture, game.distance, shake_x, shake_y);
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
        WIDTH as f32 * 0.5 + game.player_x - course_center(game.distance) + shake_x,
        PLAYER_Y + shake_y + game.fall_timer * 115.0,
        game.elapsed,
        game.lean,
    );
    if game.fall_timer > 0.0 {
        centered_text(
            d,
            "¡FUERA DEL AGUA!",
            330,
            34,
            Color::new(255, 225, 80, 255),
        );
    }
    draw_hud(d, game);

    match game.state {
        GameState::Title => draw_title(d),
        GameState::Countdown => draw_countdown(d, game.countdown),
        GameState::Finished => draw_finish(d, game),
        GameState::Racing => {}
    }
}

fn draw_water(
    d: &mut RaylibDrawHandle,
    water_texture: &Texture2D,
    distance: f32,
    shake_x: f32,
    shake_y: f32,
) {
    // Cielo diurno celeste sobre el horizonte.
    d.draw_rectangle_gradient_v(
        0,
        0,
        WIDTH,
        HORIZON_Y as i32 + 8,
        Color::new(64, 174, 244, 255),
        Color::new(157, 226, 255, 255),
    );
    draw_pixel_cloud(d, 105, 58, 1.0);
    draw_pixel_cloud(d, 735, 82, 0.72);
    draw_pixel_cloud(d, 425, 35, 0.52);

    // Mar exterior claro; la textura más intensa queda reservada al camino.
    d.draw_rectangle_gradient_v(
        0,
        HORIZON_Y as i32,
        WIDTH,
        HEIGHT - HORIZON_Y as i32,
        Color::new(72, 190, 226, 255),
        Color::new(17, 102, 170, 255),
    );

    // Cada franja sigue el centro curvo de la pista y conserva píxeles nítidos.
    let road_top = HORIZON_Y + shake_y;
    let road_bottom = HEIGHT as f32;
    let strip_height = 4.0;
    let scroll = (distance * 0.72).rem_euclid(water_texture.height as f32);
    let mut y = road_top;
    while y < road_bottom {
        let perspective = ((y - road_top) / (PLAYER_Y - HORIZON_Y)).clamp(0.075, 1.45);
        let depth = if y <= PLAYER_Y {
            620.0 * (1.0 / perspective - 1.0)
        } else {
            -(y - PLAYER_Y) / 1.15
        };
        let world_y = distance + depth;
        let (center_x, _, scale) = project_point(course_center(world_y), world_y, distance);
        let half_width = ROAD_HALF_WIDTH * scale;
        let source_y = (scroll + perspective * water_texture.height as f32)
            .rem_euclid(water_texture.height as f32);
        d.draw_texture_pro(
            water_texture,
            Rectangle::new(0.0, source_y, water_texture.width as f32, 5.0),
            Rectangle::new(
                center_x - half_width + shake_x,
                y,
                half_width * 2.0,
                strip_height + 1.0,
            ),
            Vector2::zero(),
            0.0,
            Color::new(255, 255, 255, 220),
        );
        let curb_index = ((world_y / 72.0).floor() as i32).rem_euclid(2);
        let edge_color = if curb_index == 0 {
            Color::new(255, 218, 55, 245)
        } else {
            Color::new(121, 245, 255, 245)
        };
        d.draw_rectangle(
            (center_x - half_width + shake_x) as i32,
            y as i32,
            3,
            (strip_height + 1.0) as i32,
            edge_color,
        );
        d.draw_rectangle(
            (center_x + half_width + shake_x - 3.0) as i32,
            y as i32,
            3,
            (strip_height + 1.0) as i32,
            edge_color,
        );
        y += strip_height;
    }
    d.draw_circle_gradient(
        WIDTH / 2,
        HORIZON_Y as i32 - 18,
        86.0,
        Color::new(255, 226, 139, 190),
        Color::new(255, 226, 139, 0),
    );
}

fn draw_pixel_cloud(d: &mut RaylibDrawHandle, x: i32, y: i32, scale: f32) {
    let unit = (12.0 * scale).max(4.0) as i32;
    let shadow = Color::new(184, 229, 246, 255);
    let white = Color::new(245, 253, 255, 255);
    d.draw_rectangle(x, y + unit, unit * 6, unit * 2, shadow);
    d.draw_rectangle(x + unit, y, unit * 2, unit * 3, white);
    d.draw_rectangle(x + unit * 3, y + unit / 2, unit * 2, unit * 2, white);
    d.draw_rectangle(x, y + unit, unit * 6, unit, white);
}

fn course_center(world_y: f32) -> f32 {
    // Línea central cerrada con curvas arcade, eses rápidas y dos horquillas.
    // Catmull-Rom hace que cada giro sea continuo y redondeado.
    const CENTERS: [f32; 16] = [
        480.0, 690.0, 815.0, 690.0, 355.0, 112.0, 188.0, 535.0, 828.0, 742.0, 410.0, 128.0, 205.0,
        588.0, 790.0, 610.0,
    ];

    let progress = world_y.rem_euclid(CIRCUIT_SECTION_LENGTH) / CIRCUIT_SECTION_LENGTH;
    let scaled = progress * CENTERS.len() as f32;
    let index = scaled.floor() as usize % CENTERS.len();
    let t = scaled - scaled.floor();
    let p0 = CENTERS[(index + CENTERS.len() - 1) % CENTERS.len()];
    let p1 = CENTERS[index];
    let p2 = CENTERS[(index + 1) % CENTERS.len()];
    let p3 = CENTERS[(index + 2) % CENTERS.len()];
    let t2 = t * t;
    let t3 = t2 * t;

    0.5 * ((2.0 * p1)
        + (-p0 + p2) * t
        + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t2
        + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t3)
}

fn course_tangent(world_y: f32) -> f32 {
    let sample = 28.0;
    (course_center(world_y + sample) - course_center(world_y - sample)) / (sample * 2.0)
}

fn project_point(world_x: f32, world_y: f32, distance: f32) -> (f32, f32, f32) {
    let depth = world_y - distance;
    let scale = if depth >= 0.0 {
        (1.0 / (1.0 + depth / 620.0)).clamp(0.12, 1.0)
    } else {
        (1.0 - depth / 210.0).clamp(1.0, 1.7)
    };
    // La cámara permanece sobre la línea central actual. Los puntos futuros
    // se rotan por la tangente actual, como una cámara que mira hacia el giro.
    let camera_center = course_center(distance);
    let camera_heading = course_tangent(distance);
    let heading_offset = camera_heading * depth;
    let screen_x = WIDTH as f32 * 0.5 + (world_x - camera_center - heading_offset) * scale * 1.90;
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
        draw_pixel_gate(d, left, right, y, scale, color, gate.passed);
    }

    let (finish_x, finish_y, finish_scale) =
        project_point(course_center(COURSE_LENGTH), COURSE_LENGTH, game.distance);
    let finish_y = finish_y + sy;
    if finish_y > -100.0 && finish_y < HEIGHT as f32 + 100.0 {
        draw_finish_arch(d, finish_x, finish_y, finish_scale);
    }
}

fn draw_finish_arch(d: &mut RaylibDrawHandle, center_x: f32, water_y: f32, scale: f32) {
    let half_span = ROAD_HALF_WIDTH * 0.78 * scale;
    let left = center_x - half_span;
    let right = center_x + half_span;
    let post_width = (28.0 * scale).max(4.0);
    let post_height = (118.0 * scale).max(16.0);
    let outline = (5.0 * scale).max(1.0);
    let top = water_y - post_height;
    let dark = Color::new(25, 31, 66, 255);
    let blue = Color::new(25, 104, 207, 255);
    let cyan = Color::new(98, 238, 255, 255);

    for x in [left, right] {
        d.draw_rectangle(
            (x - post_width * 0.5 - outline) as i32,
            (top - outline) as i32,
            (post_width + outline * 2.0).ceil() as i32,
            (post_height + outline * 2.0).ceil() as i32,
            dark,
        );
        d.draw_rectangle(
            (x - post_width * 0.5) as i32,
            top as i32,
            post_width.ceil() as i32,
            post_height.ceil() as i32,
            blue,
        );
        d.draw_rectangle(
            (x - post_width * 0.22) as i32,
            top as i32,
            (post_width * 0.25).max(1.0).ceil() as i32,
            post_height.ceil() as i32,
            cyan,
        );
        d.draw_rectangle(
            (x - post_width * 0.78) as i32,
            (water_y - 9.0 * scale) as i32,
            (post_width * 1.56).ceil() as i32,
            (11.0 * scale).max(2.0).ceil() as i32,
            dark,
        );
    }

    // Tablero a cuadros suspendido entre las torres.
    let columns = 12;
    let rows = 3;
    let tile_width = (right - left) / columns as f32;
    let tile_height = (16.0 * scale).max(2.0);
    let banner_top = top - tile_height;
    d.draw_rectangle(
        (left - outline) as i32,
        (banner_top - outline) as i32,
        (right - left + outline * 2.0).ceil() as i32,
        (tile_height * rows as f32 + outline * 2.0).ceil() as i32,
        dark,
    );
    for row in 0..rows {
        for column in 0..columns {
            let color = if (row + column) % 2 == 0 {
                Color::WHITE
            } else {
                Color::new(25, 31, 66, 255)
            };
            d.draw_rectangle(
                (left + column as f32 * tile_width) as i32,
                (banner_top + row as f32 * tile_height) as i32,
                tile_width.ceil() as i32,
                tile_height.ceil() as i32,
                color,
            );
        }
    }

    let font_size = (24.0 * scale).max(8.0) as i32;
    let label_width = d.measure_text("META", font_size);
    d.draw_text(
        "META",
        center_x as i32 - label_width / 2,
        (banner_top - font_size as f32 - 5.0 * scale) as i32,
        font_size,
        Color::new(255, 224, 70, 255),
    );
}

fn draw_pixel_gate(
    d: &mut RaylibDrawHandle,
    left: f32,
    right: f32,
    water_y: f32,
    scale: f32,
    color: Color,
    passed: bool,
) {
    let alpha = if passed { 115 } else { 255 };
    let dark = Color::new(34, 38, 76, alpha);
    let gold = Color::new(color.r, color.g, color.b, alpha);
    let cyan = Color::new(104, 240, 255, alpha);
    let white = Color::new(238, 253, 255, alpha);
    let post_width = (22.0 * scale).max(3.0);
    let post_height = (82.0 * scale).max(9.0);
    let outline = (4.0 * scale).max(1.0);
    let top_y = water_y - post_height;

    for x in [left, right] {
        d.draw_rectangle(
            (x - post_width * 0.5 - outline) as i32,
            (top_y - outline) as i32,
            (post_width + outline * 2.0).ceil() as i32,
            (post_height + outline * 2.0).ceil() as i32,
            dark,
        );
        d.draw_rectangle(
            (x - post_width * 0.5) as i32,
            top_y as i32,
            post_width.ceil() as i32,
            post_height.ceil() as i32,
            gold,
        );

        // Luces cuadradas alternas, sin suavizado, como hardware de 16 bits.
        for block in 0..4 {
            let block_size = (8.0 * scale).max(2.0);
            let block_y = top_y + (13.0 + block as f32 * 17.0) * scale;
            d.draw_rectangle(
                (x - block_size * 0.5) as i32,
                block_y as i32,
                block_size.ceil() as i32,
                block_size.ceil() as i32,
                if block % 2 == 0 { cyan } else { white },
            );
        }

        // Base escalonada para que el pilar tenga una silueta sólida.
        d.draw_rectangle(
            (x - post_width * 0.75) as i32,
            (water_y - 8.0 * scale) as i32,
            (post_width * 1.5).ceil() as i32,
            (10.0 * scale).max(2.0).ceil() as i32,
            dark,
        );
        d.draw_rectangle(
            (x - post_width * 0.58) as i32,
            (water_y - 7.0 * scale) as i32,
            (post_width * 1.16).ceil() as i32,
            (6.0 * scale).max(1.0).ceil() as i32,
            cyan,
        );
    }

    // Arco hecho de bloques individuales sobre una curva, no una línea simple.
    let segments = 11;
    let span = right - left;
    for segment in 0..segments {
        let u = segment as f32 / (segments - 1) as f32;
        let x = left + span * u;
        let arch = (u * std::f32::consts::PI).sin();
        let y = top_y - arch * 29.0 * scale;
        let block_width = (span / segments as f32 + 3.0 * scale).max(3.0);
        let block_height = (13.0 * scale).max(2.0);
        d.draw_rectangle(
            (x - block_width * 0.5 - outline) as i32,
            (y - block_height * 0.5 - outline) as i32,
            (block_width + outline * 2.0).ceil() as i32,
            (block_height + outline * 2.0).ceil() as i32,
            dark,
        );
        d.draw_rectangle(
            (x - block_width * 0.5) as i32,
            (y - block_height * 0.5) as i32,
            block_width.ceil() as i32,
            block_height.ceil() as i32,
            if segment % 2 == 0 { gold } else { cyan },
        );
        if !passed && segment % 3 == 1 {
            let shine = (4.0 * scale).max(1.0);
            d.draw_rectangle(
                (x - shine * 0.5) as i32,
                (y - shine * 0.5) as i32,
                shine.ceil() as i32,
                shine.ceil() as i32,
                white,
            );
        }
    }
}

fn draw_rider(
    d: &mut RaylibDrawHandle,
    manta_sheet: &Texture2D,
    x: f32,
    y: f32,
    time: f32,
    lean: f32,
) {
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
        -lean * 13.0,
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
    let rank = if game.elapsed < 30.0 && game.missed == 0 {
        "RANGO S"
    } else if game.elapsed < 39.0 {
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
