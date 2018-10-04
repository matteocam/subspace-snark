use bn::*;

pub type VecG = Vec<G1>;

pub struct Matrix
{
    _m: VecG,
pub    nr: usize,
pub    nc: usize,
pub    N: usize
}

impl Matrix {

    // NB: Given column by column
    pub fn new(nr:usize, nc:usize, v:&VecG) -> Matrix {
        let l = v.len();
        assert_eq!(nr*nc, l);
        Matrix {
             _m: v.clone(),
             nr: nr,
             nc: nc,
             N: nr*nc
        }
    }

    pub fn get(&self, i: usize, j:usize) -> G1 {
        let idx = self.nr*j + i;
        let range = 0..self.N;
        assert!(range.contains(&idx));
        self._m[idx]
    }

    fn get_col(&self, c: usize) -> VecG {
        self._m[self.nr*c..self.nr*(c+1)].to_vec()
    }
}


pub fn inner_product(v: &Vec<Fr>, w: &VecG) -> G1
{
    let mut res = G1::one();
    for i in 0..v.len() {
        let tmp = w[i]*v[i];
        res = res+tmp;
    }
    res
}

pub fn vector_matrix_mult(v: &Vec<Fr>, m:&Matrix, res: &mut VecG) {
    // the result should contain every column of m multiplied by v
    for c in 0..m.nc {
        res.push(inner_product(&v, &m.get_col(c)));
    }

}

pub fn scalar_vector_mult(a: &Fr, v: &Vec<Fr>, res: & mut Vec<Fr>)
{
    for i in 0..v.len() {
        res.push(*a * v[i]);
    }
}
// XXX: Make sure vectors are passed right without being copied

pub fn vec_to_G2(v: & Vec<Fr>) ->  Vec< G2>
{
    c![G2::one()*x, for x in v.clone()]
}
