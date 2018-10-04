extern crate bn;
extern crate rand;

use bn::*;
use rand::ThreadRng;

use matrix::*;

pub type SnarkMtx = Matrix<G1>;

pub struct PP
{
pub     l: usize,
pub     t: usize,
pub    rng: ThreadRng,
}

impl PP {
    pub fn new(l: usize, t: usize) -> PP {
        PP {l:l, t:t, rng:rand::thread_rng()}
    }

    pub fn randomFldElem(&mut self) -> Fr {
        Fr::random(&mut self.rng)
    }
}


pub struct EK {
    p: VecG,
}
pub struct VK {
    c: Vec<G2>,
    a: G2,
}

pub type Crs = (EK, VK);

pub type Proof = G1;

// Change so that allocation occurs earlier

pub fn keygen(pp: &mut PP, m: SnarkMtx) -> Crs
{
    let mut k: Vec<Fr>  = Vec::with_capacity(pp.l);
    for _ in 1..pp.l {
        k.push(pp.randomFldElem());
    }

    let a = pp.randomFldElem();
    let mut p: Vec<G1> = Vec::with_capacity(pp.t);

    vector_matrix_mult(&k, &m, &mut p, G1::one());
    let mut c: Vec<Fr> = Vec::with_capacity(pp.l);

    scalar_vector_mult(&a, &k, &mut c);
    (EK {p:p}, VK {c: vec_to_G2(&c), a: G2::one()*a})
}

pub fn prove(pp : &mut PP, ek: &EK,x: &Vec<Fr>) -> G1 {
    inner_product(x, &ek.p, G1::one())
}

pub fn verify(vk: &VK, y: &VecG, pi: &Proof) -> bool {
    let mut res = Gt::one();
    for i in 0..y.len() {
        res = res * pairing(y[i],vk.c[i]);
    }
    res == pairing(*pi, vk.a)
}
