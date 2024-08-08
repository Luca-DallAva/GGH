// August 2024
// Author: Luca Dall'Ava
//

extern crate nalgebra;

use nalgebra::{DMatrix, DVector};

// Public function for finding the linear decomposition of the vector wrt to given basis
// Need a control in case the matrix is not invertible, that is Option<DVector<f64>>

pub fn linear_decomposition(vector: &DVector<f64>, basis: &DMatrix<f64>) -> Option<DVector<f64>> {
    basis.clone().lu().solve(vector)
}

// Public function for computing the Hadamard ratio of the basis. We will need it to be close to 1 
// in order to have a good orthogonal basis
// We consider a basis as columns of a matrix

pub fn hadamard_ratio(basis: &DMatrix<f64>) -> f64 {
    let n = basis.nrows();
    let d = basis.determinant();

    let product_of_norms: f64 = (0..n)
        .map(|i| basis.column(i).norm()).product();

    (d.abs() / product_of_norms).powf(1.0 / n as f64)
}

pub fn babai_closest_vector(basis: &DMatrix<f64>, target: &DVector<f64>) -> Option<DVector<f64>> {
    if let Some(vect) = linear_decomposition(target, basis) {
        let hr = hadamard_ratio(&basis);
        if hr > 0.95 {
            // We transform the matrix and the vector into integer valued objects, then we ri transform them later on
            // we avoid some hustle with computations with floating points
            let coord_vect = vect.map(|x| x.round()).map(|x| x as i64);
            // let babai_cv = basis.map(|x| x as i64) * coord_vect;
            let mut babai_cv = DVector::<i64>::zeros(basis.ncols());
            for i in 0..basis.ncols() {
                babai_cv += coord_vect[i]*basis.column(i).map(|x| x as i64);
            }
            Some(babai_cv.map(|x| x as f64))
        } else {
            println!("Not orthogonal enough. Please change lattice basis.");
            None
        }
    } else {
        None
    }
}

// Testing function

// pub fn main() {
//     // Example vectors
//     let vector = DVector::from_vec(vec![1.0, 2.0, 3.0]);
//     let target = DVector::from_vec(vec![1.3, 2.5, 3.1]);

//     // // Example good basis
//     // let basis = DMatrix::from_vec(3, 3, vec![
//     //     1.0, 5.5, 0.0,
//     //     0.0, 3.0, 1.5,
//     //     0.5, 0.0, 1.0,
//     // ]);

//     // Example bad basis
//     let basis = DMatrix::from_vec(3, 3, vec![
//         1.0, 5.5, 0.0,
//         1.0, 3.5, 1.5,
//         1.0, 0.5, 1.5,
//     ]);

//     match linear_decomposition(&vector, &basis) {
//         Some(coefficients) => println!("Decomposition coefficients: {}", coefficients),
//         None => println!("The basis is not invertible."),
//     }

//     match babai_closest_vector(&basis, &target) {
//         Some(closest_vector) => println!("Closest vector: {}", closest_vector),
//         None => println!("Could not find the closest vector."),
//     }

//     println!("Hadamard ratio: {}", hadamard_ratio(&basis));
// }
