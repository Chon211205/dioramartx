use crate::core::vec3::Vec3;
use crate::materials::material::Material;
use crate::objects::cube::Cube;
use crate::objects::object::Object;

pub fn create_galaga_planet() -> Vec<Object> {
    let mut objects = Vec::new();

    let white = Material::new(
        Vec3::new(0.95, 0.95, 0.95),
        1.0,
        0.45,
        0.0,
        0.02,
    );

    let red = Material::new(
        Vec3::new(0.95, 0.02, 0.02),
        1.0,
        0.55,
        0.0,
        0.03,
    );

    let blue = Material::new(
        Vec3::new(0.02, 0.20, 0.95),
        1.0,
        0.55,
        0.0,
        0.03,
    );

    let pixel_size = 0.34;
    let spacing = 0.34;

    let sprite = [
        "........W........",
        "........W........",
        ".......WWW.......",
        ".......WWW.......",
        "...R...WWW...R...",
        "...R..WWWWW..R...",
        "...W..WWWWW..W...",
        ".R.WBWWWRWWB.W.R.",
        ".R.BWWWRRRWWB.R.",
        ".W.WWWWR.RWWWW.W.",
        ".W.WWWWWWWWWWW.W.",
        ".WWWWRWWWWWRWWWW.",
        ".WWW.RWWWWWR.WWW.",
        ".WW..R.....R..WW.",
        ".W...RR...RR...W.",
        "W.....W...W.....W",
    ];

    let rows = sprite.len();
    let cols = sprite[0].chars().count();

    for (row, line) in sprite.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '.' {
                continue;
            }

            let material = match ch {
                'R' => red,
                'B' => blue,
                _ => white,
            };

            let x =
                (col as f32 - (cols as f32 - 1.0) * 0.5)
                    * spacing;

            let y =
                ((rows as f32 - 1.0) * 0.5 - row as f32)
                    * spacing;

            add_pixel(
                &mut objects,
                Vec3::new(
                    x,
                    y,
                    0.0,
                ),
                pixel_size,
                material,
            );
        }
    }

    objects
}

fn add_pixel(
    objects: &mut Vec<Object>,
    center: Vec3,
    size: f32,
    material: Material,
) {
    objects.push(
        Object::Cube(
            Cube::new(
                Vec3::new(
                    center.x,
                    center.y,
                    -0.15,
                ),
                size,
                material,
            ),
        ),
    );

    objects.push(
        Object::Cube(
            Cube::new(
                Vec3::new(
                    center.x,
                    center.y,
                    0.15,
                ),
                size,
                material,
            ),
        ),
    );
}
