mod acceleration;
mod core;
mod materials;
mod objects;
mod renderer;
mod scene;
mod textures;
mod worlds;

use raylib::audio::RaylibAudio;
use raylib::prelude::*;

use crate::core::camera::Camera;
use crate::core::framebuffer::Framebuffer;
use crate::core::vec3::Vec3;

use crate::materials::material::Material;

use crate::objects::cone::Cone;
use crate::objects::cube::Cube;
use crate::objects::cylinder::Cylinder;
use crate::objects::ellipsoid::Ellipsoid;
use crate::objects::hemisphere::Hemisphere;
use crate::objects::object::Object;
use crate::objects::plane::Plane;
use crate::objects::sphere::Sphere;
use crate::objects::torus::Torus;

use crate::scene::light::Light;
use crate::scene::scene::Scene;
use crate::scene::state::SceneState;

const RENDER_WIDTH: i32 = 1280;
const RENDER_HEIGHT: i32 = 720;
const GALAXY_COUNT: usize = 2;

struct Sparkle {
    position: Vector2,
    velocity: Vector2,
    radius: f32,
    active: bool,
    rotation: f32,
    rotation_speed: f32,
    color: Color,
}

struct PlanetCollider {
    center: Vector2,
    radius: f32,
}

struct Viewport {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(Clone, Copy, PartialEq)]
enum GalaxyTransition {
    None,
    ZoomOut,
    Warp,
    ZoomIn,
}

