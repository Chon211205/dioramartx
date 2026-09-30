pub mod galaxy1;
pub mod galaxy2;
pub mod galaxy3;

use crate::scene::planet::PlanetDefinition;

pub fn galaxy_registry(galaxy: usize) -> Vec<PlanetDefinition> {
    match galaxy {
        0 => galaxy1::create_galaxy(),
        1 => galaxy2::create_galaxy(),
        2 => galaxy3::create_galaxy(),
        _ => Vec::new(),
    }
}
