pub mod egg;
pub mod forest;
pub mod ice_lava;
pub mod tree_planet;
pub mod water;

use crate::core::vec3::Vec3;
use crate::scene::planet::PlanetDefinition;

use egg::create_egg_diorama;
use forest::create_forest_diorama;
use ice_lava::create_ice_lava_diorama;
use tree_planet::create_tree_planet_world;
use water::create_water_diorama;

pub fn planet_registry() -> Vec<PlanetDefinition> {
    vec![
        PlanetDefinition::new(
            "Forest Planet",
            create_forest_diorama,
            Vec3::new(
                -4.2,
                0.55,
                0.0,
            ),
            0.42,
            1.0,
            Vec3::new(
                0.0,
                -0.95,
                0.12,
            ),
        ),

        PlanetDefinition::new(
            "Water Planet",
            create_water_diorama,
            Vec3::new(
                -2.2,
                1.15,
                -0.15,
            ),
            0.42,
            1.05,
            Vec3::new(
                0.0,
                -0.95,
                0.12,
            ),
        ),

        PlanetDefinition::new(
            "Ice & Lava Planet",
            create_ice_lava_diorama,
            Vec3::new(
                0.0,
                0.35,
                0.10,
            ),
            0.42,
            1.10,
            Vec3::new(
                0.0,
                -0.95,
                0.12,
            ),
        ),

        PlanetDefinition::new(
            "Egg Planet",
            create_egg_diorama,
            Vec3::new(
                2.1,
                1.0,
                -0.10,
            ),
            0.42,
            0.95,
            Vec3::new(
                0.0,
                -0.90,
                0.12,
            ),
        ),

        PlanetDefinition::new(
            "Tree Planet",
            create_tree_planet_world,
            Vec3::new(
                4.2,
                0.50,
                0.0,
            ),
            0.34,
            1.10,
            Vec3::new(
                0.0,
                -0.95,
                0.12,
            ),
        ),
    ]
}