use super::helpers;

pub fn run() {
    // This is an exmaple of a single-line comment
    // Each line comment starts with //
    helpers::example_title(format!("Example {}: {}", 1, "Hello World"));
    println!("Hello World!");
    println!("I'm a Rustacean!");
    /*
    * This is a block comment, by convention/for style the lines for a block
    * comment start with a *
    (but it's not actually necessary)
    */
    // Notice how block comments can be used within an expression
    let x = 5 + /* 90 + */ 5;
    println!("\nIs x 100 or 10? x = {}", x);

    helpers::section_title(format!(
        "Print with a series of macros in std::fmt, including:"
    ));
    println!("format! writes formatted text to String");
    print!("print! does the same as format! but also prints to stdout\n");
    eprint!("eprint! does the same as format! but also prints to stderr\n");
    println!("println! does the same as print! but appends a newline (\\n)");
    eprintln!("eprintln! does the same as eprint! but appends a newline (\\n)");

    helpers::section_title(format!("Formatting:"));
    println!("Use {{}} to include variables");
    println!("{{}} days in December -> {} days in December", 31);
    eprintln!(" (When the arguments are ..., 31)");

    println!("There are a lot of formatting patterns than can be used");

    println!("\n\t1. You can number the arguments positionally like {{0}} and {{1}}:");
    print!("{{0}}, this is {{1}}. {{1}}, meet {{0}}. Have fun! -> ");
    println!("{0}, this is {1}. {1}, meet {0}. Have fun!", "Alice", "Bob");
    eprintln!(" (When the arguments are ..., \"Alice\", \"Bob\")");

    println!("\n\t2. You can name arguments like {{subject}} and {{index}}:");
    print!("{{subject}} {{verb}} {{object}} -> ");
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );
    eprintln!(" (When the arguments are ...,");
    eprintln!("\t\tobject = \"the lazy dog\",");
    eprintln!("\t\tsubject = \"the quick brown fox\",");
    eprintln!("\t\tverb = \"jumps over\"");
    eprintln!(" )");

    println!("\n\t3. Special formatting can be specified after a :, like {{:b}}");
    println!("\tSpecifically:");
    println!("\t\tnone - Display");
    println!("\t\t? - Debug");
    println!("\t\tx? - Debug with lowercase hexidecimal integers");
    println!("\t\tX? - Debug with uppercase hexidecimal integers");
    println!("\t\to - octal");
    println!("\t\tx - LowerHex");
    println!("\t\tX - UpperHex");
    println!("\t\tp - Pointer");
    println!("\t\tb - Binary");
    println!("\t\te - LowerExp");
    println!("\t\tE - UpperExp");

    println!("\n\t3a. Number formats can be specified, like {{:b}} for binary:");
    print!("{{}} of {{:b}} people know binary, the other half doesn't -> ");
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    eprintln!(" (When the arguments are ..., 1, 2)");
    print!("{{}} of {{:o}} people know octal, the other 90% don't -> ");
    println!("{} of {:o} people know octal, the other 90% don't", 1, 10);
    eprintln!(" (When the arguments are ..., 1, 10)");
    print!("{{}} of {{:x}} people know hex, the other 99.96% don't -> ");
    println!("{} of {:x} people know hex, the other 99.96% don't", 1, 3199);
    eprintln!(" (When the arguments are ..., 1, 3199)");
    print!("{{}} of {{:X}} people know hex, the other 96.77% don't -> ");
    println!("{} of {:X} people know hex, the other 96.77% don't", 1, 31);
    eprintln!(" (When the arguments are ..., 1, 31)");
    print!("{{:?}} -> ");
    println!("{:?}", (1, 2, 3, 4));
    eprintln!(" (When the arguments are ..., (1, 2, 3, 4))");

    println!("\n\t3b. Align text/numbers with < ^ >, like {{value:>width$}}");
    print!("{{value:<width$}} -> ");
    println!("\"{value:<width$}\"", value = 31, width = 10);
    eprintln!(" (When the arguments are ...,");
    eprintln!("\t\tvalue = 31,");
    eprintln!("\t\twidth = 10,");
    eprintln!(" )");
    print!("{{0:^1$}} -> ");
    println!("\"{0:^1$}\"", 31, 10);
    eprintln!(" (When the arguments are ..., 31, 10)");
    print!("{{0:>10}} -> ");
    println!("\"{0:>10}\"", 31);
    eprintln!(" (When the arguments are ..., 31)");

    println!("\n\t3c. You can pad numbers with zeroes, or anything with anything, really");
    print!("{{0:0>1$}} -> ");
    println!("\"{0:0>1$}\"", 31, 10);
    eprintln!(" (When the arguments are ..., 31)");
    print!("{{0:*^7}} -> ");
    println!("\"{0:*^7}\"", "fig");
    eprintln!(" (When the arguments are ..., \"fig\")");
    print!("{{0:-<9}} -> ");
    println!("\"{0:-<9}\"", "fig");
    eprintln!(" (When the arguments are ..., \"fig\")");

    println!("\n\t4. Rust will ensure the correct number of arguments are needed.");
    print!("println!(\"My name is {{0}}, {{1}} {{0}}\", \"Bond\"); -> ");
    println!("error: invalid reference to positional argument 1 (there is 1 argument)");

    println!("\n\t5. Custom types like structs won't print.");
    // struct Structure(i32);
    // println!("This struct '{}' won't print...", Structure(3));

    println!("\n\t6. You can truncate decimals");
    let pi = 3.1415926;
    print!("Pi is roughly {{:.*}} -> ");
    println!("Pi is roughly {:.*}", 3, pi);
    eprintln!(" (When the arguments are ..., 3, {pi})", pi = pi);
}