use raylib::audio::RaylibAudio;
use raylib::prelude::*;

const WIDTH: i32 = 960;
const HEIGHT: i32 = 720;
const PLAYER_Y: f32 = 650.0;

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Title,
    Playing,
    Paused,
    GameOver,
}

#[derive(Clone, Copy)]
struct Bullet {
    pos: Vector2,
    vel: Vector2,
    friendly: bool,
    damage: i32,
}

#[derive(Clone, Copy)]
struct Enemy {
    pos: Vector2,
    home: Vector2,
    kind: u8,
    hp: i32,
    diving: bool,
    dive_time: f32,
    phase: f32,
    shot_timer: f32,
}

#[derive(Clone, Copy)]
struct Particle {
    pos: Vector2,
    vel: Vector2,
    life: f32,
    color: Color,
}

#[derive(Clone, Copy)]
struct Star {
    pos: Vector2,
    speed: f32,
    size: f32,
}

#[derive(Clone, Copy)]
struct PowerUp {
    pos: Vector2,
    kind: u8,
}

struct Game {
    mode: Mode,
    player_x: f32,
    lives: i32,
    score: u32,
    high_score: u32,
    wave: u32,
    bullets: Vec<Bullet>,
    enemies: Vec<Enemy>,
    particles: Vec<Particle>,
    powerups: Vec<PowerUp>,
    stars: Vec<Star>,
    shoot_cooldown: f32,
    invulnerable: f32,
    rapid_fire: f32,
    shield: i32,
    combo: u32,
    combo_timer: f32,
    wave_banner: f32,
    formation_time: f32,
    rng: u32,
}

fn next_random(rng: &mut u32) -> f32 {
    *rng = rng.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
    ((*rng >> 8) as f32) / ((u32::MAX >> 8) as f32)
}

impl Game {
    fn new() -> Self {
        let mut game = Self {
            mode: Mode::Title,
            player_x: WIDTH as f32 * 0.5,
            lives: 5,
            score: 0,
            high_score: 0,
            wave: 1,
            bullets: Vec::new(),
            enemies: Vec::new(),
            particles: Vec::new(),
            powerups: Vec::new(),
            stars: Vec::new(),
            shoot_cooldown: 0.0,
            invulnerable: 0.0,
            rapid_fire: 0.0,
            shield: 0,
            combo: 0,
            combo_timer: 0.0,
            wave_banner: 0.0,
            formation_time: 0.0,
            rng: 0x51A7_C0DE,
        };
        for i in 0..100 {
            let x = game.random() * WIDTH as f32;
            let y = game.random() * HEIGHT as f32;
            let speed = 25.0 + game.random() * 95.0;
            let size = if i % 9 == 0 { 2.0 } else { 1.0 };
            game.stars.push(Star {
                pos: Vector2::new(x, y),
                speed,
                size,
            });
        }
        game
    }

    fn random(&mut self) -> f32 {
        self.rng = self.rng.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        ((self.rng >> 8) as f32) / ((u32::MAX >> 8) as f32)
    }

    fn start(&mut self) {
        self.player_x = WIDTH as f32 * 0.5;
        self.lives = 5;
        self.score = 0;
        self.wave = 1;
        self.bullets.clear();
        self.particles.clear();
        self.powerups.clear();
        self.shield = 0;
        self.rapid_fire = 0.0;
        self.combo = 0;
        self.invulnerable = 1.5;
        self.mode = Mode::Playing;
        self.spawn_wave();
    }

    fn spawn_wave(&mut self) {
        self.enemies.clear();
        self.bullets.retain(|bullet| bullet.friendly);
        self.formation_time = 0.0;
        self.wave_banner = 2.0;
        let rows = 3 + ((self.wave.saturating_sub(1) / 8).min(1)) as i32;
        let columns = 6 + ((self.wave.saturating_sub(1) / 5).min(2)) as i32;
        for row in 0..rows {
            for column in 0..columns {
                let kind = if row == 0 { 1 } else { 0 };
                let home = Vector2::new(
                    WIDTH as f32 * 0.5 + (column as f32 - (columns - 1) as f32 * 0.5) * 68.0,
                    105.0 + row as f32 * 55.0,
                );
                let shot_timer = 4.0 + self.random() * 6.0;
                self.enemies.push(Enemy {
                    pos: Vector2::new(home.x, -80.0 - row as f32 * 45.0),
                    home,
                    kind,
                    hp: if kind == 1 { 2 } else { 1 },
                    diving: false,
                    dive_time: 0.0,
                    phase: column as f32 * 0.7 + row as f32,
                    shot_timer,
                });
            }
        }
        if self.wave % 5 == 0 {
            self.enemies.push(Enemy {
                pos: Vector2::new(WIDTH as f32 * 0.5, -140.0),
                home: Vector2::new(WIDTH as f32 * 0.5, 78.0),
                kind: 2,
                hp: 30,
                diving: false,
                dive_time: 0.0,
                phase: 0.0,
                shot_timer: 1.0,
            });
        }
    }

