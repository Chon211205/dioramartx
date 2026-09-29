use crate::core::vec3::Vec3;
use crate::materials::material::Material;
use crate::objects::cube::Cube;
use crate::objects::object::Object;

pub fn create_pyramid_planet() -> Vec<Object> {
    let mut objects = Vec::new();

    let sandstone_light = Material::new(
        Vec3::new(
            0.90,
            0.82,
            0.62,
        ),
        1.0,
        0.18,
        0.0,
        0.0,
    );

    let sandstone_mid = Material::new(
        Vec3::new(
            0.78,
            0.70,
            0.50,
        ),
        1.0,
        0.16,
        0.0,
        0.0,
    );

    let sandstone_dark = Material::new(
        Vec3::new(
            0.64,
            0.56,
            0.40,
        ),
        1.0,
        0.14,
        0.0,
        0.0,
    );

    let gold = Material::new(
        Vec3::new(
            1.0,
            0.82,
            0.20,
        ),
        1.0,
        0.85,
        0.0,
        0.05,
    );

    let block_size =
        0.52;

    let levels =
        6;

    for level in
        0..levels
    {
        let grid_size =
            levels
                - level;

        let y =
            level as f32
                * block_size;

        let material =
            match level % 3 {
                0 => sandstone_dark,
                1 => sandstone_mid,
                _ => sandstone_light,
            };

        let offset =
            (
                grid_size as f32
                    - 1.0
            )
                * block_size
                * 0.5;

        for x in
            0..grid_size
        {
            for z in
                0..grid_size
            {
                let px =
                    x as f32
                        * block_size
                        - offset;

                let pz =
                    z as f32
                        * block_size
                        - offset;

                objects.push(
                    Object::Cube(
                        Cube::new(
                            Vec3::new(
                                px,
                                y,
                                pz,
                            ),
                            block_size,
                            material,
                        ),
                    ),
                );
            }
        }
    }

    objects.push(
        Object::Cube(
            Cube::new(
                Vec3::new(
                    0.0,
                    levels as f32
                        * block_size,
                    0.0,
                ),
                block_size
                    * 0.60,
                gold,
            ),
        ),
    );

    objects
}