fn main() {
    let (mut rl, thread) =
        raylib::init()
            .size(800, 600)
            .title("Galaxy Diorama")
            .build();

    let audio =
        RaylibAudio::init_audio_device()
            .expect("No se pudo iniciar el audio");

    let starbit_sound =
        audio
            .new_sound(
                "assets/sounds/starbit.mp3",
            )
            .expect(
                "No se pudo cargar starbit.mp3",
            );

    let mut level_music =
        audio
            .new_music(
                "assets/sounds/level.mp3",
            )
            .expect(
                "No se pudo cargar level.mp3",
            );

    level_music.set_looping(true);

    let monitor =
        raylib::core::window::get_current_monitor();

    let screen_width =
        raylib::core::window::get_monitor_width(
            monitor,
        );

    let screen_height =
        raylib::core::window::get_monitor_height(
            monitor,
        );

    rl.set_window_size(
        screen_width,
        screen_height,
    );

    rl.toggle_fullscreen();

    rl.set_target_fps(60);
    rl.hide_cursor();

    let mut framebuffer =
        Framebuffer::new(
            RENDER_WIDTH,
            RENDER_HEIGHT,
        );

    let render_image =
        unsafe {
            Image::from_raw(
                raylib::ffi::GenImageColor(
                    RENDER_WIDTH,
                    RENDER_HEIGHT,
                    Color::BLACK.into(),
                ),
            )
        };

    let mut render_texture =
        rl.load_texture_from_image(
            &thread,
            &render_image,
        )
        .expect(
            "No se pudo crear la textura del framebuffer",
        );

    render_texture.set_texture_filter(
        &thread,
        TextureFilter::TEXTURE_FILTER_BILINEAR,
    );

    let planets =
        worlds::planet_registry();

    let focused_scenes: Vec<Scene> =
        planets
            .iter()
            .map(|planet| {
                Scene::new(
                    (planet.create)(),
                )
            })
            .collect();

    let hit_material =
        Material::new(
            Vec3::new(
                1.0,
                1.0,
                1.0,
            ),
            1.0,
            0.0,
            0.0,
            0.0,
        );

    let path_yellow_material =
        Material::new(
            Vec3::new(
                1.0,
                0.82,
                0.05,
            ),
            0.95,
            0.80,
            0.0,
            0.15,
        );

    let light =
        Light::new(
            Vec3::new(
                -2.5,
                6.0,
                6.5,
            ),
            Vec3::new(
                1.0,
                1.0,
                1.0,
            ),
            1.35,
        );

    let mut camera =
        Camera::new(
            Vec3::new(
                0.0,
                0.0,
                0.0,
            ),
            9.5,
            60.0,
        );

    let mut state =
        SceneState::Galaxy;

    let mut selected_planet:
        Option<usize> =
        None;

    let mut sparkles:
        Vec<Sparkle> =
        Vec::new();

    let mut sparkle_score:
        u32 =
        0;

    let mut sparkle_spawn_timer =
        0.0;

    let mut sparkle_spawn_id:
        u32 =
        0;

    let mut focused_velocity_x:
        f32 =
        50.0;

    let mut focused_velocity_y:
        f32 =
        0.0;

    let mut current_galaxy:
        usize =
        0;

    let mut galaxy_transition =
        GalaxyTransition::None;

    let mut transition_timer:
        f32 =
        0.0;

    while !rl.window_should_close() {
        let current_screen_width =
            rl.get_screen_width();

        let current_screen_height =
            rl.get_screen_height();

        let dt =
            rl.get_frame_time()
                .min(0.033);

        let current_time =
            rl.get_time()
                as f32;

        let mouse_position =
            rl.get_mouse_position();

        let viewport =
            calculate_viewport(
                current_screen_width
                    as f32,
                current_screen_height
                    as f32,
            );

        if state == SceneState::Galaxy
            && galaxy_transition
                == GalaxyTransition::None
            && rl.is_key_pressed(
                KeyboardKey::KEY_N,
            )
        {
            galaxy_transition =
                GalaxyTransition::ZoomOut;

            transition_timer =
                0.0;

            sparkles.clear();
        }

        if galaxy_transition
            != GalaxyTransition::None
        {
            transition_timer +=
                dt;

            match galaxy_transition {
                GalaxyTransition::ZoomOut => {
                    if transition_timer
                        >= 0.85
                    {
                        current_galaxy =
                            (
                                current_galaxy
                                    + 1
                            )
                                % GALAXY_COUNT;

                        galaxy_transition =
                            GalaxyTransition::Warp;

                        transition_timer =
                            0.0;

                        camera =
                            Camera::new(
                                Vec3::new(
                                    0.0,
                                    0.0,
                                    0.0,
                                ),
                                9.5,
                                60.0,
                            );
                    }
                }

                GalaxyTransition::Warp => {
                    if transition_timer
                        >= 0.85
                    {
                        galaxy_transition =
                            GalaxyTransition::ZoomIn;

                        transition_timer =
                            0.0;
                    }
                }

                GalaxyTransition::ZoomIn => {
                    if transition_timer
                        >= 1.0
                    {
                        galaxy_transition =
                            GalaxyTransition::None;

                        transition_timer =
                            0.0;
                    }
                }

                GalaxyTransition::None => {}
            }
        }

        if galaxy_transition
            == GalaxyTransition::None
        {
            match state {
                SceneState::Galaxy => {
                    if rl.is_mouse_button_pressed(
                        MouseButton::MOUSE_BUTTON_LEFT,
                    ) {
                        if point_inside_viewport(
                            mouse_position,
                            &viewport,
                        ) {
                            let render_mouse =
                                screen_to_render(
                                    mouse_position,
                                    &viewport,
                                );

                            let ray =
                                camera.get_ray(
                                    render_mouse.x,
                                    render_mouse.y,
                                    RENDER_WIDTH
                                        as f32,
                                    RENDER_HEIGHT
                                        as f32,
                                );

                            let mut closest =
                                f32::INFINITY;

                            let mut selected_index:
                                Option<usize> =
                                None;

                            for (
                                index,
                                planet,
                            ) in planets
                                .iter()
                                .enumerate()
                            {
                                let position =
                                    galaxy_position(
                                        planet.position,
                                        index,
                                        current_galaxy,
                                    );

                                let hit_object =
                                    Sphere::new(
                                        position,
                                        planet.hit_radius,
                                        hit_material,
                                    );

                                if let Some(
                                    distance,
                                ) =
                                    hit_object
                                        .intersect(
                                            &ray.origin,
                                            &ray.direction,
                                        )
                                {
                                    if distance
                                        < closest
                                    {
                                        closest =
                                            distance;

                                        selected_index =
                                            Some(index);
                                    }
                                }
                            }

                            if let Some(index) =
                                selected_index
                            {
                                selected_planet =
                                    Some(index);

                                focused_velocity_x =
                                    50.0;

                                focused_velocity_y =
                                    0.0;

                                camera =
                                    Camera::new(
                                        Vec3::new(
                                            0.0,
                                            0.0,
                                            0.0,
                                        ),
                                        5.2,
                                        60.0,
                                    );

                                level_music
                                    .play_stream();

                                state =
                                    SceneState::Focused;
                            }
                        }
                    }

                    if rl.is_mouse_button_down(
                        MouseButton::MOUSE_BUTTON_RIGHT,
                    ) {
                        let delta =
                            rl.get_mouse_delta();

                        camera.rotate(
                            delta.x,
                            delta.y,
                        );
                    }
                }

                SceneState::Focused => {
                    if rl.is_key_pressed(
                        KeyboardKey::KEY_BACKSPACE,
                    ) {
                        level_music
                            .stop_stream();

                        state =
                            SceneState::Galaxy;

                        selected_planet =
                            None;

                        focused_velocity_x =
                            50.0;

                        focused_velocity_y =
                            0.0;

                        camera =
                            Camera::new(
                                Vec3::new(
                                    0.0,
                                    0.0,
                                    0.0,
                                ),
                                9.5,
                                60.0,
                            );
                    } else {
                        if rl.is_mouse_button_down(
                            MouseButton::MOUSE_BUTTON_LEFT,
                        ) {
                            let delta =
                                rl.get_mouse_delta();

                            camera.rotate(
                                delta.x,
                                delta.y,
                            );

                            if dt > 0.0001 {
                                focused_velocity_x =
                                    delta.x
                                        / dt;

                                focused_velocity_y =
                                    delta.y
                                        / dt;
                            }
                        } else {
                            camera.rotate(
                                focused_velocity_x
                                    * dt,
                                focused_velocity_y
                                    * dt,
                            );

                            let damping =
                                0.985_f32
                                    .powf(
                                        dt * 60.0,
                                    );

                            focused_velocity_x *=
                                damping;

                            focused_velocity_y *=
                                damping;

                            let automatic_speed =
                                50.0;

                            if focused_velocity_x
                                .abs()
                                < automatic_speed
                            {
                                let direction =
                                    if focused_velocity_x
                                        < 0.0
                                    {
                                        -1.0
                                    } else {
                                        1.0
                                    };

                                let target =
                                    direction
                                        * automatic_speed;

                                focused_velocity_x +=
                                    (
                                        target
                                            - focused_velocity_x
                                    )
                                        * 0.8
                                        * dt;
                            }

                            if focused_velocity_y
                                .abs()
                                < 0.5
                            {
                                focused_velocity_y =
                                    0.0;
                            }
                        }
                    }
                }
            }

            let wheel =
                rl.get_mouse_wheel_move();

            if wheel != 0.0 {
                camera.zoom(
                    wheel,
                );
            }
        }

        if state
            == SceneState::Galaxy
            && galaxy_transition
                == GalaxyTransition::None
        {
            sparkle_spawn_timer +=
                dt;

            if sparkle_spawn_timer
                >= 0.70
            {
                sparkle_spawn_id +=
                    1;

                spawn_sparkle(
                    &mut sparkles,
                    &viewport,
                    current_time,
                    sparkle_spawn_id,
                );

                sparkle_spawn_timer =
                    0.0;
            }

            let planet_colliders =
                create_planet_colliders(
                    &planets,
                    &camera,
                    &viewport,
                    current_galaxy,
                );

            update_sparkles(
                &mut sparkles,
                &planet_colliders,
                dt,
                &viewport,
            );

            collect_sparkles(
                &mut sparkles,
                mouse_position,
                &mut sparkle_score,
                &starbit_sound,
            );
        }

        if state
            == SceneState::Focused
        {
            level_music
                .update_stream();

            if !level_music
                .is_stream_playing()
            {
                level_music
                    .play_stream();
            }
        }

        match state {
            SceneState::Galaxy => {
                let rotation =
                    current_time
                        * 0.25;

                let world_scale =
                    transition_world_scale(
                        galaxy_transition,
                        transition_timer,
                    );

                let mut galaxy_objects =
                    Vec::new();

                let mut nodes:
                    Vec<Vec3> =
                    Vec::new();

                for (
                    index,
                    planet,
                ) in planets
                    .iter()
                    .enumerate()
                {
                    let position =
                        galaxy_position(
                            planet.position,
                            index,
                            current_galaxy,
                        );

                    let node_position =
                        (
                            position
                                + planet.node_offset
                        )
                            * world_scale;

                    nodes.push(
                        node_position,
                    );
                }

                if nodes.len() > 1 {
                    for i in
                        0..nodes.len() - 1
                    {
                        add_path_scaled(
                            &mut galaxy_objects,
                            nodes[i],
                            nodes[i + 1],
                            path_yellow_material,
                            world_scale,
                        );
                    }
                }

                for node in &nodes {
                    add_planet_node_scaled(
                        &mut galaxy_objects,
                        *node,
                        path_yellow_material,
                        world_scale,
                    );
                }

                for (
                    index,
                    planet,
                ) in planets
                    .iter()
                    .enumerate()
                {
                    let position =
                        galaxy_position(
                            planet.position,
                            index,
                            current_galaxy,
                        )
                            * world_scale;

                    let preview =
                        transform_objects_rotated(
                            (planet.create)(),
                            position,
                            planet.preview_scale
                                * world_scale,
                            rotation,
                        );

                    galaxy_objects
                        .extend(
                            preview,
                        );
                }

                let galaxy_scene =
                    Scene::new(
                        galaxy_objects,
                    );

                renderer::raytracer::render_rotated_with_skybox(
                    &mut framebuffer,
                    &galaxy_scene,
                    &light,
                    &camera,
                    rotation,
                    current_galaxy,
                );
            }

            SceneState::Focused => {
                if let Some(index) =
                    selected_planet
                {
                    renderer::raytracer::render_with_skybox(
                        &mut framebuffer,
                        &focused_scenes[index],
                        &light,
                        &camera,
                        current_galaxy,
                    );
                }
            }
        }

        let pixel_bytes =
            unsafe {
                std::slice::from_raw_parts(
                    framebuffer
                        .pixels()
                        .as_ptr()
                        as *const u8,
                    framebuffer
                        .pixels()
                        .len()
                        * std::mem::size_of::<Color>(),
                )
            };

        render_texture
            .update_texture(
                pixel_bytes,
            )
            .expect(
                "No se pudo actualizar la textura",
            );

        let mut d =
            rl.begin_drawing(
                &thread,
            );

        d.clear_background(
            Color::BLACK,
        );

        let source =
            Rectangle::new(
                0.0,
                0.0,
                RENDER_WIDTH
                    as f32,
                RENDER_HEIGHT
                    as f32,
            );

        let destination =
            Rectangle::new(
                viewport.x,
                viewport.y,
                viewport.width,
                viewport.height,
            );

        d.draw_texture_pro(
            &render_texture,
            source,
            destination,
            Vector2::new(
                0.0,
                0.0,
            ),
            0.0,
            Color::WHITE,
        );

        if galaxy_transition
            == GalaxyTransition::None
        {
            match state {
                SceneState::Galaxy => {
                    draw_sparkles(
                        &mut d,
                        &sparkles,
                    );

                    d.draw_text(
                        &format!(
                            "Galaxia {}",
                            current_galaxy
                                + 1
                        ),
                        30,
                        30,
                        30,
                        Color::WHITE,
                    );

                    d.draw_text(
                        "Click izquierdo - Seleccionar",
                        30,
                        70,
                        20,
                        Color::LIGHTGRAY,
                    );

                    d.draw_text(
                        "Click derecho - Rotar mapa",
                        30,
                        100,
                        20,
                        Color::LIGHTGRAY,
                    );

                    d.draw_text(
                        "Rueda - Zoom",
                        30,
                        130,
                        20,
                        Color::LIGHTGRAY,
                    );

                    d.draw_text(
                        "N - Viajar a otra galaxia",
                        30,
                        160,
                        20,
                        Color::LIGHTGRAY,
                    );

                    d.draw_text(
                        &format!(
                            "Destellos: {}",
                            sparkle_score,
                        ),
                        30,
                        200,
                        24,
                        Color::new(
                            255,
                            235,
                            70,
                            255,
                        ),
                    );
                }

                SceneState::Focused => {
                    if let Some(index) =
                        selected_planet
                    {
                        d.draw_text(
                            planets[index]
                                .name,
                            30,
                            30,
                            32,
                            Color::WHITE,
                        );
                    }

                    d.draw_text(
                        "Click izquierdo - Girar libremente",
                        30,
                        75,
                        20,
                        Color::LIGHTGRAY,
                    );

                    d.draw_text(
                        "Suelta - Mantener inercia",
                        30,
                        105,
                        20,
                        Color::LIGHTGRAY,
                    );

                    d.draw_text(
                        "Rueda - Zoom",
                        30,
                        135,
                        20,
                        Color::LIGHTGRAY,
                    );

                    d.draw_text(
                        "BACKSPACE - Regresar",
                        30,
                        165,
                        20,
                        Color::LIGHTGRAY,
                    );
                }
            }
        }

        if galaxy_transition
            != GalaxyTransition::None
        {
            draw_transition(
                &mut d,
                current_screen_width,
                current_screen_height,
                galaxy_transition,
                transition_timer,
            );
        }

        d.draw_fps(
            current_screen_width
                - 110,
            20,
        );

        draw_star_cursor(
            &mut d,
            mouse_position,
        );
    }
}