    fn update(&mut self, rl: &mut RaylibHandle, dt: f32) {
        self.update_stars(dt);
        match self.mode {
            Mode::Title => {
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    self.start();
                }
                return;
            }
            Mode::Paused => {
                if rl.is_key_pressed(KeyboardKey::KEY_P) {
                    self.mode = Mode::Playing;
                }
                return;
            }
            Mode::GameOver => {
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    self.start();
                }
                return;
            }
            Mode::Playing => {}
        }
        if rl.is_key_pressed(KeyboardKey::KEY_P) {
            self.mode = Mode::Paused;
            return;
        }

        self.formation_time += dt;
        self.wave_banner = (self.wave_banner - dt).max(0.0);
        self.shoot_cooldown = (self.shoot_cooldown - dt).max(0.0);
        self.invulnerable = (self.invulnerable - dt).max(0.0);
        self.rapid_fire = (self.rapid_fire - dt).max(0.0);
        self.combo_timer = (self.combo_timer - dt).max(0.0);
        if self.combo_timer <= 0.0 {
            self.combo = 0;
        }

        let mut movement = 0.0;
        if rl.is_key_down(KeyboardKey::KEY_LEFT) || rl.is_key_down(KeyboardKey::KEY_A) {
            movement -= 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) || rl.is_key_down(KeyboardKey::KEY_D) {
            movement += 1.0;
        }
        self.player_x = (self.player_x + movement * 430.0 * dt).clamp(42.0, WIDTH as f32 - 42.0);

        if (rl.is_key_down(KeyboardKey::KEY_SPACE) || rl.is_key_down(KeyboardKey::KEY_UP))
            && self.shoot_cooldown <= 0.0
        {
            let spread = if self.rapid_fire > 0.0 { 13.0 } else { 0.0 };
            for offset in if spread > 0.0 {
                vec![-spread, spread]
            } else {
                vec![0.0]
            } {
                self.bullets.push(Bullet {
                    pos: Vector2::new(self.player_x + offset, PLAYER_Y - 28.0),
                    vel: Vector2::new(offset * 0.8, -650.0),
                    friendly: true,
                    damage: 1,
                });
            }
            self.shoot_cooldown = if self.rapid_fire > 0.0 { 0.11 } else { 0.22 };
        }

        self.update_enemies(dt);
        self.update_bullets(dt);
        self.update_powerups(dt);
        self.update_particles(dt);

        if self.enemies.is_empty() {
            self.wave += 1;
            self.spawn_wave();
        }
    }

    fn update_stars(&mut self, dt: f32) {
        for star in &mut self.stars {
            star.pos.y += star.speed * dt;
            if star.pos.y > HEIGHT as f32 {
                star.pos.y = 0.0;
            }
        }
    }

    fn update_enemies(&mut self, dt: f32) {
        let player_x = self.player_x;
        let formation_time = self.formation_time;
        let dive_chance = (0.045 + self.wave as f32 * 0.006).min(0.16) * dt;
        let mut shots = Vec::new();
        let mut enemy_bullets = self
            .bullets
            .iter()
            .filter(|bullet| !bullet.friendly)
            .count();
        let rng = &mut self.rng;
        for enemy in &mut self.enemies {
            enemy.shot_timer -= dt;
            if !enemy.diving {
                let entry = (formation_time * 1.8).min(1.0);
                enemy.pos.x = enemy.home.x + (formation_time * 1.25 + enemy.phase).sin() * 18.0;
                enemy.pos.y += (enemy.home.y - enemy.pos.y) * (5.0 * dt).min(1.0) * entry;
                if formation_time > 2.0 && next_random(rng) < dive_chance && enemy.kind != 2 {
                    enemy.diving = true;
                    enemy.dive_time = 0.0;
                }
            } else {
                enemy.dive_time += dt;
                let t = enemy.dive_time;
                enemy.pos.y += (185.0 + t * 78.0) * dt;
                enemy.pos.x +=
                    ((player_x - enemy.pos.x) * 0.65 + (t * 5.0 + enemy.phase).sin() * 150.0) * dt;
                if enemy.pos.y > HEIGHT as f32 + 55.0 {
                    enemy.pos = Vector2::new(enemy.home.x, -45.0);
                    enemy.diving = false;
                }
            }
            if enemy.kind == 2 {
                enemy.pos.x = WIDTH as f32 * 0.5 + (formation_time * 0.8).sin() * 260.0;
            }
            if enemy.shot_timer <= 0.0 && enemy.pos.y > 20.0 && enemy_bullets < 5 {
                let direction =
                    Vector2::new(player_x - enemy.pos.x, PLAYER_Y - enemy.pos.y).normalize();
                let count = 1;
                for i in 0..count {
                    let angle = (i as f32 - (count - 1) as f32 * 0.5) * 0.22;
                    let rotated = Vector2::new(
                        direction.x * angle.cos() - direction.y * angle.sin(),
                        direction.x * angle.sin() + direction.y * angle.cos(),
                    );
                    shots.push(Bullet {
                        pos: enemy.pos,
                        vel: rotated * if enemy.kind == 2 { 300.0 } else { 245.0 },
                        friendly: false,
                        damage: 1,
                    });
                }
                enemy_bullets += count;
                enemy.shot_timer = if enemy.kind == 2 {
                    2.0
                } else {
                    4.8 + next_random(rng) * 4.5
                };
            }
        }
        self.bullets.extend(shots);
    }

    fn update_bullets(&mut self, dt: f32) {
        for bullet in &mut self.bullets {
            bullet.pos += bullet.vel * dt;
        }
        let mut remove_bullets = vec![false; self.bullets.len()];
        let mut killed = Vec::new();
        let mut player_hit = false;
        for (bi, bullet) in self.bullets.iter().enumerate() {
            if bullet.friendly {
                for (ei, enemy) in self.enemies.iter_mut().enumerate() {
                    let radius = if enemy.kind == 2 { 48.0 } else { 22.0 };
                    if enemy.pos.distance(bullet.pos) < radius {
                        enemy.hp -= bullet.damage;
                        remove_bullets[bi] = true;
                        if enemy.hp <= 0 {
                            killed.push(ei);
                        }
                        break;
                    }
                }
            } else if self.invulnerable <= 0.0
                && bullet.pos.distance(Vector2::new(self.player_x, PLAYER_Y)) < 24.0
            {
                remove_bullets[bi] = true;
                player_hit = true;
            }
        }
        if player_hit {
            self.hit_player();
        }
        killed.sort_unstable();
        killed.dedup();
        for index in killed.into_iter().rev() {
            if index >= self.enemies.len() {
                continue;
            }
            let enemy = self.enemies.remove(index);
            self.combo += 1;
            self.combo_timer = 2.0;
            self.score += (if enemy.kind == 2 {
                5000
            } else if enemy.diving {
                240
            } else {
                100
            }) * self.combo.min(8);
            self.explode(enemy.pos, if enemy.kind == 2 { 42 } else { 14 });
            if self.random() < 0.10 {
                let kind = if self.random() < 0.55 { 0 } else { 1 };
                self.powerups.push(PowerUp {
                    pos: enemy.pos,
                    kind,
                });
            }
        }
        let mut index = 0usize;
        self.bullets.retain(|bullet| {
            let keep = !remove_bullets[index]
                && bullet.pos.y > -40.0
                && bullet.pos.y < HEIGHT as f32 + 40.0
                && bullet.pos.x > -40.0
                && bullet.pos.x < WIDTH as f32 + 40.0;
            index += 1;
            keep
        });

        if self.invulnerable <= 0.0
            && self
                .enemies
                .iter()
                .any(|enemy| enemy.pos.distance(Vector2::new(self.player_x, PLAYER_Y)) < 30.0)
        {
            self.hit_player();
        }
    }

    fn hit_player(&mut self) {
        if self.shield > 0 {
            self.shield -= 1;
            self.invulnerable = 1.0;
            return;
        }
        self.lives -= 1;
        self.combo = 0;
        self.invulnerable = 2.2;
        self.explode(Vector2::new(self.player_x, PLAYER_Y), 24);
        if self.lives <= 0 {
            self.high_score = self.high_score.max(self.score);
            self.mode = Mode::GameOver;
        }
    }

    fn update_powerups(&mut self, dt: f32) {
        for powerup in &mut self.powerups {
            powerup.pos.y += 105.0 * dt;
        }
        let mut rapid = false;
        let mut shield = false;
        self.powerups.retain(|powerup| {
            let caught = powerup.pos.distance(Vector2::new(self.player_x, PLAYER_Y)) < 30.0;
            if caught {
                if powerup.kind == 0 {
                    rapid = true;
                } else {
                    shield = true;
                }
            }
            !caught && powerup.pos.y < HEIGHT as f32 + 30.0
        });
        if rapid {
            self.rapid_fire = 10.0;
        }
        if shield {
            self.shield = (self.shield + 1).min(3);
        }
    }

    fn explode(&mut self, pos: Vector2, count: usize) {
        for i in 0..count {
            let angle = self.random() * std::f32::consts::TAU;
            let speed = 60.0 + self.random() * 240.0;
            let life = 0.35 + self.random() * 0.65;
            self.particles.push(Particle {
                pos,
                vel: Vector2::new(angle.cos(), angle.sin()) * speed,
                life,
                color: if i % 2 == 0 {
                    Color::YELLOW
                } else {
                    Color::ORANGE
                },
            });
        }
    }

    fn update_particles(&mut self, dt: f32) {
        for particle in &mut self.particles {
            particle.pos += particle.vel * dt;
            particle.vel *= 0.97;
            particle.life -= dt;
        }
        self.particles.retain(|particle| particle.life > 0.0);
    }

    fn draw(&self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::new(3, 5, 24, 255));
        for star in &self.stars {
            d.draw_rectangle(
                star.pos.x as i32,
                star.pos.y as i32,
                star.size as i32,
                star.size as i32,
                Color::new(170, 220, 255, 220),
            );
        }
        for powerup in &self.powerups {
            draw_powerup(d, *powerup);
        }
        for enemy in &self.enemies {
            draw_enemy(d, *enemy, self.formation_time);
        }
        for bullet in &self.bullets {
            let color = if bullet.friendly {
                Color::new(80, 255, 245, 255)
            } else {
                Color::new(255, 72, 118, 255)
            };
            d.draw_rectangle(
                (bullet.pos.x - 3.0) as i32,
                (bullet.pos.y - 8.0) as i32,
                6,
                16,
                color,
            );
        }
        for particle in &self.particles {
            d.draw_rectangle(
                particle.pos.x as i32,
                particle.pos.y as i32,
                4,
                4,
                Color::new(
                    particle.color.r,
                    particle.color.g,
                    particle.color.b,
                    (particle.life * 255.0).min(255.0) as u8,
                ),
            );
        }
        if self.mode == Mode::Playing || self.mode == Mode::Paused {
            draw_player(d, self.player_x, self.invulnerable, self.shield);
        }
        self.draw_hud(d);
        match self.mode {
            Mode::Title => {
                overlay(
                    d,
                    "STAR SQUADRON",
                    "ENTER - MODO INFINITO",
                    Color::new(85, 245, 255, 255),
                );
                d.draw_text(
                    "A/D O FLECHAS: MOVER   ESPACIO: DISPARAR   P: PAUSA",
                    170,
                    465,
                    18,
                    Color::LIGHTGRAY,
                );
            }
            Mode::Paused => overlay(d, "PAUSA", "P - CONTINUAR", Color::YELLOW),
            Mode::GameOver => overlay(
                d,
                "FIN DE LA MISIÓN",
                "ENTER - REINTENTAR",
                Color::new(255, 80, 105, 255),
            ),
            Mode::Playing => {}
        }
        if self.wave_banner > 0.0 && self.mode == Mode::Playing {
            centered(d, &format!("OLEADA {}", self.wave), 330, 38, Color::WHITE);
        }
    }

    fn draw_hud(&self, d: &mut RaylibDrawHandle) {
        d.draw_text(
            &format!("PUNTOS {:08}", self.score),
            24,
            18,
            23,
            Color::WHITE,
        );
        d.draw_text(
            &format!("RÉCORD {:08}", self.high_score),
            24,
            48,
            17,
            Color::LIGHTGRAY,
        );
        d.draw_text(
            &format!("OLEADA {}", self.wave),
            WIDTH - 165,
            20,
            21,
            Color::new(115, 235, 255, 255),
        );
        d.draw_text(
            &format!("VIDAS {}", "◆".repeat(self.lives.max(0) as usize)),
            WIDTH - 165,
            50,
            18,
            Color::new(255, 110, 130, 255),
        );
        if self.combo > 1 {
            d.draw_text(
                &format!("COMBO x{}", self.combo.min(8)),
                24,
                82,
                20,
                Color::YELLOW,
            );
        }
        if self.rapid_fire > 0.0 {
            d.draw_text(
                &format!("RÁFAGA {:.0}s", self.rapid_fire.ceil()),
                WIDTH - 165,
                80,
                17,
                Color::new(100, 255, 190, 255),
            );
        }
    }
}

