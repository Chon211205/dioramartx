use crate::core::vec3::Vec3;

pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub fov: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
}

impl Camera {
    pub fn new(
        target: Vec3,
        distance: f32,
        fov: f32,
    ) -> Self {
        let mut camera = Self {
            position: Vec3::new(0.0, 0.0, distance),
            target,
            fov,
            yaw: 0.0,
            pitch: 0.0,
            distance,
        };

        camera.update_position();

        camera
    }

    pub fn rotate(&mut self, delta_x: f32, delta_y: f32) {
        let sensitivity = 0.005;

        self.yaw -= delta_x * sensitivity;
        self.pitch += delta_y * sensitivity;

        let limit = 1.5;

        if self.pitch > limit {
            self.pitch = limit;
        }

        if self.pitch < -limit {
            self.pitch = -limit;
        }

        self.update_position();
    }

    pub fn zoom(&mut self, amount: f32) {
        self.distance -= amount * 0.5;

        if self.distance < 2.5 {
            self.distance = 2.5;
        }

        if self.distance > 15.0 {
            self.distance = 15.0;
        }

        self.update_position();
    }

    fn update_position(&mut self) {
        let horizontal_distance =
            self.distance * self.pitch.cos();

        self.position.x =
            self.target.x
            + horizontal_distance * self.yaw.sin();

        self.position.y =
            self.target.y
            + self.distance * self.pitch.sin();

        self.position.z =
            self.target.z
            + horizontal_distance * self.yaw.cos();
    }
}