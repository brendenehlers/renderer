pub enum CameraMovement {
    FORWARD,
    BACKWARD,
    LEFT,
    RIGHT,
}

pub struct Camera {
    pub camera_pos: glm::Vec3,
    pub camera_front: glm::Vec3,
    pub camera_up: glm::Vec3,
    pub camera_right: glm::Vec3,
    pub world_up: glm::Vec3,

    yaw: f32,
    pitch: f32,

    // camera settings
    move_speed: f32,
    mouse_sensitivity: f32,
    pub zoom: f32,
}

impl Camera {
    pub fn new(pos: glm::Vec3, up: glm::Vec3, yaw: f32, pitch: f32) -> Camera {
        let mut camera = Camera {
            camera_pos: pos,
            camera_front: glm::vec3(0.0, 0.0, 0.0),
            camera_up: glm::vec3(0.0, 0.0, 0.0),
            camera_right: glm::vec3(0.0, 0.0, 0.0),
            world_up: up,
            yaw: yaw,
            pitch: pitch,
            move_speed: 2.5,
            mouse_sensitivity: 0.01,
            zoom: 45.0,
        };
        camera.update_vectors();
        camera
    }

    pub fn view_matrix(&self) -> glm::Mat4 {
        glm::look_at(
            &self.camera_pos,
            &(self.camera_pos + self.camera_front),
            &self.camera_up,
        )
    }

    pub fn process_keyboard_input(&mut self, movement: CameraMovement, delta_time: &f32) {
        let velocity = self.speed(delta_time);
        match movement {
            CameraMovement::FORWARD => self.camera_pos += velocity * self.camera_front,
            CameraMovement::BACKWARD => self.camera_pos -= velocity * self.camera_front,
            CameraMovement::LEFT => {
                self.camera_pos -=
                    glm::normalize(&glm::cross(&self.camera_front, &self.camera_up)) * velocity
            }
            CameraMovement::RIGHT => {
                self.camera_pos +=
                    glm::normalize(&glm::cross(&self.camera_front, &self.camera_up)) * velocity
            }
        }
    }

    pub fn process_mouse_movement(&mut self, x_offset: &f32, y_offset: &f32) {
        let x_offset = x_offset * self.mouse_sensitivity;
        let y_offset = y_offset * self.mouse_sensitivity;

        self.yaw += x_offset;
        self.pitch += y_offset;
        if self.pitch > 89.0 {
            self.pitch = 89.0
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0
        }

        self.update_vectors();
    }

    pub fn process_scroll(&mut self, y_offset: &f32) {
        self.zoom -= y_offset;
        if self.zoom < 1.0 {
            self.zoom = 1.0;
        }
        if self.zoom > 45.0 {
            self.zoom = 45.0
        }
    }

    fn update_vectors(&mut self) {
        self.camera_front = glm::normalize(&glm::vec3(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        ));
        self.camera_right = glm::normalize(&glm::cross(&self.camera_front, &self.world_up));
        self.camera_up = glm::normalize(&glm::cross(&self.camera_right, &self.camera_front));
    }

    fn speed(&self, delta_time: &f32) -> f32 {
        self.move_speed * delta_time
    }
}