pub struct EmbeddedGame {
    game: Game,
}

impl EmbeddedGame {
    pub fn new() -> Self {
        Self { game: Game::new() }
    }

    pub fn start(&mut self) {
        self.game.start();
    }

    pub fn update(&mut self, rl: &mut RaylibHandle, dt: f32) {
        self.game.update(rl, dt);
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        self.game.draw(d);
    }
}

fn draw_player(d: &mut RaylibDrawHandle, x: f32, invulnerable: f32, shield: i32) {
    if invulnerable > 0.0 && (invulnerable * 12.0) as i32 % 2 == 0 {
        return;
    }
    const SHIP: [&str; 13] = [
        "......W......",
        ".....WWW.....",
        ".....WRW.....",
        "....WWRWW....",
        "...WWRRRWW...",
        "..BWWRRRWWB..",
        ".BBWWRRRWWBB.",
        "BBBWWRRRWWBBB",
        "....WRRRW....",
        "...WWRRRWW...",
        "..WWR...RWW..",
        "..Y.......Y..",
        ".YY.......YY.",
    ];
    draw_pixel_sprite(d, &SHIP, x, PLAYER_Y, 4, false);
    if shield > 0 {
        d.draw_circle_lines(
            x as i32,
            PLAYER_Y as i32,
            36.0,
            Color::new(90, 255, 210, 255),
        );
    }
}

