use crate::core::vec3::Vec3;
use crate::scene::planet::PlanetDefinition;
use crate::worlds::chomp_planet::create_chomp_planet_world;
use crate::worlds::brothers_planet::create_brothers_planet_world;
use crate::worlds::castle_planet::create_castle_planet;
use crate::worlds::kirby_planet::create_kirby_planet_world;
use crate::worlds::mirror_planet::{create_mirror_planet_preview, create_mirror_planet_world};
use crate::worlds::rainbow_kart_planet::create_rainbow_kart_world;

pub fn create_galaxy() -> Vec<PlanetDefinition> {
    vec![
        PlanetDefinition::new(
            "Chomp Planet",
            create_chomp_planet_world,
            Vec3::new(
                0.0,
                0.0,
                0.0,
            ),
            0.55,
            2.6,
            Vec3::new(
                0.0,
                -1.45,
                0.0,
            ),
        ),

        PlanetDefinition::new(
            "Brothers Planet",
            create_brothers_planet_world,
            Vec3::new(
                3.5,
                0.0,
                0.0,
            ),
            0.42,
            2.20,
            Vec3::new(
                0.0,
                -1.20,
                0.0,
            ),
        ),

        PlanetDefinition::new(
            "Kirby Planet",
            create_kirby_planet_world,
            Vec3::new(
                6.8,
                0.5,
                0.0,
            ),
            0.46,
            2.30,
            Vec3::new(
                0.0,
                -1.10,
                0.0,
            ),
        ),
        PlanetDefinition::new_with_preview(
            "Mirror Planet",
            create_mirror_planet_world,
            create_mirror_planet_preview,
            Vec3::new(9.0, 0.75, -2.8),
            0.62,
            1.25,
            Vec3::new(0.0, -1.35, 0.0),
        ),
        PlanetDefinition::new(
            "Castle Planet",
            create_castle_planet,
            Vec3::new(13.0, 3.0, 5.0),
            0.90,
            3.40,
            Vec3::new(0.0, -4.80, 0.0),
        ),
        PlanetDefinition::new(
            "Rainbow Kart Circuit",
            create_rainbow_kart_world,
            Vec3::new(22.0, 1.0, 5.0),
            0.24,
            2.6,
            Vec3::new(-5.0, -2.80, 0.0),
        ),
    ]
}
