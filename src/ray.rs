use vec3::Vec3f;

#[derive(Copy, Clone)]
pub struct Ray {
    pub a: Vec3f,
    pub b: Vec3f,
}

impl Ray {
    pub fn origin<'a>(&'a self) -> &'a Vec3f {
        &self.a
    }

    pub fn direction<'a>(&'a self) -> &'a Vec3f {
        &self.b
    }

    pub fn point_at_parameter<'a>(&'a self, t: f32) -> Vec3f {
        self.a + t * self.b
    }

    pub fn new(va: &Vec3f, vb: &Vec3f) -> Ray {
        Ray { a: *va, b: *vb }
    }
}

#[test]
fn test_point_at_parameter() {
    let ray = Ray {
        a: Vec3f { e: [0.0, 1.0, 2.0] },

        b: Vec3f { e: [1.0, 2.0, 3.0] },
    };
    let p = ray.point_at_parameter(2.0);
    assert_eq!(p.x(), 2.0);
    assert_eq!(p.y(), 5.0);
    assert_eq!(p.z(), 8.0);
}
