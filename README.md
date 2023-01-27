# Complex numbers "cnamerus" Rust library

### Features: 
1. Complex numbers are implemented for i64 and f64; 
1. The arithmetic operations, i.e. addition "+", subtraction "-", multiplication "*", and division "/" are implemented for i64 and f64. The division operation used on i64 complex numbers cast the result of the operation to f64; 
1. The Display trait for complex numbers implemented to display the complex numbers in the format u + i * v, e.g 4 - i * 2, or -2.67 + i * 3.4;  

### How to use this library 

1. Add to Cargo.toml file: 

```Toml
    [dependencies]
    cnamerus = {git = "https://github.com/azavgo/cnamerus", branch = "main"}
```
2. Create and print a complex number:  
```Rust
    use cnamerus::*;

    fn main(){
        let c = Complex::new(1, -2);
        println!("Complex number c = {}", c); 
        // Prints: Complex number c = 1 - i * 2
    }