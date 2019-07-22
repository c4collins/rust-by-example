use super::helpers;

fn casting() {
    #![allow(overflowing_literals)]
    helpers::section_title(format!("Casting"));
    println!("Rust doesn't provide type coercion, but you can do it explicitly using `as`");
    println!("C sometimes has undefined behaviour when casting; Rust does not.");

    let decimal = 65.4321_f32;
    let integer = decimal as u8;
    let character = integer as char;
    println!("\nWith:");
    println!("\tdecimal = 65.4321_f32");
    println!("\tinteger = decimal as u8");
    println!("\tcharacter = integer as char");
    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("\nWhen casting a value to an unsigned type, T, T::MAX + 1 is added or subtracted until it fits");
    println!("(If #![allow(overflowing_literals)] is set)");
    println!("1000 as a u16 is: {}", 1000 as u16);
    println!("1000 as a u8 is : {}", 1000 as u8);
    println!("  -1 as a u8 is : {}", (-1i8) as u8);
    println!(
        "same result as 2nd example =>\n1000 mod 256 is : {}",
        1000 % 256
    );

    println!("\nWhen casting to a signed type the bitwise result is the same as casting to the related unsigned type");
    println!("If the most significant bit of that value is 1 then the value is negative");
    println!(" 128 as a i16 is: {}", 128 as i16);
    println!(" 128 as a i8 is : {}", 128 as i8);
    println!("1000 as a u8 is : {}", 1000 as u8);
    println!(" 232 as a i8 is : {}", 232 as i8);
}

fn literals() {
    helpers::section_title(format!("Literals"));
    println!("Numeric literals can be type annotated by adding thetype as a suffix");
    println!("The type of unsuffixed literals will depend on how they are used (defaulting to i32 & f64)");

    let x = 1u8;
    let y = 2u32;
    let z = 3f32;
    let i = 1;
    let f = 2.0;

    println!("\nWith:");
    println!("\tx = 1u8");
    println!("\ty = 2u32");
    println!("\tz = 3f32");
    println!("\ti = 1");
    println!("\tf = 2.0");
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}

fn inference() {
    helpers::section_title(format!("Inference"));
    println!("The type inference engine is actually crazy good at its job");
    println!("It doesn't require you to declare a type if it can infer one from usage");

    let elem = 5u8;
    let mut vec = Vec::new();
    vec.push(elem);
    println!("\nWith:");
    println!("\telem = 5u8");
    println!("\tmut vec = Vec::new() <= the compiler complains here about vec being an unknown type (Vec<_>)");
    println!(
        "\tvec.push(elem); <= except with this push the compiler now knows it's type is Vec<u8>"
    );
    println!("vec = {:?}", vec);
}

fn aliasing() {
    helpers::section_title(format!("Aliasing"));
    println!("The `type` statement can be used to rename an existing type - but just an alias");
    println!("Aliases don't create new types, so they don't provide type safety");
    println!("Types require CamelCase names except for primitive types");

    type NanoSecond = u64;
    type Inch = u64;
    type U64T = u64;
    // Created 3 'new' types that all are u64-typed

    let nanoseconds: NanoSecond = 5 as U64T;
    let inches: Inch = 2 as U64T;

    println!(
        "{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches
    );

    println!("This is mostly used to reduce boilerplate so check the code if you want to see how it works");
}

pub fn run() {
    helpers::example_title(format!("Example {}: {}", 5, "Types"));
    println!("Rust provides several ways to define or change the type of primitive and user types");

    casting();
    literals();
    inference();
    aliasing();
}
