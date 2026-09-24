#[derive(Clone, Copy, PartialEq)]
pub enum SceneState {
    Galaxy,
    Focused,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PlanetType {
    Forest,
    Water,
    Crystal,
}