use bn::*;
use std::ops::{Mul,Add};


pub type VecG = Vec<G1>;

pub struct Matrix<T>
{
    _m: Vec<T>,
pub    nr: usize,
pub    nc: usize,
pub    N: usize
}

impl<T: Copy> Matrix<T> {

    // NB: Given column by column
    pub fn new(nr:usize, nc:usize, v:& Vec<T>) -> Matrix<T> {
        let l = v.len();
        assert_eq!(nr*nc, l);
        Matrix {
             _m: v.to_vec(),
             nr: nr,
             nc: nc,
             N: nr*nc
        }
    }

    pub fn get(&self, i: usize, j:usize) -> T {
        let idx = self.nr*j + i;
        let range = 0..self.N;
        assert!(range.contains(&idx));
        self._m[idx]
    }

    fn get_col(&self, c: usize) -> Vec<T> {
        let col = self._m[self.nr*c..self.nr*(c+1)].to_vec();
        col
    }
}


pub fn inner_product<T:Mul<U,Output = T>+Add<Output=T>+Copy, U:Copy>(v: &Vec<U>, w: &Vec<T>, zero:T) -> T
{
    let mut res:T = zero;
    for i in 0..v.len() {
        let tmp:T = w[i]*v[i];
        res = res + tmp;
    }
    res
}

pub fn vector_matrix_mult<T: Mul<U,Output = T>+Add<Output=T>+Copy,U:Copy>(v: &Vec<U>, m:&Matrix<T>, res: &mut Vec<T>, zero:T) {
    // the result should contain every column of m multiplied by v
    for c in 0..m.nc {
        res.push(inner_product(&v, &m.get_col(c), zero));
    }

}

pub fn scalar_vector_mult<T: Mul<Output = T>+Copy>(a: &T, v: &Vec<T>, res: & mut Vec<T>)
{
    for i in 0..v.len() {
        res.push((*a * v[i]));
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_col() {
        let m = Matrix::new(3, 2, &vec![(0,0), (1,0), (2,0), (0,1),  (1,1), (2,1) ]);
        //println!("{:?}", m.get_col(0));

        assert_eq!(m.get_col(0), vec![(0,0), (1,0), (2,0)]);
    }

    #[test]
    fn test_inner_product_nat() {
        let u = vec![0,1,2];
        let v = vec![1, 2, 3];
        let y = inner_product(&u,&v,0);
        assert_eq!(y, 8);
    }

    fn testEq(a:&G1, b:&G1) -> bool {
        (*a-*b).is_zero()
    }

    #[test]
    fn test_inner_product_G1() {
        let u = vec![G1::one(), G1::zero()];
        let v:Vec<Fr> = vec![Fr::one()+Fr::one(), Fr::zero()];
        let y = inner_product(&v,&u, G1::zero());

        assert!(y == G1::one()+G1::one());
    }

    #[test]
    fn test_scalar_vector_mult() {
        let a = 10;
        let u:Vec<i32> = (1..4).collect();
        let mut y = Vec::with_capacity(u.len());
        scalar_vector_mult(&a, &u, &mut y);
        assert_eq!(y, [10,20,30]);
    }

}
