use rug::{ops::Pow, Integer};

struct PCoord {
    x: Integer,
    z: Integer,
}

fn xADD(p: &Integer, x_p: &PCoord, x_q: &PCoord, x_pmq: &PCoord) -> PCoord {
    let two = Integer::from(2);
    let u = ((x_p.x.clone() - &x_p.z).modulo(p) * (x_q.x.clone() + &x_q.z).modulo(p)).modulo(p);
    let v = ((x_p.x.clone() + &x_p.z).modulo(p) * (x_q.x.clone() - &x_q.z).modulo(p)).modulo(p);
    let x = x_pmq.z.clone() * ((u.clone() + &v).secure_pow_mod(&two, p));
    let z = x_pmq.x.clone() * ((u - v).secure_pow_mod(&two, p));
    PCoord { x, z }
}

fn xDBL(p: &Integer, a: &Integer, x_p: &PCoord) -> PCoord {
    let two = Integer::from(2);
    let q = (x_p.x.clone() + &x_p.z).secure_pow_mod(&two, p);
    let r = (x_p.x.clone() - &x_p.z).secure_pow_mod(&two, p);
    let s = (q.clone() - &r).modulo(p);
    let x = (q * &r).modulo(p);
    let z: Integer = (((r + (((a.clone() + 2) >> 2) * &s)) * s) as Integer).modulo(p);
    PCoord { x, z }
}

fn ladder(m: &Integer, a: &PCoord) -> PCoord {
    let p: Integer = Integer::from(2).pow(255) - 19;
    let a: Integer = Integer::from(486662);
    todo!()
}
