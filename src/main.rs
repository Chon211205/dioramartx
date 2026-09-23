mod core;
mod materials;
mod objects;
mod renderer;
mod scene;
mod textures;
mod worlds;

use raylib::prelude::*;

use core::camera::Camera;
use core::framebuffer::Framebuffer;
use core::vec3::Vec3;

use materials::material::Material;

use objects::cone::Cone;
use objects::cylinder::Cylinder;
use objects::object::Object;
use objects::plane::Plane;
use objects::sphere::Sphere;

use scene::light::Light;
use scene::state::{
    PlanetType,
    SceneState,
};

use worlds::forest::create_forest_diorama;

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

    let galaxy_hit_objects =
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

    let forest_diorama =
        create_forest_diorama();

    let forest_preview =
        transform_objects(
            create_forest_diorama(),
            forest_position,
            0.56,
        );

    let volcanic_diorama =
        create_test_diorama(
            volcanic_material,
        );

    let crystal_diorama =
        create_test_diorama(
            crystal_material,
        );

    let mut galaxy_render_objects =
        forest_preview;

    galaxy_render_objects.push(
        Object::Sphere(
            Sphere::new(
                volcanic_position,
                1.15,
                volcanic_material,
            ),
        ),
    );

    galaxy_render_objects.push(
        Object::Sphere(
            Sphere::new(
                crystal_position,
                1.0,
                crystal_material,
            ),
        ),
    );

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
                        Option<usize> =
                        None;

                    for (index, object) in
                        galaxy_hit_objects
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
                            }

                            1 => {
                                selected_planet =
                                    Some(
                                        PlanetType::Volcanic,
                                    );
                            }

                            2 => {
                                selected_planet =
                                    Some(
                                        PlanetType::Crystal,
                                    );
                            }

                            _ => {}
                        }

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
            camera.zoom(
                wheel,
            );
        }

        match state {
            SceneState::Galaxy => {
                renderer::raytracer::render(
                    &mut framebuffer,
                    &galaxy_render_objects,
                    &light,
                    &camera,
                );
            }

            SceneState::Focused => {
                let objects =
                    get_selected_objects(
                        selected_planet,
                        &forest_diorama,
                        &volcanic_diorama,
                        &crystal_diorama,
                    );

                renderer::raytracer::render(
                    &mut framebuffer,
                    objects,
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

                d.draw_text(
                    "Click izquierdo - Seleccionar",
                    20,
                    55,
                    18,
                    Color::LIGHTGRAY,
                );

                d.draw_text(
                    "Click derecho - Rotar",
                    20,
                    80,
                    18,
                    Color::LIGHTGRAY,
                );

                d.draw_text(
                    "Rueda - Zoom",
                    20,
                    105,
                    18,
                    Color::LIGHTGRAY,
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
                    "Click izquierdo - Rotar",
                    20,
                    55,
                    18,
                    Color::LIGHTGRAY,
                );

                d.draw_text(
                    "Rueda - Zoom",
                    20,
                    80,
                    18,
                    Color::LIGHTGRAY,
                );

                d.draw_text(
                    "BACKSPACE - Regresar",
                    20,
                    105,
                    18,
                    Color::LIGHTGRAY,
                );
            }
        }
    }
}

fn get_selected_objects<'a>(
    planet: Option<PlanetType>,
    forest: &'a [Object],
    volcanic: &'a [Object],
    crystal: &'a [Object],
) -> &'a [Object] {
    match planet {
        Some(
            PlanetType::Forest,
        ) => {
            forest
        }

        Some(
            PlanetType::Volcanic,
        ) => {
            volcanic
        }

        Some(
            PlanetType::Crystal,
        ) => {
            crystal
        }

        None => {
            forest
        }
    }
}

fn planet_name(
    planet: Option<PlanetType>,
) -> &'static str {
    match planet {
        Some(
            PlanetType::Forest,
        ) => {
            "Forest Planet"
        }

        Some(
            PlanetType::Volcanic,
        ) => {
            "Volcanic Planet"
        }

        Some(
            PlanetType::Crystal,
        ) => {
            "Crystal Planet"
        }

        None => {
            ""
        }
    }
}

fn create_test_diorama(
    material: Material,
) -> Vec<Object> {
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

fn transform_objects(
    objects: Vec<Object>,
    offset: Vec3,
    scale: f32,
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
                                sphere.center
                                    * scale
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
                                cylinder.center
                                    * scale
                                    + offset,
                                cylinder.axis,
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
                                cone.center
                                    * scale
                                    + offset,
                                cone.axis,
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
                                plane.point
                                    * scale
                                    + offset,
                                plane.normal,
                                plane.material,
                            ),
                        )
                    }
                }
            },
        )
        .collect()
}