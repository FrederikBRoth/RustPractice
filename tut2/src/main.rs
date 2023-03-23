use std::fmt;
mod functions;

pub use crate::functions::functions_module;
const TEST_CONSTANT: u32 = 200;
fn main() {
    let mut x = 5;
    println!("The value of x is : {x}");
    x = 6;
    println!("The value of x is : {x}");
    println!("The value of the constant is : {TEST_CONSTANT}");

    //Shadowing example
    let y = 5;
    println!("{y}");
    let y = y + 5;

    println!("Shadowed Y : {y}");
    //Shadowed variable in encapsulation
    {
        let y = y + 10;

        println!("{y}");
    }
    println!("{y}");
    datatypes();
    //Functions from Function chapter in a module lmaoo
    functions_module::test();
    functions_module::parameter(39);
}

struct Matrix2D(i32, i32, i32, i32);

impl fmt::Display for Matrix2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(({}) ({})\n ({})({}))", self.0, self.1, self.2, self.3)
    }
}
fn datatypes() {
    println!("Data type examples: ");
    let int: u32 = 20;
    println!("Integers: {int}");
    let float: f32 = 3.0;
    println!("Float: {float}");
    let matrix = Matrix2D(2, 2, 2, 2);
    println!("Matrix struct: \n {matrix}");
    let tup: (i32, f64, u8) = (600, 3.5, 8);
    println!("Tuple variable X: {}", tup.0);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Arrays: {},{}", a[0], a[1]);
}
