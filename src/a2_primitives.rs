use super::helpers;
use std::fmt;
use std::mem;
use typename::TypeName;

fn scalar_types() {
    helpers::section_title(format!("Scalar Types"));

    println!("i8   goes from {} to {}", std::i8::MIN, std::i8::MAX);
    println!("i16  goes from {} to {}", std::i16::MIN, std::i16::MAX);
    println!("i32  goes from {} to {}", std::i32::MIN, std::i32::MAX);
    println!("i64  goes from {} to {}", std::i64::MIN, std::i64::MAX);
    println!("i128 goes from {} to {}", std::i128::MIN, std::i128::MAX);

    println!("u8   goes from {} to {}", std::u8::MIN, std::u8::MAX);
    println!("u16  goes from {} to {}", std::u16::MIN, std::u16::MAX);
    println!("u32  goes from {} to {}", std::u32::MIN, std::u32::MAX);
    println!("u64  goes from {} to {}", std::u64::MIN, std::u64::MAX);
    println!("u128 goes from {} to {}", std::u128::MIN, std::u128::MAX);

    println!(
        "f32  goes from {:.1} to {:.1}",
        std::f32::MIN,
        std::f32::MAX
    );
    println!(
        "f64  goes from {:.1} to {:.1}",
        std::f64::MIN,
        std::f64::MAX
    );

    println!("char represents scalar values at 4-bytes per 'a' 'α' or '∞'");
    println!("single-quotes ('a') are chars, double quotes \"abc\" are strings");
    println!("bool values are either true or false");
    println!("\ttrue: {}", true);
    println!("\tfalse: {}", false);

    println!(
        "And the unit type () which can only be an empty tuple:{:?}",
        ()
    );
}

fn compound_types() {
    helpers::section_title(format!("Compound Types"));
    println!("Arrays like {:?}", [1, 2, 3]);
    println!("Tuples like {:?}", (1, true));
}

fn variable_annotation() {
    helpers::section_title(format!("Variable Annotation"));
    println!("Rust tries to infer the variable type from defaults or context");
    println!("Integers default to i32, floats default to f64");
    let example_integer = 12;
    let example_float = 1.2;
    println!(
        "integer: {}:{}, float: {}:{}",
        example_integer,
        example_integer.type_name_of(),
        example_float,
        example_float.type_name_of()
    );

    println!("\nYou can use 'regular' notation after the declared variable name:");
    println!(
        "let logical:bool = true -> {}:{}",
        true,
        true.type_name_of()
    );
    let a_float: f32 = 235.25;
    println!(
        "let a_float:f32 = 235.25 -> {}:{}",
        a_float,
        a_float.type_name_of()
    );
    println!("You can use suffix annotation after the variable value:");
    println!("let an_integer = 5u8 -> {}:{}", 5u8, 5u8.type_name_of());
    println!("let an_integer = 5i64 -> {}:{}", 5i64, 5i64.type_name_of());

    println!("\nA type can be inferred if not declared and they types basically match:");
    let default = 12;
    let mut inferred = 12;
    println!(
        "default = {}:{}, inferred = {}:{}",
        default,
        default.type_name_of(),
        inferred,
        inferred.type_name_of()
    );
    inferred = std::i8::MIN;
    println!(
        "because inferred is later set to {}:{}",
        inferred,
        inferred.type_name_of()
    );

    let default = 12;
    let mut inferred = 12;
    println!(
        "default = {}:{}, inferred = {}:{}",
        default,
        default.type_name_of(),
        inferred,
        inferred.type_name_of()
    );
    inferred = std::u8::MIN;
    println!(
        "because inferred is later set to {}:{}",
        inferred,
        inferred.type_name_of()
    );

    let default = 1.2;
    let mut inferred = 1.2;
    println!(
        "default = {}:{}, inferred = {}:{}",
        default,
        default.type_name_of(),
        inferred,
        inferred.type_name_of()
    );
    inferred = std::f32::MIN;
    println!(
        "because inferred is later set to {}:{}",
        inferred,
        inferred.type_name_of()
    );

    println!("\nVariables that change must be declared as mutable with 'let mut'");
    println!("Basically the same idea as let/const in JavaScript, just different expression");
    println!("Unlike JavaScript, you can shadow non-mutables by redeclaring them i.e. let x = 1.2; let x = 12;");
    println!("The previous value type isn't inferred from a shadowed declaration");
    println!("So let mut x = 1; x = true; will error, but let x = 1; let x = true; will not")
}

fn literals() {
    helpers::section_title(format!("Literals"));
    println!("Integers, floats, chars, strings, booleans, and the unit type can be expressed as literals");

    println!("\tintegers: 12 -> {}", 12);
    println!("\tfloats: 1.2 -> {}", 1.2);
    println!("\tchars: 'a' -> {}", 'a');
    println!("\tstrings: \"abc\" -> {}", "abc");
    println!("\tbooleans: true -> {}", true);
    println!("\tunit type: () -> {:?}", ());

    println!("\nIntegers can also be expressed using base-specific notation:");
    println!("\t0x12 -> {}", 0x12);
    println!("\t0o12 -> {}", 0o12);
    println!("\t0b11 -> {}", 0b11);

    println!("\nUnderscores can be used to improve readability:");
    println!("\t1_000 -> {}", 1_000);
    println!("\t1_000_000 -> {}", 1_000_000);
    println!("\t0.000_1 -> {}", 0.000_1);
    println!("\t0.000_001 -> {}", 0.000_001);
}

