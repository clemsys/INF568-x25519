use self::super::montgomery::ladder;
use rug::{integer::Order, ops::Pow, Integer};

// little endian 32-byte array
pub type Bytes = [u8; 32];

fn curve25519(m: &Integer, x_p: &Integer) -> Integer {
    let p = Integer::from(2).pow(255) - Integer::from(19);
    let a = Integer::from(486_662);
    ladder(&p, &a, m, x_p)
}

fn x25519(m: &Bytes, x_p: &Bytes) -> Bytes {
    let mut m = *m;
    m[0] &= 248;
    m[31] &= 127;
    m[31] |= 64;

    let p = Integer::from(2).pow(255) - Integer::from(19);
    let m = Integer::from_digits(&m, Order::Lsf);
    let x_p = Integer::from_digits(x_p, Order::Lsf);
    let mut bytes: Vec<u8> = curve25519(&m.modulo(&p), &x_p.modulo(&p)).to_digits(Order::Lsf);
    bytes.resize(32, 0);
    bytes.try_into().unwrap()
}

pub fn generate_public_key(private: &Bytes) -> Bytes {
    let mut x_p: Bytes = [0u8; 32];
    x_p[0] = 9;
    x25519(private, &x_p)
}

pub fn generate_shared_secret(private: &Bytes, peer_public: &Bytes) -> Bytes {
    x25519(private, peer_public)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    fn test_curve25519(m: u32, expected: &str) {
        let x_p = Integer::from(9);
        let m = Integer::from(m);
        let expected = Integer::from_str(expected).unwrap();
        assert_eq!(curve25519(&m, &x_p), expected);
    }

    #[test]
    fn test_curve25519_2() {
        let m = 2;
        let expected =
            "14847277145635483483963372537557091634710985132825781088887140890597596352251";
        test_curve25519(m, &expected)
    }

    #[test]
    fn test_curve25519_3() {
        let m = 3;
        let expected =
            "12697861248284385512127539163427099897745340918349830473877503196793995869202";
        test_curve25519(m, &expected)
    }

    #[test]
    fn test_curve25519_4() {
        let m = 4;
        let expected =
            "55094879196667521951171181671895976763495004283458921215716618814842818532335";
        test_curve25519(m, &expected)
    }

    #[test]
    fn test_curve25519_5() {
        let m = 5;
        let expected =
            "29723531761959712214579609737676588517305008794118309711793522224007834336391";
        test_curve25519(m, &expected)
    }

    #[test]
    fn test_curve25519_7() {
        let m = 7;
        let expected =
            "6189616607995615193367150877376005513902989163470402290395604116858034460712";
        test_curve25519(m, &expected)
    }
}
