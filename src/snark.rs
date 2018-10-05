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


pub fn vec_to_G2(v: & Vec<Fr>) ->  Vec< G2>
{
    c![G2::one()* *x, for x in v]
}

// NB: Now the system is for y = Mx

pub fn keygen(pp: &mut PP, m: SnarkMtx) -> Crs
{
    let mut k: Vec<Fr>  = Vec::with_capacity(pp.l);
    for _ in 0..pp.l {
        k.push(pp.randomFldElem());
    }

    let a = pp.randomFldElem();
    let mut p: Vec<G1> = Vec::with_capacity(pp.t);

    vector_matrix_mult(&k, &m, &mut p, G1::zero());
    let mut c: Vec<Fr> = Vec::with_capacity(pp.l);

    scalar_vector_mult(&a, &k, &mut c);
    (EK {p:p}, VK {c: vec_to_G2(&c), a: G2::one()*a})
}

pub fn prove(pp : &mut PP, ek: &EK,x: &Vec<Fr>) -> G1 {
    assert_eq!(pp.l, x.len());
    inner_product(x, &ek.p, G1::zero())
}

pub fn verify(pp : &PP, vk: &VK, y: &VecG, pi: &Proof) -> bool {
    assert_eq!(pp.t, y.len());

    let mut res = Gt::one();
    for i in 0..y.len() {
        res = res * pairing(y[i],vk.c[i]);
    }
    res == pairing(*pi, vk.a)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_snark() {
        let rng = rand::thread_rng();

        let mut pp = PP {l:2, t: 2, rng:rng};

        /*
            m =  1  2
                 1  1
        */
        let m = Matrix::new(pp.l, pp.t, &vec![G1::one(), G1::one(), G1::one()+G1::one(), G1::one()]);

        /*
            x =  [0, 1]
        */
        let x:Vec<Fr> = vec![Fr::zero(), Fr::one()];

        /*
            y =  [1, 2]
        */
        let y:VecG = vec![G1::one()+G1::one(), G1::one(),];

        let (ek, vk) = keygen(&mut pp, m);

        let pi = prove(&mut pp, &ek, &x);

        let b = verify(&pp, &vk, &y, &pi);

        assert_eq!(b, true);

    }
}
