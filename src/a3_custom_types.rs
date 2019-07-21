use super::helpers;

fn c_structs() {
    helpers::section_subtitle(format!("C Structs"));
    println!("Classic C struct / JS/Python object (sort of) structure");
    #[derive(Debug)]
    struct Person<'a> {
        name: &'a str,
        age: u8,
    }
    #[derive(Debug)]
    struct Point {
        x: f32,
        y: f32,
    }
    #[derive(Debug)]
    struct Rectangle {
        p1: Point,
        p2: Point,
    }

    let name = "Connor";
    let age = 32;
    let person = Person { name, age };
    println!("\nWith: struct Person<'a> {{ name: &'a str, age: u8, }}");
    println!("And name = {}; age = {};", name, age);
    println!("\tPerson {{ name, age }} = {:?}", person);

    let p1: Point = Point { x: 0.3, y: 0.4 };
    let p2: Point = Point { x: 0.1, ..p1 };
    println!("\nWith:");
    println!("\tp1: Point = Point {{ x: 0.3, y: 0.4 }}");
    println!("\tp2: Point = Point {{ x: 0.1, ..p1 }}");
    println!("\n\tp1 coordinates x:{} y:{}", p1.x, p1.y);
    println!("\tp2 coordinates x:{} y:{}", p2.x, p2.y);
    println!("\tI used the struct update syntax to populate the fields of p2 from p1");

    let Point { x: p1_x, y: p1_y } = p1;
    let rectangle: Rectangle = Rectangle {
        p1: Point {
            x: p1_y,
            y: p1_x + 1f32,
        },
        p2: p1,
    };
    println!("\nYou can destructure structs using a let binding:");
    println!("\tlet Point {{ x: p1_x, y: p1_y }} = p1;");
    println!("\tlet rectangle: Rectangle = Rectangle {{\n\t\tp1: Point {{ x: p1_y, y: p1_x + 1f32 }},\n\t\tp2: p1\n\t}};");
    println!("\t=> {:?}", rectangle);

    fn rect_area(rect: Rectangle) {
        println!("\nCalculating the area of a rectangle");
        let length = (rect.p1.x - rect.p2.x).abs();
        let width = (rect.p1.y - rect.p2.y).abs();
        println!("The rectangle decribed by {:?} has:", rect);
        println!("\tLength: {} units", length);
        println!("\tWidth: {} units", width);
        println!("\tArea: {} square units", width * length);
    }
    rect_area(rectangle);

    fn square(lower_left: Point, size: f32) -> Rectangle {
        let p1 = Point {
            x: lower_left.x,
            y: lower_left.y,
        };
        let p2 = Point {
            x: lower_left.x + size,
            y: lower_left.y + size,
        };
        let rect = Rectangle { p1: p1, p2: p2 };
        println!("\nCreated a new square (Rectangle):\n{:?}", rect);
        return rect;
    }
    let square = square(
        Point {
            x: 10.5f32,
            y: 10.1f32,
        },
        3.1415926f32,
    );
    rect_area(square);
}

fn unit_structs() {
    helpers::section_subtitle(format!("Unit Structs"));
    println!("Fieldless structs, mostly for generics"); // TODO: Explain generics
    #[derive(Debug)]
    struct Nil;
    let nil = Nil;
    println!("struct Nil; nil = Nil; nil => {:?}", nil);
}

