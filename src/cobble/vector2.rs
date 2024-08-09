use::std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Copy, Clone)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32
}

impl ops::Sub for Vec2i {
    type Output = Self;

    fn sub(self, other: Vec2i) -> Self {
        return Vec2i {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl ops::Add for Vec2i {
    type Output = Self;

    fn add(self, other: Vec2i) -> Self {
        return Vec2i {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}


impl Vec2i {
    pub fn as_vec2f(self) -> Vec2f {
        return Vec2f {
            x: self.x as f32,
            y: self.y as f32
        };
    }
}

impl Vec2f {
    pub fn as_vec2i(self) -> Vec2i {
        return Vec2i {
            x: self.x as i32,
            y: self.y as i32
        };
    }

    pub fn len(self) -> f32 {
        return (self.x * self.x + self.y * self.y).sqrt();
    }

    pub fn normalized(self) -> Vec2f {
        let len = self.len();

        return Vec2f{
            x: self.x / len,
            y: self.y / len
        };
    }
}

impl ops::Mul<f32> for Vec2f {
    type Output = Self;

    fn mul(self, other: f32) -> Self::Output {
        return Vec2f {
            x: self.x * other,
            y: self.y * other
        };
    }
}