fn draw_enemy(d: &mut RaylibDrawHandle, enemy: Enemy, time: f32) {
    const DRONE_OPEN: [&str; 9] = [
        "Y.........Y",
        ".Y..PPP..Y.",
        "..YPPPPP Y..",
        "PPPPBBBBPPP",
        ".PPWBBBWPP.",
        "..PBBBBBP..",
        "...PBPBP...",
        "..P.....P..",
        ".P.......P.",
    ];
    const DRONE_CLOSED: [&str; 9] = [
        "...Y...Y...",
        "..YPPPPP Y..",
        ".PPPPPPPPP.",
        "PPPPBBBBPPP",
        ".PPWBBBWPP.",
        "..PBBBBBP..",
        "...PBPBP...",
        "....P.P....",
        "...P...P...",
    ];
    const GUARD: [&str; 10] = [
        "Y.Y.....Y.Y",
        ".YY.GGG.YY.",
        "..GGGGGGG..",
        ".GGYGGGYGG.",
        "GGGGBBBGGGG",
        ".GGWBBBWGG.",
        "..GGBBBGG..",
        "...GBBBG...",
        "..G.G.G.G..",
        ".G.......G.",
    ];
    const COMMANDER: [&str; 13] = [
        "M.M.......M.M",
        ".MMM.MMM.MMM.",
        "..MMMMMMMMM..",
        ".MMCCMMMCCMM.",
        "MMMCCCCCCCMMM",
        "MMMMBBBBBMMMM",
        ".MMWBBBBBWMM.",
        "..MMBBBBBMM..",
        "...MBBBBBM...",
        "..MMMBMBMMM..",
        ".MM...M...MM.",
        "MM.........MM",
        "M...........M",
    ];
    let alternate = ((time * 7.0 + enemy.phase) as i32) % 2 == 0;
    match enemy.kind {
        0 => draw_pixel_sprite(
            d,
            if alternate {
                &DRONE_OPEN
            } else {
                &DRONE_CLOSED
            },
            enemy.pos.x,
            enemy.pos.y,
            4,
            enemy.diving,
        ),
        1 => draw_pixel_sprite(d, &GUARD, enemy.pos.x, enemy.pos.y, 4, enemy.diving),
        _ => draw_pixel_sprite(d, &COMMANDER, enemy.pos.x, enemy.pos.y, 6, false),
    }
    if enemy.kind == 2 {
        d.draw_rectangle(
            (enemy.pos.x - 42.0) as i32,
            (enemy.pos.y - 55.0) as i32,
            84,
            7,
            Color::new(35, 25, 55, 255),
        );
        d.draw_rectangle(
            (enemy.pos.x - 42.0) as i32,
            (enemy.pos.y - 55.0) as i32,
            (84.0 * enemy.hp as f32 / 30.0) as i32,
            7,
            Color::new(255, 80, 125, 255),
        );
    }
}