fn galaxy_position(
    original: Vec3,
    index: usize,
    galaxy: usize,
) -> Vec3 {
    match galaxy {
        0 => original,

        1 => {
            match index {
                0 => {
                    Vec3::new(
                        -3.8,
                        1.20,
                        0.25,
                    )
                }

                1 => {
                    Vec3::new(
                        -2.0,
                        -0.20,
                        -0.20,
                    )
                }

                2 => {
                    Vec3::new(
                        0.0,
                        0.90,
                        0.15,
                    )
                }

                3 => {
                    Vec3::new(
                        2.0,
                        -0.15,
                        -0.20,
                    )
                }

                4 => {
                    Vec3::new(
                        3.8,
                        1.10,
                        0.25,
                    )
                }

                _ => {
                    let angle =
                        index as f32
                            * 1.3;

                    Vec3::new(
                        angle.cos()
                            * 4.0,
                        angle.sin()
                            * 0.8,
                        angle.sin()
                            * 0.5,
                    )
                }
            }
        }

        _ => original,
    }
}

fn transition_world_scale(
    transition: GalaxyTransition,
    timer: f32,
) -> f32 {
    match transition {
        GalaxyTransition::None => {
            1.0
        }

        GalaxyTransition::ZoomOut => {
            let t =
                (
                    timer
                        / 0.85
                )
                    .clamp(
                        0.0,
                        1.0,
                    );

            let t =
                smoothstep(
                    t,
                );

            1.0
                - t
                    * 0.82
        }

        GalaxyTransition::Warp => {
            0.18
        }

        GalaxyTransition::ZoomIn => {
            let t =
                (
                    timer
                        / 1.0
                )
                    .clamp(
                        0.0,
                        1.0,
                    );

            let t =
                smoothstep(
                    t,
                );

            0.18
                + t
                    * 0.82
        }
    }
}

