// August 2024
// Author: Luca Dall'Ava
//
// I consider integer matrices

extern crate nalgebra;
extern crate rand;

use nalgebra::DMatrix;
use rand::Rng;
use rand::thread_rng;


// Function to multiply a row by -1
// public function
pub fn multiply_row_by_minus_one(mat: &mut DMatrix<i64>, row: usize) {
    for j in 0..mat.ncols() {
        mat[(row, j)] *= -1;
    }
}

// Function to replace a row r by r + mult*r', for r' another row and mult a constant
// public function
pub fn modify_row(matrix: &mut DMatrix<i64>, target_row: usize, source_row: usize, multiplier: i64) {
    // Clone the source row to avoid borrowing issues
    let source = matrix.row(source_row).clone_owned();
    let mut target = matrix.row_mut(target_row);
    target += source * multiplier;
}

// Generate a random unitary matrix
// public function
pub fn random_unitary(size: usize) -> DMatrix<i64> {
    
    // Initiate random number generator as a mutable variable
    let mut rng = thread_rng();

    // Start with the identity matrix
    let mut matrix = DMatrix::identity(size, size);

    // Shuffle rows to create random permutation matrix
    for i in 0..size {
        let j = rng.gen_range(0..size);
        matrix.swap_rows(i, j);
    }

    // Shuffle columns to create random permutation matrix
    for i in 0..size {
        let j = rng.gen_range(0..size);
        matrix.swap_columns(i, j);
    }

    // Randomly flip signs of rows
    for i in 0..size {
        // Randomly decide if we multiply or not
        if rng.gen_bool(0.5) {
            // Multiply row i by -1
            multiply_row_by_minus_one(&mut matrix, i);
        }
    }

    // Modify rows randomly
    for i in 0..size {
        // Randomly decide the row 
        let j = rng.gen_range(0..=size-1);
        if  j!=i {
            // Find a multiplier
            let mult = rng.gen_range(size..=5*size);
            modify_row(&mut matrix, i, j, mult as i64);
        }
    }


    //We need a control in order to avoid floating point errors. This is unfortunate, and something that can be improved on
    if matrix.map(|x| x as f64).determinant().abs() == 1.0 {
        matrix
    } else {
        random_unitary(size)
    }
    
}

// Main procedure for testing
// public function

// pub fn main() {
//     // called here to save resources
//     use std::io::{stdin, stdout, Write};

//     print!("Please insert the key dimension: ");
//     stdout().flush().unwrap(); // Flush the output buffer: that is, it prints the string immediately
//     let mut input_line = String::new();
//     stdin().read_line(&mut input_line).expect("Failed to read line");
//     let size: usize = input_line.trim().parse().expect("Input not an integer");
    
//     // Generate a random unitary matrix with integer coefficients
//     let random_unitary_matrix = random_unitary(size);
//     println!("Random Unitary Matrix:");
//     println!("Matrix:\n{}", random_unitary_matrix);
//     // need to tranform it into floating point matrix to apply the determinant
//     println!("\nIs the matrix unitary? {}", random_unitary_matrix.clone().map(|x| x as f64).determinant());
// }