fn draw_pixel_sprite(
    d: &mut RaylibDrawHandle,
    pattern: &[&str],
    center_x: f32,
    center_y: f32,
    pixel: i32,
    diving: bool,
) {
    let width = pattern
        .iter()
        .map(|row| row.chars().count())
        .max()
        .unwrap_or(0) as i32;
    let height = pattern.len() as i32;
    let origin_x = center_x as i32 - width * pixel / 2;
    let origin_y = center_y as i32 - height * pixel / 2;
    for (row, line) in pattern.iter().enumerate() {
        for (column, symbol) in line.chars().enumerate() {
            let color = match symbol {
                'W' => Color::new(245, 250, 255, 255),
                'R' => Color::new(242, 52, 65, 255),
                'B' => Color::new(35, 88, 225, 255),
                'Y' => Color::new(255, 220, 40, 255),
                'P' => Color::new(238, 65, 158, 255),
                'G' => Color::new(70, 220, 92, 255),
                'M' => Color::new(176, 62, 235, 255),
                'C' => Color::new(52, 225, 240, 255),
                _ => continue,
            };
            let x = if diving {
                origin_x + (height - 1 - row as i32) * pixel
            } else {
                origin_x + column as i32 * pixel
            };
            let y = if diving {
                origin_y + column as i32 * pixel
            } else {
                origin_y + row as i32 * pixel
            };
            d.draw_rectangle(x, y, pixel, pixel, color);
        }
    }
}