fn smoothstep(
    value: f32,
) -> f32 {
    value
        * value
        * (
            3.0
                - 2.0
                    * value
        )
}

fn draw_transition(
    d: &mut RaylibDrawHandle<'_>,
    width: i32,
    height: i32,
    transition: GalaxyTransition,
    timer: f32,
) {
    match transition {
        GalaxyTransition::ZoomOut => {
            let t =
                (
                    timer
                        / 0.85
                )
                    .clamp(
                        0.0,
                        1.0,
                    );

            let alpha =
                (
                    t
                        * 120.0
                )
                    as u8;

            d.draw_rectangle(
                0,
                0,
                width,
                height,
                Color::new(
                    0,
                    0,
                    20,
                    alpha,
                ),
            );
        }

        GalaxyTransition::Warp => {
            let t =
                (
                    timer
                        / 0.85
                )
                    .clamp(
                        0.0,
                        1.0,
                    );

            draw_warp(
                d,
                width,
                height,
                t,
            );

            let flash_strength =
                1.0
                    - (
                        t * 2.0
                            - 1.0
                    )
                        .abs();

            let flash_alpha =
                (
                    flash_strength
                        .clamp(
                            0.0,
                            1.0,
                        )
                        * 220.0
                )
                    as u8;

            d.draw_rectangle(
                0,
                0,
                width,
                height,
                Color::new(
                    255,
                    255,
                    255,
                    flash_alpha,
                ),
            );

            d.draw_text(
                "Viajando...",
                width / 2
                    - 75,
                height
                    - 90,
                26,
                Color::WHITE,
            );
        }

        GalaxyTransition::ZoomIn => {
            let t =
                (
                    timer
                        / 1.0
                )
                    .clamp(
                        0.0,
                        1.0,
                    );

            let alpha =
                (
                    (
                        1.0
                            - t
                    )
                        * 120.0
                )
                    as u8;

            d.draw_rectangle(
                0,
                0,
                width,
                height,
                Color::new(
                    0,
                    0,
                    20,
                    alpha,
                ),
            );
        }

        GalaxyTransition::None => {}
    }
}

