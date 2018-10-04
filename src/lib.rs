#![feature(range_contains)]
#[macro_use(c)]

extern crate cute;
extern crate bn;
extern crate rand;

use bn::*;
use rand::ThreadRng;

type VecG = Vec<G1>;

pub struct Matrix
{
    _m: VecG,
    nr: usize,
    nc: usize,
    N: usize
}

impl Matrix {

    // NB: Given column by column
    fn new(nr:usize, nc:usize, v:&VecG) -> Matrix {
        let l = v.len();
        assert_eq!(nr*nc, l);
        Matrix {
             _m: v.clone(),
             nr: nr,
             nc: nc,
             N: nr*nc
        }
    }

    fn get(&self, i: usize, j:usize) -> G1 {
        let idx = self.nr*j + i;
        let range = 0..self.N;
        assert!(range.contains(&idx));
        self._m[idx]
    }

    fn get_col(&self, c: usize) -> VecG {
        self._m[self.nr*c..self.nr*(c+1)].to_vec()
    }
}


pub fn test() {
    let rng = rand::thread_rng();

    let mut pp = PP {l:1, t: 2, rng:rng};

    let m = Matrix::new(pp.l, pp.t, &vec![G1::one(), G1::one()]);

    let x:Vec<Fr> = vec![Fr::one(), Fr::zero()];

    let y:VecG = vec![G1::one()];

    let (ek, vk) = keygen(&mut pp, m);

    let pi = prove(&mut pp, &ek, &x);

    let b = verify(&vk, &y, &pi);

    println!("Result is {}.", b);

}

pub fn testbn() {
    let rng = &mut rand::thread_rng();

    // Construct private keys
    let alice_sk = Fr::random(rng);
    let bob_sk = Fr::random(rng);
    let carol_sk = Fr::random(rng);

    // Construct public keys
    let alice_pk = G1::one() * alice_sk;
    let bob_pk = G1::one() * bob_sk;
    let carol_pk = G1::one() * carol_sk;

    // Round one:
    let alice_dh_1 = bob_pk * carol_sk;
    let bob_dh_1 = carol_pk * alice_sk;
    let carol_dh_1 = alice_pk * bob_sk;

    // Round two:
    let alice_dh_2 = alice_dh_1 * alice_sk;
    let bob_dh_2 = bob_dh_1 * bob_sk;
    let carol_dh_2 = carol_dh_1 * carol_sk;

    // All parties should arrive to the same shared secret
    assert!(alice_dh_2 == bob_dh_2 && bob_dh_2 == carol_dh_2);

    println!("All done!");
}

struct PP
{
    l: usize,
    t: usize,
    rng: ThreadRng,
}

impl PP {
    fn new(l: usize, t: usize) -> PP {
        PP {l:l, t:t, rng:rand::thread_rng()}
    }

    fn randomFldElem(&mut self) -> Fr {
        Fr::random(&mut self.rng)
    }
}

// XXX: How to represent a matrix in Rust?

struct EK {
    p: VecG,
}
struct VK {
    c: Vec<G2>,
    a: G2,
}

type Crs = (EK, VK);

type Proof = G1;

fn inner_product(v: &Vec<Fr>, w: &VecG) -> G1
{
    let mut res = G1::one();
    for i in 0..v.len() {
        let tmp = w[i]*v[i];
        res = res+tmp;
    }
    res
}

fn vector_matrix_mult(v: &Vec<Fr>, m:&Matrix, res: &mut VecG) {
    // the result should contain every column of m multiplied by v
    for c in 0..m.nc {
        res.push(inner_product(&v, &m.get_col(c)));
    }

}

fn scalar_vector_mult(a: &Fr, v: &Vec<Fr>, res: & mut Vec<Fr>)
{
    for i in 0..v.len() {
        res.push(*a * v[i]);
    }
}
// XXX: Make sure vectors are passed right without being copied

fn vec_to_G2(v: & Vec<Fr>) ->  Vec< G2>
{
    c![G2::one()*x, for x in v.clone()]
}
// Change so that allocation occurs earlier

fn keygen(pp: &mut PP, m: Matrix) -> Crs
{
    let mut k: Vec<Fr>  = Vec::with_capacity(pp.l);
    for _ in 1..pp.l {
        k.push(pp.randomFldElem());
    }

    let a = pp.randomFldElem();
    let mut p: Vec<G1> = Vec::with_capacity(pp.t);

    vector_matrix_mult(&k, &m, & mut p);
    let mut c: Vec<Fr> = Vec::with_capacity(pp.l);

    scalar_vector_mult(&a, &k, & mut c);
    (EK {p:p}, VK {c: vec_to_G2(&c), a: G2::one()*a})
}

fn prove(pp : &mut PP, ek: &EK,x: &Vec<Fr>) -> G1 {
    inner_product(x, &ek.p)
}

fn verify(vk: &VK, y: &VecG, pi: &Proof) -> bool {
    let mut res = Gt::one();
    for i in 0..y.len() {
        res = res * pairing(y[i],vk.c[i]);
    }
    res == pairing(*pi, vk.a)
}