fn tuple_structs() {
    helpers::section_subtitle(format!("Tuple Structs"));
    println!("These are basically just tuples with names");

    struct Pair(i32, f32);
    let pair = Pair(1, 0.1);
    println!("\nWith:");
    println!("\tstruct Pair(i32, f32)");
    println!("\tpair = Pair(1, 0.1)");
    println!("\tpair.0 = {:?}; pair.1 = {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("\ntuple structs can also be destructured with let:");
    println!("\t`let Pair(integer, decimal) = pair`");
    println!("\tinteger = {:?}; decimal = {:?}", integer, decimal);
}

fn structures() {
    helpers::section_title(format!("Structures"));
    println!("There are 3 types of structs that can be created with the struct keyword");
    c_structs();
    unit_structs();
    tuple_structs();
}

fn use_enum() {
    helpers::section_subtitle(format!("use-ing Enums"));
    println!("The use declaration can bue used to bind enums to a more local scope");

    enum Status {
        Rich,
        Poor,
    }

    enum Work {
        Civilian,
        Soldier,
    }

    println!("\nWith:");
    println!("\tenum Status {{ Rich, Poor }}");
    println!("\tenum Work {{ Civilian, Soldier }}");

    use Status::{Poor, Rich};
    println!("Explicitly use needed items => use Status::{{Poor, Rich}};");
    use Work::*;
    println!("Automatically use each name => user Work::*;");

    let status = Poor;
    let work = Civilian;

    println!("\nAnd with:");
    println!("\tlet status = Poor");
    println!("\tlet work = Civilian");

    match status {
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor don't have any money..."),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
}

fn c_like_enums() {
    helpers::section_subtitle(format!("C-like Enums"));
    println!("enums can be used like C-like enums");

    enum Number {
        Zero,
        One,
        Two,
    }
    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }
    println!("\nWith:");
    println!("\timplicit discriminator (starts at 0) => enum Number {{ Zero, One, Two }}");
    print!("\texplicit discriminator => ");
    println!("enum Color {{ Red = 0xff0000, Green = 0x00ff00, Blue = 0x0000ff }}");

    println!("Number::Zero is {}", Number::Zero as i32);
    println!("Number::One is {}", Number::One as i32);

    println!("Roses are #{:06x}", Color::Red as i32);
    println!("Violets are #{:06x}", Color::Blue as i32);
    println!("Color::Green is #{:06x}", Color::Green as i32);
}

fn linked_list() {
    use List::{ Cons, Nil };
    helpers::section_title(format!("Testcase: Linked-List"));
    enum List { Cons(u32, Box<List>), Nil }
    impl List {
        fn new() -> List { Nil }
        fn prepend(self, elem: u32) -> List {
            Cons(elem, Box::new(self))
        }
        fn len(&self) -> u32 {
            match *self {
                Cons(_, ref tail) => 1 + tail.len(),
                Nil => 0,
            }
        }
        fn stringify (&self) -> String {
            match *self {
                Cons(head, ref tail) => {
                    format!("{}, {}", head, tail.stringify())
                },
                Nil => {
                    format!("Nil")
                },
            }
        }
    }

    let mut list = List::new();

    for x in 1..11 {
        list = list.prepend(x);
    }

    println!("\nWith:");
    println!("\tuse List::{{ Cons, Nil }};");
    println!("\tList {{ Cons(u32, Box<List>), Nil }}");
    println!("\timpl List {{ new(), prepend(u32), len(), stringify() }}");
    println!("let mut list = List::new();");
    println!("did this a few times => list = list.prepend(#u32);");
    println!("The linked List has length: {}", list.len());
    println!("The linked list is: {}", list.stringify());
}

fn enumerators() {
    helpers::section_title(format!("Enumerators"));
    println!("The enum keyword allows for the creation of a type of a variant");
    println!("Any variant that is valid as a struct is valid as an enum");

    enum WebEvent {
        // Unit-like
        PageLoad,
        PageUnload,
        // Like tuple structs
        KeyPress(char),
        Paste(String),
        // Like C structs
        Click { x: i64, y: i64 },
    }

    fn inspect(event: WebEvent) {
        match event {
            WebEvent::PageLoad => println!("\tpage loaded"),
            WebEvent::PageUnload => println!("\tpage unloaded"),
            WebEvent::KeyPress(c) => println!("\tpressed '{}'", c),
            WebEvent::Paste(s) => println!("\tpasted \"{}\"", s),
            WebEvent::Click { x, y } => println!("\tclicked at x={}, y={}", x, y),
        }
    }

    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    println!("This is mostly just to have a complete example with no comments");
    println!("It's pretty straight-forward, so read the code if you're interested");
    println!("More examples are coming");

    use_enum();
    c_like_enums();
    linked_list();
}

fn constants() {
    helpers::section_title(format!("Constants"));
    println!("Rust has two constant types:");
    println!("\tconst - An unchangeable value");
    println!("\tstatic - A (possibly) mutable variable with a 'static lifetime");
    println!("\t\tAccessing or modifying a mutabe static variable is unsafe.");

    static LANGUAGE: &str = "Rust";
    const THRESHOLD: i32 = 10;

    fn is_big(n: i32) -> bool {
        n > THRESHOLD
    }

    let n = 16;


    println!("\nWith:");
    println!("\tstatic LANGUAGE: &str = \"{}\";", LANGUAGE);
    println!("\tconst THRESHOLD: i32 = {};", THRESHOLD);
    println!("\tfn is_big(n: i32) -> bool {{ n > THRESHOLD }}");
    println!("\tlet n = {};", n);
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

}

pub fn run() {
    helpers::example_title(format!("Example {}: {}", 3, "Custom Types"));
    println!("Rust has two main custom data types:");
    println!("\tstruct - a structure");
    println!("\tenum - an enumerator");

    println!("\nConstants can also be created using the const or static keywords");

    structures();
    enumerators();
    constants();
}