fn draw_warp(
    d: &mut RaylibDrawHandle<'_>,
    width: i32,
    height: i32,
    progress: f32,
) {
    let center =
        Vector2::new(
            width as f32
                * 0.5,
            height as f32
                * 0.5,
        );

    let max_distance =
        (
            width.max(
                height,
            )
        )
            as f32
            * 0.75;

    for i in 0..100 {
        let seed =
            i as f32
                * 19.731
                + 3.17;

        let angle =
            pseudo_random(
                seed,
            )
                * std::f32::consts::PI
                * 2.0;

        let base =
            pseudo_random(
                seed * 2.31,
            );

        let speed =
            0.25
                + pseudo_random(
                    seed * 4.91,
                )
                    * 0.75;

        let movement =
            (
                progress
                    * speed
            )
                .fract();

        let distance =
            (
                base
                    + movement
            )
                .fract()
                * max_distance;

        let streak =
            15.0
                + progress
                    * 140.0
                + pseudo_random(
                    seed * 7.11,
                )
                    * 40.0;

        let start =
            Vector2::new(
                center.x
                    + angle.cos()
                        * distance,

                center.y
                    + angle.sin()
                        * distance,
            );

        let end =
            Vector2::new(
                center.x
                    + angle.cos()
                        * (
                            distance
                                + streak
                        ),

                center.y
                    + angle.sin()
                        * (
                            distance
                                + streak
                        ),
            );

        let color_value =
            pseudo_random(
                seed * 9.7,
            );

        let color =
            if color_value < 0.25 {
                Color::new(
                    120,
                    190,
                    255,
                    230,
                )
            } else if color_value
                < 0.50
            {
                Color::new(
                    210,
                    150,
                    255,
                    230,
                )
            } else if color_value
                < 0.75
            {
                Color::new(
                    255,
                    240,
                    130,
                    230,
                )
            } else {
                Color::new(
                    255,
                    255,
                    255,
                    230,
                )
            };

        d.draw_line_ex(
            start,
            end,
            2.0
                + progress
                    * 2.5,
            color,
        );
    }

    d.draw_circle_v(
        center,
        10.0
            + progress
                * 45.0,
        Color::new(
            255,
            255,
            255,
            70,
        ),
    );
}

fn calculate_viewport(
    screen_width: f32,
    screen_height: f32,
) -> Viewport {
    let render_aspect =
        RENDER_WIDTH as f32
            / RENDER_HEIGHT as f32;

    let screen_aspect =
        screen_width
            / screen_height;

    let (
        width,
        height,
    ) =
        if screen_aspect
            > render_aspect
        {
            (
                screen_height
                    * render_aspect,
                screen_height,
            )
        } else {
            (
                screen_width,
                screen_width
                    / render_aspect,
            )
        };

    Viewport {
        x:
            (
                screen_width
                    - width
            )
                * 0.5,

        y:
            (
                screen_height
                    - height
            )
                * 0.5,

        width,
        height,
    }
}

fn point_inside_viewport(
    point: Vector2,
    viewport: &Viewport,
) -> bool {
    point.x >= viewport.x
        && point.x
            <= viewport.x
                + viewport.width
        && point.y
            >= viewport.y
        && point.y
            <= viewport.y
                + viewport.height
}

fn screen_to_render(
    point: Vector2,
    viewport: &Viewport,
) -> Vector2 {
    Vector2::new(
        (
            point.x
                - viewport.x
        )
            / viewport.width
            * RENDER_WIDTH
                as f32,

        (
            point.y
                - viewport.y
        )
            / viewport.height
            * RENDER_HEIGHT
                as f32,
    )
}

fn world_to_screen(
    world: Vec3,
    camera: &Camera,
    viewport: &Viewport,
) -> Option<Vector2> {
    let forward =
        (
            camera.target
                - camera.position
        )
            .normalize();

    let reference_up =
        Vec3::new(
            0.0,
            1.0,
            0.0,
        );

    let mut right =
        forward.cross(
            &reference_up,
        );

    if right.length()
        < 0.001
    {
        right =
            Vec3::new(
                1.0,
                0.0,
                0.0,
            );
    }

    right =
        right.normalize();

    let up =
        right
            .cross(
                &forward,
            )
            .normalize();

    let relative =
        world
            - camera.position;

    let camera_x =
        relative.dot(
            &right,
        );

    let camera_y =
        relative.dot(
            &up,
        );

    let camera_z =
        relative.dot(
            &forward,
        );

    if camera_z <= 0.01 {
        return None;
    }

    let aspect =
        RENDER_WIDTH
            as f32
            / RENDER_HEIGHT
                as f32;

    let fov =
        camera.fov
            .to_radians();

    let focal =
        1.0
            / (
                fov
                    * 0.5
            )
                .tan();

    let ndc_x =
        camera_x
            * focal
            / aspect
            / camera_z;

    let ndc_y =
        camera_y
            * focal
            / camera_z;

    let render_x =
        (
            ndc_x
                + 1.0
        )
            * 0.5
            * RENDER_WIDTH
                as f32;

    let render_y =
        (
            1.0
                - ndc_y
        )
            * 0.5
            * RENDER_HEIGHT
                as f32;

    Some(
        Vector2::new(
            viewport.x
                + render_x
                    / RENDER_WIDTH
                        as f32
                    * viewport.width,

            viewport.y
                + render_y
                    / RENDER_HEIGHT
                        as f32
                    * viewport.height,
        ),
    )
}

