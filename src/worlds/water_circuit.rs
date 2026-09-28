use std::f32::consts::TAU;

use crate::core::vec3::Vec3;
use crate::materials::material::Material;
use crate::objects::cylinder::Cylinder;
use crate::objects::object::Object;
use crate::objects::sphere::Sphere;

pub fn create_water_circuit_world() -> Vec<Object> {
    let mut objects = Vec::new();

    let water = Material::new(
        Vec3::new(0.10, 0.68, 1.0),
        0.95,
        0.85,
        0.0,
        0.12,
    );

    let water_light = Material::new(
        Vec3::new(0.35, 0.90, 1.0),
        0.95,
        0.95,
        0.0,
        0.15,
    );

    let foam = Material::new(
        Vec3::new(0.90, 0.98, 1.0),
        1.0,
        0.75,
        0.0,
        0.04,
    );

    let yellow = Material::new(
        Vec3::new(1.0, 0.82, 0.05),
        0.95,
        0.90,
        0.0,
        0.12,
    );

    let green = Material::new(
        Vec3::new(0.25, 1.0, 0.18),
        0.95,
        0.85,
        0.0,
        0.10,
    );

    let green_dark = Material::new(
        Vec3::new(0.06, 0.55, 0.16),
        0.90,
        0.60,
        0.0,
        0.06,
    );

    let white = Material::new(
        Vec3::new(0.94, 0.97, 1.0),
        0.90,
        0.60,
        0.0,
        0.06,
    );

    let gold = Material::new(
        Vec3::new(1.0, 0.72, 0.10),
        0.95,
        1.0,
        0.0,
        0.16,
    );

    let blue = Material::new(
        Vec3::new(0.03, 0.28, 0.95),
        0.90,
        0.75,
        0.0,
        0.08,
    );

    add_water_track(
        &mut objects,
        water,
        water_light,
        foam,
    );

    add_yellow_gate_group(
        &mut objects,
        yellow,
    );

    add_green_tunnel(
        &mut objects,
        green,
        green_dark,
    );

    add_finish_gate(
        &mut objects,
        gold,
        yellow,
    );

    add_start_panels(
        &mut objects,
        blue,
        yellow,
    );

    add_floating_platforms(
        &mut objects,
        white,
        gold,
    );

    add_track_supports(
        &mut objects,
        white,
    );

    add_water_splashes(
        &mut objects,
        foam,
        water_light,
    );

    objects
}

fn track_point(t: f32) -> Vec3 {
    let t = t.clamp(0.0, 1.0);

    if t < 0.18 {
        let u = t / 0.18;

        Vec3::new(
            -8.0 + u * 6.0,
            -0.7 + u * 1.8,
            3.8 - u * 2.0,
        )
    } else if t < 0.38 {
        let u =
            (t - 0.18)
                / 0.20;

        let angle =
            std::f32::consts::PI
                + u * std::f32::consts::PI;

        Vec3::new(
            -1.8
                + angle.cos()
                    * 4.0,
            1.0
                + (u * std::f32::consts::PI)
                    .sin()
                    * 1.1,
            0.8
                + angle.sin()
                    * 3.3,
        )
    } else if t < 0.55 {
        let u =
            (t - 0.38)
                / 0.17;

        Vec3::new(
            2.0 + u * 4.2,
            1.0 + u * 2.6,
            0.8 - u * 1.6,
        )
    } else if t < 0.72 {
        let u =
            (t - 0.55)
                / 0.17;

        let angle =
            u * std::f32::consts::PI
                * 1.45;

        Vec3::new(
            6.2
                + angle.sin()
                    * 2.0,
            3.6
                + angle.sin()
                    * 0.7,
            -0.8
                + angle.cos()
                    * 2.5,
        )
    } else if t < 0.87 {
        let u =
            (t - 0.72)
                / 0.15;

        Vec3::new(
            7.9 - u * 4.5,
            3.4 - u * 1.4,
            -3.1 + u * 1.8,
        )
    } else {
        let u =
            (t - 0.87)
                / 0.13;

        let angle =
            u * TAU;

        Vec3::new(
            3.5
                + angle.cos()
                    * 2.2,
            2.0
                + u * 2.0,
            -1.3
                + angle.sin()
                    * 2.2,
        )
    }
}

fn track_side(
    t: f32,
) -> Vec3 {
    let p0 =
        track_point(
            (t - 0.002)
                .max(0.0),
        );

    let p1 =
        track_point(
            (t + 0.002)
                .min(1.0),
        );

    let direction =
        (
            p1 - p0
        )
            .normalize();

    let up =
        Vec3::new(
            0.0,
            1.0,
            0.0,
        );

    let mut side =
        direction.cross(
            &up,
        );

    if side.length()
        < 0.001
    {
        side =
            Vec3::new(
                1.0,
                0.0,
                0.0,
            );
    }

    side.normalize()
}

