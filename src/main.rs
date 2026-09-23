mod core;
mod materials;
mod objects;
mod renderer;
mod scene;

use raylib::prelude::*;

use core::camera::Camera;
use core::framebuffer::Framebuffer;
use core::vec3::Vec3;

use materials::material::Material;

use objects::object::Object;
use objects::sphere::Sphere;

use scene::light::Light;
use scene::state::{
    PlanetType,
    SceneState,
};

fn main() {
    const WIDTH: i32 = 800;
    const HEIGHT: i32 = 600;

    let (mut rl, thread) =
        raylib::init()
            .size(WIDTH, HEIGHT)
            .title("Galaxy Diorama")
            .build();

    rl.set_target_fps(60);

    let mut framebuffer =
        Framebuffer::new(
            WIDTH,
            HEIGHT,
        );

    let forest_material =
        Material::new(
            Vec3::new(
                0.15,
                0.75,
                0.25,
            ),
            0.8,
            0.4,
            0.0,
            0.05,
        );

    let volcanic_material =
        Material::new(
            Vec3::new(
                0.9,
                0.2,
                0.05,
            ),
            0.75,
            0.5,
            0.0,
            0.1,
        );

    let crystal_material =
        Material::new(
            Vec3::new(
                0.25,
                0.65,
                1.0,
            ),
            0.6,
            0.9,
            0.25,
            0.3,
        );

    let forest_position =
        Vec3::new(
            -2.7,
            0.8,
            0.0,
        );

    let volcanic_position =
        Vec3::new(
            0.0,
            -0.8,
            -0.6,
        );

    let crystal_position =
        Vec3::new(
            2.7,
            0.9,
            0.2,
        );

    let galaxy_objects =
        vec![
            Object::Sphere(
                Sphere::new(
                    forest_position,
                    1.0,
                    forest_material,
                ),
            ),

            Object::Sphere(
                Sphere::new(
                    volcanic_position,
                    1.15,
                    volcanic_material,
                ),
            ),

            Object::Sphere(
                Sphere::new(
                    crystal_position,
                    1.0,
                    crystal_material,
                ),
            ),
        ];

    let light =
        Light::new(
            Vec3::new(
                -3.0,
                5.0,
                5.0,
            ),
            Vec3::new(
                1.0,
                1.0,
                1.0,
            ),
            1.0,
        );

    let mut camera =
        Camera::new(
            Vec3::new(
                0.0,
                0.0,
                0.0,
            ),
            8.0,
            60.0,
        );

    let mut state =
        SceneState::Galaxy;

    let mut selected_planet:
        Option<PlanetType> =
        None;

    while !rl.window_should_close() {
        match state {
            SceneState::Galaxy => {
                if rl.is_mouse_button_pressed(
                    MouseButton::MOUSE_BUTTON_LEFT,
                ) {
                    let mouse =
                        rl.get_mouse_position();

                    let ray =
                        camera.get_ray(
                            mouse.x,
                            mouse.y,
                            WIDTH as f32,
                            HEIGHT as f32,
                        );

                    let mut closest =
                        f32::INFINITY;

                    let mut selected_index:
                        Option<usize> = None;

                    for (
                        index,
                        object,
                    ) in galaxy_objects
                        .iter()
                        .enumerate()
                    {
                        if let Some(distance) =
                            object.intersect(
                                &ray.origin,
                                &ray.direction,
                            )
                        {
                            if distance < closest {
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
                        match index {
                            0 => {
                                selected_planet =
                                    Some(
                                        PlanetType::Forest,
                                    );

                                camera.focus_on(
                                    forest_position,
                                    3.2,
                                );
                            }

                            1 => {
                                selected_planet =
                                    Some(
                                        PlanetType::Volcanic,
                                    );

                                camera.focus_on(
                                    volcanic_position,
                                    3.5,
                                );
                            }

                            2 => {
                                selected_planet =
                                    Some(
                                        PlanetType::Crystal,
                                    );

                                camera.focus_on(
                                    crystal_position,
                                    3.2,
                                );
                            }

                            _ => {}
                        }

                        state =
                            SceneState::Focused;
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
                    state =
                        SceneState::Galaxy;

                    selected_planet =
                        None;

                    camera =
                        Camera::new(
                            Vec3::new(
                                0.0,
                                0.0,
                                0.0,
                            ),
                            8.0,
                            60.0,
                        );
                }

                if rl.is_mouse_button_pressed(
                    MouseButton::MOUSE_BUTTON_LEFT,
                ) {
                    let mouse =
                        rl.get_mouse_position();

                    let ray =
                        camera.get_ray(
                            mouse.x,
                            mouse.y,
                            WIDTH as f32,
                            HEIGHT as f32,
                        );

                    if let Some(index) =
                        selected_planet_index(
                            selected_planet,
                        )
                    {
                        if galaxy_objects[index]
                            .intersect(
                                &ray.origin,
                                &ray.direction,
                            )
                            .is_some()
                        {
                            state =
                                SceneState::Diorama;

                            camera =
                                Camera::new(
                                    Vec3::new(
                                        0.0,
                                        0.0,
                                        0.0,
                                    ),
                                    5.0,
                                    60.0,
                                );
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

            SceneState::Diorama => {
                if rl.is_key_pressed(
                    KeyboardKey::KEY_BACKSPACE,
                ) {
                    state =
                        SceneState::Focused;

                    match selected_planet {
                        Some(
                            PlanetType::Forest,
                        ) => {
                            camera.focus_on(
                                forest_position,
                                3.2,
                            );
                        }

                        Some(
                            PlanetType::Volcanic,
                        ) => {
                            camera.focus_on(
                                volcanic_position,
                                3.5,
                            );
                        }

                        Some(
                            PlanetType::Crystal,
                        ) => {
                            camera.focus_on(
                                crystal_position,
                                3.2,
                            );
                        }

                        None => {}
                    }
                }

                if rl.is_mouse_button_down(
                    MouseButton::MOUSE_BUTTON_LEFT,
                ) {
                    let delta =
                        rl.get_mouse_delta();

                    camera.rotate(
                        delta.x,
                        delta.y,
                    );
                }
            }
        }

        let wheel =
            rl.get_mouse_wheel_move();

        if wheel != 0.0 {
            camera.zoom(wheel);
        }

        match state {
            SceneState::Galaxy
            | SceneState::Focused => {
                renderer::raytracer::render(
                    &mut framebuffer,
                    &galaxy_objects,
                    &light,
                    &camera,
                );
            }

            SceneState::Diorama => {
                let diorama_objects =
                    create_test_diorama(
                        selected_planet,
                        forest_material,
                        volcanic_material,
                        crystal_material,
                    );

                renderer::raytracer::render(
                    &mut framebuffer,
                    &diorama_objects,
                    &light,
                    &camera,
                );
            }
        }

        let mut d =
            rl.begin_drawing(
                &thread,
            );

        d.clear_background(
            Color::BLACK,
        );

        framebuffer.draw(
            &mut d,
        );

        match state {
            SceneState::Galaxy => {
                d.draw_text(
                    "Selecciona un planeta",
                    20,
                    20,
                    24,
                    Color::WHITE,
                );
            }

            SceneState::Focused => {
                d.draw_text(
                    planet_name(
                        selected_planet,
                    ),
                    20,
                    20,
                    28,
                    Color::WHITE,
                );

                d.draw_text(
                    "Click en el planeta para ingresar",
                    20,
                    55,
                    20,
                    Color::WHITE,
                );

                d.draw_text(
                    "BACKSPACE - Regresar",
                    20,
                    85,
                    18,
                    Color::LIGHTGRAY,
                );
            }

            SceneState::Diorama => {
                d.draw_text(
                    planet_name(
                        selected_planet,
                    ),
                    20,
                    20,
                    28,
                    Color::WHITE,
                );

                d.draw_text(
                    "BACKSPACE - Regresar",
                    20,
                    55,
                    18,
                    Color::LIGHTGRAY,
                );
            }
        }
    }
}

fn selected_planet_index(
    planet: Option<PlanetType>,
) -> Option<usize> {
    match planet {
        Some(PlanetType::Forest) =>
            Some(0),

        Some(PlanetType::Volcanic) =>
            Some(1),

        Some(PlanetType::Crystal) =>
            Some(2),

        None =>
            None,
    }
}

fn planet_name(
    planet: Option<PlanetType>,
) -> &'static str {
    match planet {
        Some(PlanetType::Forest) =>
            "Forest Planet",

        Some(PlanetType::Volcanic) =>
            "Volcanic Planet",

        Some(PlanetType::Crystal) =>
            "Crystal Planet",

        None =>
            "",
    }
}

fn create_test_diorama(
    planet: Option<PlanetType>,
    forest_material: Material,
    volcanic_material: Material,
    crystal_material: Material,
) -> Vec<Object> {
    let material =
        match planet {
            Some(
                PlanetType::Forest,
            ) =>
                forest_material,

            Some(
                PlanetType::Volcanic,
            ) =>
                volcanic_material,

            Some(
                PlanetType::Crystal,
            ) =>
                crystal_material,

            None =>
                forest_material,
        };

    vec![
        Object::Sphere(
            Sphere::new(
                Vec3::new(
                    0.0,
                    0.0,
                    0.0,
                ),
                1.8,
                material,
            ),
        ),
    ]
}