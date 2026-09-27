mod acceleration;
mod core;
mod materials;
mod objects;
mod renderer;
mod scene;
mod textures;
mod worlds;

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

fn main() {
    const RENDER_WIDTH: i32 = 1280;
    const RENDER_HEIGHT: i32 = 720;

    let (mut rl, thread) =
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
            .map(
                |planet| {
                    Scene::new(
                        (planet.create)(),
                    )
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

    let galaxy_hit_objects: Vec<Object> =
        planets
            .iter()
            .map(
                |planet| {
                    Object::Sphere(
                        Sphere::new(
                            planet.position,
                            planet.hit_radius,
                            hit_material,
                        ),
                    )
                },
            )
            .collect();

    let nodes: Vec<Vec3> =
        planets
            .iter()
            .map(
                |planet| {
                    planet.position
                        + planet.node_offset
                },
            )
            .collect();

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
                            Some(
                                index,
                            );

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
                        * 0.25;

                let mut galaxy_objects =
                    Vec::new();

                if nodes.len()
                    > 1
                {
                    for i in
                        0..nodes.len() - 1
                    {
                        add_path(
                            &mut galaxy_objects,
                            nodes[i],
                            nodes[i + 1],
                            path_yellow_material,
                        );
                    }
                }

                for node in
                    &nodes
                {
                    add_planet_node(
                        &mut galaxy_objects,
                        *node,
                        path_yellow_material,
                    );
                }

                for planet in
                    &planets
                {
                    let preview =
                        transform_objects_rotated(
                            (planet.create)(),
                            planet.position,
                            planet.preview_scale,
                            rotation,
                        );

                    galaxy_objects.extend(
                        preview,
                    );
                }

                let galaxy_scene =
                    Scene::new(
                        galaxy_objects,
                    );

                renderer::raytracer::render_rotated(
                    &mut framebuffer,
                    &galaxy_scene,
                    &light,
                    &camera,
                    rotation,
                );
            }

            SceneState::Focused => {
                if let Some(
                    index,
                ) =
                    selected_planet
                {
                    renderer::raytracer::render(
                        &mut framebuffer,
                        &focused_scenes[index],
                        &light,
                        &camera,
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

        let mouse_position =
            rl.get_mouse_position();

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
            }

            SceneState::Focused => {
                if let Some(
                    index,
                ) =
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

        draw_star_cursor(
            &mut d,
            mouse_position,
        );

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
        end
            - start;

    let length =
        direction.length();

    if length
        < 0.001
    {
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

fn draw_star_cursor(
    d: &mut RaylibDrawHandle,
    mouse: Vector2,
) {
    const POINTS: usize = 10;

    let outer_radius =
        25.0;

    let inner_radius =
        12.0;

    let inner_outer_radius =
        19.0;

    let inner_inner_radius =
        8.5;

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
            (i + 1)
                % POINTS;

        d.draw_triangle(
            outer[next],
            outer[i],
            mouse,
            border,
        );
    }

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
            (i + 1)
                % POINTS;

        d.draw_triangle(
            inner[next],
            inner[i],
            mouse,
            fill,
        );
    }
}