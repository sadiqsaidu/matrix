// cholesky factorization algorithm
fn cholesky_factorization(a: &Vec<Vec<f64>>) -> Option<Vec<Vec<f64>>> {
    let n = a.len();
    let mut l = vec![vec![0.0; n]; n];

    for i in 0..n {
        for j in 0..(i+1) {
            let mut s = 0.0;
            for k in 0..j {
                s += l[i][k] * l[j][k];
            }
            if i == j {
                l[i][j] = (a[i][j] - s).sqrt();
            } else {
                l[i][j] = (a[i][j] - s) / l[j][j];
            }
            if l[j][j] <= 0.0 {
                return None;
            }
        }
    }

    Some(l)
}

fn main() {
    let a = vec![
        vec![4.0, 12.0, -16.0],
        vec![12.0, 37.0, -43.0],
        vec![-16.0, -43.0, 98.0],
    ];


    match cholesky_factorization(&a) {
        Some(l) => println!("Cholesky factor:\n{:?}", l),
        None => println!("Matrix is not positive definite."),
    }
}