use super::helpers;

pub fn run(){
    helpers::example_title(format!("Exmaple 7: Expressions"));
    println!("A Rust program is mostly a series of statements");
    println!("Statements in Rust include declaring a variable binding, and any expression that ends with a ;");
    println!("Blocks ({{}}) are expressions too - the last expression in the block will be returned");
    println!("If the last line ends with ; then () will be returned");

    let x = 5u32;
    let y = {
        let x_squared = x * x;
        let x_cubed = x_squared * x;
        x_cubed + x_squared + x
    };
    let z = { 2 * x; };
    println!("\nWith:");
    println!("\tlet x = 5u32;");
    println!("\tlet y = {{\n\t\tlet x_squared = x * x;\n\t\tlet x_cubed = x_squared * x;\n\t\tx_cubed + x_squared + x\n\t}};");
    println!("\tlet z = {{ 2 * x; }};");
    println!("x: {:?}, y: {:?}, z: {:?}", x, y, z);
}