#[derive(Debug)]
#[derive(PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub struct Vector2 {
    pub x: f64,
    pub y: f64
}

impl Vector3 {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }
    
    pub fn add(&mut self, other: &Self) -> Self {
        return Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
    
    pub fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y && self.z == other.z;
    }

    pub fn times(&mut self, other: &Self) -> Self {
        return Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

mod tests_func {
    use crate::math::Vector3;

    fn vector3_times() -> bool {
        let mut vec3_1 = Vector3 {
            x: 2.1,
            y: 3.2,
            z: 4.3,
        };
        let vec3_2 = Vector3 {
            x: 5.1,
            y: 6.2,
            z: 7.3,
        };

        let result = vec3_1.add(&vec3_2);
        if result.x == 7.2 && result.y == 9.4 && result.z == 11.6 {
            return true;   
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector3_eq() {
        let vec3_1 = Vector3 {
            x: 2.1,
            y: 3.2,
            z: 4.3,
        };
        let vec3_2 = Vector3 {
            x: 5.1,
            y: 6.2,
            z: 7.3,
        };
        
        let result = vec3_1.eq(&vec3_2);
        assert_eq!(result, true);
    }

    #[test]
    fn test_vector3_times() {
        let mut vec3_1 = Vector3 {
            x: 2.1,
            y: 3.2,
            z: 4.3,
        };
        let vec3_2 = Vector3 {
            x: 5.1,
            y: 6.2,
            z: 7.3,
        };

        let result = vec3_1.add(&vec3_2);
        assert_eq!(result, Vector3 {x: 7.2, y: 9.4, z: 11.6});
    }
}