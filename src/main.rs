mod acceleration;
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
use objects::cube::Cube;
use objects::cylinder::Cylinder;
use objects::object::Object;
use objects::plane::Plane;
use objects::sphere::Sphere;

use scene::light::Light;
use scene::scene::Scene;
use scene::state::{
    PlanetType,
    SceneState,
};

use worlds::forest::create_forest_diorama;
use worlds::water::create_water_diorama;

fn main() {
    const RENDER_WIDTH: i32 = 1280;
    const RENDER_HEIGHT: i32 = 720;

    let (
        mut rl,
        thread,
    ) =
        raylib::init()
            .size(
                800,
                600,
            )
            .title(
                "Galaxy Diorama",
            )
            .build();

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

    rl.set_target_fps(
        60,
    );

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

    let water_material =
        Material::new(
            Vec3::new(
                0.04,
                0.42,
                0.78,
            ),
            0.48,
            0.90,
            0.16,
            0.12,
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

    let water_position =
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
                    water_position,
                    1.15,
                    water_material,
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

    let forest_objects =
        create_forest_diorama();

    let water_objects =
        create_water_diorama();

    let crystal_objects =
        create_test_diorama(
            crystal_material,
        );

    let forest_preview =
        transform_objects(
            create_forest_diorama(),
            forest_position,
            0.56,
        );

    let water_preview =
        transform_objects(
            create_water_diorama(),
            water_position,
            0.56,
        );

    let forest_scene =
        Scene::new(
            forest_objects,
        );

    let water_scene =
        Scene::new(
            water_objects,
        );

    let crystal_scene =
        Scene::new(
            crystal_objects,
        );

    let mut galaxy_objects =
        Vec::new();

    galaxy_objects.extend(
        forest_preview,
    );

    galaxy_objects.extend(
        water_preview,
    );

    galaxy_objects.push(
        Object::Sphere(
            Sphere::new(
                crystal_position,
                1.0,
                crystal_material,
            ),
        ),
    );

    let galaxy_scene =
        Scene::new(
            galaxy_objects,
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

    let mistery_block_light =
        Light::new(
            Vec3::new(
                2.02,
                1.71,
                -0.78,
            ),
            Vec3::new(
                1.0,
                0.82,
                0.22,
            ),
            3.0
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
        let current_screen_width =
            rl.get_screen_width();

        let current_screen_height =
            rl.get_screen_height();

        match state {
            SceneState::Galaxy => {
                if rl.is_mouse_button_pressed(
                    MouseButton::MOUSE_BUTTON_LEFT,
                ) {
                    let mouse =
                        rl.get_mouse_position();

                    let ray_x =
                        mouse.x
                            / current_screen_width
                                as f32
                            * RENDER_WIDTH
                                as f32;

                    let ray_y =
                        mouse.y
                            / current_screen_height
                                as f32
                            * RENDER_HEIGHT
                                as f32;

                    let ray =
                        camera.get_ray(
                            ray_x,
                            ray_y,
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
                        object,
                    ) in galaxy_hit_objects
                        .iter()
                        .enumerate()
                    {
                        if let Some(
                            distance,
                        ) =
                            object.intersect(
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
                                    Some(
                                        index,
                                    );
                            }
                        }
                    }

                    if let Some(
                        index,
                    ) =
                        selected_index
                    {
                        selected_planet =
                            match index {
                                0 => {
                                    Some(
                                        PlanetType::Forest,
                                    )
                                }

                                1 => {
                                    Some(
                                        PlanetType::Water,
                                    )
                                }

                                2 => {
                                    Some(
                                        PlanetType::Crystal,
                                    )
                                }

                                _ => {
                                    None
                                }
                            };

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
                    &galaxy_scene,
                    &light,
                    &camera,
                );
            }

            SceneState::Focused => {
                let selected_scene =
                    get_selected_scene(
                        selected_planet,
                        &forest_scene,
                        &water_scene,
                        &crystal_scene,
                    );

            let active_light =
                match selected_planet {
                    Some(PlanetType::Water) => {
                        &mistery_block_light
                    }

                    _ => {
                        &light
                    }
                };

            renderer::raytracer::render(
                &mut framebuffer,
                selected_scene,
                active_light,
                &camera,
            );
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

        let actual_screen_width =
            d.get_screen_width()
                as f32;

        let actual_screen_height =
            d.get_screen_height()
                as f32;

        let render_aspect =
            RENDER_WIDTH as f32
                / RENDER_HEIGHT as f32;

        let screen_aspect =
            actual_screen_width
                / actual_screen_height;

        let (
            destination_width,
            destination_height,
        ) =
            if screen_aspect
                > render_aspect
            {
                (
                    actual_screen_height
                        * render_aspect,

                    actual_screen_height,
                )
            } else {
                (
                    actual_screen_width,

                    actual_screen_width
                        / render_aspect,
                )
            };

        let offset_x =
            (
                actual_screen_width
                    - destination_width
            )
                * 0.5;

        let offset_y =
            (
                actual_screen_height
                    - destination_height
            )
                * 0.5;

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
                offset_x,
                offset_y,
                destination_width,
                destination_height,
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

        match state {
            SceneState::Galaxy => {
                d.draw_text(
                    "Selecciona un planeta",
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
                    "Click derecho - Rotar",
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
            }

            SceneState::Focused => {
                d.draw_text(
                    planet_name(
                        selected_planet,
                    ),
                    30,
                    30,
                    32,
                    Color::WHITE,
                );

                d.draw_text(
                    "Click izquierdo - Rotar",
                    30,
                    75,
                    20,
                    Color::LIGHTGRAY,
                );

                d.draw_text(
                    "Rueda - Zoom",
                    30,
                    105,
                    20,
                    Color::LIGHTGRAY,
                );

                d.draw_text(
                    "BACKSPACE - Regresar",
                    30,
                    135,
                    20,
                    Color::LIGHTGRAY,
                );
            }
        }

        d.draw_fps(
            current_screen_width
                - 110,
            20,
        );
    }
}

fn get_selected_scene<'a>(
    planet: Option<PlanetType>,
    forest: &'a Scene,
    water: &'a Scene,
    crystal: &'a Scene,
) -> &'a Scene {
    match planet {
        Some(
            PlanetType::Forest,
        ) => {
            forest
        }

        Some(
            PlanetType::Water,
        ) => {
            water
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
            PlanetType::Water,
        ) => {
            "Water Planet"
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

                    Object::Cube(
                        cube,
                    ) => {
                        Object::Cube(
                            Cube::from_basis(
                                cube.center
                                    * scale
                                    + offset,

                                cube.half_size
                                    * 2.0
                                    * scale,

                                cube.right,

                                cube.up,

                                cube.forward,

                                cube.material,
                            ),
                        )
                    }
                }
            },
        )
        .collect()
}