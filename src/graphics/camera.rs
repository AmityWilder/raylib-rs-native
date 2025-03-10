use crate::prelude::*;

pub struct Camera3D {
    pub position: Position3,
    /// Camera target it looks-at
    pub target: Position3,
    /// Camera up vector (rotation over its axis)
    pub up: Direction3,
    /// Camera field-of-view aperture in Y (degrees) in perspective, used as near plane width in orthographic
    pub fovy: Degrees,
    pub projection: CameraProjection,
}

pub type Camera = Camera3D;

pub enum CameraProjection {
    Perspective,
    Orthographic,
}

pub enum CameraMode {
    /// // Camera custom, controlled by user (UpdateCamera() does nothing)
    Custom,
    Free,
    /// Camera orbital, around target, zoom supported
    Orbital,
    FirstPerson,
    ThirdPerson,
}

impl Camera {
    pub fn forward(&self) -> Vector3 {
        (self.target - self.position).normalize()
    }

    pub fn up(&self) -> Vector3 {
        self.up.normalize()
    }

    pub fn right(&self) -> Vector3 {
        let forward = self.forward();
        let up = self.up();
        forward.cross_product(up).normalize()
    }

    pub fn move_forward(&mut self, distance: Units, move_in_world_plane: bool) {
        let mut forward = self.forward();

        if move_in_world_plane {
            forward.y = 0.0;
            forward = forward.normalize();
        }

        forward *= distance;

        self.position += forward;
        self.target   += forward;
    }

    pub fn move_up(&mut self, distance: Units) {
        let up = self.up() * distance;

        self.position += up;
        self.target   += up;
    }

    pub fn move_right(&mut self, distance: Units, move_in_world_plane: bool) {
        let mut right = self.right();

        if move_in_world_plane {
            right.y = 0.0;
            right = right.normalize();
        }

        right *= distance;

        self.position += right;
        self.target   += right;
    }

    pub fn move_to_target(&mut self, delta: Units) {
        let distance = (self.position.distance(self.target) + delta).max(f32::EPSILON);
        self.position = self.target + self.forward() * -distance;
    }

    /// Rotates the camera around its up vector
    /// Yaw is "looking left and right"
    /// If rotateAroundTarget is false, the camera rotates around its position
    /// Note: angle must be provided in radians
    pub fn yaw(&mut self, angle: Radians, rotate_around_target: bool) {
        let target = (self.target - self.position) // view vector
            .rotate_by_axis_angle(self.up(), angle); // Rotate view vector around up axis

        if rotate_around_target {
            // Move position relative to target
            self.position = self.target - target;
        } else { // rotate around self.position
            // Move target relative to position
            self.target = self.position + target;
        }
    }

    /// Rotates the camera around its right vector, pitch is "looking up and down"
    ///  - lockView prevents camera overrotation (aka "somersaults")
    ///  - rotateAroundTarget defines if rotation is around target or around its position
    ///  - rotateUp rotates the up direction as well (typically only usefull in CAMERA_FREE)
    /// NOTE: angle must be provided in radians
    pub fn pitch(&mut self, mut angle: Radians, lock_view: bool, rotate_around_target: bool, rotate_up: bool) {
        // Up direction
        let up = self.up();

        // View vector
        let target_position = self.target - self.position;

        if lock_view {
            // In these camera modes we clamp the Pitch angle
            // to allow only viewing straight up or down.

            // Clamp view up
            let max_angle_up = up.angle(target_position) - 0.001;

            // Clamp view down
            let max_angle_down = -(-up).angle(target_position) + 0.001;

            angle = angle.clamp(max_angle_down, max_angle_up)
        }

        // Rotation axis
        let right = self.right();

        // Rotate view vector around right axis
        let target_position = target_position.rotate_by_axis_angle(right, angle);

        if rotate_around_target {
            // Move position relative to target
            self.position = self.target - target_position;
        } else { // rotate around self.position
            // Move target relative to position
            self.target = self.position + target_position;
        }

        if rotate_up {
            // Rotate up direction around right axis
            self.up = self.up.rotate_by_axis_angle(right, angle);
        }
    }

    // Rotates the camera around its forward vector
    // Roll is "turning your head sideways to the left or right"
    // Note: angle must be provided in radians
    pub fn roll(&mut self, angle: Radians) {
        todo!()
    }

    pub fn view_matrix(&self) -> Matrix {
        todo!()
    }

    pub fn projection_matrix(&self, aspect: f32) -> Matrix {
        todo!()
    }

    pub const CULL_DISTANCE_NEAR: f32 =    0.01;
    pub const CULL_DISTANCE_FAR:  f32 = 1000.0 ;

    pub const MOVE_SPEED: Ratio<f32, Second> = Ratio(5.4, Second);
    pub const PAN_SPEED:  Ratio<f32, Second> = Ratio(0.2, Second);

    /// Camera orbital speed in CAMERA_ORBITAL mode
    pub const ORBITAL_SPEED:  Ratio<Radians, Second> = Ratio(0.5 , Second);
    pub const ROTATION_SPEED: Ratio<Radians, Second> = Ratio(0.03, Second);

    pub const MOUSE_MOVE_SENSITIVITY: f32 = 0.003;
}
