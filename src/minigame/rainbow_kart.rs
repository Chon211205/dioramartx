use raylib::prelude::*;

const WIDTH: i32 = 960;
const HEIGHT: i32 = 640;
const PLAYER_Y: f32 = 490.0;
const LAPS: u32 = 3;
const ROAD_WIDTH: f32 = 54.0;
const TRACK_HALF_WIDTH: f32 = 5.6;
const TRACK_SEGMENTS: usize = POINTS.len() - 1;
const TRACK_SUBDIVISIONS: usize = 18;

const POINTS: [Vector2; 20] = [
    Vector2 { x: 170.0, y: 160.0 }, Vector2 { x: 770.0, y: 160.0 },
    Vector2 { x: 825.0, y: 205.0 }, Vector2 { x: 825.0, y: 370.0 },
    Vector2 { x: 770.0, y: 420.0 }, Vector2 { x: 410.0, y: 420.0 },
    Vector2 { x: 355.0, y: 465.0 }, Vector2 { x: 355.0, y: 535.0 },
    Vector2 { x: 410.0, y: 575.0 }, Vector2 { x: 795.0, y: 575.0 },
    Vector2 { x: 850.0, y: 530.0 }, Vector2 { x: 850.0, y: 95.0 },
    Vector2 { x: 795.0, y: 50.0 }, Vector2 { x: 210.0, y: 50.0 },
    Vector2 { x: 150.0, y: 95.0 }, Vector2 { x: 150.0, y: 455.0 },
    Vector2 { x: 105.0, y: 505.0 }, Vector2 { x: 60.0, y: 505.0 },
    Vector2 { x: 60.0, y: 205.0 }, Vector2 { x: 170.0, y: 160.0 },
];

#[derive(Clone, Copy, PartialEq)]
enum State { Title, Countdown, Racing, Finished }

#[derive(Clone, Copy)]
struct BoxItem { segment: usize, t: f32, active: bool, timer: f32 }

#[derive(Clone, Copy)]
struct Banana { position: Vector2 }

struct Game {
    state: State,
    segment: usize,
    segment_t: f32,
    lane: f32,
    speed: f32,
    lap: u32,
    countdown: f32,
    elapsed: f32,
    boost: f32,
    has_item: bool,
    spin: f32,
    boxes: Vec<BoxItem>,
    bananas: Vec<Banana>,
}

impl Game {
    fn new() -> Self {
        let mut game = Self { state: State::Title, segment: 0, segment_t: 0.0, lane: 0.0,
            speed: 0.0, lap: 1, countdown: 3.0, elapsed: 0.0, boost: 0.0,
            has_item: false, spin: 0.0, boxes: Vec::new(), bananas: Vec::new() };
        game.reset_boxes();
        game
    }

    fn reset_boxes(&mut self) {
        self.boxes.clear();
        for segment in [0usize, 2, 4, 8, 10, 12, 15, 18] {
            for t in [0.35, 0.55, 0.75] {
                self.boxes.push(BoxItem { segment, t, active: true, timer: 0.0 });
            }
        }
    }

    fn start(&mut self) {
        self.state = State::Countdown; self.segment = 0; self.segment_t = 0.0;
        self.lane = 0.0; self.speed = 0.0; self.lap = 1; self.countdown = 3.0;
        self.elapsed = 0.0; self.boost = 0.0; self.has_item = false; self.spin = 0.0;
        self.bananas.clear(); self.reset_boxes();
    }

