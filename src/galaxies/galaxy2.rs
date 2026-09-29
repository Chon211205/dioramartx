use crate::core::vec3::Vec3;
use crate::scene::planet::PlanetDefinition;

use crate::worlds::cube_planet::create_cube_planet;
use crate::worlds::industrial_planet::create_industrial_planet;
use crate::worlds::pyramid_planet::create_pyramid_planet;
use crate::worlds::water_circuit::create_water_circuit_world;
use crate::worlds::brick_planet::create_brick_planet;
use crate::worlds::pokeball_planet::create_pokeball_planet;
use crate::worlds::galaga_planet::create_galaga_planet;

pub fn create_galaxy() -> Vec<PlanetDefinition> {
    vec![
        PlanetDefinition::new(
            "Cube Planet",
            create_cube_planet,
            Vec3::new(
                -4.2,
                0.6,
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
            "Water Race",
            create_water_circuit_world,
            Vec3::new(
                -1.3,
                0.6,
                0.0,
            ),
            0.12,
            1.50,
            Vec3::new(
                0.0,
                -1.05,
                0.10,
            ),
        ),

        PlanetDefinition::new(
            "Pyramid Planet",
            create_pyramid_planet,
            Vec3::new(
                2.4,
                0.6,
                -2.0,
            ),
            0.30,
            1.60,
            Vec3::new(
                0.0,
                -1.05,
                0.10,
            ),
        ),

        PlanetDefinition::new(
            "Industrial Planet",
            create_industrial_planet,
            Vec3::new(
                2.4,
                0.6,
                2.0,
            ),
            0.30,
            1.60,
            Vec3::new(
                0.0,
                -1.05,
                0.10,
            ),
        ),

        PlanetDefinition::new(
            "Brick Planet",
            create_brick_planet,
            Vec3::new(
                5.8,
                0.6,
                -2.0,
            ),
            0.32,
            1.55,
            Vec3::new(
                0.0,
                -1.05,
                0.10,
            ),
        ),

        PlanetDefinition::new(
            "Pokeball Planet",
            create_pokeball_planet,
            Vec3::new(
                5.8,
                0.6,
                2.0,
            ),
            0.30,
            1.70,
            Vec3::new(
                0.0,
                -1.05,
                0.10,
            ),
        ),

        PlanetDefinition::new(
            "Galaga Planet",
            create_galaga_planet,
            Vec3::new(
                9.2,
                0.6,
                2.0,
            ),
            0.22,
            1.75,
            Vec3::new(
                0.0,
                -1.05,
                0.10,
            ),
        ),

    ]
}