fn add_water_track(
    objects: &mut Vec<Object>,
    water: Material,
    water_light: Material,
    foam: Material,
) {
    let segments =
        150;

    let lane_offsets =
        [
            -0.75,
            -0.38,
            0.0,
            0.38,
            0.75,
        ];

    for i in
        0..segments
    {
        let t0 =
            i as f32
                / segments
                    as f32;

        let t1 =
            (
                i + 1
            )
                as f32
                / segments
                    as f32;

        let p0 =
            track_point(t0);

        let p1 =
            track_point(t1);

        let middle_t =
            (
                t0 + t1
            )
                * 0.5;

        let side =
            track_side(
                middle_t,
            );

        for (
            lane_index,
            offset,
        ) in lane_offsets
            .iter()
            .enumerate()
        {
            let start =
                p0
                    + side
                        * *offset;

            let end =
                p1
                    + side
                        * *offset;

            let direction =
                end - start;

            let length =
                direction.length();

            if length
                <= 0.001
            {
                continue;
            }

            let material =
                if lane_index % 2
                    == 0
                {
                    water
                } else {
                    water_light
                };

            objects.push(
                Object::Cylinder(
                    Cylinder::new_oriented(
                        (
                            start
                                + end
                        )
                            * 0.5,
                        direction.normalize(),
                        0.24,
                        length
                            + 0.10,
                        material,
                    ),
                ),
            );
        }

        if i % 2
            == 0
        {
            let left =
                (
                    p0 + p1
                )
                    * 0.5
                    + side
                        * 1.02;

            let right =
                (
                    p0 + p1
                )
                    * 0.5
                    - side
                        * 1.02;

            objects.push(
                Object::Sphere(
                    Sphere::new(
                        left,
                        0.12,
                        foam,
                    ),
                ),
            );

            objects.push(
                Object::Sphere(
                    Sphere::new(
                        right,
                        0.12,
                        foam,
                    ),
                ),
            );
        }
    }
}

fn add_yellow_gate_group(
    objects: &mut Vec<Object>,
    material: Material,
) {
    let positions =
        [
            0.13,
            0.155,
            0.18,
            0.205,
        ];

    for t in positions {
        let center =
            track_point(t)
                + Vec3::new(
                    0.0,
                    0.60,
                    0.0,
                );

        let forward =
            (
                track_point(
                    (t + 0.01)
                        .min(1.0),
                )
                    - track_point(
                        (t - 0.01)
                            .max(0.0),
                    )
            )
                .normalize();

        add_vertical_ring(
            objects,
            center,
            forward,
            1.20,
            0.12,
            26,
            material,
        );
    }
}

fn add_green_tunnel(
    objects: &mut Vec<Object>,
    green: Material,
    green_dark: Material,
) {
    for i in
        0..9
    {
        let t =
            0.65
                + i as f32
                    * 0.012;

        let center =
            track_point(t)
                + Vec3::new(
                    0.0,
                    0.15,
                    0.0,
                );

        let forward =
            (
                track_point(
                    (t + 0.006)
                        .min(1.0),
                )
                    - track_point(
                        (t - 0.006)
                            .max(0.0),
                    )
            )
                .normalize();

        add_vertical_ring(
            objects,
            center,
            forward,
            1.15,
            0.11,
            24,
            if i % 2
                == 0
            {
                green
            } else {
                green_dark
            },
        );
    }
}

