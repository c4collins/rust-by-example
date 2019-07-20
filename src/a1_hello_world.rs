use super::helpers;
use std::fmt::{self, Display, Formatter};

fn macros() {
    helpers::section_title(format!("Macros"));
    println!("Print with a series of macros in std::fmt, including:");
    println!("format! writes formatted text to String");
    print!("print! does the same as format! but also prints to stdout\n");
    eprint!("eprint! does the same as format! but also prints to stderr\n");
    println!("println! does the same as print! but appends a newline (\\n)");
    eprintln!("eprintln! does the same as eprint! but appends a newline (\\n)");
}

fn formatting() {
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
    println!(
        "{} of {:b} people know binary, the other half doesn't",
        1, 2
    );
    eprintln!(" (When the arguments are ..., 1, 2)");
    print!("{{}} of {{:o}} people know octal, the other 90% don't -> ");
    println!("{} of {:o} people know octal, the other 90% don't", 1, 10);
    eprintln!(" (When the arguments are ..., 1, 10)");
    print!("{{}} of {{:x}} people know hex, the other 99.96% don't -> ");
    println!(
        "{} of {:x} people know hex, the other 99.96% don't",
        1, 3199
    );
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

    println!("\n\t5. Custom types like structs won't print. See Debug for how to print them.");
    // struct Structure(i32);
    // println!("This struct '{}' won't print...", Structure(3));

    println!("\n\t6. You can truncate decimals");
    let pi = 3.1415926;
    print!("Pi is roughly {{:.*}} -> ");
    println!("Pi is roughly {:.*}", 3, pi);
    eprintln!(" (When the arguments are ..., 3, {pi})", pi = pi);
}

fn debug() {
    helpers::section_title(format!("Debug"));
    println!("To use the std::fmt traits they need to be implemented on the type");
    println!("Automatic impelementations are provided for types from std, but others must be manually implemented.");
    println!("\nThe fmt::Debug trait can derive a fmt::Debug implementation for all types:");

    #[derive(Debug)]
    struct Structure(i32);

    #[derive(Debug)]
    struct Deep(Structure);

    println!("Now {:?} will print", Structure(3));
    println!("Now {:?} will print", Deep(Structure(7)));

    println!("\nYou can use Debug {{:?}} formatting to print any of the std types too (I don't show this here)");
    println!("\nRust can also pretty-print with the built in formatter {{:#?}}");

    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    };

    let name = "Connor";
    let age = 32;
    let person = Person { name, age };

    println!("{:#?}", person);

    println!("You can manually implement fmt::Display if you need to.");
}

fn display() {
    helpers::section_title(format!("Display"));
    println!("fmt::Debug looks ugly");

    #[derive(Debug)]
    struct MinMax(i64, i64);
    impl fmt::Display for MinMax {
        // This trait requires 'fmt' with this exact signature
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Write strictly the first element into the supplied output
            // stream: `f`. Returns `fmt::Result` which indicates whether the
            // operation succeeded or failed. Note that `write!` uses syntax which
            // is very similar to `println!` and `format!`, but it will write the
            // formatted string into a buffer (the first argument)
            write!(f, "({}, {})", self.0, self.1)
        }
    }

    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }
    impl fmt::Display for Point2D {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }

    #[derive(Debug)]
    struct Complex {
        real: f64,
        imag: f64,
    }
    impl fmt::Display for Complex {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "{real} + {imaginary}i",
                real = self.real,
                imaginary = self.imag
            )
        }
    }

    let minmax = MinMax(0, 14);
    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    let point = Point2D { x: 3.3, y: 7.2 };
    let complex_number = Complex {
        real: 3.3,
        imag: 7.2,
    };

    println!("Compare Structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    println!(
        "Display: The big range is {big} and the small is {small}",
        small = small_range,
        big = big_range
    );
    println!(
        "Debug: The big range is {:?} and the small is {:?}",
        big_range, small_range,
    );

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    println!("Compare complex number:");
    println!("Display: {}", complex_number);
    println!("Debug: {:?}", complex_number);
}

fn list() {
    helpers::section_title(format!("Testcase: List"));
    println!("Each write! generates a fmt::Result, but they can be chained together with a ?");

    struct List(Vec<i32>);
    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let vec = &self.0;
            write!(f, "[")?;
            for (count, v) in vec.iter().enumerate() {
                if count != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}: {}", count, v)?;
            }
            write!(f, "]")
        }
    }

    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}

fn formatting_traits() {
    helpers::section_title(format!("Formatting Traits"));
    println!("The same value can be formatted different ways depending on which argument is used");

    let foo: i64 = 3735928559;
    println!("{{}}, foo -> {}", foo);
    println!("0x{{:X}}, foo -> 0x{:X}", foo);
    println!("0o{{:o}}, foo -> 0o{:o}", foo);

    struct City {
        name: &'static str,
        lat: f32,
        lon: f32,
    }

    impl Display for City {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
            let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

            write!(
                f,
                "{}: {:.3}°{} {:.3}°{}",
                self.name,
                self.lat.abs(),
                lat_c,
                self.lon.abs(),
                lon_c
            )
        }
    }

    println!("\nCities:");
    for city in [
        City {
            name: "Dublin",
            lat: 53.347778,
            lon: -6.259722,
        },
        City {
            name: "Oslo",
            lat: 59.95,
            lon: 10.75,
        },
        City {
            name: "Vancouver",
            lat: 49.25,
            lon: -123.1,
        },
    ]
    .iter()
    {
        println!("\t- {}", city);
    }

    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    impl Display for Color {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            let hex_red = format!("{:X}", self.red);
            let hex_green = format!("{:X}", self.green);
            let hex_blue = format!("{:X}", self.blue);
            let hex_rgb = format!("{0:0>2}{1:0>2}{2:0>2}", hex_red, hex_green, hex_blue);
            write!(
                f,
                "RGB ({red}, {green}, {blue}) 0x{hex_rgb} #{hex_rgb}",
                red=self.red, green=self.green, blue=self.blue, hex_rgb=hex_rgb
            )
        }
    }

    println!("\nColors:");
    for color  in [
        Color {red: 128, green: 255, blue: 90},
        Color {red: 0, green: 3, blue: 254},
        Color {red: 0, green: 0, blue: 0},
    ].iter() {
        println!("{}", color);
    }
}

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

    macros();
    formatting();
    debug();
    display();
    list();
    formatting_traits();
}
