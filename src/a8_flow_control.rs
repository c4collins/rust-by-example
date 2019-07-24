use super::helpers;

fn if_else() {
    helpers::section_title(format!("`if` and `else`"));
    println!(
        "`if` and `else` work basically the same as JS, except no parens around the conditions"
    );
    println!("\nAn if statement is declared like this:");
    println!("\tif expression {{ ... }} if else expression {{ ... }} else {{ ... }}");
    println!("\nThe code for this section has some more examples if you need them.\n");

    let n = 9;
    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is 0", n);
    }

    let threshold = 10;
    let divisor = 3;
    let big_n = if n < threshold && n > -threshold {
        println!(", and that's too small, increase it by {}x", threshold);
        threshold * n
    } else {
        println!(", and that's too big, divide it by {}", divisor);
        n / divisor
    };
    println!("{} => {}", n, big_n);
}

fn loop_flow() {
    helpers::section_title(format!("(Infinite) `loop`"));
    println!("As a start, Rust provides the `loop` keyword for infinite loops.");
    println!("I don't use these often but obviously they're necessary for e.g. game dev");
    println!("\nA `'label` is declared like this:");
    println!("\tloop {{ ... }}");
    println!("if you need more than that, again, there are examples in the code");
    println!("\n`break` and `continue` can be used to escape the loop or skip to the next iteration respectively");

    let mut count = 0u32;
    println!("\nWe can count forever");
    loop {
        count += 1;

        if count == 3 {
            println!("Three");
            continue;
        }
        println!("count: {}", count);

        if count == 5 {
            println!("Or, you know, that'll do for now");
            break;
        }
    }

    count = 0;
    println!("\nLet's do a fizzbuzz");
    loop {
        count += 1;
        if count > 100 {
            println!("...The Aristocrats!");
            break;
        }
        let is_divisible_by_three = count % 3u32 == 0;
        let is_divisible_by_five = count % 5u32 == 0;
        let is_divisible_by_three_and_five = is_divisible_by_five && is_divisible_by_three;
        if is_divisible_by_three_and_five {
            print!("FizzBuzz, ");
            continue;
        }
        if is_divisible_by_five {
            print!("Buzz, ");
            continue;
        }
        if is_divisible_by_three {
            print!("Fizz, ");
            continue;
        }
        print!("{}, ", count);
        // I made this more efficient, but then changed it back
        // Because I didn't need break or continue in my revised example.
    }

    nesting_and_labels();
    returning_from_loops();
}

fn nesting_and_labels() {
    helpers::section_subtitle(format!("Nesting and Labels"));
    println!(
        "Loops can be labelled, so when you have nested loops you can name them to keep track"
    );
    println!("This allows you to use `break` and continue on specific loops.");
    println!("\nA `'label` is declared like this:");
    println!("\t'loop_label: loop {{ ... }}");
    println!("And it's used ike this:");
    println!("\tbreak 'label_name; || continue 'label_name;");
    println!("\nAgain, the example is in the code\n");

    'outer: loop {
        println!("Entered the 'outer loop");
        'inner: loop {
            println!("Entered the 'inner loop");
            break 'outer;
        }
    }
    println!("Exited 'outer loop (and 'inner because it was nested in 'outer)");
}

fn returning_from_loops() {
    helpers::section_subtitle(format!("Nesting and Labels"));
    println!("Rust allows you to return a value from a loop for e.g. retrying an operation");
    println!("You have to do two things:");
    println!("\tAssign the loop to a binding: e.g. let thing = loop {{ ... }}");
    println!("\tAdd a value to return after the break statement: e.g. break expression_to_return;");
    println!("\nAgain, the example is in the code\n");

    let mut count = 0;
    let result = loop {
        count += 1;
        if count >= 10 {
            break count * count;
        }
    };
    assert_eq!(result, 100);
    println!("Super efficient program: {}^2 = {}", count, result);
}

