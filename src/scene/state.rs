#[derive(Clone, Copy, PartialEq)]
pub enum SceneState {
    Galaxy,
    Focused,
    Diorama,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PlanetType {
    Forest,
    Volcanic,
    Crystal,
}