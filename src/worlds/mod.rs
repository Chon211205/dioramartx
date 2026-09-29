pub mod cube_planet;
pub mod egg;
pub mod forest;
pub mod galaxy_selector;
pub mod ice_lava;
pub mod tree_planet;
pub mod water;
pub mod water_circuit;
pub mod pyramid_planet;
pub mod industrial_planet;
pub mod brick_planet;
pub mod pokeball_planet;
pub mod galaga_planet;
pub mod castle_planet;
pub mod chomp_planet;

use crate::core::vec3::Vec3;
use crate::scene::planet::PlanetDefinition;

use cube_planet::create_cube_planet;
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
            Vec3::new(-4.2, 0.55, 0.0),
            0.42,
            1.0,
            Vec3::new(0.0, -0.95, 0.12),
        ),
        PlanetDefinition::new(
            "Water Planet",
            create_water_diorama,
            Vec3::new(-2.2, 1.15, -0.15),
            0.42,
            1.05,
            Vec3::new(0.0, -0.95, 0.12),
        ),
        PlanetDefinition::new(
            "Ice & Lava Planet",
            create_ice_lava_diorama,
            Vec3::new(0.0, 0.35, 0.10),
            0.42,
            1.10,
            Vec3::new(0.0, -0.95, 0.12),
        ),
        PlanetDefinition::new(
            "Egg Planet",
            create_egg_diorama,
            Vec3::new(2.1, 1.0, -0.10),
            0.42,
            0.95,
            Vec3::new(0.0, -0.90, 0.12),
        ),
        PlanetDefinition::new(
            "Tree Planet",
            create_tree_planet_world,
            Vec3::new(4.2, 0.50, 0.0),
            0.34,
            1.10,
            Vec3::new(0.0, -0.95, 0.12),
        ),
        PlanetDefinition::new(
            "Cube Planet",
            create_cube_planet,
            Vec3::new(5.8, 0.45, -0.10),
            0.42,
            1.20,
            Vec3::new(0.0, -0.95, 0.12),
        ),
    ]
}
