// August 2024
// Author: Luca Dall'Ava
//
// Toy implementation of the GGH protocol as in 
// Hoffstein, Pipher, Silverman - An introduction to mathematical cryptography (2nd),
// §7.8 The GGH Public Key Cryptosystem.

// Known issues: floating point arithmetic in nalgebra is tricky, there are tons of errors that might add up
// ideally you would like to use a message with (at least) double digit entries

// A few comments on the code as I learn while coding:
// - borrowing = &name_of_var: allows you to use a reference to a value rather than owning the value itself. 
// This means you can access and use the data without taking responsibility for its lifecycle. Rust enforces
// borrowing rules to ensure that references are always valid and that data is not modified unexpectedly.
// .clone() istead gives you a copy which can be modified

extern crate nalgebra;
extern crate rand;

use nalgebra::{DMatrix, DVector};
use rand::Rng;
use rand::thread_rng;
use std::io::{stdin, stdout, Write};
use std::io;

// Call the implemented module
mod rand_unitary_zzmatrix;
mod babai;
mod rand_basis;

// Recursive public function for computing a well-behaved basis
pub fn good_basis(size: usize, parameter: i64) -> DMatrix<f64> {
    let basis = rand_basis::random_basis(size, parameter);

    let hr = babai::hadamard_ratio(&basis);
    if hr > 0.95 {
        // we require that the Hadamard ratio is close to 1
        basis
    } else {
        // otherwise we consider a different matrix
        good_basis(size, parameter)
    }
}

// Initialize protocol function
// We define ggh_encrypt and ggh_decrypt as closures that capture the public_key, basis, and delta_param variables from their environment.
// This allows them to be used outside the init function while still having access to these variables.
// See also https://doc.rust-lang.org/book/ch13-01-closures.html
pub fn init(size: usize, parameter: i64, delta_param: i64) -> (DMatrix<f64>, impl Fn(DVector<f64>) -> DVector<f64>, impl Fn(DVector<f64>) -> DVector<f64>) {
    let g_basis = good_basis(size, parameter);

    // Printing Tests
    // println!("Basis:\n{}", g_basis);
    // println!("Basis Hadamard ratio\n{}", babai::hadamard_ratio(&g_basis));

    let mut unitary: DMatrix<f64> = DMatrix::identity(size, size);
    let delta_iteration: i64 = 8 - 1;
    let mut counter = 0;

    // we take a product of unitary matrices
    // we force the determinant to be +-1 in order to avoid floating point errors. Something to improve on
    while counter <= delta_iteration {
        // We convert it to a floating point matrix
        let unit_rand = rand_unitary_zzmatrix::random_unitary(size).map(|x| x as f64);
        let unit_temp = unitary.clone() * unit_rand;
        if unit_temp.determinant().abs() == 1.0  {
            unitary = unit_temp;
            counter += 1;
        }
    }
    // println!("det:\n{}", &unitary.determinant());
    // println!("Unitary matrix:\n{}", &unitary);

    // We convert it to a floating point matrix
    let public_key: DMatrix<f64> = g_basis.clone() * unitary.clone();

    // We overwrite the unitary matrix with zeros
    unitary.fill(0.0);

    // public_key is moved into the closure and then used again. We need to clone public_key before moving it into the closure.
    // We need to do it twice, one time for each closure which requires to handle it
    let public_key_clone_for_encrypt = public_key.clone();
    let public_key_clone_for_decrypt = public_key.clone();
    
    // Encryption function
    let ggh_encrypt = move |message: DVector<f64>| -> DVector<f64> {
        let dim = public_key_clone_for_encrypt.ncols();

        // Initiate random number generator as a mutable variable
        let mut rng = thread_rng();

        // Fill in the data field of a vector and use it to produce it
        let data: Vec<i64> = (0..dim)
            .map(|_| rng.gen_range(-delta_param..=delta_param))
            .collect();
        
        
                                                                            // We convert it to a floating vector
        let e: DVector<f64> = public_key_clone_for_encrypt.clone() * message + DVector::from_vec(data).map(|x| x as f64);

        e
    };

    // Decryption function
    let ggh_decrypt = move |enc_message: DVector<f64>| -> DVector<f64> {
        // Compute closest vector to e in the lattice
        // cvp_e and .try_inverse() are OptionMatrix/Vector, so we need to unwrap them before multiplying
        // using unwrap can cause the program to panic if the value is None, therefore one must use unwrap_or and provide a return value
        // We need to clone g_basis so that we do not modify it at each iteration
        let g_basis_clone = g_basis.clone();
        
        let cvp_e = babai::babai_closest_vector(&g_basis_clone, &enc_message).unwrap_or(DVector::zeros(enc_message.len()));
        //println!("CVP:\n{}", cvp_e);

        // Decription by inversion of matrix
        // let decrypted_e: DVector<f64> = public_key_clone_for_decrypt.clone().try_inverse().unwrap_or(DMatrix::zeros(public_key_clone_for_decrypt.nrows(), public_key_clone_for_decrypt.ncols())) * cvp_e.clone();
        // println!("INverse matrix:\n{}",  public_key_clone_for_decrypt.clone().try_inverse().unwrap_or(DMatrix::zeros(public_key_clone_for_decrypt.nrows(),public_key_clone_for_decrypt.nrows())));
        let decrypted_e: DVector<f64> = babai::linear_decomposition(&cvp_e,&public_key_clone_for_decrypt).unwrap();
        
        // artificially rounded to avoid floating point errors due to nalgebra and f64.
        decrypted_e.map(|x| x.round())
    };

    (public_key, ggh_encrypt, ggh_decrypt)
}

// Main function for testing
fn main() {
    
    print!("Please insert the key dimension: ");
    stdout().flush().unwrap(); // Flush the output buffer: that is, it prints the string immediately
    let mut input_line = String::new();
    stdin().read_line(&mut input_line).expect("Failed to read line");
    let dim: usize = input_line.trim().parse().expect("Input not an integer");

    // print!("Please insert the parameter dimension: ");
    // stdout().flush().unwrap(); // Flush the output buffer: that is, it prints the string immediately
    // let mut input_line = String::new();
    // stdin().read_line(&mut input_line).expect("Failed to read line");
    // let par: usize = input_line.trim().parse().expect("Input not a number");
    let par: usize = 2*dim + 10;

    // Ask for a vector-message
    print!("Enter the integer elements of the vector separated by spaces: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input into a vector of f64
    let elements: Vec<f64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not a valid number"))
        .collect();

    // Convert the vector into a DVector<i64>
    let message =  DVector::from_vec(elements);

    let (public_key, ggh_encrypt, ggh_decrypt) = init(dim, par as i64, 2);

    println!("Public key\n{}", public_key);
    println!("Public key Hadamard ratio\n{}", babai::hadamard_ratio(&public_key));

    // I do everything in order so that I don't have to clone anything
    println!("Original message: {}", message);
    
    let encrypted_message = ggh_encrypt(message.clone());
    println!("Encrypted message: {}", encrypted_message);
    let decrypted_message = ggh_decrypt(encrypted_message);
    println!("Decrypted message: {}", decrypted_message);
    
}