fn operators() {
    helpers::section_title(format!("Operators"));
    println!("Operators and perator precedence in Rust is similar to other C-like languages");
    println!("Unlike the other languages I'm used to types MUST match");
    println!("\t+ addition: 1.1 + 2.0 = {}", 1.1 + 2.0);
    println!("\t- subtraction: 1 - 2 = {}", 1 - 2);
    println!("\t* multiplication: 14f32 * 2.5 = {}", 14f32 * 2.5);
    println!("\t/ division: 5 / 2 = {}", 5 / 2);
    println!("\t% modulo: 5f64 % 2.4 = {}", 5f64 % 2.4);

    println!("\nBoolean operators:");
    println!("\t&& AND: true && false = {}", true && false);
    println!("\t|| OR: true || false = {}", true || false);
    println!("\t! NOT: !true = {}", !true);

    println!("\nBitwise operators:");
    println!("\t& bitwise AND: 0011 & 0101 = {:04b}", 0b0011 & 0b0101);
    println!("\t| bitwise OR: 0011 | 0101 = {:04b}", 0b0011 | 0b0101);
    println!("\t^ bitwise XOR: 0011 ^ 0101 = {:04b}", 0b0011 ^ 0b0101);
    println!("\t<< left-shift: 1 << 5 = {}", 1 << 5);
    println!("\t>> right-shift: 0x80 >> 2 = 0x{:x}", 0x80 >> 2);

    println!("\nThe usual comparison operators (==, !=, <, >, <=, >=)");
    println!("And assignment operators (=, +=, -=, *=, /=, %=, &=, |=, ^=, <<=, >>=)");
}

fn tuples() {
    helpers::section_title(format!("Tuples"));
    println!("A tuple is a collection of values of differnt types.");
    println!("Tuples are created using parentheses ()");
    println!("Each tuple is a value with a type signature (T1, T2, ...) were T1, T2 are the types of its members");
    println!("Function can use tuples to return multiple values");

    let long_tuple = (
        1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true,
    );
    println!("\nValues can be extracted from tuples using tuple indexing:");
    println!("\tif long_tuple = {:?};", long_tuple);
    println!("\tlong_tuple.0; => {}", long_tuple.0);
    println!("\tlong_tuple.6; => {}", long_tuple.6);

    let tuple_of_tuples = ((2i64, 3i8, -4i16, (-5f32, true, 'a')), (1i32, false));
    println!("\nTuples can be tuple members, and can be printed with Debug {{:?}} if they have <= 12 members:");
    println!("\t{:?}", tuple_of_tuples);

    println!("\nTo create a single-element tuple, you must include the comma");
    println!("\tone element tuple: (5,) => {:?}", (5,));
    println!("\tjust an integer: (5) => {:?}", (5));

    fn reverse(pair: (i32, bool)) -> (bool, i32) {
        // let can be used deconstructively on tuples
        let (integer, boolean) = pair;

        (boolean, integer)
    }
    let pair = (25, true);
    println!("\nlet can be used deconstructively on tuples like let (integer, boolean) = pair;");
    println!("\tpair = {:?}; reverse(pair) => {:?}", pair, reverse(pair));

    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("\nA struct that is a tuple can be treated like a tuple");
    println!("\tmatrix => {:?}", matrix);
    println!("\tmatrix.0 => {:?}", matrix.0);

    impl fmt::Display for Matrix {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "\t( {:.1} {:.1} )\n\t( {:.1} {:.1} )",
                self.0, self.1, self.2, self.3
            )
        }
    }
    println!("But it's better to implement std::fmt::Display for better formatting");
    println!("Matrix:\n{}", matrix);
    fn transpose(matrix: Matrix) -> Matrix {
        Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
    }
    println!("Transpose:\n{}", transpose(matrix));
}

fn arrays_and_slices () {
    helpers::section_title(format!("Arrays and Slices"));
    println!("Arrays are collections of objects of the same type, and are initialized with square brackets []");
    println!("Their size, which is known at compile time, is part of their type signature [T; size]");

    println!("\nSlices are similar to arrays, except their size is not known at compile time");
    println!("A slice is a reference to a length of another piece of data");
    println!("The first parameter is a pointer to the data, and the second parameter is the length of the slice");
    println!("Slices can be used to borrow sections of an array and have the type signature &[T]");
    
    let xs:[i32; 5] = [1, 2, 3, 4, 5]; // Fixed size array
    println!("\nWith a fixed size array xs:[i32; 5] = [1, 2, 3, 4, 5]:");
    println!("\tfirst element of the array: xs[0] => {}", xs[0]);
    println!("\tsecond element of the array: xs[1] => {}", xs[1]);
    println!("\tsize of the array: xs.len() => {}", xs.len());
    println!("\tarray occupies {} bytes on the stack", mem::size_of_val(&xs));
    
    fn analyze_slice(slice: &[i32]){
        println!("\tWith a borrowed slice slice:&[i32] = {:?}:", slice);
        println!("\tfirst element of the slice: slice[0] => {}", slice[0]);
        println!("\tsecond element of the slice: slice[1] => {}", slice[1]);
        println!("\tsize of the slice: slice.len() => {}", slice.len());
        println!("\tslice occupies {} bytes on the stack", mem::size_of_val(slice));
    }

    println!("\nYou can borrow a whole array as a slice:");
    analyze_slice(&xs);

    let ys:[i32; 500] = [0; 500]; // Elements initialized to the same value
    println!("\nWith a array  of elements with the same initial value ys:[i32; 500] = [0; 500]:");
    println!("\nYou can borrow a section of the array as a slice:");
    analyze_slice(&ys[35 .. 58]);
    
    println!("\nUsing an out-of-bound index will cause a compile error");
}

pub fn run() {
    helpers::example_title(format!("Example {}: {}", 2, "Primitives"));
    scalar_types();
    compound_types();
    variable_annotation();
    literals();
    operators();
    tuples();
    arrays_and_slices();
}