    fn update(&mut self, rl: &mut RaylibHandle, dt: f32) {
        if self.state == State::Title {
            if rl.is_key_pressed(KeyboardKey::KEY_ENTER) { self.start(); }
            return;
        }
        if self.state == State::Finished {
            if rl.is_key_pressed(KeyboardKey::KEY_ENTER) { self.start(); }
            return;
        }
        if self.state == State::Countdown {
            self.countdown -= dt;
            if self.countdown <= 0.0 { self.state = State::Racing; self.speed = 145.0; }
            return;
        }

        self.elapsed += dt; self.boost = (self.boost - dt).max(0.0); self.spin = (self.spin-dt).max(0.0);
        let accelerating = rl.is_key_down(KeyboardKey::KEY_W) || rl.is_key_down(KeyboardKey::KEY_UP);
        let braking = rl.is_key_down(KeyboardKey::KEY_S) || rl.is_key_down(KeyboardKey::KEY_DOWN);
        let mut steer = 0.0;
        if rl.is_key_down(KeyboardKey::KEY_A) || rl.is_key_down(KeyboardKey::KEY_LEFT) { steer -= 1.0; }
        if rl.is_key_down(KeyboardKey::KEY_D) || rl.is_key_down(KeyboardKey::KEY_RIGHT) { steer += 1.0; }
        if self.spin <= 0.0 { self.lane += steer * 92.0 * dt; }
        self.lane = self.lane.clamp(-ROAD_WIDTH * 0.48, ROAD_WIDTH * 0.48);
        let target = if self.spin > 0.0 { 45.0 } else if self.boost > 0.0 { 285.0 }
            else if braking { 80.0 } else if accelerating { 190.0 } else { 125.0 };
        self.speed += (target-self.speed)*(3.4*dt).min(1.0);

        let a=POINTS[self.segment]; let b=POINTS[self.segment+1];
        let length=(b-a).length().max(1.0);
        self.segment_t += self.speed*dt/length;
        while self.segment_t >= 1.0 {
            self.segment_t -= 1.0; self.segment += 1;
            if self.segment >= POINTS.len()-1 {
                self.segment=0; self.lap+=1; self.reset_boxes();
                if self.lap > LAPS { self.lap=LAPS; self.state=State::Finished; return; }
            }
        }

        let kart = self.kart_position();
        for item in &mut self.boxes {
            if !item.active { item.timer -= dt; if item.timer <= 0.0 { item.active=true; } }
            if item.active && item.segment == self.segment && (item.t-self.segment_t).abs()<0.055 {
                let pos=point_on_segment(item.segment,item.t,0.0);
                if pos.distance(kart)<48.0 { item.active=false; item.timer=5.0; self.has_item=true; }
            }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) && self.has_item {
            self.has_item=false;
            if ((self.elapsed*10.0) as i32)%2==0 { self.boost=1.3; }
            else { self.bananas.push(Banana { position: kart }); }
        }
        if self.bananas.iter().any(|banana| banana.position.distance(kart)<14.0) {
            self.spin=0.8; self.speed*=0.5;
        }
    }

    fn kart_position(&self) -> Vector2 { point_on_segment(self.segment,self.segment_t,self.lane) }
}

fn point_on_segment(segment: usize, t: f32, lane: f32) -> Vector2 {
    let a=POINTS[segment]; let b=POINTS[segment+1]; let direction=(b-a).normalize();
    let side=Vector2::new(-direction.y,direction.x);
    a+(b-a)*t+side*lane
}

pub struct EmbeddedGame { game: Game }
impl EmbeddedGame {
    pub fn new() -> Self { Self { game: Game::new() } }
    pub fn start(&mut self) { self.game.start(); }
    pub fn update(&mut self, rl: &mut RaylibHandle, dt: f32) { self.game.update(rl,dt); }
    pub fn draw(&self, d: &mut RaylibDrawHandle) { draw_game(d,&self.game); }
}