fn while_flow() {
    helpers::section_title(format!("`while` Loop"));
    println!("`while` works the same as the basic `loop` but takes a conditional expression:");
    println!("\t while x > y {{ ... }}");
    println!("These examples are all really simple but long because of the grammar so again, example in the code");
    println!("\nOkay, lets do that improved fizzbuzz from before, but in a while loop.");
    let mut count = 0;
    while count < 100 {
        count += 1;
        if count % 3 == 0 {
            print!("Fizz");
        }
        if count % 5 == 0 {
            print!("Buzz");
        }
        if !(count % 3 == 0) && !(count % 5 == 0) {
            print!("{}", count);
        }
        print!(", ");
    }
    println!("...The Aristocrats!");
}

fn for_flow() {
    helpers::section_title(format!("`for` Loops"));
    println!("They're really `for x in y` loops, where x is a fresh binding and y is a collection");

    for_range();
    for_iterators();
}

fn for_range() {
    helpers::section_subtitle(format!("`for` and `range`"));
    println!("Rust provides a `for in` construct to iterate through `Iterator`s");
    println!("A shortcut for creating an `Iterator` are the range notations: a..b && a..=b");
    println!("a is inclusive, b is exclusive, =b is inclusive:");
    println!("\tfor in 0..5 => 0, 1, 2, 3, 4");
    println!("\tfor in 0..=5 => 0, 1, 2, 3, 4, 5");
    println!("All together it looks like (for 10 loops):");
    println!("\tfor x in 0..11 {{ ... }}");
    println!("\tfor x in 0..=10 {{ ... }}");

    println!("\n...fizzbuzz again...");
    for n in 1..=100 {
        let by3 = n % 3 == 0;
        let by5 = n % 5 == 0;
        if by3 {
            print!("Fizz");
        }
        if by5 {
            print!("Buzz");
        }
        if !by3 && !by5 {
            print!("{}", n);
        }
        print!(", ");
    }
    println!("... it's not going to be funny a third time");
}

fn for_iterators() {
    helpers::section_subtitle(format!("`for` and `range`"));
    println!("By default the `for` loop applies the into_iter function on the collection supplied");
    println!("But there are more options to convert a collection into an `Iterator`");
    println!("\titer");
    println!("\tinto_iter");
    println!("\titer_mut");

    println!("\niter - borrows each element for the duration of each iteration");
    println!("The collection is left unchanged and available after the loop is complete");

    let names = vec!["Blueberry", "Friday", "Connor"];
    for name in names.iter() {
        match name {
            &"Connor" => println!("I'm a cat!"), // Borrow
            _ => println!("Meow meow meow"),
        }
    }
    println!("{:?}", names);

    println!("\ninto_iter - consumes the collection so each iteration consumes one element");
    println!(
        "When the loop in complete the collection is no longer available as it has been moved"
    );
    // for name in names.into_iter() {
    for name in names {
        // It's the default
        match name {
            "Connor" => println!("I'm a cat!"), // Actual
            _ => println!("Meow meow meow"),
        }
    }
    // println!("{:?}", names); // This is now an error

    println!("\niter_mut - Mutably borrows each element, like iter but mutable");
    println!("When the loop is complete the collection exists and may have been modified");
    let mut names = vec!["Blueberry", "Friday", "Connor"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Connor" => "I'm a cat!", // Mutable borrow
            _ => "Meow meow meow",
        }
    }
    println!("{:?}\n", names);

    let mut numbers = [
        2u64, 425u64, 12u64, 235u64, 568u64, 234u64, 893u64, 257u64, 2456u64,
    ];
    for n in numbers.iter_mut() {
        *n = *n * *n
    }
    println!("{:?}", numbers);
}

fn match_flow() {
    helpers::section_title(format!("`match` Matching"));
    println!("`match` works basically the same way as a JS `switch` statement");
    println!("There is a catch-all value of `_`, just like default: in JS");
    println!("So it looks something like:");
    println!("\tmatch expression {{ condition => result, _ => result }}");
    println!("\tlet var = match expression {{ condition => result, _ => result }}");
    println!("`match ` requires complete coverage of all possible cases");
    println!("So you only need the catch-all condition (`_`) if you didn't satisfy that:");
    println!("\tmatch boolean_expression {{ true => result, false => result }}");

    let number = 13;
    println!("\nTeel me about {}", number);
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is prime"),
        13..=19 => println!("A teen"),
        _ => println!("Not so special"),
    }

    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} => {}", boolean, binary);

    match_guards();
    match_binding();
    match_destructuring();
}

