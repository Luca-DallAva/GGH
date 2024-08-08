// August 2024
// Author: Luca Dall'Ava
//

extern crate nalgebra;
extern crate rand;

use nalgebra::{DMatrix};
use rand::Rng;
use rand::thread_rng;


// Public function for generating a random integer valued basis
pub fn random_basis(size: usize, parameter : i64) -> DMatrix<f64> {
    
    // Initiate random number generator as a mutable variable
    let mut rng = thread_rng();

    // we fill in the data field of a matrix and later we use it to produce the matrix
    // we use the .map() method to iterate and .collect() to define a collection out of it
    let data: Vec<i64> = (0..size * size)
        .map(|_| rng.gen_range(-parameter..=parameter)).collect();
    // We convert it to a floating point matrix
    let matrix = DMatrix::from_vec(size, size, data).map(|x| x as f64);
    
    matrix

}

// Testing function

// pub fn main() {

//     // called here to save resources
//     use std::io::{stdin, stdout, Write};

//     print!("Please insert the key dimension: ");
//     stdout().flush().unwrap(); // Flush the output buffer: that is, it prints the string immediately
//     let mut input_line = String::new();
//     stdin().read_line(&mut input_line).expect("Failed to read line");
//     let dim: usize = input_line.trim().parse().expect("Input not an integer");
    
//     print!("Please insert the parameter dimension: ");
//     stdout().flush().unwrap(); // Flush the output buffer: that is, it prints the string immediately
//     let mut input_line = String::new();
//     stdin().read_line(&mut input_line).expect("Failed to read line");
//     let par: usize = input_line.trim().parse().expect("Input not a number");

//     let matrix = random_basis(dim, par as i64);
//     println!("Random matrix:\n{}", matrix);
// }