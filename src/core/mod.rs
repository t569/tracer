use crate::{random_float, random_float_interval};

// some global constants
pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;


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

impl<T: Default> Vec3<T> {
    fn default() -> Self {
        Vec3 {
            x: T::default(),
            y: T::default(),
            z: T::default(),
        }
    }
}


// this is a redefinition of the Add trait for Vec3
// that is why it is written as std::ops::Add

impl<T: std::ops::Add<Output = T>> std::ops::Add for Vec3<T> {

    // to implement the Add trait, we need to specify the Output type
    // which is the same type as the Vec3 we are adding
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

// this also means T supports the Add trait
// lets define the Add trait for a reference to Vec3
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

// this properly implements += and makes sure the type T implements AddAssign(+=)

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


impl<T: PartialEq> PartialEq for Vec3<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Vec3<f64> {
    pub fn mag(&self) -> f64 {
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

    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn dot_explicit(a: &Self, b: &Self) -> f64
    {
        a.x * b.x + a.y * b.y + a.z * b.z
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
    pub fn random_in_unit_disk() -> Vec3<f64>
    {
        loop{
            let p = Vec3::new(crate::random_float_interval(-1.0,1.0), random_float_interval(-1.0, 1.0), 0.0);
            if p.dot(&p) < 1.0{
                return p;
            }
        }
    }
    pub fn random_unit_vector() -> Self{

        // keep generating vectors in range [-1, 1] if it lies outside the sphere, but in the rectangle, generate a new one
        loop {
            let p = Vec3::random_range_vec3(-1.0, 1.0);
            let lensq = p.dot(&p);

            // black hole condition: to prevent blowing up of really small vectors when normalised
            if lensq <= 1.0 && 1e-160 < lensq
            {
                return p / lensq.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3<f64>) -> Vec3<f64>
    {
        let on_unit_sphere = Self::random_unit_vector();
        if Self::dot_explicit(&on_unit_sphere, normal) > 0.0
        {
            on_unit_sphere
        }
        else {
            -on_unit_sphere
        }

    }

    // generate random Vec3s
    pub fn random_vec3() -> Vec3<f64>
    {
        Vec3::new(random_float(), random_float(), random_float())
    }

    pub fn random_range_vec3(min: f64, max: f64) -> Vec3<f64>
    {
        Vec3::new(random_float_interval(min, max), random_float_interval(min, max), random_float_interval(min, max))
    }

    pub fn near_zero(&self) -> bool{
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }

    // reflection of the an incident ray on a surface
    pub fn reflect(v: &Vec3<f64>, n: &Vec3<f64>) -> Vec3<f64>
    {
        v - &(2.0 * Vec3::dot_explicit(v, n) * (*n))
    }

    pub fn refract(uv: &Vec3<f64>, n: &Vec3<f64>, etai_over_etat: f64) -> Vec3<f64>
    {
        let cos_theta = Vec3::dot_explicit(&-uv, &n).min(1.0);
        let r_out_perpendicular = etai_over_etat * *(&(*uv + (n * cos_theta))); // dont consider the nonsense syntax here smh
        let r_out_parallel = - ((1.0 - r_out_perpendicular.dot(&r_out_perpendicular)).abs().sqrt() * *n);

        r_out_parallel + r_out_perpendicular
    }


}

// the junk code lives!!!
impl std::ops::Mul<Vec3<f64>> for f64{
    type Output = Vec3<f64>;

    fn mul(self, other: Vec3<f64>) -> Vec3<f64> {
        Vec3 {
            x: self as f64 * other.x,
            y: self as f64 * other.y,
            z: self as f64 * other.z,
        }
    }

}

// for attenuation
impl std::ops::Mul for Vec3<f64>
{
    type Output = Vec3<f64>;

    fn mul(self, rhs: Vec3<f64>) -> Vec3<f64>
    {
        Vec3::new(
            self.x() * rhs.x(),
            self.y() * rhs.y(),
            self.z() * rhs.z()
        )
    }
}


