use std::sync::OnceLock;

use crate::core::vec3::Vec3;
use crate::materials::material::Material;
use crate::objects::cuboid::Cuboid;
use crate::objects::hemisphere::Hemisphere;
use crate::objects::object::Object;
use crate::textures::texture::TextureMap;

static STONE_COLOR: OnceLock<TextureMap> = OnceLock::new();
static STONE_NORMAL: OnceLock<TextureMap> = OnceLock::new();
static STONE_ROUGHNESS: OnceLock<TextureMap> = OnceLock::new();
static ROCK_COLOR: OnceLock<TextureMap> = OnceLock::new();
static ROCK_NORMAL: OnceLock<TextureMap> = OnceLock::new();
static ROCK_ROUGHNESS: OnceLock<TextureMap> = OnceLock::new();
static WOOD_COLOR: OnceLock<TextureMap> = OnceLock::new();
static WOOD_NORMAL: OnceLock<TextureMap> = OnceLock::new();
static WOOD_ROUGHNESS: OnceLock<TextureMap> = OnceLock::new();

fn stone_material() -> Material {
    Material::textured(
        Vec3::new(0.76, 0.78, 0.76), 1.0, 0.18, 0.0, 0.01,
        Some(STONE_COLOR.get_or_init(|| TextureMap::from_file("assets/textures/castle/Bricks066_1K-PNG_Color.png"))),
        Some(STONE_NORMAL.get_or_init(|| TextureMap::from_file("assets/textures/castle/Bricks066_1K-PNG_NormalGL.png"))),
        Some(STONE_ROUGHNESS.get_or_init(|| TextureMap::from_file("assets/textures/castle/Bricks066_1K-PNG_Roughness.png"))),
        None,
    )
}

fn plain(color: Vec3, emission: f32) -> Material {
    Material::new(color, 1.0, 0.28, 0.0, emission)
}

fn rock_material() -> Material {
    Material::textured(
        Vec3::new(0.52,0.50,0.47),1.0,0.16,0.0,0.01,
        Some(ROCK_COLOR.get_or_init(|| TextureMap::from_file("assets/textures/rock/Rock041_1K-PNG_Color.png"))),
        Some(ROCK_NORMAL.get_or_init(|| TextureMap::from_file("assets/textures/rock/Rock041_1K-PNG_NormalGL.png"))),
        Some(ROCK_ROUGHNESS.get_or_init(|| TextureMap::from_file("assets/textures/rock/Rock041_1K-PNG_Roughness.png"))),
        None,
    )
}

fn wood_material() -> Material {
    Material::textured(
        Vec3::new(0.62,0.42,0.22),1.0,0.24,0.0,0.03,
        Some(WOOD_COLOR.get_or_init(|| TextureMap::from_file("assets/textures/Face4Cube/Wood025_1K-PNG_Color.png"))),
        Some(WOOD_NORMAL.get_or_init(|| TextureMap::from_file("assets/textures/Face4Cube/Wood025_1K-PNG_NormalGL.png"))),
        Some(WOOD_ROUGHNESS.get_or_init(|| TextureMap::from_file("assets/textures/Face4Cube/Wood025_1K-PNG_Roughness.png"))),
        None,
    )
}

fn block(objects: &mut Vec<Object>, center: Vec3, size: Vec3, material: Material) {
    objects.push(Object::Cuboid(Cuboid::from_basis(
        center, size, Vec3::new(1.0,0.0,0.0), Vec3::new(0.0,1.0,0.0), Vec3::new(0.0,0.0,1.0), material,
    )));
}

pub fn create_tower_house_world() -> Vec<Object> {
    let mut objects=Vec::new();
    let stone=stone_material();
    let trim=plain(Vec3::new(0.60,0.62,0.60),0.0);
    let roof=plain(Vec3::new(0.16,0.07,0.28),0.01);
    let dark=plain(Vec3::new(0.018,0.012,0.028),0.0);
    let glass=Material::new(
        Vec3::new(0.045,0.085,0.14),
        0.58,
        0.95,
        0.06,
        0.72,
    );
    let rock=rock_material();
    let wood=wood_material();

    objects.push(Object::Hemisphere(Hemisphere::new_with_materials(
        Vec3::new(0.0,-2.35,0.0),2.65,Vec3::new(0.0,-1.0,0.0),rock,rock,
    )));

    // Los cuatro cuerpos escalonados de la torre de la referencia.
    block(&mut objects,Vec3::new(0.0,-1.20,0.0),Vec3::new(3.35,1.85,2.45),stone);
    block(&mut objects,Vec3::new(-0.12,0.38,0.0),Vec3::new(3.05,1.42,2.25),stone);
    block(&mut objects,Vec3::new(0.08,1.73,0.0),Vec3::new(2.72,1.34,2.05),stone);
    block(&mut objects,Vec3::new(0.0,3.05,0.0),Vec3::new(3.05,1.32,2.18),stone);

    // Cornisas y balcones salientes.
    for (y,size) in [(-0.22,Vec3::new(3.65,0.15,2.72)),(1.05,Vec3::new(3.38,0.14,2.48)),(2.36,Vec3::new(3.18,0.15,2.38)),(3.73,Vec3::new(3.35,0.14,2.45))] {
        block(&mut objects,Vec3::new(0.0,y,0.0),size,trim);
    }

    // Contrafuertes verticales de la planta baja.
    for x in [-1.48,-1.05,1.05,1.48] {
        block(&mut objects,Vec3::new(x,-1.02,1.30),Vec3::new(0.16,1.55,0.22),trim);
    }

    add_front_details(&mut objects,dark,glass,trim,wood);
    add_side_windows(&mut objects,glass,trim);
    add_roof(&mut objects,roof,stone,glass);

    // La base rocosa es el primer objeto. Bajamos la arquitectura completa
    // como una sola pieza para que sus cimientos entren ligeramente en la roca.
    for object in objects.iter_mut().skip(1) {
        if let Object::Cuboid(cuboid)=object {
            cuboid.center.y-=0.38;
        }
    }
    objects
}

