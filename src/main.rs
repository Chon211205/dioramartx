mod acceleration;
mod core;
mod galaxies;
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
use crate::scene::planet::PlanetDefinition;
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

#[derive(Clone, Copy, PartialEq)]
enum SelectorTransition {
    None,
    Opening,
    Closing,
}

fn main() {
    let (mut rl, thread) = raylib::init()
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

    let mut world_1_music =
        audio
            .new_music(
                "assets/sounds/w1.mp3",
            )
            .expect(
                "No se pudo cargar w1.mp3",
            );

    world_1_music.set_looping(true);
    world_1_music.set_volume(1.0);

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

    let galaxies_planets:
        Vec<Vec<PlanetDefinition>> =
        vec![
            galaxies::galaxy_registry(0),
            galaxies::galaxy_registry(1),
        ];

    let focused_scenes:
        Vec<Vec<Scene>> =
        galaxies_planets
            .iter()
            .map(
                |planets| {
                    planets
                        .iter()
                        .map(
                            |planet| {
                                Scene::new(
                                    (planet.create)(),
                                )
                            },
                        )
                        .collect()
                },
            )
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

    let mut sparkle_spawn_timer:
        f32 =
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

    let mut next_galaxy:
        usize =
        0;

    let mut galaxy_selector_open =
        false;

    let mut galaxy_selector_index:
        usize =
        0;

    let mut galaxy_transition =
        GalaxyTransition::None;

    let mut transition_timer:
        f32 =
        0.0;

    let mut transition_refresh_scene =
        false;

    let mut selector_transition =
        SelectorTransition::None;

    let mut selector_transition_timer:
        f32 =
        0.0;

    world_1_music.play_stream();

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

        if current_galaxy == 0 {
            world_1_music.update_stream();

            if !world_1_music
                .is_stream_playing()
            {
                world_1_music
                    .play_stream();
            }
        } else if world_1_music
            .is_stream_playing()
        {
            world_1_music
                .stop_stream();
        }

        if state == SceneState::Galaxy
            && galaxy_transition
                == GalaxyTransition::None
            && selector_transition
                == SelectorTransition::None
        {
            if rl.is_key_pressed(
                KeyboardKey::KEY_N,
            ) {
                if galaxy_selector_open {
                    selector_transition =
                        SelectorTransition::Closing;

                    selector_transition_timer =
                        0.0;
                } else {
                    galaxy_selector_index =
                        current_galaxy;

                    selector_transition =
                        SelectorTransition::Opening;

                    selector_transition_timer =
                        0.0;
                }
            }
        }

        if selector_transition
            != SelectorTransition::None
        {
            selector_transition_timer +=
                dt;

            match selector_transition {
                SelectorTransition::Opening => {
                    if selector_transition_timer
                        >= 0.65
                    {
                        galaxy_selector_open =
                            true;

                        selector_transition =
                            SelectorTransition::None;

                        selector_transition_timer =
                            0.0;
                    }
                }

                SelectorTransition::Closing => {
                    if selector_transition_timer
                        >= 0.65
                    {
                        galaxy_selector_open =
                            false;

                        selector_transition =
                            SelectorTransition::None;

                        selector_transition_timer =
                            0.0;

                        transition_refresh_scene =
                            true;
                    }
                }

                SelectorTransition::None => {}
            }
        }

        if galaxy_selector_open
            && galaxy_transition
                == GalaxyTransition::None
            && selector_transition
                == SelectorTransition::None
        {
            if rl.is_key_pressed(
                KeyboardKey::KEY_ESCAPE,
            ) {
                selector_transition =
                    SelectorTransition::Closing;

                selector_transition_timer =
                    0.0;
            }

            if rl.is_key_pressed(
                KeyboardKey::KEY_LEFT,
            )
                || rl.is_key_pressed(
                    KeyboardKey::KEY_A,
                )
            {
                if galaxy_selector_index
                    == 0
                {
                    galaxy_selector_index =
                        GALAXY_COUNT - 1;
                } else {
                    galaxy_selector_index -=
                        1;
                }
            }

            if rl.is_key_pressed(
                KeyboardKey::KEY_RIGHT,
            )
                || rl.is_key_pressed(
                    KeyboardKey::KEY_D,
                )
            {
                galaxy_selector_index =
                    (
                        galaxy_selector_index
                            + 1
                    )
                        % GALAXY_COUNT;
            }

            if rl.is_mouse_button_pressed(
                MouseButton::MOUSE_BUTTON_LEFT,
            )
                && point_inside_viewport(
                    mouse_position,
                    &viewport,
                )
            {
                let selector_camera =
                    create_selector_camera();

                let render_mouse =
                    screen_to_render(
                        mouse_position,
                        &viewport,
                    );

                let ray =
                    selector_camera
                        .get_ray(
                            render_mouse.x,
                            render_mouse.y,
                            RENDER_WIDTH
                                as f32,
                            RENDER_HEIGHT
                                as f32,
                        );

                let centers =
                    selector_centers();

                let mut closest =
                    f32::INFINITY;

                let mut clicked:
                    Option<usize> =
                    None;

                for index in
                    0..GALAXY_COUNT
                {
                    let sphere =
                        Sphere::new(
                            centers[index],
                            2.65,
                            hit_material,
                        );

                    if let Some(
                        distance,
                    ) =
                        sphere.intersect(
                            &ray.origin,
                            &ray.direction,
                        )
                    {
                        if distance
                            < closest
                        {
                            closest =
                                distance;

                            clicked =
                                Some(index);
                        }
                    }
                }

                if let Some(index) =
                    clicked
                {
                    galaxy_selector_index =
                        index;
                }
            }

            if rl.is_key_pressed(
                KeyboardKey::KEY_ENTER,
            )
            {
                if galaxy_selector_index
                    == current_galaxy
                {
                    selector_transition =
                        SelectorTransition::Closing;

                    selector_transition_timer =
                        0.0;
                } else {
                    next_galaxy =
                        galaxy_selector_index;

                    galaxy_selector_open =
                        false;

                    galaxy_transition =
                        GalaxyTransition::ZoomOut;

                    transition_timer =
                        0.0;

                    transition_refresh_scene =
                        false;

                    selected_planet =
                        None;

                    sparkles.clear();

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
                            next_galaxy;

                        galaxy_transition =
                            GalaxyTransition::Warp;

                        transition_timer =
                            0.0;

                        transition_refresh_scene =
                            true;

                        selected_planet =
                            None;

                        sparkles.clear();

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

                        transition_refresh_scene =
                            true;
                    }
                }

                GalaxyTransition::None => {}
            }
        }

        if galaxy_transition
            == GalaxyTransition::None
            && !galaxy_selector_open
            && selector_transition
                == SelectorTransition::None
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

                            let planets =
                                &galaxies_planets[
                                    current_galaxy
                                ];

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
                                let hit =
                                    Sphere::new(
                                        planet.position,
                                        planet.hit_radius,
                                        hit_material,
                                    );

                                if let Some(
                                    distance,
                                ) =
                                    hit.intersect(
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

                        transition_refresh_scene =
                            true;

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
                                    delta.x / dt;

                                focused_velocity_y =
                                    delta.y / dt;
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
                                        dt
                                            * 60.0,
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
                camera.zoom(wheel);
            }
        }

        if state
            == SceneState::Galaxy
            && galaxy_transition
                == GalaxyTransition::None
            && !galaxy_selector_open
            && selector_transition
                == SelectorTransition::None
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

            let planets =
                &galaxies_planets[
                    current_galaxy
                ];

            let planet_colliders =
                create_planet_colliders(
                    planets,
                    &camera,
                    &viewport,
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

        if galaxy_selector_open {
            let selector_camera =
                create_selector_camera();

            let selector_scene =
                create_galaxy_selector_scene(
                    galaxy_selector_index,
                    current_galaxy,
                    current_time,
                );

            renderer::raytracer::render_rotated_with_skybox(
                &mut framebuffer,
                &selector_scene,
                &light,
                &selector_camera,
                0.0,
                galaxy_selector_index,
            );
        } else {
            match state {
                SceneState::Galaxy => {
                    let should_render =
                        galaxy_transition
                            == GalaxyTransition::None
                            || transition_refresh_scene;

                    if should_render {
                        let rotation =
                            current_time
                                * 0.25;

                        let planets =
                            &galaxies_planets[
                                current_galaxy
                            ];

                        let mut galaxy_objects =
                            Vec::new();

                        let nodes:
                            Vec<Vec3> =
                            planets
                                .iter()
                                .map(
                                    |planet| {
                                        planet.position
                                            + planet.node_offset
                                    },
                                )
                                .collect();

                        if nodes.len() > 1 {
                            for index in
                                0..nodes.len() - 1
                            {
                                add_path_scaled(
                                    &mut galaxy_objects,
                                    nodes[index],
                                    nodes[index + 1],
                                    path_yellow_material,
                                    1.0,
                                );
                            }
                        }

                        for node in &nodes {
                            add_planet_node_scaled(
                                &mut galaxy_objects,
                                *node,
                                path_yellow_material,
                                1.0,
                            );
                        }

                        for planet in planets {
                            let preview =
                                transform_objects_rotated(
                                    (planet.create)(),
                                    planet.position,
                                    planet.preview_scale,
                                    rotation,
                                );

                            galaxy_objects
                                .extend(preview);
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

                        transition_refresh_scene =
                            false;
                    }
                }

                SceneState::Focused => {
                    if let Some(index) =
                        selected_planet
                    {
                        renderer::raytracer::render_with_skybox(
                            &mut framebuffer,
                            &focused_scenes[
                                current_galaxy
                            ][index],
                            &light,
                            &camera,
                            current_galaxy,
                        );
                    }
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

        let mut transition_scale =
            if state
                == SceneState::Galaxy
                && galaxy_transition
                    != GalaxyTransition::None
            {
                transition_world_scale(
                    galaxy_transition,
                    transition_timer,
                )
            } else {
                1.0
            };

        if selector_transition
            == SelectorTransition::Opening
        {
            let t =
                (
                    selector_transition_timer
                        / 0.65
                )
                    .clamp(
                        0.0,
                        1.0,
                    );

            transition_scale =
                1.0
                    - smoothstep(t)
                        * 0.75;
        }

        if selector_transition
            == SelectorTransition::Closing
            && galaxy_selector_open
        {
            let t =
                (
                    selector_transition_timer
                        / 0.65
                )
                    .clamp(
                        0.0,
                        1.0,
                    );

            transition_scale =
                1.0
                    - smoothstep(t)
                        * 0.40;
        }

        let destination_width =
            viewport.width
                * transition_scale;

        let destination_height =
            viewport.height
                * transition_scale;

        let destination =
            Rectangle::new(
                viewport.x
                    + (
                        viewport.width
                            - destination_width
                    )
                        * 0.5,

                viewport.y
                    + (
                        viewport.height
                            - destination_height
                    )
                        * 0.5,

                destination_width,
                destination_height,
            );

        let mut transition_alpha =
            if state
                == SceneState::Galaxy
            {
                match galaxy_transition {
                    GalaxyTransition::None => {
                        255
                    }

                    GalaxyTransition::ZoomOut => {
                        let t =
                            (
                                transition_timer
                                    / 0.85
                            )
                                .clamp(
                                    0.0,
                                    1.0,
                                );

                        (
                            255.0
                                * (
                                    1.0
                                        - smoothstep(
                                            t,
                                        )
                                )
                        )
                            as u8
                    }

                    GalaxyTransition::Warp => {
                        0
                    }

                    GalaxyTransition::ZoomIn => {
                        let t =
                            (
                                transition_timer
                                    / 1.0
                            )
                                .clamp(
                                    0.0,
                                    1.0,
                                );

                        (
                            255.0
                                * smoothstep(
                                    t,
                                )
                        )
                            as u8
                    }
                }
            } else {
                255
            };

        match selector_transition {
            SelectorTransition::Opening => {
                let t =
                    (
                        selector_transition_timer
                            / 0.65
                    )
                        .clamp(
                            0.0,
                            1.0,
                        );

                transition_alpha =
                    (
                        255.0
                            * (
                                1.0
                                    - smoothstep(
                                        t,
                                    )
                            )
                    )
                        as u8;
            }

            SelectorTransition::Closing => {
                if galaxy_selector_open {
                    let t =
                        (
                            selector_transition_timer
                                / 0.65
                        )
                            .clamp(
                                0.0,
                                1.0,
                            );

                    transition_alpha =
                        (
                            255.0
                                * (
                                    1.0
                                        - smoothstep(
                                            t,
                                        )
                                )
                        )
                            as u8;
                }
            }

            SelectorTransition::None => {}
        }

        d.draw_texture_pro(
            &render_texture,
            source,
            destination,
            Vector2::new(
                0.0,
                0.0,
            ),
            0.0,
            Color::new(
                255,
                255,
                255,
                transition_alpha,
            ),
        );

        if galaxy_selector_open
            && selector_transition
                == SelectorTransition::None
        {
            draw_selector_interface(
                &mut d,
                &viewport,
                current_screen_width,
                current_screen_height,
                galaxy_selector_index,
                current_galaxy,
            );
        } else if galaxy_transition
            == GalaxyTransition::None
            && selector_transition
                == SelectorTransition::None
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
                        "N - Selector de galaxias",
                        30,
                        160,
                        20,
                        Color::LIGHTGRAY,
                    );

                    draw_sparkle_counter(
                        &mut d,
                        sparkle_score,
                    );
                }

                SceneState::Focused => {
                    if let Some(index) =
                        selected_planet
                    {
                        let planets =
                            &galaxies_planets[
                                current_galaxy
                            ];

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

        if selector_transition
            == SelectorTransition::Opening
        {
            let total =
                1.30;

            let t =
                (
                    selector_transition_timer
                        / total
                )
                    .clamp(
                        0.0,
                        1.0,
                    );

            draw_warp(
                &mut d,
                current_screen_width,
                current_screen_height,
                t,
            );

            let flash =
                1.0
                    - (
                        t * 2.0
                            - 1.0
                    )
                        .abs();

            d.draw_rectangle(
                0,
                0,
                current_screen_width,
                current_screen_height,
                Color::new(
                    255,
                    255,
                    255,
                    (
                        flash
                            .clamp(
                                0.0,
                                1.0,
                            )
                            * 170.0
                    )
                        as u8,
                ),
            );

            let darkness =
                if selector_transition_timer
                    < 0.65
                {
                    let phase =
                        (
                            selector_transition_timer
                                / 0.65
                        )
                            .clamp(
                                0.0,
                                1.0,
                            );

                    smoothstep(
                        phase,
                    )
                } else {
                    let phase =
                        (
                            (
                                selector_transition_timer
                                    - 0.65
                            )
                                / 0.65
                        )
                            .clamp(
                                0.0,
                                1.0,
                            );

                    1.0
                        - smoothstep(
                            phase,
                        )
                };

            d.draw_rectangle(
                0,
                0,
                current_screen_width,
                current_screen_height,
                Color::new(
                    0,
                    0,
                    15,
                    (
                        darkness
                            * 110.0
                    )
                        as u8,
                ),
            );
        }

        if selector_transition
            == SelectorTransition::Closing
        {
            let t =
                (
                    selector_transition_timer
                        / 0.65
                )
                    .clamp(
                        0.0,
                        1.0,
                    );

            d.draw_rectangle(
                0,
                0,
                current_screen_width,
                current_screen_height,
                Color::new(
                    0,
                    0,
                    15,
                    (
                        smoothstep(t)
                            * 180.0
                    )
                        as u8,
                ),
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

fn create_selector_camera() -> Camera {
    Camera::new(
        Vec3::new(
            0.0,
            0.0,
            0.0,
        ),
        11.5,
        52.0,
    )
}

fn selector_centers()
    -> [Vec3; GALAXY_COUNT]
{
    [
        Vec3::new(
            -3.45,
            0.0,
            0.0,
        ),

        Vec3::new(
            3.45,
            0.0,
            0.0,
        ),
    ]
}

fn create_galaxy_selector_scene(
    selected: usize,
    current: usize,
    time: f32,
) -> Scene {
    let mut objects =
        Vec::new();

    let centers =
        selector_centers();

    let blue_core =
        Material::new(
            Vec3::new(
                0.18,
                0.65,
                1.0,
            ),
            0.90,
            0.75,
            0.0,
            0.15,
        );

    let blue_inner =
        Material::new(
            Vec3::new(
                0.15,
                0.95,
                0.72,
            ),
            0.95,
            0.80,
            0.0,
            0.18,
        );

    let orange_core =
        Material::new(
            Vec3::new(
                1.0,
                0.35,
                0.08,
            ),
            0.95,
            0.95,
            0.0,
            0.22,
        );

    let orange_inner =
        Material::new(
            Vec3::new(
                1.0,
                0.82,
                0.16,
            ),
            0.95,
            1.0,
            0.0,
            0.20,
        );

    let green_planet =
        Material::new(
            Vec3::new(
                0.12,
                0.65,
                0.22,
            ),
            0.90,
            0.35,
            0.0,
            0.03,
        );

    let dark_planet =
        Material::new(
            Vec3::new(
                0.10,
                0.06,
                0.08,
            ),
            0.85,
            0.30,
            0.0,
            0.05,
        );

    let yellow =
        Material::new(
            Vec3::new(
                1.0,
                0.80,
                0.05,
            ),
            0.98,
            1.0,
            0.0,
            0.25,
        );

    let selected_material =
        Material::new(
            Vec3::new(
                1.0,
                0.95,
                0.20,
            ),
            1.0,
            1.0,
            0.0,
            0.35,
        );

    let current_material =
        Material::new(
            Vec3::new(
                0.20,
                1.0,
                0.55,
            ),
            0.95,
            0.90,
            0.0,
            0.20,
        );

    add_selector_nebula(
        &mut objects,
        centers[0],
        0,
        selected == 0,
        time,
    );

    add_selector_nebula(
        &mut objects,
        centers[1],
        1,
        selected == 1,
        time,
    );

    add_selector_galaxy(
        &mut objects,
        centers[0],
        0,
        selected,
        current,
        time,
        blue_core,
        blue_inner,
        green_planet,
        dark_planet,
        yellow,
        selected_material,
        current_material,
    );

    add_selector_galaxy(
        &mut objects,
        centers[1],
        1,
        selected,
        current,
        time,
        orange_core,
        orange_inner,
        green_planet,
        dark_planet,
        yellow,
        selected_material,
        current_material,
    );

    let bridge_start =
        centers[0]
            + Vec3::new(
                1.30,
                0.55,
                0.0,
            );

    let bridge_end =
        centers[1]
            + Vec3::new(
                -1.30,
                0.55,
                0.0,
            );

    add_selector_bridge(
        &mut objects,
        bridge_start,
        bridge_end,
        yellow,
        time,
    );

    Scene::new(objects)
}

#[allow(clippy::too_many_arguments)]
fn add_selector_galaxy(
    objects: &mut Vec<Object>,
    center: Vec3,
    galaxy_index: usize,
    selected: usize,
    current: usize,
    time: f32,
    core_material: Material,
    inner_material: Material,
    green_material: Material,
    dark_material: Material,
    yellow_material: Material,
    selected_material: Material,
    current_material: Material,
) {
    let pulse_strength =
        if selected == galaxy_index {
            0.08
        } else {
            0.025
        };

    let pulse =
        1.0
            + (
                time
                    * 2.0
                    + galaxy_index
                        as f32
            )
                .sin()
                * pulse_strength;

    objects.push(
        Object::Sphere(
            Sphere::new(
                center,
                if selected
                    == galaxy_index
                {
                    0.86 * pulse
                } else {
                    0.70 * pulse
                },
                core_material,
            ),
        ),
    );

    objects.push(
        Object::Sphere(
            Sphere::new(
                center
                    + Vec3::new(
                        0.0,
                        0.0,
                        -0.12,
                    ),
                0.52,
                inner_material,
            ),
        ),
    );

    add_selector_orbit(
        objects,
        center,
        1.18,
        22,
        yellow_material,
    );

    add_selector_orbit(
        objects,
        center,
        1.65,
        28,
        yellow_material,
    );

    add_selector_orbit(
        objects,
        center,
        2.12,
        34,
        yellow_material,
    );

    let planet_count = 7;

    for i in 0..planet_count {
        let ring =
            match i % 3 {
                0 => 1.18,
                1 => 1.65,
                _ => 2.12,
            };

        let speed =
            if galaxy_index == 0 {
                0.25
                    + i as f32
                        * 0.018
            } else {
                -0.30
                    - i as f32
                        * 0.015
            };

        let angle =
            i as f32
                / planet_count as f32
                * std::f32::consts::PI
                * 2.0
                + time * speed;

        let squash =
            if galaxy_index == 0 {
                0.55
            } else {
                0.48
            };

        let position =
            center
                + Vec3::new(
                    angle.cos()
                        * ring,
                    angle.sin()
                        * ring
                        * squash,
                    (
                        angle
                            * 1.7
                    )
                        .sin()
                        * 0.20,
                );

        let radius =
            match i % 4 {
                0 => 0.17,
                1 => 0.22,
                2 => 0.14,
                _ => 0.19,
            };

        let material =
            if galaxy_index == 0 {
                if i % 2 == 0 {
                    green_material
                } else {
                    dark_material
                }
            } else if i % 3 == 0 {
                yellow_material
            } else {
                dark_material
            };

        objects.push(
            Object::Sphere(
                Sphere::new(
                    position,
                    radius,
                    material,
                ),
            ),
        );
    }

    if selected == galaxy_index {
        add_selector_orbit(
            objects,
            center,
            2.55,
            42,
            selected_material,
        );

        objects.push(
            Object::Sphere(
                Sphere::new(
                    center
                        + Vec3::new(
                            0.0,
                            -2.62,
                            0.0,
                        ),
                    0.13,
                    selected_material,
                ),
            ),
        );
    }

    if current == galaxy_index {
        objects.push(
            Object::Sphere(
                Sphere::new(
                    center
                        + Vec3::new(
                            0.0,
                            2.58,
                            0.0,
                        ),
                    0.12,
                    current_material,
                ),
            ),
        );
    }
}

fn add_selector_nebula(
    objects: &mut Vec<Object>,
    center: Vec3,
    galaxy_index: usize,
    is_selected: bool,
    time: f32,
) {
    let (
        color_a,
        color_b,
        color_c,
    ) =
        if galaxy_index == 0 {
            (
                Vec3::new(
                    0.05,
                    0.32,
                    0.65,
                ),
                Vec3::new(
                    0.05,
                    0.75,
                    0.85,
                ),
                Vec3::new(
                    0.08,
                    0.55,
                    0.28,
                ),
            )
        } else {
            (
                Vec3::new(
                    0.35,
                    0.05,
                    0.55,
                ),
                Vec3::new(
                    0.85,
                    0.08,
                    0.45,
                ),
                Vec3::new(
                    1.0,
                    0.28,
                    0.05,
                ),
            )
        };

    let brightness =
        if is_selected {
            1.0
        } else {
            0.32
        };

    let specular =
        if is_selected {
            0.80
        } else {
            0.18
        };

    let material_a =
        Material::new(
            color_a * brightness,
            0.85,
            specular,
            0.0,
            0.0,
        );

    let material_b =
        Material::new(
            color_b * brightness,
            0.90,
            specular,
            0.0,
            0.0,
        );

    let material_c =
        Material::new(
            color_c * brightness,
            0.95,
            specular,
            0.0,
            0.0,
        );

    let direction =
        if galaxy_index == 0 {
            1.0
        } else {
            -1.0
        };

    let rotation_speed =
        if is_selected {
            0.10
        } else {
            0.035
        };

    let scale =
        if is_selected {
            1.06
        } else {
            0.94
        };

    for arm in 0..3 {
        let arm_offset =
            arm as f32
                / 3.0
                * std::f32::consts::PI
                * 2.0;

        for i in 0..42 {
            let t =
                i as f32 / 42.0;

            let radius =
                (
                    0.65
                        + t * 2.45
                )
                    * scale;

            let spiral =
                arm_offset
                    + direction
                        * (
                            t * 5.8
                                + time
                                    * rotation_speed
                        );

            let wobble =
                (
                    i as f32
                        * 1.73
                        + arm as f32
                            * 2.9
                )
                    .sin()
                    * 0.16;

            let position =
                center
                    + Vec3::new(
                        spiral.cos()
                            * radius
                            + (
                                i as f32
                                    * 5.31
                                    + arm as f32
                            )
                                .sin()
                                * 0.10,

                        spiral.sin()
                            * radius
                            * 0.42
                            + (
                                i as f32
                                    * 3.77
                                    + arm as f32
                                        * 1.9
                            )
                                .cos()
                                * 0.08,

                        -0.70
                            - t * 0.20
                            + wobble
                                * 0.10,
                    );

            let base_radius =
                0.045
                    + (
                        i as f32
                            * 0.91
                    )
                        .sin()
                        .abs()
                        * 0.045;

            let particle_radius =
                if is_selected {
                    base_radius * 1.18
                } else {
                    base_radius * 0.75
                };

            let material =
                match i % 3 {
                    0 => material_a,
                    1 => material_b,
                    _ => material_c,
                };

            objects.push(
                Object::Sphere(
                    Sphere::new(
                        position,
                        particle_radius,
                        material,
                    ),
                ),
            );
        }
    }

    let core_particles =
        if is_selected {
            34
        } else {
            20
        };

    for i in 0..core_particles {
        let angle =
            i as f32
                / core_particles as f32
                * std::f32::consts::PI
                * 2.0
                + time
                    * rotation_speed
                    * direction;

        let radius =
            (
                0.25
                    + (
                        i as f32
                            * 1.91
                    )
                        .sin()
                        .abs()
                        * 0.55
            )
                * scale;

        let position =
            center
                + Vec3::new(
                    angle.cos()
                        * radius,
                    angle.sin()
                        * radius
                        * 0.50,
                    -0.82,
                );

        objects.push(
            Object::Sphere(
                Sphere::new(
                    position,
                    if is_selected {
                        0.075
                    } else {
                        0.045
                    },
                    material_b,
                ),
            ),
        );
    }
}

fn add_selector_orbit(
    objects: &mut Vec<Object>,
    center: Vec3,
    radius: f32,
    segments: usize,
    material: Material,
) {
    for i in 0..segments {
        let angle =
            i as f32
                / segments as f32
                * std::f32::consts::PI
                * 2.0;

        let position =
            center
                + Vec3::new(
                    angle.cos()
                        * radius,
                    angle.sin()
                        * radius
                        * 0.48,
                    0.16,
                );

        objects.push(
            Object::Sphere(
                Sphere::new(
                    position,
                    0.035,
                    material,
                ),
            ),
        );
    }
}

fn add_selector_bridge(
    objects: &mut Vec<Object>,
    start: Vec3,
    end: Vec3,
    material: Material,
    time: f32,
) {
    let direction =
        end - start;

    let length =
        direction.length();

    if length > 0.001 {
        objects.push(
            Object::Cylinder(
                Cylinder::new_oriented(
                    (
                        start + end
                    )
                        * 0.5,
                    direction.normalize(),
                    0.035,
                    length,
                    material,
                ),
            ),
        );
    }

    let t =
        (time * 0.42).fract();

    let point =
        start
            + (end - start) * t;

    objects.push(
        Object::Sphere(
            Sphere::new(
                point,
                0.12,
                material,
            ),
        ),
    );
}

fn draw_selector_interface(
    d: &mut RaylibDrawHandle<'_>,
    viewport: &Viewport,
    screen_width: i32,
    screen_height: i32,
    selected: usize,
    current: usize,
) {
    let selector_camera =
        create_selector_camera();

    let centers =
        selector_centers();

    let left =
        world_to_screen(
            centers[0],
            &selector_camera,
            viewport,
        );

    let right =
        world_to_screen(
            centers[1],
            &selector_camera,
            viewport,
        );

    let title =
        "SELECCIONA UNA GALAXIA";

    let title_size = 34;

    let title_width =
        d.measure_text(
            title,
            title_size,
        );

    d.draw_text(
        title,
        screen_width
            / 2
            - title_width
                / 2,
        35,
        title_size,
        Color::WHITE,
    );

    if let Some(point) = left {
        draw_selector_label(
            d,
            point,
            "GALAXIA 1",
            selected == 0,
            current == 0,
        );
    }

    if let Some(point) = right {
        draw_selector_label(
            d,
            point,
            "GALAXIA 2",
            selected == 1,
            current == 1,
        );
    }

    let selected_text =
        match selected {
            0 => {
                "Galaxia 1 seleccionada"
            }

            1 => {
                "Galaxia 2 seleccionada"
            }

            _ => "",
        };

    let size = 23;

    let width =
        d.measure_text(
            selected_text,
            size,
        );

    d.draw_text(
        selected_text,
        screen_width
            / 2
            - width
                / 2,
        screen_height
            - 105,
        size,
        Color::new(
            255,
            225,
            80,
            255,
        ),
    );

    let controls =
        "A / D o Flechas - Seleccionar    ENTER - Viajar    N / ESC - Cerrar";

    let size = 18;

    let width =
        d.measure_text(
            controls,
            size,
        );

    d.draw_text(
        controls,
        screen_width
            / 2
            - width
                / 2,
        screen_height
            - 65,
        size,
        Color::LIGHTGRAY,
    );
}

fn draw_selector_label(
    d: &mut RaylibDrawHandle<'_>,
    center: Vector2,
    name: &str,
    selected: bool,
    current: bool,
) {
    let name_size =
        if selected {
            28
        } else {
            23
        };

    let width =
        d.measure_text(
            name,
            name_size,
        );

    d.draw_text(
        name,
        center.x
            as i32
            - width
                / 2,
        center.y
            as i32
            + 155,
        name_size,
        if selected {
            Color::new(
                255,
                225,
                70,
                255,
            )
        } else {
            Color::WHITE
        },
    );

    if current {
        let text =
            "ACTUAL";

        let size = 17;

        let width =
            d.measure_text(
                text,
                size,
            );

        d.draw_text(
            text,
            center.x
                as i32
                - width
                    / 2,
            center.y
                as i32
                + 190,
            size,
            Color::new(
                80,
                245,
                135,
                255,
            ),
        );
    } else if selected {
        let text =
            "ENTER PARA VIAJAR";

        let size = 16;

        let width =
            d.measure_text(
                text,
                size,
            );

        d.draw_text(
            text,
            center.x
                as i32
                - width
                    / 2,
            center.y
                as i32
                + 190,
            size,
            Color::new(
                255,
                225,
                70,
                255,
            ),
        );
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
                    timer / 0.85
                )
                    .clamp(
                        0.0,
                        1.0,
                    );

            1.0
                - smoothstep(t)
                    * 0.82
        }

        GalaxyTransition::Warp => {
            0.18
        }

        GalaxyTransition::ZoomIn => {
            let t =
                (
                    timer / 1.0
                )
                    .clamp(
                        0.0,
                        1.0,
                    );

            0.18
                + smoothstep(t)
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
                    timer / 0.85
                )
                    .clamp(
                        0.0,
                        1.0,
                    );

            d.draw_rectangle(
                0,
                0,
                width,
                height,
                Color::new(
                    0,
                    0,
                    20,
                    (
                        t * 120.0
                    )
                        as u8,
                ),
            );
        }

        GalaxyTransition::Warp => {
            let t =
                (
                    timer / 0.85
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

            let flash =
                1.0
                    - (
                        t * 2.0
                            - 1.0
                    )
                        .abs();

            d.draw_rectangle(
                0,
                0,
                width,
                height,
                Color::new(
                    255,
                    255,
                    255,
                    (
                        flash
                            .clamp(
                                0.0,
                                1.0,
                            )
                            * 220.0
                    )
                        as u8,
                ),
            );
        }

        GalaxyTransition::ZoomIn => {
            let t =
                (
                    timer / 1.0
                )
                    .clamp(
                        0.0,
                        1.0,
                    );

            d.draw_rectangle(
                0,
                0,
                width,
                height,
                Color::new(
                    0,
                    0,
                    20,
                    (
                        (
                            1.0
                                - t
                        )
                            * 120.0
                    )
                        as u8,
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
            width as f32 * 0.5,
            height as f32 * 0.5,
        );

    let max_distance =
        width.max(height)
            as f32
            * 0.75;

    for i in 0..100 {
        let seed =
            i as f32
                * 19.731
                + 3.17;

        let angle =
            pseudo_random(seed)
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
                progress * speed
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

        let value =
            pseudo_random(
                seed * 9.7,
            );

        let color =
            if value < 0.25 {
                Color::new(
                    120,
                    190,
                    255,
                    230,
                )
            } else if value < 0.50 {
                Color::new(
                    210,
                    150,
                    255,
                    230,
                )
            } else if value < 0.75 {
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
}

fn calculate_viewport(
    screen_width: f32,
    screen_height: f32,
) -> Viewport {
    let render_aspect =
        RENDER_WIDTH
            as f32
            / RENDER_HEIGHT
                as f32;

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

    if right.length() < 0.001 {
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
                fov * 0.5
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
    planets: &[PlanetDefinition],
    camera: &Camera,
    viewport: &Viewport,
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

    if camera_right.length() < 0.001 {
        camera_right =
            Vec3::new(
                1.0,
                0.0,
                0.0,
            );
    }

    camera_right =
        camera_right.normalize();

    for planet in planets {
        let center =
            match world_to_screen(
                planet.position,
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
            planet.position
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
            seed * 12.9898
        )
            .sin()
            * 43758.5453;

    value - value.floor()
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
    if sparkles.len() >= 30 {
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
                12.0
                    + size_random
                        * 8.0,

            active: true,

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
    let gravity = 360.0;

    for sparkle in
        sparkles.iter_mut()
    {
        if !sparkle.active {
            continue;
        }

        sparkle.velocity.y +=
            gravity * dt;

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

        if sparkle.position.x < left {
            sparkle.position.x =
                left;

            sparkle.velocity.x =
                sparkle.velocity
                    .x
                    .abs()
                    * 0.72;
        }

        if sparkle.position.x > right {
            sparkle.position.x =
                right;

            sparkle.velocity.x =
                -sparkle.velocity
                    .x
                    .abs()
                    * 0.72;
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
            .max(0.001);

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

    if normal_velocity >= 0.0 {
        return;
    }

    let restitution = 0.72;

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
}

fn collect_sparkles(
    sparkles: &mut Vec<Sparkle>,
    mouse: Vector2,
    score: &mut u32,
    starbit_sound:
        &raylib::audio::Sound<'_>,
) {
    let cursor_radius = 21.0;

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

            *score += 1;

            starbit_sound.play();
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
    const POINTS: usize = 8;

    let outer =
        sparkle.radius;

    let inner =
        sparkle.radius
            * 0.28;

    let mut vertices =
        [
            Vector2::new(
                0.0,
                0.0,
            );
            POINTS
        ];

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
            (i + 1) % POINTS;

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

fn draw_sparkle_counter(
    d: &mut RaylibDrawHandle<'_>,
    sparkle_score: u32,
) {
    let panel_x = 28.0;
    let panel_y = 190.0;
    let panel_width = 112.0;
    let panel_height = 48.0;

    d.draw_rectangle_rounded(
        Rectangle::new(
            panel_x,
            panel_y,
            panel_width,
            panel_height,
        ),
        0.40,
        10,
        Color::new(
            20,
            25,
            35,
            220,
        ),
    );

    d.draw_rectangle_rounded_lines(
        Rectangle::new(
            panel_x,
            panel_y,
            panel_width,
            panel_height,
        ),
        0.40,
        10,
        Color::new(
            210,
            235,
            255,
            240,
        ),
    );

    let center =
        Vector2::new(
            panel_x + 25.0,
            panel_y + 24.0,
        );

    let rotation =
        -std::f32::consts::PI
            / 2.0;

    let outer_radius =
        16.0;

    let inner_radius =
        7.5;

    let mut points =
        [
            Vector2::new(
                0.0,
                0.0,
            );
            10
        ];

    for i in 0..10 {
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

        points[i] =
            Vector2::new(
                center.x
                    + angle.cos()
                        * radius,
                center.y
                    + angle.sin()
                        * radius,
            );
    }

    for i in 1..9 {
        d.draw_triangle(
            points[0],
            points[i],
            points[i + 1],
            Color::new(
                160,
                245,
                255,
                255,
            ),
        );
    }

    let inner_outer_radius =
        11.0;

    let inner_inner_radius =
        5.0;

    let mut inner_points =
        [
            Vector2::new(
                0.0,
                0.0,
            );
            10
        ];

    for i in 0..10 {
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

        inner_points[i] =
            Vector2::new(
                center.x
                    + angle.cos()
                        * radius,
                center.y
                    + angle.sin()
                        * radius,
            );
    }

    for i in 1..9 {
        d.draw_triangle(
            inner_points[0],
            inner_points[i],
            inner_points[i + 1],
            Color::new(
                190,
                40,
                220,
                255,
            ),
        );
    }

    d.draw_circle(
        center.x as i32 - 4,
        center.y as i32 - 3,
        2.5,
        Color::WHITE,
    );

    let text =
        format!(
            "x{}",
            sparkle_score,
        );

    d.draw_text(
        &text,
        (
            panel_x
                + 55.0
                + 2.0
        )
            as i32,
        (
            panel_y
                + 10.0
                + 2.0
        )
            as i32,
        27,
        Color::new(
            0,
            0,
            0,
            180,
        ),
    );

    d.draw_text(
        &text,
        (
            panel_x
                + 55.0
        )
            as i32,
        (
            panel_y
                + 10.0
        )
            as i32,
        27,
        Color::WHITE,
    );
}

fn draw_star_cursor(
    d: &mut RaylibDrawHandle<'_>,
    mouse: Vector2,
) {
    const POINTS: usize = 10;

    let outer_radius = 25.0;
    let inner_radius = 12.0;

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
        [
            Vector2::new(
                0.0,
                0.0,
            );
            POINTS
        ];

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
            (i + 1) % POINTS;

        d.draw_triangle(
            outer[next],
            outer[i],
            mouse,
            border,
        );
    }

    let inner_outer_radius = 19.0;
    let inner_inner_radius = 8.5;

    let mut inner =
        [
            Vector2::new(
                0.0,
                0.0,
            );
            POINTS
        ];

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
            (i + 1) % POINTS;

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
                0.55 * scale,
                0.10 * scale,
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
                        0.07 * scale,
                        0.0,
                    ),
                Vec3::new(
                    0.0,
                    1.0,
                    0.0,
                ),
                0.40 * scale,
                0.08 * scale,
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
                -0.02 * scale,
                0.0,
            );

    let end =
        end
            + Vec3::new(
                0.0,
                -0.02 * scale,
                0.0,
            );

    let direction =
        end - start;

    let length =
        direction.length();

    if length < 0.001 {
        return;
    }

    let axis =
        direction.normalize();

    let center =
        (
            start + end
        )
            * 0.5;

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                center,
                axis,
                0.075 * scale,
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
                            Cube::from_basis_faces(
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

                                cube.right_material,
                                cube.left_material,
                                cube.top_material,
                                cube.bottom_material,
                                cube.front_material,
                                cube.back_material,
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