use crate::core::vec3::Vec3;
use crate::materials::material::Material;
use crate::objects::cube::Cube;
use crate::objects::object::Object;

const W: usize = 18;
const H: usize = 20;
const PIXEL: f32 = 0.22;
const FRONT_Z: f32 = 0.44;
const BACK_Z: f32 = -0.44;
const INNER_LAYERS: [f32; 3] = [-0.22, 0.0, 0.22];

const MARIO: [&str; H] = [
    "..................",
    "....RRRRRRRR......",
    "...RRRRRRRRRRRR...",
    "...RRRRRRRRRRRR...",
    "..BBBBSSSBBBB.....",
    "..BBSSSSSSSSBBB...",
    "..BBSSSSSSSSSSS...",
    "..BBSSSSSSSS......",
    "...SSSSSSSSS......",
    "..RRRBBBRRRR......",
    "..RRBBBBBBRRR.....",
    ".RRRRBBBBBBRRR....",
    ".RRRRBBBBBBRRR....",
    "..SSRRRBB.........",
    "..SSSRRBB.........",
    "..BBBBRRBB........",
    ".BBBBRRRRBB.......",
    ".BBB......BB......",
    ".SSS......SSS.....",
    "..................",
];

const LUIGI: [&str; H] = [
    "..................",
    ".....WWWWWW.......",
    "...WWWWWWWWWWWW...",
    "...WWWWWWWWWWWW...",
    "..GGGGYYYGGGG.....",
    "..GGYYYYYYYYGGG...",
    "..GGYYYYYYYYYYYY..",
    "..GGYYYYYYYYY.....",
    "...YYYYYYYYYY.....",
    "..GGGWWWWGGG......",
    "..GGWWWWWWGGG.....",
    ".GGGWWWWWWWWGG....",
    ".GGGWWWWWWWWGG....",
    "..YYGGGWW.........",
    "..YYYGGWW.........",
    "..GGGGWWWW........",
    ".GGGGWWWWGG.......",
    ".GGG......GG......",
    ".YYY......YYY.....",
    "..................",
];

pub fn create_brothers_planet_world() -> Vec<Object> {
    let mut objects = Vec::new();

    let mario_red = Material::new(Vec3::new(0.86, 0.16, 0.06), 1.0, 0.20, 0.0, 0.03);
    let mario_brown = Material::new(Vec3::new(0.50, 0.40, 0.03), 1.0, 0.12, 0.0, 0.03);
    let mario_skin = Material::new(Vec3::new(0.98, 0.62, 0.26), 1.0, 0.10, 0.0, 0.02);

    let luigi_green = Material::new(Vec3::new(0.18, 0.62, 0.04), 1.0, 0.18, 0.0, 0.03);
    let luigi_white = Material::new(Vec3::new(0.92, 0.92, 0.92), 1.0, 0.10, 0.0, 0.02);
    let luigi_skin = Material::new(Vec3::new(0.98, 0.78, 0.08), 1.0, 0.10, 0.0, 0.02);

    let side_dark = Material::new(Vec3::new(0.16, 0.14, 0.10), 1.0, 0.12, 0.0, 0.02);

    add_inner_volume(&mut objects, &MARIO, &LUIGI, side_dark);
    add_mario_face(&mut objects, mario_red, mario_brown, mario_skin);
    add_luigi_face(&mut objects, luigi_green, luigi_white, luigi_skin);

    objects
}

fn add_inner_volume(
    objects: &mut Vec<Object>,
    front: &[&str; H],
    back: &[&str; H],
    material: Material,
) {
    for row in 0..H {
        for col in 0..W {
            let front_c = char_at(front[row], col);
            let back_c = char_at(back[row], col);

            if front_c != '.' || back_c != '.' {
                let x = pixel_x(col);
                let y = pixel_y(row);

                for z in INNER_LAYERS {
                    objects.push(Object::Cube(Cube::new(
                        Vec3::new(x, y, z),
                        PIXEL * 0.98,
                        material,
                    )));
                }
            }
        }
    }
}

fn add_mario_face(
    objects: &mut Vec<Object>,
    red: Material,
    brown: Material,
    skin: Material,
) {
    for row in 0..H {
        for col in 0..W {
            let c = char_at(MARIO[row], col);

            let material = match c {
                'R' => Some(red),
                'B' => Some(brown),
                'S' => Some(skin),
                _ => None,
            };

            if let Some(material) = material {
                objects.push(Object::Cube(Cube::new(
                    Vec3::new(pixel_x(col), pixel_y(row), FRONT_Z),
                    PIXEL * 0.98,
                    material,
                )));
            }
        }
    }
}

fn add_luigi_face(
    objects: &mut Vec<Object>,
    green: Material,
    white: Material,
    skin: Material,
) {
    for row in 0..H {
        for col in 0..W {
            let c = char_at(LUIGI[row], col);

            let material = match c {
                'G' => Some(green),
                'W' => Some(white),
                'Y' => Some(skin),
                _ => None,
            };

            if let Some(material) = material {
                objects.push(Object::Cube(Cube::new(
                    Vec3::new(pixel_x(col), pixel_y(row), BACK_Z),
                    PIXEL * 0.98,
                    material,
                )));
            }
        }
    }
}

fn char_at(row: &str, col: usize) -> char {
    row.as_bytes()[col] as char
}

fn pixel_x(col: usize) -> f32 {
    (col as f32 - (W as f32 - 1.0) * 0.5) * PIXEL
}

fn pixel_y(row: usize) -> f32 {
    ((H as f32 - 1.0) * 0.5 - row as f32) * PIXEL
}