fn add_vertical_ring(
    objects: &mut Vec<Object>,
    center: Vec3,
    forward: Vec3,
    radius: f32,
    thickness: f32,
    segments: usize,
    material: Material,
) {
    let up =
        Vec3::new(
            0.0,
            1.0,
            0.0,
        );

    let mut right =
        forward.cross(
            &up,
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

    let real_up =
        right
            .cross(
                &forward,
            )
            .normalize();

    for i in
        0..segments
    {
        let angle =
            i as f32
                / segments
                    as f32
                * TAU;

        let position =
            center
                + right
                    * (
                        angle.cos()
                            * radius
                    )
                + real_up
                    * (
                        angle.sin()
                            * radius
                    );

        objects.push(
            Object::Sphere(
                Sphere::new(
                    position,
                    thickness,
                    material,
                ),
            ),
        );
    }
}

fn add_start_panels(
    objects: &mut Vec<Object>,
    blue: Material,
    yellow: Material,
) {
    let base =
        track_point(
            0.035,
        );

    let side =
        track_side(
            0.035,
        );

    for i in
        0..5
    {
        let center =
            base
                - side
                    * (
                        1.4
                            + i as f32
                                * 0.55
                    )
                + Vec3::new(
                    0.0,
                    0.35,
                    0.0,
                );

        objects.push(
            Object::Sphere(
                Sphere::new(
                    center,
                    0.30,
                    blue,
                ),
            ),
        );

        objects.push(
            Object::Sphere(
                Sphere::new(
                    center
                        + Vec3::new(
                            0.0,
                            0.18,
                            0.0,
                        ),
                    0.10,
                    yellow,
                ),
            ),
        );
    }
}

fn add_finish_gate(
    objects: &mut Vec<Object>,
    gold: Material,
    yellow: Material,
) {
    let t =
        0.78;

    let center =
        track_point(t)
            + Vec3::new(
                0.0,
                1.2,
                0.0,
            );

    let forward =
        (
            track_point(
                t + 0.01,
            )
                - track_point(
                    t - 0.01,
                )
        )
            .normalize();

    add_vertical_ring(
        objects,
        center,
        forward,
        1.50,
        0.16,
        28,
        gold,
    );

    let side =
        track_side(t);

    for i in
        0..9
    {
        let angle =
            i as f32
                / 8.0
                * std::f32::consts::PI;

        let position =
            center
                + side
                    * (
                        angle.cos()
                            * 1.90
                    )
                + Vec3::new(
                    0.0,
                    angle.sin()
                        * 1.90,
                    0.0,
                );

        objects.push(
            Object::Sphere(
                Sphere::new(
                    position,
                    0.16,
                    yellow,
                ),
            ),
        );
    }
}

fn add_floating_platforms(
    objects: &mut Vec<Object>,
    white: Material,
    gold: Material,
) {
    let platform_data =
        [
            (
                Vec3::new(
                    1.5,
                    2.0,
                    3.6,
                ),
                0.65,
            ),
            (
                Vec3::new(
                    6.8,
                    1.3,
                    -4.7,
                ),
                0.55,
            ),
            (
                Vec3::new(
                    8.4,
                    2.0,
                    -4.0,
                ),
                0.48,
            ),
        ];

    for (
        center,
        radius,
    ) in platform_data
    {
        objects.push(
            Object::Cylinder(
                Cylinder::new(
                    center,
                    radius,
                    0.18,
                    white,
                ),
            ),
        );

        objects.push(
            Object::Sphere(
                Sphere::new(
                    center
                        + Vec3::new(
                            0.0,
                            0.18,
                            0.0,
                        ),
                    radius
                        * 0.55,
                    white,
                ),
            ),
        );

        objects.push(
            Object::Cylinder(
                Cylinder::new(
                    center
                        + Vec3::new(
                            0.0,
                            0.40,
                            0.0,
                        ),
                    0.035,
                    0.55,
                    gold,
                ),
            ),
        );

        objects.push(
            Object::Sphere(
                Sphere::new(
                    center
                        + Vec3::new(
                            radius
                                * 0.20,
                            0.66,
                            0.0,
                        ),
                    0.055,
                    gold,
                ),
            ),
        );

        objects.push(
            Object::Sphere(
                Sphere::new(
                    center
                        + Vec3::new(
                            0.0,
                            0.66,
                            radius
                                * 0.20,
                        ),
                    0.055,
                    gold,
                ),
            ),
        );
    }
}

fn add_track_supports(
    objects: &mut Vec<Object>,
    material: Material,
) {
    let support_points =
        [
            0.04,
            0.12,
            0.25,
            0.34,
            0.45,
            0.56,
            0.68,
            0.79,
            0.90,
        ];

    for t in support_points {
        let point =
            track_point(t);

        let bottom_y =
            -4.8;

        let height =
            (
                point.y
                    - bottom_y
            )
                .max(
                    0.5,
                );

        objects.push(
            Object::Cylinder(
                Cylinder::new(
                    Vec3::new(
                        point.x,
                        bottom_y
                            + height
                                * 0.5,
                        point.z,
                    ),
                    0.07,
                    height,
                    material,
                ),
            ),
        );
    }
}

fn add_water_splashes(
    objects: &mut Vec<Object>,
    foam: Material,
    water: Material,
) {
    let points =
        [
            0.34,
            0.37,
            0.52,
            0.73,
            0.86,
        ];

    for (
        group,
        t,
    ) in points
        .iter()
        .enumerate()
    {
        let base =
            track_point(
                *t,
            );

        for i in
            0..7
        {
            let angle =
                i as f32
                    / 7.0
                    * TAU;

            let radius =
                0.25
                    + i as f32
                        * 0.035;

            let position =
                base
                    + Vec3::new(
                        angle.cos()
                            * radius,
                        0.18
                            + (
                                i as f32
                                    * 0.7
                            )
                                .sin()
                                .abs()
                                * 0.35,
                        angle.sin()
                            * radius,
                    );

            objects.push(
                Object::Sphere(
                    Sphere::new(
                        position,
                        if group % 2
                            == 0
                        {
                            0.07
                        } else {
                            0.055
                        },
                        if i % 2
                            == 0
                        {
                            foam
                        } else {
                            water
                        },
                    ),
                ),
            );
        }
    }
}