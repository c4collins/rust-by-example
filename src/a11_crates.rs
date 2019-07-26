use super::helpers;

pub fn run () {
    helpers::example_title(format!("Example 11: Crates"));
    println!("A crate is a compilation unit in Rust.");
    println!("Along the same lines as Python libraries and JavaScript packages.");

    println!("\nWhen a rust program is compiled by `rustc`, the base file given to the compiler is treated as the crate file");
    println!("If that file has `mod` declarations in it, the module contents will be placed where the declarations are, before compilation");
    println!("This means that modules are never compiled on their own, only as complete crates");

    println!("\nA crate can be compiled into a binary [default] or a library (exe or dll for anyone older than me)");

    libraries();
}

fn libraries () {
    helpers::section_title(format!("Crate as Library and `extern crate`"));
    println!("Liraries are declared in Cargo.toml and autoimported into the global scope");
    println!("\nThe same visibility rules that apply to modules apply to libraries");

    rary::public_function();
    // rary::private_function(); // private
    rary::indirect_access();

    println!("\nNote: Rust by Example is a bit out of date here, but the learning was useful");
}