use crate::core::ray::Ray;
use crate::core::vec3::Vec3;

pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub fov: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,

    target_goal: Vec3,
    distance_goal: f32,
    transitioning: bool,
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

            target_goal: target,
            distance_goal: distance,
            transitioning: false,
        };

        camera.update_position();

        camera
    }

    pub fn rotate(
        &mut self,
        delta_x: f32,
        delta_y: f32,
    ) {
        if self.transitioning {
            return;
        }

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

    pub fn zoom(
        &mut self,
        amount: f32,
    ) {
        if self.transitioning {
            return;
        }

        self.distance -= amount * 0.5;

        if self.distance < 2.5 {
            self.distance = 2.5;
        }

        if self.distance > 15.0 {
            self.distance = 15.0;
        }

        self.distance_goal = self.distance;

        self.update_position();
    }

    pub fn focus_on(
        &mut self,
        target: Vec3,
        distance: f32,
    ) {
        self.target = target;
        self.target_goal = target;

        self.distance = distance;
        self.distance_goal = distance;

        self.yaw = 0.0;
        self.pitch = 0.0;

        self.transitioning = false;

        self.update_position();
    }

    pub fn start_focus(
        &mut self,
        target: Vec3,
        distance: f32,
    ) {
        self.target_goal = target;
        self.distance_goal = distance;

        self.yaw = 0.0;
        self.pitch = 0.0;

        self.transitioning = true;
    }

    pub fn update_transition(&mut self) {
        if !self.transitioning {
            return;
        }

        let speed = 0.08;

        self.target.x +=
            (self.target_goal.x - self.target.x) * speed;

        self.target.y +=
            (self.target_goal.y - self.target.y) * speed;

        self.target.z +=
            (self.target_goal.z - self.target.z) * speed;

        self.distance +=
            (self.distance_goal - self.distance) * speed;

        let target_difference =
            (self.target_goal - self.target).length();

        let distance_difference =
            (self.distance_goal - self.distance).abs();

        if target_difference < 0.01
            && distance_difference < 0.01
        {
            self.target = self.target_goal;
            self.distance = self.distance_goal;
            self.transitioning = false;
        }

        self.update_position();
    }

    pub fn is_transitioning(&self) -> bool {
        self.transitioning
    }

    pub fn get_ray(
        &self,
        screen_x: f32,
        screen_y: f32,
        width: f32,
        height: f32,
    ) -> Ray {
        let aspect_ratio = width / height;

        let forward =
            (self.target - self.position).normalize();

        let world_up =
            Vec3::new(0.0, 1.0, 0.0);

        let right =
            forward
                .cross(&world_up)
                .normalize();

        let up =
            right
                .cross(&forward)
                .normalize();

        let scale =
            (self.fov.to_radians() * 0.5).tan();

        let px =
            (
                2.0
                    * ((screen_x + 0.5) / width)
                    - 1.0
            )
                * aspect_ratio
                * scale;

        let py =
            (
                1.0
                    - 2.0
                        * ((screen_y + 0.5) / height)
            )
                * scale;

        let direction =
            (
                forward
                    + right * px
                    + up * py
            )
                .normalize();

        Ray::new(
            self.position,
            direction,
        )
    }

    fn update_position(&mut self) {
        let horizontal_distance =
            self.distance * self.pitch.cos();

        self.position.x =
            self.target.x
                + horizontal_distance
                    * self.yaw.sin();

        self.position.y =
            self.target.y
                + self.distance
                    * self.pitch.sin();

        self.position.z =
            self.target.z
                + horizontal_distance
                    * self.yaw.cos();
    }
}