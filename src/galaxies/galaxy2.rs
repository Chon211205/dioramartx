use crate::core::vec3::Vec3;
use crate::scene::planet::PlanetDefinition;

use crate::worlds::cube_planet::create_cube_planet;
use crate::worlds::water_circuit::create_water_circuit_world;

pub fn create_galaxy() -> Vec<PlanetDefinition> {
    vec![
        PlanetDefinition::new(
            "Cube Planet",
            create_cube_planet,
            Vec3::new(
                -2.5,
                0.45,
                0.0,
            ),
            0.42,
            1.50,
            Vec3::new(
                0.0,
                -1.05,
                0.10,
            ),
        ),

        PlanetDefinition::new(
            "Water Circuit",
            create_water_circuit_world,
            Vec3::new(
                2.5,
                0.55,
                0.0,
            ),
            0.15,
            1.50,
            Vec3::new(
                0.0,
                -1.05,
                0.10,
            ),
        ),
    ]
}