fn create_planet_colliders(
    planets: &[scene::planet::PlanetDefinition],
    camera: &Camera,
    viewport: &Viewport,
    current_galaxy: usize,
) -> Vec<PlanetCollider> {
    let mut colliders =
        Vec::new();

    let forward =
        (
            camera.target
                - camera.position
        )
            .normalize();

    let reference_up =
        Vec3::new(
            0.0,
            1.0,
            0.0,
        );

    let mut camera_right =
        forward.cross(
            &reference_up,
        );

    if camera_right.length()
        < 0.001
    {
        camera_right =
            Vec3::new(
                1.0,
                0.0,
                0.0,
            );
    }

    camera_right =
        camera_right.normalize();

    for (
        index,
        planet,
    ) in planets
        .iter()
        .enumerate()
    {
        let position =
            galaxy_position(
                planet.position,
                index,
                current_galaxy,
            );

        let center =
            match world_to_screen(
                position,
                camera,
                viewport,
            ) {
                Some(value) => value,
                None => continue,
            };

        let collider_world_radius =
            planet.hit_radius
                * planet.preview_scale;

        let edge_world =
            position
                + camera_right
                    * collider_world_radius;

        let edge =
            match world_to_screen(
                edge_world,
                camera,
                viewport,
            ) {
                Some(value) => value,
                None => continue,
            };

        let dx =
            edge.x
                - center.x;

        let dy =
            edge.y
                - center.y;

        let radius =
            (
                dx * dx
                    + dy * dy
            )
                .sqrt()
                * 0.92;

        if radius > 2.0 {
            colliders.push(
                PlanetCollider {
                    center,
                    radius,
                },
            );
        }
    }

    colliders
}

fn pseudo_random(
    seed: f32,
) -> f32 {
    let value =
        (
            seed
                * 12.9898
        )
            .sin()
            * 43758.5453;

    value
        - value.floor()
}

fn sparkle_color(
    value: f32,
) -> Color {
    let index =
        (
            value * 6.0
        )
            .floor()
            as i32;

    match index {
        0 => {
            Color::new(
                235,
                55,
                55,
                255,
            )
        }

        1 => {
            Color::new(
                70,
                220,
                90,
                255,
            )
        }

        2 => {
            Color::new(
                65,
                135,
                255,
                255,
            )
        }

        3 => {
            Color::new(
                170,
                80,
                235,
                255,
            )
        }

        4 => {
            Color::new(
                210,
                220,
                230,
                255,
            )
        }

        _ => {
            Color::new(
                255,
                220,
                55,
                255,
            )
        }
    }
}

fn spawn_sparkle(
    sparkles: &mut Vec<Sparkle>,
    viewport: &Viewport,
    time: f32,
    spawn_id: u32,
) {
    if sparkles.len()
        >= 30
    {
        return;
    }

    let seed =
        time
            + spawn_id
                as f32
                * 3.731;

    let x_random =
        pseudo_random(
            seed * 17.31,
        );

    let speed_random =
        pseudo_random(
            seed * 31.73,
        );

    let direction_random =
        pseudo_random(
            seed * 47.19,
        );

    let size_random =
        pseudo_random(
            seed * 61.53,
        );

    let spin_random =
        pseudo_random(
            seed * 77.11,
        );

    let color_random =
        pseudo_random(
            seed * 91.37,
        );

    let x =
        viewport.x
            + 35.0
            + x_random
                * (
                    viewport.width
                        - 70.0
                );

    let horizontal_speed =
        (
            direction_random
                - 0.5
        )
            * 120.0;

    sparkles.push(
        Sparkle {
            position:
                Vector2::new(
                    x,
                    viewport.y
                        - 25.0,
                ),

            velocity:
                Vector2::new(
                    horizontal_speed,
                    15.0
                        + speed_random
                            * 35.0,
                ),

            radius:
                7.0
                    + size_random
                        * 5.0,

            active:
                true,

            rotation:
                spin_random
                    * std::f32::consts::PI
                    * 2.0,

            rotation_speed:
                -4.0
                    + spin_random
                        * 8.0,

            color:
                sparkle_color(
                    color_random,
                ),
        },
    );
}

fn update_sparkles(
    sparkles: &mut Vec<Sparkle>,
    colliders: &[PlanetCollider],
    dt: f32,
    viewport: &Viewport,
) {
    let gravity =
        360.0;

    for sparkle in
        sparkles.iter_mut()
    {
        if !sparkle.active {
            continue;
        }

        sparkle.velocity.y +=
            gravity
                * dt;

        sparkle.position.x +=
            sparkle.velocity.x
                * dt;

        sparkle.position.y +=
            sparkle.velocity.y
                * dt;

        sparkle.rotation +=
            sparkle.rotation_speed
                * dt;

        for collider in colliders {
            collide_sparkle_planet(
                sparkle,
                collider,
            );
        }

        let left =
            viewport.x
                + sparkle.radius;

        let right =
            viewport.x
                + viewport.width
                - sparkle.radius;

        if sparkle.position.x
            < left
        {
            sparkle.position.x =
                left;

            sparkle.velocity.x =
                sparkle.velocity
                    .x
                    .abs()
                    * 0.72;

            sparkle.rotation_speed +=
                1.0;
        }

        if sparkle.position.x
            > right
        {
            sparkle.position.x =
                right;

            sparkle.velocity.x =
                -sparkle.velocity
                    .x
                    .abs()
                    * 0.72;

            sparkle.rotation_speed -=
                1.0;
        }

        if sparkle.position.y
            > viewport.y
                + viewport.height
                + 80.0
        {
            sparkle.active =
                false;
        }
    }

    sparkles.retain(
        |sparkle| {
            sparkle.active
        },
    );
}

