use rug::Integer;

#[derive(Clone)]
struct PCoord {
    x: Integer,
    z: Integer,
}

impl std::ops::Mul<bool> for PCoord {
    type Output = Self;
    /// constant time multiplication of a `PCoord` by a bool
    fn mul(self, rhs: bool) -> Self {
        let f = u32::from(rhs);
        let zero = Self {
            x: Integer::from(0),
            z: Integer::from(0),
        };
        Self {
            x: self.x * f + zero.x * (1 - f),
            z: self.z * f + zero.z * (1 - f),
        }
    }
}

impl std::ops::Add<Self> for PCoord {
    type Output = Self;
    /// coordinate-wise addition of two `PCoord`
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            z: self.z + rhs.z,
        }
    }
}

impl PCoord {
    /// coordinate-wise modulo of a `PCoord`
    fn modulo(self, p: &Integer) -> Self {
        Self {
            x: self.x.modulo(p),
            z: self.z.modulo(p),
        }
    }

    /// replace x by x/z and z by 1
    fn normalize(self, p: &Integer) -> Self {
        Self {
            x: (self.x * self.z.invert(p).unwrap()).modulo(p),
            z: Integer::from(1),
        }
    }
}

fn x_add(p: &Integer, x_p: &PCoord, x_q: &PCoord, x_pmq: &PCoord) -> PCoord {
    let two = Integer::from(2);
    let u = ((x_p.x.clone() - &x_p.z).modulo(p) * (x_q.x.clone() + &x_q.z).modulo(p)).modulo(p);
    let v = ((x_p.x.clone() + &x_p.z).modulo(p) * (x_q.x.clone() - &x_q.z).modulo(p)).modulo(p);
    let x = x_pmq.z.clone() * ((u.clone() + &v).secure_pow_mod(&two, p));
    let z = x_pmq.x.clone() * ((u - v).secure_pow_mod(&two, p));
    PCoord { x, z }
}

fn x_dbl(p: &Integer, a: &Integer, x_p: &PCoord) -> PCoord {
    let two = Integer::from(2);
    let q = (x_p.x.clone() + &x_p.z).secure_pow_mod(&two, p);
    let r = (x_p.x.clone() - &x_p.z).secure_pow_mod(&two, p);
    let s = (q.clone() - &r).modulo(p);
    let x = (q * &r).modulo(p);
    let ap2o4ts = ((((a.clone() + 2) * (Integer::from(4).invert(p).unwrap()).modulo(p)) * &s)
        as Integer)
        .modulo(p);
    let z: Integer = (((r + ap2o4ts) * s) as Integer).modulo(p);
    PCoord { x, z }
}

pub fn ladder(p: &Integer, a: &Integer, m: &Integer, x_p: &Integer) -> Integer {
    let u = PCoord {
        x: x_p.clone(),
        z: Integer::from(1),
    };
    let mut x_0 = PCoord {
        x: Integer::from(1),
        z: Integer::from(0),
    };
    let mut x_1 = u.clone();
    for i in (0..m.significant_bits()).rev() {
        let add = x_add(p, &x_0, &x_1, &u);
        let dbl_0 = x_dbl(p, a, &x_0);
        let dbl_1 = x_dbl(p, a, &x_1);
        let bit = m.get_bit(i);
        x_0 = (add.clone() * bit + dbl_0 * !bit).modulo(p);
        x_1 = (dbl_1 * bit + add * !bit).modulo(p);
    }
    x_0.normalize(p).x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_x_dbl() {
        let p = Integer::from(101);
        let a = Integer::from(49);
        let x_p = PCoord {
            x: Integer::from(2),
            z: Integer::from(1),
        };
        let dbl = x_dbl(&p, &a, &x_p);
        assert_eq!(dbl.x, Integer::from(9));
        assert_eq!(dbl.z, Integer::from(16));
    }

    fn test_ladder(p: u32, a: u32, m: u32, x: u32, expected: u32) {
        let p = Integer::from(p);
        let a = Integer::from(a);
        let x = Integer::from(x);
        let m = Integer::from(m);
        let expected = Integer::from(expected);
        let result = ladder(&p, &a, &m, &x);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_ladder_101_2() {
        test_ladder(101, 49, 2, 2, 70);
    }

    #[test]
    fn test_ladder_101_3() {
        test_ladder(101, 49, 3, 2, 59);
    }

    #[test]
    fn test_ladder_101_77() {
        test_ladder(101, 49, 77, 2, 8);
    }

    #[test]
    fn test_ladder_1009_2() {
        test_ladder(1009, 682, 2, 7, 284);
    }

    #[test]
    fn test_ladder_1009_3() {
        test_ladder(1009, 682, 3, 7, 759);
    }

    #[test]
    fn test_ladder_1009_5() {
        test_ladder(1009, 682, 5, 7, 1000);
    }

    #[test]
    fn test_ladder_1009_34() {
        test_ladder(1009, 682, 34, 7, 286);
    }

    #[test]
    fn test_ladder_1009_104() {
        test_ladder(1009, 682, 104, 7, 810);
    }

    #[test]
    fn test_ladder_1009_947() {
        test_ladder(1009, 682, 947, 7, 755);
    }
}
