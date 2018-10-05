#![feature(range_contains)]
#[macro_use]

extern crate cute;

mod matrix;
mod snark;



extern crate bn;
extern crate rand;

use bn::*;

use matrix::*;
use snark::*;

pub fn test() {
    let rng = rand::thread_rng();

    let mut pp = PP {l:1, t: 2, rng:rng};

    let m = Matrix::new(pp.l, pp.t, &vec![G1::one(), G1::one()]);

    let x:Vec<Fr> = vec![Fr::one(), Fr::zero()];

    let y:VecG = vec![G1::one()];

    let (ek, vk) = keygen(&mut pp, m);

    let pi = prove(&mut pp, &ek, &x);

    let b = verify(&pp, &vk, &y, &pi);

    println!("Result is {}.", b);

}
