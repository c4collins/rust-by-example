use super::helpers;
use std::convert::From;
use std::fmt;
use typename::TypeName;

fn from_and_into() {
    helpers::section_title(format!("From and Into Traits"));
    println!("From and Into traits are inherently linked");
    println!("If you can convert Into type A, you should be able to convert From type A");

    let my_str = "hello";
    let my_string = String::from(my_str);

    println!("\nWith:");
    println!("\tlet my_str = \"{}\";", my_str);
    println!("\tmy_string = String::from(my_str)");
    println!(
        "my_str => {}: {};\nmy_string => {}: {}",
        my_str,
        TypeName::type_name_of(my_str),
        my_string,
        TypeName::type_name_of(&my_string)
    );

    #[derive(Debug)]
    struct Number {
        value: i32,
    }
    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }
    let num = Number::from(30);
    println!("\nYou can define the same type of conversion for a custom type");
    println!("With:");
    println!("\tstruct Number {{ value: i32 }}");
    println!("\timpl From<i32> for Number {{\n\t\tfn from(item: i32) -> Self {{\n\t\t\tNumber {{ value: item }}\n\t\t}}\n\t}}");
    println!("\tnum = Number::from(30)");
    println!("num is {:?}", num);

    let int = 5;
    let num2: Number = int.into();
    println!("\nYou get Into for free by defining From");
    println!("With:");
    println!("\tint = {}", int);
    println!("\tnum2: Number = int.into()");
    println!("num2 is {:?}", num2);
}

fn to_and_from_strings() {
    helpers::section_title(format!("To- and From-Strings"));
    println!("Converting to a String is simple as well, you just need to implement ToString");
    println!("But rather than doing it directly it's better to implement the fmt::Display trait");
    println!("Doing it this way automatically provided ToString plus allows printing with print!/format! as shown in Example 1");

    struct Circle { radius: f32 }
    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Circle of radius {}", self.radius)
        }
    }
    let circle = Circle { radius: 6.1 };
    println!("\nWith:");
    println!("\tstruct Circle {{ radius: f32 }}");
    println!("\timpl fmt::Display for Circle {{\n\t\tfn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{\n\t\t\twrite!(f, \"Circle of radius {{}}\", self.radius)\n\t}}");
    println!("\tcircle = Circle {{ radius: 6.1 }}");
    println!("{}", circle.to_string());

    let parsed: i32 = "5".parse().unwrap();
    let turbofish_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbofish_parsed;

    println!("\nOne of the more common types is to convert a string to a number");
    println!("You can use the ToString trait to impl this on custom types");
    println!("There are two approaches here: without type inference, or with 'turbofish' syntax");
    println!("With:");
    println!("\tparsed: i32 = \"5\".parse().unwrap()");
    println!("\tturbofish_parsed = \"10\".parse::<i32>().unwrap()");
    println!("\tsum = parsed + turbofish_parsed");
    println!("{} + {} = {}", parsed, turbofish_parsed, sum);
}

pub fn run() {
    helpers::example_title(format!("Example {}: {}", 6, "Conversion"));
    println!("Rust converts between types using traits.");
    println!("Generic conversions use From and Into traits");
    println!("However, specific cases, such as Strings, have more specific traits");

    from_and_into();
    to_and_from_strings();
}
