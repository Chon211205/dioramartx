use crate::core::vec3::Vec3;
use crate::scene::planet::PlanetDefinition;

use crate::worlds::cube_planet::create_cube_planet;

pub fn create_galaxy() -> Vec<PlanetDefinition> {
    vec![
        PlanetDefinition::new(
            "Cube Planet",
            create_cube_planet,
            Vec3::new(
                0.0,
                0.3,
                0.0,
            ),
            0.42,
            1.25,
            Vec3::new(
                0.0,
                -1.05,
                0.10,
            ),
        ),
    ]
}