fn collide_sparkle_planet(
    sparkle: &mut Sparkle,
    planet: &PlanetCollider,
) {
    let dx =
        sparkle.position.x
            - planet.center.x;

    let dy =
        sparkle.position.y
            - planet.center.y;

    let distance_squared =
        dx * dx
            + dy * dy;

    let minimum_distance =
        sparkle.radius
            + planet.radius;

    if distance_squared
        >= minimum_distance
            * minimum_distance
    {
        return;
    }

    let distance =
        distance_squared
            .sqrt()
            .max(
                0.001,
            );

    let normal =
        Vector2::new(
            dx / distance,
            dy / distance,
        );

    let penetration =
        minimum_distance
            - distance;

    sparkle.position.x +=
        normal.x
            * penetration;

    sparkle.position.y +=
        normal.y
            * penetration;

    let normal_velocity =
        sparkle.velocity.x
            * normal.x
            + sparkle.velocity.y
                * normal.y;

    if normal_velocity
        >= 0.0
    {
        return;
    }

    let restitution =
        0.72;

    let impulse =
        (
            1.0
                + restitution
        )
            * normal_velocity;

    sparkle.velocity.x -=
        impulse
            * normal.x;

    sparkle.velocity.y -=
        impulse
            * normal.y;

    sparkle.velocity.x *=
        0.97;

    sparkle.velocity.y *=
        0.97;

    let tangent_velocity =
        -sparkle.velocity.x
            * normal.y
            + sparkle.velocity.y
                * normal.x;

    sparkle.rotation_speed +=
        tangent_velocity
            * 0.015;
}

fn collect_sparkles(
    sparkles: &mut Vec<Sparkle>,
    mouse: Vector2,
    score: &mut u32,
    starbit_sound: &raylib::audio::Sound<'_>,
) {
    let cursor_radius =
        21.0;

    for sparkle in
        sparkles.iter_mut()
    {
        if !sparkle.active {
            continue;
        }

        let dx =
            sparkle.position.x
                - mouse.x;

        let dy =
            sparkle.position.y
                - mouse.y;

        let total_radius =
            sparkle.radius
                + cursor_radius;

        if dx * dx
            + dy * dy
            <= total_radius
                * total_radius
        {
            sparkle.active =
                false;

            *score +=
                1;

            starbit_sound
                .play();
        }
    }

    sparkles.retain(
        |sparkle| {
            sparkle.active
        },
    );
}

fn draw_sparkles(
    d: &mut RaylibDrawHandle<'_>,
    sparkles: &[Sparkle],
) {
    for sparkle in sparkles {
        if sparkle.active {
            draw_sparkle(
                d,
                sparkle,
            );
        }
    }
}

fn draw_sparkle(
    d: &mut RaylibDrawHandle<'_>,
    sparkle: &Sparkle,
) {
    const POINTS: usize =
        8;

    let outer =
        sparkle.radius;

    let inner =
        sparkle.radius
            * 0.28;

    let mut vertices =
        [Vector2::new(
            0.0,
            0.0,
        ); POINTS];

    for i in 0..POINTS {
        let radius =
            if i % 2 == 0 {
                outer
            } else {
                inner
            };

        let angle =
            sparkle.rotation
                + i as f32
                    * std::f32::consts::PI
                    / 4.0;

        vertices[i] =
            Vector2::new(
                sparkle.position.x
                    + angle.cos()
                        * radius,

                sparkle.position.y
                    + angle.sin()
                        * radius,
            );
    }

    for i in 0..POINTS {
        let next =
            (
                i + 1
            )
                % POINTS;

        d.draw_triangle(
            vertices[next],
            vertices[i],
            sparkle.position,
            sparkle.color,
        );
    }

    d.draw_circle_v(
        sparkle.position,
        sparkle.radius
            * 0.18,
        Color::WHITE,
    );
}

fn draw_star_cursor(
    d: &mut RaylibDrawHandle<'_>,
    mouse: Vector2,
) {
    const POINTS: usize =
        10;

    let outer_radius =
        25.0;

    let inner_radius =
        12.0;

    let rotation =
        -std::f32::consts::PI
            / 2.0
            + 0.15;

    let border =
        Color::new(
            151,
            242,
            248,
            255,
        );

    let fill =
        Color::new(
            13,
            83,
            198,
            255,
        );

    let mut outer =
        [Vector2::new(
            0.0,
            0.0,
        ); POINTS];

    for i in 0..POINTS {
        let radius =
            if i % 2 == 0 {
                outer_radius
            } else {
                inner_radius
            };

        let angle =
            rotation
                + i as f32
                    * std::f32::consts::PI
                    / 5.0;

        outer[i] =
            Vector2::new(
                mouse.x
                    + angle.cos()
                        * radius,

                mouse.y
                    + angle.sin()
                        * radius,
            );
    }

    for i in 0..POINTS {
        let next =
            (
                i + 1
            )
                % POINTS;

        d.draw_triangle(
            outer[next],
            outer[i],
            mouse,
            border,
        );
    }

    let inner_outer_radius =
        19.0;

    let inner_inner_radius =
        8.5;

    let mut inner =
        [Vector2::new(
            0.0,
            0.0,
        ); POINTS];

    for i in 0..POINTS {
        let radius =
            if i % 2 == 0 {
                inner_outer_radius
            } else {
                inner_inner_radius
            };

        let angle =
            rotation
                + i as f32
                    * std::f32::consts::PI
                    / 5.0;

        inner[i] =
            Vector2::new(
                mouse.x
                    + angle.cos()
                        * radius,

                mouse.y
                    + angle.sin()
                        * radius,
            );
    }

    for i in 0..POINTS {
        let next =
            (
                i + 1
            )
                % POINTS;

        d.draw_triangle(
            inner[next],
            inner[i],
            mouse,
            fill,
        );
    }
}