fn add_front_details(objects:&mut Vec<Object>,dark:Material,glass:Material,trim:Material,wood:Material){
    block(objects,Vec3::new(0.72,-1.70,1.245),Vec3::new(0.62,0.82,0.12),dark);
    block(objects,Vec3::new(0.72,-1.65,1.32),Vec3::new(0.42,0.62,0.08),wood);
    for (y,z,xs) in [
        (-1.18,1.255,[-1.16,-0.58,0.0,0.58]),
        (0.35,1.155,[-1.05,-0.35,0.35,1.05]),
        (1.67,1.055,[-0.85,-0.28,0.28,0.85]),
        (3.05,1.125,[-1.05,-0.35,0.35,1.05]),
    ] {
        for x in xs {
            if y < -1.0 && x > 0.4 { continue; }
            window(objects,Vec3::new(x,y,z),glass,trim);
        }
    }
}

fn add_side_windows(objects:&mut Vec<Object>,glass:Material,trim:Material){
    for (y,wall_x,zs) in [
        (-1.25,1.675,[-0.62,0.12,0.72]),
        (0.35,1.525,[-0.58,0.10,0.66]),
        (1.70,1.360,[-0.52,0.08,0.56]),
        (3.05,1.525,[-0.58,0.10,0.66]),
    ] {
        for side in [-1.0_f32,1.0] {
            for z in zs {
                block(objects,Vec3::new(side*(wall_x+0.035),y,z),Vec3::new(0.10,0.48,0.22),trim);
                block(objects,Vec3::new(side*(wall_x+0.075),y,z),Vec3::new(0.045,0.35,0.14),glass);
            }
        }
    }
}

fn window(objects:&mut Vec<Object>,position:Vec3,glass:Material,trim:Material){
    block(objects,position,Vec3::new(0.34,0.58,0.13),trim);
    block(objects,position+Vec3::new(0.0,0.0,0.075),Vec3::new(0.22,0.43,0.055),glass);
    block(objects,position+Vec3::new(0.0,0.0,0.11),Vec3::new(0.035,0.44,0.025),trim);
}

fn add_roof(objects:&mut Vec<Object>,roof:Material,stone:Material,glass:Material){
    let slope=0.58_f32;
    let forward_front=Vec3::new(0.0,-slope,1.0).normalize();
    let up_front=Vec3::new(0.0,1.0,slope).normalize();
    let forward_back=Vec3::new(0.0,slope,1.0).normalize();
    let up_back=Vec3::new(0.0,1.0,-slope).normalize();
    objects.push(Object::Cuboid(Cuboid::from_basis(Vec3::new(0.0,4.08,0.64),Vec3::new(3.65,0.20,1.68),Vec3::new(1.0,0.0,0.0),up_front,forward_front,roof)));
    objects.push(Object::Cuboid(Cuboid::from_basis(Vec3::new(0.0,4.08,-0.64),Vec3::new(3.65,0.20,1.68),Vec3::new(1.0,0.0,0.0),up_back,forward_back,roof)));
    // Cumbrera horizontal: cubre por completo la unión de las dos pendientes.
    block(objects,Vec3::new(0.0,4.53,0.0),Vec3::new(3.62,0.20,0.22),roof);

    // Frontones de piedra que cierran los huecos triangulares bajo la V del
    // techo. Las franjas decrecen hacia la cumbrera para seguir la pendiente.
    for side in [-1.0_f32,1.0] {
        for (y,width) in [(3.80,1.82),(3.98,1.42),(4.16,1.02),(4.34,0.60)] {
            block(objects,Vec3::new(side*1.54,y,0.0),Vec3::new(0.18,0.22,width),stone);
        }
    }

    // Tres buhardillas en cada lado. Los cuerpos penetran la cubierta y los
    // tejadillos se solapan con ellos para que ninguna pieza quede flotando.
    for side in [-1.0_f32,1.0] {
        for x in [-1.05,0.0,1.05] {
            block(objects,Vec3::new(x,4.43,side*0.88),Vec3::new(0.50,0.72,0.48),stone);
            block(objects,Vec3::new(x,4.43,side*1.145),Vec3::new(0.20,0.34,0.08),glass);
            let r=Vec3::new(1.0,0.0,0.0);
            let u=Vec3::new(0.0,0.82,side*0.57);
            let f=Vec3::new(0.0,-side*0.57,0.82);
            objects.push(Object::Cuboid(Cuboid::from_basis(
                Vec3::new(x,4.78,side*0.90),Vec3::new(0.66,0.14,0.62),r,u,f,roof,
            )));
        }
    }
}
