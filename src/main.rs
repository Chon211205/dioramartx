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
use objects::ellipsoid::Ellipsoid;
use objects::hemisphere::Hemisphere;
use objects::object::Object;
use objects::plane::Plane;
use objects::sphere::Sphere;
use objects::torus::Torus;

use scene::light::Light;
use scene::scene::Scene;
use scene::state::{
    PlanetType,
    SceneState,
};

use worlds::egg::create_egg_diorama;
use worlds::forest::create_forest_diorama;
use worlds::ice_lava::create_ice_lava_diorama;
use worlds::tree_planet::create_tree_planet_world;
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

    let egg_material =
        Material::new(
            Vec3::new(
                0.90,
                0.88,
                0.80,
            ),
            0.88,
            0.30,
            0.0,
            0.06,
        );

    let tree_material =
        Material::new(
            Vec3::new(
                0.45,
                0.30,
                0.18,
            ),
            0.85,
            0.30,
            0.0,
            0.05,
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

    let forest_position =
        Vec3::new(
            -4.2,
            0.55,
            0.0,
        );

    let water_position =
        Vec3::new(
            -2.2,
            1.15,
            -0.15,
        );

    let crystal_position =
        Vec3::new(
            0.0,
            0.35,
            0.10,
        );

    let egg_position =
        Vec3::new(
            2.1,
            1.00,
            -0.10,
        );

    let tree_position =
        Vec3::new(
            4.2,
            0.50,
            0.0,
        );

    let forest_node =
        forest_position
            + Vec3::new(
                0.0,
                -0.95,
                0.12,
            );

    let water_node =
        water_position
            + Vec3::new(
                0.0,
                -0.95,
                0.12,
            );

    let crystal_node =
        crystal_position
            + Vec3::new(
                0.0,
                -0.95,
                0.12,
            );

    let egg_node =
        egg_position
            + Vec3::new(
                0.0,
                -0.90,
                0.12,
            );

    let tree_node =
        tree_position
            + Vec3::new(
                0.0,
                -0.95,
                0.12,
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
                    1.05,
                    water_material,
                ),
            ),

            Object::Sphere(
                Sphere::new(
                    crystal_position,
                    1.10,
                    crystal_material,
                ),
            ),

            Object::Sphere(
                Sphere::new(
                    egg_position,
                    0.95,
                    egg_material,
                ),
            ),

            Object::Sphere(
                Sphere::new(
                    tree_position,
                    1.10,
                    tree_material,
                ),
            ),
        ];

    let forest_scene =
        Scene::new(
            create_forest_diorama(),
        );

    let water_scene =
        Scene::new(
            create_water_diorama(),
        );

    let crystal_scene =
        Scene::new(
            create_ice_lava_diorama(),
        );

    let egg_scene =
        Scene::new(
            create_egg_diorama(),
        );

    let tree_scene =
        Scene::new(
            create_tree_planet_world(),
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

                    if let Some(index) =
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

                                3 => {
                                    Some(
                                        PlanetType::Egg,
                                    )
                                }

                                4 => {
                                    Some(
                                        PlanetType::Tree,
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
                            9.5,
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
                let time =
                    rl.get_time()
                        as f32;

                let rotation =
                    time
                        * 0.30;

                let forest_preview =
                    transform_objects_rotated(
                        create_forest_diorama(),
                        forest_position,
                        0.42,
                        rotation,
                    );

                let water_preview =
                    transform_objects_rotated(
                        create_water_diorama(),
                        water_position,
                        0.42,
                        rotation * 0.80,
                    );

                let crystal_preview =
                    transform_objects_rotated(
                        create_ice_lava_diorama(),
                        crystal_position,
                        0.42,
                        rotation * 1.10,
                    );

                let egg_preview =
                    transform_objects_rotated(
                        create_egg_diorama(),
                        egg_position,
                        0.42,
                        rotation * 0.70,
                    );

                let tree_preview =
                    transform_objects_rotated(
                        create_tree_planet_world(),
                        tree_position,
                        0.34,
                        rotation * 0.90,
                    );

                let mut galaxy_objects =
                    Vec::new();

                add_path(
                    &mut galaxy_objects,
                    forest_node,
                    water_node,
                    path_yellow_material,
                );

                add_path(
                    &mut galaxy_objects,
                    water_node,
                    crystal_node,
                    path_yellow_material,
                );

                add_path(
                    &mut galaxy_objects,
                    crystal_node,
                    egg_node,
                    path_yellow_material,
                );

                add_path(
                    &mut galaxy_objects,
                    egg_node,
                    tree_node,
                    path_yellow_material,
                );

                add_planet_node(
                    &mut galaxy_objects,
                    forest_node,
                    path_yellow_material,
                );

                add_planet_node(
                    &mut galaxy_objects,
                    water_node,
                    path_yellow_material,
                );

                add_planet_node(
                    &mut galaxy_objects,
                    crystal_node,
                    path_yellow_material,
                );

                add_planet_node(
                    &mut galaxy_objects,
                    egg_node,
                    path_yellow_material,
                );

                add_planet_node(
                    &mut galaxy_objects,
                    tree_node,
                    path_yellow_material,
                );

                galaxy_objects.extend(
                    forest_preview,
                );

                galaxy_objects.extend(
                    water_preview,
                );

                galaxy_objects.extend(
                    crystal_preview,
                );

                galaxy_objects.extend(
                    egg_preview,
                );

                galaxy_objects.extend(
                    tree_preview,
                );

                let galaxy_scene =
                    Scene::new(
                        galaxy_objects,
                    );

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
                        &egg_scene,
                        &tree_scene,
                    );

                renderer::raytracer::render(
                    &mut framebuffer,
                    selected_scene,
                    &light,
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
            RENDER_WIDTH
                as f32
                / RENDER_HEIGHT
                    as f32;

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
    egg: &'a Scene,
    tree: &'a Scene,
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

        Some(
            PlanetType::Egg,
        ) => {
            egg
        }

        Some(
            PlanetType::Tree,
        ) => {
            tree
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
            "Ice & Lava Planet"
        }

        Some(
            PlanetType::Egg,
        ) => {
            "Egg Planet"
        }

        Some(
            PlanetType::Tree,
        ) => {
            "Tree Planet"
        }

        None => {
            ""
        }
    }
}

fn add_planet_node(
    objects: &mut Vec<Object>,
    position: Vec3,
    material: Material,
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
                0.55,
                0.10,
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
                        0.07,
                        0.0,
                    ),
                Vec3::new(
                    0.0,
                    1.0,
                    0.0,
                ),
                0.40,
                0.08,
                material,
            ),
        ),
    );
}

fn add_path(
    objects: &mut Vec<Object>,
    start: Vec3,
    end: Vec3,
    material: Material,
) {
    let start =
        start
            + Vec3::new(
                0.0,
                -0.02,
                0.0,
            );

    let end =
        end
            + Vec3::new(
                0.0,
                -0.02,
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
            start
                + end
        )
            * 0.5;

    objects.push(
        Object::Cylinder(
            Cylinder::new_oriented(
                center,
                axis,
                0.075,
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
                                ) + offset,

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
                                ) + offset,

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
                                ) + offset,

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
                                ) + offset,

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
                                ) + offset,

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
                                ) + offset,

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
                                ) + offset,

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
                                ) + offset,

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