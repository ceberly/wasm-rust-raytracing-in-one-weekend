type Float = f32;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: Float,
    pub y: Float,
    pub z: Float,
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    v.fdiv(v.length())
}

pub fn dot(v1: &Vec3, v2: &Vec3) -> Float {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

impl Vec3 {
    fn length(self) -> Float {
        let squared = self.x * self.x + self.y * self.y + self.z * self.z;
        squared.sqrt()
    }

    pub fn fmul(self, rhs: Float) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }

    pub fn fdiv(self, rhs: Float) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }

    pub fn vadd(self, rhs: &Vec3) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }

    pub fn vsub(self, rhs: &Vec3) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}