fn add_planet_node_scaled(
    objects: &mut Vec<Object>,
    position: Vec3,
    material: Material,
    scale: f32,
) {
    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                position,
                Vec3::new(
                    0.0,
                    1.0,
                    0.0,
                ),
                0.55
                    * scale,
                0.10
                    * scale,
                material,
            ),
        ),
    );

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                position
                    + Vec3::new(
                        0.0,
                        0.07
                            * scale,
                        0.0,
                    ),
                Vec3::new(
                    0.0,
                    1.0,
                    0.0,
                ),
                0.40
                    * scale,
                0.08
                    * scale,
                material,
            ),
        ),
    );
}

fn add_path_scaled(
    objects: &mut Vec<Object>,
    start: Vec3,
    end: Vec3,
    material: Material,
    scale: f32,
) {
    let start =
        start
            + Vec3::new(
                0.0,
                -0.02
                    * scale,
                0.0,
            );

    let end =
        end
            + Vec3::new(
                0.0,
                -0.02
                    * scale,
                0.0,
            );

    let direction =
        end
            - start;

    let length =
        direction.length();

    if length < 0.001 {
        return;
    }

    let axis =
        direction.normalize();

    let center =
        (
            start
                + end
        )
            * 0.5;

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                center,
                axis,
                0.075
                    * scale,
                length,
                material,
            ),
        ),
    );
}

fn rotate_y(
    point: Vec3,
    angle: f32,
) -> Vec3 {
    let cos_a =
        angle.cos();

    let sin_a =
        angle.sin();

    Vec3::new(
        point.x
            * cos_a
            + point.z
                * sin_a,

        point.y,

        -point.x
            * sin_a
            + point.z
                * cos_a,
    )
}

fn transform_objects_rotated(
    objects: Vec<Object>,
    offset: Vec3,
    scale: f32,
    rotation: f32,
) -> Vec<Object> {
    objects
        .into_iter()
        .map(
            |object| {
                match object {
                    Object::Sphere(
                        sphere,
                    ) => {
                        Object::Sphere(
                            Sphere::new(
                                rotate_y(
                                    sphere.center
                                        * scale,
                                    rotation,
                                )
                                    + offset,

                                sphere.radius
                                    * scale,

                                sphere.material,
                            ),
                        )
                    }

                    Object::Cylinder(
                        cylinder,
                    ) => {
                        Object::Cylinder(
                            Cylinder::new_oriented(
                                rotate_y(
                                    cylinder.center
                                        * scale,
                                    rotation,
                                )
                                    + offset,

                                rotate_y(
                                    cylinder.axis,
                                    rotation,
                                ),

                                cylinder.radius
                                    * scale,

                                cylinder.height
                                    * scale,

                                cylinder.material,
                            ),
                        )
                    }

                    Object::Cone(
                        cone,
                    ) => {
                        Object::Cone(
                            Cone::new_oriented(
                                rotate_y(
                                    cone.center
                                        * scale,
                                    rotation,
                                )
                                    + offset,

                                rotate_y(
                                    cone.axis,
                                    rotation,
                                ),

                                cone.radius
                                    * scale,

                                cone.height
                                    * scale,

                                cone.material,
                            ),
                        )
                    }

                    Object::Plane(
                        plane,
                    ) => {
                        Object::Plane(
                            Plane::new(
                                rotate_y(
                                    plane.point
                                        * scale,
                                    rotation,
                                )
                                    + offset,

                                rotate_y(
                                    plane.normal,
                                    rotation,
                                ),

                                plane.material,
                            ),
                        )
                    }

                    Object::Cube(
                        cube,
                    ) => {
                        Object::Cube(
                            Cube::from_basis(
                                rotate_y(
                                    cube.center
                                        * scale,
                                    rotation,
                                )
                                    + offset,

                                cube.half_size
                                    * 2.0
                                    * scale,

                                rotate_y(
                                    cube.right,
                                    rotation,
                                ),

                                rotate_y(
                                    cube.up,
                                    rotation,
                                ),

                                rotate_y(
                                    cube.forward,
                                    rotation,
                                ),

                                cube.material,
                            ),
                        )
                    }

                    Object::Hemisphere(
                        hemisphere,
                    ) => {
                        Object::Hemisphere(
                            Hemisphere::new_with_materials(
                                rotate_y(
                                    hemisphere.center
                                        * scale,
                                    rotation,
                                )
                                    + offset,

                                hemisphere.radius
                                    * scale,

                                rotate_y(
                                    hemisphere.normal,
                                    rotation,
                                ),

                                hemisphere.material,

                                hemisphere.flat_material,
                            ),
                        )
                    }

                    Object::Torus(
                        torus,
                    ) => {
                        Object::Torus(
                            Torus::new(
                                rotate_y(
                                    torus.center
                                        * scale,
                                    rotation,
                                )
                                    + offset,

                                torus.major_radius
                                    * scale,

                                torus.minor_radius
                                    * scale,

                                torus.material,
                            ),
                        )
                    }

                    Object::Ellipsoid(
                        ellipsoid,
                    ) => {
                        Object::Ellipsoid(
                            Ellipsoid::new(
                                rotate_y(
                                    ellipsoid.center
                                        * scale,
                                    rotation,
                                )
                                    + offset,

                                ellipsoid.radii
                                    * scale,

                                ellipsoid.material,
                            ),
                        )
                    }
                }
            },
        )
        .collect()
}