use crate::core::vec3::Vec3;
use crate::scene::planet::PlanetDefinition;
use crate::worlds::chomp_planet::create_chomp_planet_world;
use crate::worlds::brothers_planet::create_brothers_planet_world;

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
    ]
}