fn match_destructuring() {
    helpers::section_title(format!("`match` Destructuring"));
    tuple_destructuring();
    enum_destructuring();
    pointers_references();
    struct_destructuring();
}
fn tuple_destructuring() {
    helpers::section_subtitle(format!("`tuple` Destructuring"));
    println!("Tuples can be destructured in match like this:");
    println!("\t(0, y) => println!(\"x is 0, y is {{}}\", y)");

    let pair = (0, -2);
    println!("\nTell me about {:?}", pair);
    match pair {
        (0, y) => println!("x is nothing, y is {}", y),
        (x, 0) => println!("x is {}, y is nothing", x),
        _ => println!("Neither x nor y are nothing"),
    }
}
fn enum_destructuring() {
    helpers::section_subtitle(format!("`enum` Destructuring"));
    println!("Enums destructure basically the same as tuples but with a name");
    println!("\tColor::RGB(r, g, b) => println!(\"rgb color => red: {{}}, green: {{}}, blue: {{}}\", r, g, b)");

    enum Color {
        Red,
        Green,
        Blue,
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    let colors = vec![
        Color::Red,
        Color::Green,
        Color::Blue,
        Color::RGB(122, 17, 40),
        Color::HSV(122, 15, 64),
        Color::HSL(56, 126, 117),
        Color::CMY(25, 24, 15),
        Color::CMYK(25, 62, 3, 164),
    ];
    println!("\nWhat colour is it?");
    for color in colors {
        match color {
            Color::Red => println!("The color is red"),
            Color::Green => println!("The color is green"),
            Color::Blue => println!("The color is blue"),
            Color::RGB(r, g, b) => println!("rgb color => red: {}, green: {}, blue: {}", r, g, b),
            Color::HSV(h, s, v) => {
                println!("hsv color => hue: {}, saturation: {}, value: {}", h, s, v)
            }
            Color::HSL(h, s, l) => println!(
                "hsl color => hue: {}, saturation: {}, lightness: {}",
                h, s, l
            ),
            Color::CMY(c, m, y) => {
                println!("cmy color => cyan: {}, magenta: {}, yellow: {}", c, m, y)
            }
            Color::CMYK(c, m, y, k) => println!(
                "cmyk color => cyan: {}, magenta: {}, yellow: {}, black: {}",
                c, m, y, k
            ),
            // Doesn't need a _ case because all possibilities are accounted for
        }
    }
}

fn pointers_references() {
    helpers::section_subtitle(format!("Pointers and References"));
    println!("Rust makes a strong distinction between destructuring and dereferencing");
    println!("Borrowing/refs is an advanced topic we'll get to later, this is more how-to-use than what-the-heck");
    println!("When you reference a value for a borrow that's Destructuring");
    println!("\tDestructuring uses `&`, `ref`, and `ref mut`");
    println!("When you need to get values from a reference you need to dereference it");
    println!("\tDereferencing uses `*`");

    println!("\nI'll start with a reference value, because it's simpler");
    let reference = &4;
    match reference {
        // Use the reference value, destructure it here to borrow the value
        &val => println!("Got a value via destructuring: {:?}", val),
    }
    // or
    match *reference {
        // dereference the value ahead of time
        val => println!("Got a value via destructuring: {:?}", val),
    }

    println!("\nNow what about starting with a non-referenced value?");
    println!("You can just declare a value as a ref => let ref thing = expression;:");
    println!("\tlet not_a_ref = 3;");
    println!("\tlet ref is_a_ref = 3;");

    println!("\nOr, you can use `ref` / `ref mut` to handle it in the block");
    let value = 5;
    match value {
        // use ref to create a reference
        ref r => println!("Got a reference to a value: {:?}", r),
    }
    let mut mut_value = 6;
    match mut_value {
        ref mut m => {
            *m += 10; // Since we got a reference we need to dereference it before we cann add to it
            println!("We added 10 and got {:?}", m);
        }
    }
}

fn struct_destructuring() {
    helpers::section_subtitle(format!("De`struct`uring"));
    println!("Structs can be completely destructured down to the deepest branch");
    println!("It's very similar to JS object destructuring");
    println!(
        "\tFoo {{ x: (1, b), y }} => println!(\"First of x is 1, b = {{}}, y = {{}}\", b, y),"
    );

    struct Foo {
        x: (u32, u32),
        y: u32,
    }
    let foos = vec![
        Foo { x: (1, 2), y: 3 },
        Foo { x: (2, 2), y: 2 },
        Foo { x: (0, 2), y: 3 },
    ];
    for foo in foos {
        match foo {
            Foo { x: (1, b), y } => println!("First of x is 1, b = {}, y = {}", b, y),
            Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),
            Foo { y, .. } => println!("y = {}, we don't care about x", y),
        }
    }
}