fn draw_powerup(d: &mut RaylibDrawHandle, powerup: PowerUp) {
    let color = if powerup.kind == 0 {
        Color::new(75, 255, 180, 255)
    } else {
        Color::new(80, 190, 255, 255)
    };
    d.draw_circle(
        powerup.pos.x as i32,
        powerup.pos.y as i32,
        13.0,
        Color::new(20, 35, 70, 255),
    );
    d.draw_circle_lines(powerup.pos.x as i32, powerup.pos.y as i32, 13.0, color);
    d.draw_text(
        if powerup.kind == 0 { "R" } else { "S" },
        powerup.pos.x as i32 - 6,
        powerup.pos.y as i32 - 9,
        18,
        color,
    );
}

fn overlay(d: &mut RaylibDrawHandle, title: &str, subtitle: &str, color: Color) {
    d.draw_rectangle(0, 0, WIDTH, HEIGHT, Color::new(2, 4, 20, 185));
    centered(d, title, 265, 52, color);
    centered(d, subtitle, 385, 25, Color::WHITE);
}

fn centered(d: &mut RaylibDrawHandle, text: &str, y: i32, size: i32, color: Color) {
    let width = d.measure_text(text, size);
    d.draw_text(text, (WIDTH - width) / 2, y, size, color);
}

#[allow(dead_code)]
fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Star Squadron")
        .resizable()
        .build();
    rl.set_target_fps(60);

    let audio = RaylibAudio::init_audio_device().expect("No se pudo iniciar el audio");
    let mut galaga_music = audio
        .new_music("assets/sounds/galaga.mp3")
        .expect("No se pudo cargar galaga.mp3");
    galaga_music.set_looping(true);
    galaga_music.set_volume(0.85);
    galaga_music.play_stream();

    let mut game = EmbeddedGame::new();
    while !rl.window_should_close() {
        galaga_music.update_stream();
        if !galaga_music.is_stream_playing() {
            galaga_music.play_stream();
        }
        let dt = rl.get_frame_time().min(0.033);
        game.update(&mut rl, dt);
        let mut d = rl.begin_drawing(&thread);
        game.draw(&mut d);
    }
    galaga_music.stop_stream();
}
