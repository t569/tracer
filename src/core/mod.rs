#[derive(Debug, Clone, Copy)]
pub struct Vec3<T>{
    x: T,
    y: T,
    z: T,
}


impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3 { x, y, z }
    }

    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(&self) -> &T {
        &self.y
    }

    pub fn z(&self) -> &T {
        &self.z
    }
}

impl<T: Default> Default for Vec3<T> {
    fn default() -> Self {
        Vec3 {
            x: T::default(),
            y: T::default(),
            z: T::default(),
        }
    }
}

impl<T: std::ops::Add<Output = T>> std::ops::Add for Vec3<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}


// this is to compensate for the fact that we want to add two references to Vec3
// and we do not want to move the original Vec3
impl<T: std::ops::Add<Output = T> + Copy> std::ops::Add for &Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T: std::ops::AddAssign<T> + Copy> std::ops::AddAssign for Vec3<T> {
    // mutable borrow of self to preserve the original Vec3
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T: std::ops::Sub<Output = T>> std::ops::Sub for Vec3<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

// this is also to avoid moving the original Vec3
impl<T: std::ops::Sub<Output = T> + Copy> std::ops::Sub for &Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: std::ops::SubAssign<T> + Copy> std::ops::SubAssign for Vec3<T> {
    // mutable borrow of self to preserve the original Vec3
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

// the idea for multiplication is that the scalar is copied and not borrowed
impl<T: std::ops::Mul<Output = T> + Copy> std::ops::Mul<T> for Vec3<T> {
    type Output = Self;

    fn mul(self, scalar: T) -> Self {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl<T: std::ops::Mul<Output = T> + Copy> std::ops::Mul<T> for &Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, scalar: T) -> Self::Output {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl<T: std::ops::MulAssign + Copy> std::ops::MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, scalar: T) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl<T: std::ops::Div<Output = T> + Copy> std::ops::Div<T> for Vec3<T> {
    type Output = Self;

    fn div(self, scalar: T) -> Self {
        Vec3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl<T: std::ops::Div<Output = T> + Copy> std::ops::Div<T> for &Vec3<T> {
    type Output = Vec3<T>;

    fn div(self, scalar: T) -> Self::Output {
        Vec3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl<T: std::ops::DivAssign + Copy> std::ops::DivAssign<T> for Vec3<T> {
    fn div_assign(&mut self, scalar: T) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl<T: std::ops::Neg<Output = T>> std::ops::Neg for Vec3<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T: std::ops::Neg<Output = T> + Copy> std::ops::Neg for &Vec3<T> {
    type Output = Vec3<T>;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// this is basically just for it to be called with println!("{:?}", vec3_instance)
impl<T: std::fmt::Display> std::fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

// create a copy without a move of the original Vec3, it should be cheap
impl <T: Copy> Vec3<T> {
    pub fn copy(&self) -> Self {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl<T: PartialEq> PartialEq for Vec3<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Vec3<f32> {
    pub fn mag(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let magnitude = self.mag();
        if magnitude > 0.0 {
            Vec3 {
                x: self.x / magnitude,
                y: self.y / magnitude,
                z: self.z / magnitude,
            }
        } else {
            Vec3::default()
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn origin() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn unit_vector(v: &Self) -> Self {
        let magnitude = v.mag();
        if magnitude > 0.0 {
            Vec3 {
                x: v.x / magnitude,
                y: v.y / magnitude,
                z: v.z / magnitude,
            }
        } else {
            Vec3::default()
        }
    }


}