fn match_guards() {
    helpers::section_subtitle(format!("Guards"));
    println!("A guard is a boolean expression limiting access to match paths");
    println!("\t(x, y) if x == y => println!(\"These are twins\"),");

    let pair = (2, -2);
    println!("\nTell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}

fn match_binding() {
    helpers::section_subtitle(format!("`match` Binding"));
    println!("Indirectly accessing a variable makes it impossible to use without rebinding");
    println!("`match` provides the @ sigil for binding values to names");
    println!("\tn @ 1...12 => println!(\"I'm a child of age {{:?}}\", n),");

    fn age() -> u32 {
        33
    }
    println!("\nTell me what type of person you are");

    match age() {
        0 => println!("I'm not yet a year old"),
        n @ 1...12 => println!("I'm a child of age {:?}", n),
        n @ 13...19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }
}

fn if_let() {
    helpers::section_title(format!("`if let` Flow"));
    println!("Match can be kind of messy, so `if let` exits to make it cleaner when possible");
    println!("And also allows for various failure options to be specified");
    println!("The basic syntax is kind of weird and confusing, but here's what I think:");
    println!("\tif let Some(i) = number <= condition of if (e.g. if number == Some(i) {{ ... }})");
    println!("\t{{ ... }} else if condition {{ ... }} else {{ ... }} <= normal if syntax");
    println!("\nI think of it like:");
    println!("\tif c == Foo::Some(_) as defined in the enum");
    println!("\tlet i = _");
    println!("\tthen run like a normal block {{ ... }}");

    println!("\nHere are some examples of `if let` with else and if else conditions:");
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoji: Option<i32> = None;
    let i_like_letters = false;

    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. It's a letter?");
    }
    if let Some(i) = emoji {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Not a number. It's a letter?");
    } else {
        println!("I don't like letters. It's an emoji. :)");
    }

    println!("\nHonestly it seemed like a weird addition to the language (and I still don't really understand the syntax)");
    println!(
        "Until I learned that Enums can't be compared for equality with == but `if let` works"
    );
    println!("So the examples I gave earlier about the syntax are incorrect syntactically");
    println!("But they're still useful (to me anyway) for understanding how `if let` works");
    enum Foo {
        Bar,
        Baz,
        Qux(u32),
    }
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    } // runs
    if let Foo::Bar = b {
        println!("b is foobar");
    } // doesn't run
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    } // runs plus extracts value to use

    println!("\nAnd the challenge:");
    println!("\nwas: if Foo::Bar == a {{ println!(\"a is foobar\"); }}");
    println!("\nnow: if let Foo::Bar = a {{ println!(\"a is foobar\"); }}");
    // if Foo::Bar == a { println!("a is foobar"); }
    if let Foo::Bar = a { println!("a is foobar"); }
    println!("I'm not sure how that was supposed to be a challenge, it's literally the previous example")
}

fn while_let() {
    helpers::section_title(format!("`while let` Flow"));
    println!("Unsurprisingly, `while let` is extremely similar to `if let`, except loopier");
    println!("\twhile let Some(i) = optional {{ ... , optional }}");
    println!("`while let` doesn't support any form of else");

    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, done!");
            optional = None; // Classy
        } else {
            println!("`i` is `{:?}`, try again.", i);
            optional = Some(i + 1);
        }
    }
}

pub fn run() {
    helpers::example_title(format!("Example 8: Flow Control"));
    println!("Rust has a number of ways to manage control flow:");
    println!("\tif / else");
    println!("\tloop");
    println!("\twhile");
    println!("\tfor (and range)");
    println!("\tmatch");
    println!("\tif let");
    println!("\twhile let");

    if_else();
    loop_flow();
    while_flow();
    for_flow();
    match_flow();
    if_let();
    while_let();
}
