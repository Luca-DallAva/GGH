# GGH
Toy implementation in Rust of the GGH protocol as in [Hoffstein, Pipher, Silverman - An introduction to mathematical cryptography (2nd)](https://link.springer.com/book/10.1007/978-0-387-77993-5), §7.8 The GGH Public Key Cryptosystem.

## Guide:
All the code is based on the ['nalgebra'](https://docs.rs/nalgebra/latest/nalgebra/) crate. Moreover,
- 'babai.rs' contains an implementation of Babai's algorithm as well as Hadamard ratio (for the basis consists of columns of a matrix);
- 'rand_basis.rs' produces a random basis of ZZ^n;
- 'rand_unitary_zzmatrix.rs' contains an implementation for random unitary matrices with integer entries. Notice that we need some control on the determinant as floating point arithmetic seems to be annoying with ['nalgebra'](https://docs.rs/nalgebra/latest/nalgebra/);
- 'main.rs' contains the whole implementation, consisting of an initialization function and two closures: ggh_encrypt and ggh_decrypt (see https://doc.rust-lang.org/book/ch13-01-closures.html). 

## Known issues:
- floating point arithmetic in ['nalgebra'](https://docs.rs/nalgebra/latest/nalgebra/) is tricky, as there are tons of errors that might add up. Ideally, you would like to use a message with (at least) double-digit entries;
- the implementation is meant as a toy example, and it lacks the optimization and security that one might want for a real-world application.