fn draw_game(d: &mut RaylibDrawHandle, game: &Game) {
    d.clear_background(Color::new(3,2,18,255));
    for i in 0..120 { d.draw_circle(((i*97+13)%d.get_screen_width() as usize)as i32,((i*53+7)%d.get_screen_height() as usize)as i32,1.2,Color::WHITE); }

    let kart = track_position(game.segment, game.segment_t, game.lane);
    let tangent = track_tangent(game.segment, game.segment_t);
    let camera = Camera3D::perspective(
        kart - tangent * 10.5 + Vector3::new(0.0, 5.0, 0.0),
        kart + tangent * 11.0 + Vector3::new(0.0, 0.7, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        58.0,
    );

    {
        let mut world = d.begin_mode3D(camera);
        draw_track_3d(&mut world);
        draw_objects_3d(&mut world, game);
        draw_kart_3d(&mut world, game);
    }

    draw_minimap(d,game);
    d.draw_rectangle_rounded(Rectangle::new(18.0,18.0,205.0,86.0),0.2,8,Color::new(5,4,30,220));
    d.draw_text(&format!("VUELTA {}/{}",game.lap,LAPS),32,30,24,Color::WHITE);
    d.draw_text(&format!("{} km/h",game.speed as i32),32,65,20,Color::SKYBLUE);
    d.draw_rectangle_rounded(Rectangle::new(790.0,20.0,145.0,68.0),0.2,8,Color::new(5,4,30,220));
    d.draw_text(if game.has_item{"OBJETO LISTO"}else{"SIN OBJETO"},805,44,18,if game.has_item{Color::YELLOW}else{Color::GRAY});
    match game.state {
        State::Title => { overlay(d); centered(d,"RAINBOW KART",160,62,Color::YELLOW); centered(d,"El circuito completo",235,28,Color::WHITE); centered(d,"W/S acelerar · A/D conducir · ESPACIO objeto",345,21,Color::LIGHTGRAY); centered(d,"PRESIONA ENTER",470,30,Color::WHITE); },
        State::Countdown => centered(d,if game.countdown>2.0{"3"}else if game.countdown>1.0{"2"}else{"1"},245,100,Color::WHITE),
        State::Finished => { overlay(d); centered(d,"¡3 VUELTAS COMPLETAS!",190,46,Color::YELLOW); centered(d,&format!("Tiempo: {:.2} s",game.elapsed),285,32,Color::WHITE); centered(d,"ENTER - Otra carrera",430,26,Color::SKYBLUE); },
        State::Racing => {}
    }
}

fn catmull_rom(p0: Vector3, p1: Vector3, p2: Vector3, p3: Vector3, t: f32) -> Vector3 {
    let t2 = t * t;
    let t3 = t2 * t;
    (p1 * 2.0
        + (p2 - p0) * t
        + (p0 * 2.0 - p1 * 5.0 + p2 * 4.0 - p3) * t2
        + (-p0 + p1 * 3.0 - p2 * 3.0 + p3) * t3)
        * 0.5
}

fn control_point(index: isize) -> Vector3 {
    let wrapped = index.rem_euclid(TRACK_SEGMENTS as isize) as usize;
    let point = POINTS[wrapped];
    let x = (point.x - 470.0) * 0.055;
    let z = (point.y - 315.0) * 0.055;
    let y = ((wrapped as f32 * 1.37).sin() * 0.7 + (wrapped as f32 * 0.53).cos() * 0.35).max(-0.6);
    Vector3::new(x, y, z)
}

fn track_center(segment: usize, t: f32) -> Vector3 {
    let i = segment as isize;
    catmull_rom(
        control_point(i - 1),
        control_point(i),
        control_point(i + 1),
        control_point(i + 2),
        t.clamp(0.0, 1.0),
    )
}

fn track_tangent(segment: usize, t: f32) -> Vector3 {
    let step = 0.004;
    let (before_segment, before_t) = if t > step {
        (segment, t - step)
    } else {
        ((segment + TRACK_SEGMENTS - 1) % TRACK_SEGMENTS, 1.0 + t - step)
    };
    let (after_segment, after_t) = if t + step < 1.0 {
        (segment, t + step)
    } else {
        ((segment + 1) % TRACK_SEGMENTS, t + step - 1.0)
    };
    (track_center(after_segment, after_t) - track_center(before_segment, before_t)).normalize()
}

fn track_position(segment: usize, t: f32, lane: f32) -> Vector3 {
    let center = track_center(segment, t);
    let tangent = track_tangent(segment, t);
    let side = Vector3::new(-tangent.z, 0.0, tangent.x).normalize();
    center + side * (lane / ROAD_WIDTH * TRACK_HALF_WIDTH)
}

fn draw_quad_3d<D: RaylibDraw3D>(d: &mut D, a: Vector3, b: Vector3, c: Vector3, e: Vector3, color: Color) {
    d.draw_triangle3D(a, b, c, color);
    d.draw_triangle3D(a, c, e, color);
    d.draw_triangle3D(c, b, a, color);
    d.draw_triangle3D(e, c, a, color);
}

fn draw_track_3d<D: RaylibDraw3D>(d: &mut D) {
    let colors = [Color::RED, Color::ORANGE, Color::YELLOW, Color::GREEN, Color::SKYBLUE, Color::BLUE, Color::PURPLE];
    let total = TRACK_SEGMENTS * TRACK_SUBDIVISIONS;

    for sample in 0..total {
        let next = (sample + 1) % total;
        let segment = sample / TRACK_SUBDIVISIONS;
        let t = (sample % TRACK_SUBDIVISIONS) as f32 / TRACK_SUBDIVISIONS as f32;
        let next_segment = next / TRACK_SUBDIVISIONS;
        let next_t = (next % TRACK_SUBDIVISIONS) as f32 / TRACK_SUBDIVISIONS as f32;
        let center_a = track_center(segment, t);
        let center_b = track_center(next_segment, next_t);
        let side_a = Vector3::new(-track_tangent(segment, t).z, 0.0, track_tangent(segment, t).x).normalize();
        let side_b = Vector3::new(-track_tangent(next_segment, next_t).z, 0.0, track_tangent(next_segment, next_t).x).normalize();

        let base_a_l = center_a - side_a * (TRACK_HALF_WIDTH + 0.25) - Vector3::new(0.0, 0.18, 0.0);
        let base_a_r = center_a + side_a * (TRACK_HALF_WIDTH + 0.25) - Vector3::new(0.0, 0.18, 0.0);
        let base_b_l = center_b - side_b * (TRACK_HALF_WIDTH + 0.25) - Vector3::new(0.0, 0.18, 0.0);
        let base_b_r = center_b + side_b * (TRACK_HALF_WIDTH + 0.25) - Vector3::new(0.0, 0.18, 0.0);
        draw_quad_3d(d, base_a_l, base_a_r, base_b_r, base_b_l, Color::new(8, 5, 18, 255));

        for band in 0..7 {
            let left = -TRACK_HALF_WIDTH + TRACK_HALF_WIDTH * 2.0 * band as f32 / 7.0;
            let right = -TRACK_HALF_WIDTH + TRACK_HALF_WIDTH * 2.0 * (band + 1) as f32 / 7.0;
            let inset = 0.035;
            let a_l = center_a + side_a * (left + inset);
            let a_r = center_a + side_a * (right - inset);
            let b_l = center_b + side_b * (left + inset);
            let b_r = center_b + side_b * (right - inset);
            draw_quad_3d(d, a_l, a_r, b_r, b_l, colors[(band + sample / 3) % 7]);
        }
    }
}

fn draw_objects_3d<D: RaylibDraw3D>(d: &mut D, game: &Game) {
    for item in &game.boxes {
        if item.active {
            let mut p = track_position(item.segment, item.t, 0.0);
            p.y += 0.75;
            d.draw_cube(p, 0.75, 0.75, 0.75, Color::new(60, 210, 255, 220));
            d.draw_cube_wires(p, 0.78, 0.78, 0.78, Color::WHITE);
        }
    }
}

fn draw_kart_3d<D: RaylibDraw3D>(d: &mut D, game: &Game) {
    let tangent = track_tangent(game.segment, game.segment_t);
    let side = Vector3::new(-tangent.z, 0.0, tangent.x).normalize();
    let mut p = track_position(game.segment, game.segment_t, game.lane);
    p.y += 0.42;
    d.draw_cube(p, 1.35, 0.45, 1.8, Color::RED);
    d.draw_cube(p + Vector3::new(0.0, 0.43, 0.0), 0.9, 0.52, 0.9, Color::GREEN);
    for lateral in [-0.68, 0.68] {
        for longitudinal in [-0.58, 0.58] {
            d.draw_cube(p + side * lateral + tangent * longitudinal - Vector3::new(0.0, 0.2, 0.0), 0.28, 0.35, 0.42, Color::BLACK);
        }
    }
}

#[derive(Clone, Copy)]
struct RoadSection { center: f32, y: f32, half: f32 }

fn advance_cursor(mut segment:usize,mut t:f32,mut distance:f32)->(usize,f32){
    while distance>0.0 {
        let length=(POINTS[segment+1]-POINTS[segment]).length().max(1.0);
        let remaining=(1.0-t)*length;
        if distance<=remaining { t+=distance/length; break; }
        distance-=remaining; segment=(segment+1)%(POINTS.len()-1); t=0.0;
    }
    (segment,t)
}

fn road_section(game:&Game,distance:f32)->RoadSection{
    let (segment,t)=advance_cursor(game.segment,game.segment_t,distance);
    let point=point_on_segment(segment,t,0.0);
    let player=point_on_segment(game.segment,game.segment_t,0.0);
    let forward=(POINTS[game.segment+1]-POINTS[game.segment]).normalize();
    let right=Vector2::new(-forward.y,forward.x);
    let curve=(point-player).dot(right);
    let scale=(1.0/(1.0+distance/285.0)).clamp(0.075,1.0);
    RoadSection{center:WIDTH as f32*0.5+curve*scale*2.05,y:92.0+(PLAYER_Y-92.0)*scale,half:510.0*scale}
}

fn draw_quad(d:&mut RaylibDrawHandle,a:Vector2,b:Vector2,c:Vector2,e:Vector2,color:Color){
    d.draw_triangle(a,b,c,color); d.draw_triangle(a,c,e,color);
}

fn draw_track_25d(d:&mut RaylibDrawHandle,game:&Game){
    let colors=[Color::RED,Color::ORANGE,Color::YELLOW,Color::GREEN,Color::SKYBLUE,Color::BLUE,Color::PURPLE];
    let mut sections=Vec::new();
    for i in 0..66 { sections.push(road_section(game,i as f32*20.0)); }
    for i in (0..sections.len()-1).rev(){
        let near=sections[i]; let far=sections[i+1];
        let row_near=RoadSection{center:near.center+(far.center-near.center)*0.08,y:near.y+(far.y-near.y)*0.08,half:near.half+(far.half-near.half)*0.08};
        let row_far=RoadSection{center:near.center+(far.center-near.center)*0.84,y:near.y+(far.y-near.y)*0.84,half:near.half+(far.half-near.half)*0.84};
        draw_quad(d,Vector2::new(near.center-near.half,near.y+6.0),Vector2::new(near.center+near.half,near.y+6.0),Vector2::new(far.center+far.half,far.y+3.0),Vector2::new(far.center-far.half,far.y+3.0),Color::new(4,2,12,255));
        for band in 0..7 {
            let gap=0.018;
            let f0=band as f32/7.0+gap; let f1=(band+1)as f32/7.0-gap;
            let n0=row_near.center-row_near.half+row_near.half*2.0*f0; let n1=row_near.center-row_near.half+row_near.half*2.0*f1;
            let q0=row_far.center-row_far.half+row_far.half*2.0*f0; let q1=row_far.center-row_far.half+row_far.half*2.0*f1;
            draw_quad(d,Vector2::new(n0,row_near.y),Vector2::new(n1,row_near.y),Vector2::new(q1,row_far.y),Vector2::new(q0,row_far.y),colors[(band+i)%7]);
            d.draw_line_ex(Vector2::new(n0,row_near.y),Vector2::new(n1,row_near.y),(row_near.half*0.018).max(1.0),brighten(colors[(band+i)%7]));
        }
    }
}

fn brighten(c:Color)->Color{Color::new(c.r.saturating_add(75),c.g.saturating_add(75),c.b.saturating_add(75),255)}

fn forward_distance(game:&Game,target_segment:usize,target_t:f32)->f32{
    let mut segment=game.segment; let mut t=game.segment_t; let mut total=0.0;
    for _ in 0..POINTS.len(){
        let length=(POINTS[segment+1]-POINTS[segment]).length();
        if segment==target_segment && target_t>=t { return total+(target_t-t)*length; }
        total+=(1.0-t)*length; segment=(segment+1)%(POINTS.len()-1); t=0.0;
    }
    9999.0
}

fn marker_screen(game:&Game,segment:usize,t:f32,lane:f32)->Option<(Vector2,f32)>{
    let distance=forward_distance(game,segment,t);
    if distance>1350.0{return None;}
    let road=road_section(game,distance);
    let scale=(road.half/270.0).clamp(0.08,1.0);
    Some((Vector2::new(road.center+lane*scale*8.0,road.y),scale))
}

fn draw_objects_25d(d:&mut RaylibDrawHandle,game:&Game){
    for item in &game.boxes { if item.active { if let Some((p,s))=marker_screen(game,item.segment,item.t,0.0){
        let size=28.0*s; d.draw_rectangle((p.x-size)as i32,(p.y-size*2.0)as i32,(size*2.0)as i32,(size*2.0)as i32,Color::new(70,220,255,220));
        d.draw_rectangle_lines((p.x-size)as i32,(p.y-size*2.0)as i32,(size*2.0)as i32,(size*2.0)as i32,Color::WHITE);
        let fs=(30.0*s).max(10.0)as i32; d.draw_text("?",p.x as i32-d.measure_text("?",fs)/2,(p.y-size*1.85)as i32,fs,Color::WHITE);
    }}}
}

fn draw_player_kart(d:&mut RaylibDrawHandle,game:&Game){
    let sway=game.lane/ROAD_WIDTH*105.0; let x=WIDTH as f32*0.5+sway; let y=PLAYER_Y+18.0;
    d.draw_rectangle((x-75.0)as i32,(y+4.0)as i32,150,42,Color::new(6,6,12,255));
    d.draw_rectangle((x-66.0)as i32,(y-14.0)as i32,132,48,Color::new(196,12,25,255));
    d.draw_rectangle((x-49.0)as i32,(y-27.0)as i32,98,34,Color::new(25,125,50,255));
    d.draw_rectangle((x-39.0)as i32,(y-62.0)as i32,78,38,Color::new(245,205,160,255));
    d.draw_rectangle((x-29.0)as i32,(y-76.0)as i32,58,22,Color::new(205,18,24,255));
    d.draw_rectangle((x-52.0)as i32,(y-25.0)as i32,16,22,Color::WHITE); d.draw_rectangle((x+36.0)as i32,(y-25.0)as i32,16,22,Color::WHITE);
    d.draw_rectangle((x-68.0)as i32,(y+23.0)as i32,30,25,Color::BLACK); d.draw_rectangle((x+38.0)as i32,(y+23.0)as i32,30,25,Color::BLACK);
    d.draw_rectangle((x-27.0)as i32,(y+18.0)as i32,54,20,Color::new(245,230,215,255));
    d.draw_rectangle((x-18.0)as i32,(y+23.0)as i32,36,10,Color::new(205,25,35,255));
    if game.boost>0.0 { d.draw_triangle(Vector2::new(x-24.0,y+42.0),Vector2::new(x,y+86.0),Vector2::new(x+24.0,y+42.0),Color::ORANGE); }
}

fn draw_minimap(d:&mut RaylibDrawHandle,game:&Game){
    let origin=Vector2::new(745.0,440.0); let scale=0.20;
    d.draw_rectangle_rounded(Rectangle::new(730.0,425.0,210.0,195.0),0.12,8,Color::new(4,3,25,210));
    for i in 0..POINTS.len()-1 { let a=origin+POINTS[i]*scale-Vector2::new(8.0,8.0); let b=origin+POINTS[i+1]*scale-Vector2::new(8.0,8.0); d.draw_line_ex(a,b,7.0,Color::WHITE); d.draw_line_ex(a,b,4.0,rainbow_minimap_color(i)); }
    let p=origin+game.kart_position()*scale-Vector2::new(8.0,8.0); d.draw_circle(p.x as i32,p.y as i32,6.0,Color::RED);
}

fn rainbow_minimap_color(i:usize)->Color{[Color::RED,Color::ORANGE,Color::YELLOW,Color::GREEN,Color::SKYBLUE,Color::BLUE,Color::PURPLE][i%7]}

fn overlay(d:&mut RaylibDrawHandle){d.draw_rectangle(0,0,WIDTH,HEIGHT,Color::new(2,1,18,190));}
fn centered(d:&mut RaylibDrawHandle,text:&str,y:i32,size:i32,color:Color){let w=d.measure_text(text,size);d.draw_text(text,(WIDTH-w)/2,y,size,color);}
