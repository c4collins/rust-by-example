use super::helpers;

fn fizzbuzz_to(n: u32) {
    fn is_divisible_by(n: u32, by_n: u32) -> bool {
        if n == 0 {
            return false;
        } // edge case
        n % by_n == 0
    }
    fn fizzbuzz(n: u32) {
        if is_divisible_by(n, 3) {
            print!("Fizz");
        }
        if is_divisible_by(n, 5) {
            print!("Buzz");
        }
        if !is_divisible_by(n, 3) && !is_divisible_by(n, 5) {
            print!("{}", n);
        }
        print!(", ");
    }
    for i in 1..=n {
        fizzbuzz(i);
    }
    println!("... And that's FizzBuzz again");
}

fn methods() {
    helpers::section_title(format!("Methods"));
    println!("Methods are functions attached to objects, like every other programming language");
    println!(
        "Methods have access to the object's data and methods with the `self` keyword, like Python"
    );
    println!("\t(but as &self, which is syntactic sugar for e.g. self: &Point)");
    println!("\nMethods are defined in an `impl` block related to the object");
    println!("You can implement static methods that don't need to be called by an instance like:");
    println!("\timpl Point {{ fn new(x: f64, y: f64) -> Point {{ ... }}");
    println!("\t\tthen call with :: like:\tPoint::new(3.0, 4.0)");
    println!("You can implement instance methods like:");
    println!("\timpl Rectangle {{\n\t\tfn area(&self) -> f64 {{ ... }}\n\t}}");
    println!("\timpl Rectangle {{\n\t\tfn translate(&mut self) {{ ... }}\n\t}}");
    println!(
        "\t\tthen call with . like:\tRectangle.area() (self is implicitly passed, like Python)"
    );

    println!("\nWith:");
    println!("\tstruct Point {{ x: f64, y: f64 }}");
    println!("\timpl Point {{\n\t\tfn origin() -> Point {{ ... }}\n\t\tfn new(x:f64, y: f64) -> Point {{ ... }}\n\t}}");
    println!("\tstruct Rectangle {{ p1: Point, p2: Point }}");
    println!("\timpl Rectangle {{\n\t\tfn area(&self) -> f64 {{ ... }}\n\t\tfn perimiter(&self) -> f64 {{ ... }}\n\t\tfn translate(&mut self, x: f64, y:f64) {{ ... }}\n\t}}");
    println!("\tstruct Pair ( Box<i32>, Box<i32> );");
    println!("\t");

    struct Point {
        x: f64,
        y: f64,
    }

    impl Point {
        fn origin() -> Point {
            Point { x: 0.0, y: 0.0 }
        }
        fn new(x: f64, y: f64) -> Point {
            Point { x: x, y: y }
        }
    }

    struct Rectangle {
        p1: Point,
        p2: Point,
    }

    impl Rectangle {
        fn area(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;

            ((x1 - x2) * (y1 - y2)).abs()
        }

        fn perimeter(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;
            2.0 * ((x2 - x1).abs() + (y2 - y1).abs())
        }

        fn translate(&mut self, x: f64, y: f64) {
            self.p1.x += x;
            self.p2.x += x;
            self.p1.y += y;
            self.p2.y += y;
        }
    }

    struct Pair(Box<i32>, Box<i32>);
    impl Pair {
        fn destroy(self) {
            let Pair(first, second) = self;
            println!("Destroying {} and {}", first, second);
            // first and second go out of scope and get freed
        }
    }
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());
    // rectangle.translate(1.0, 1.0); // Needs to be mutable as that's what the fn needs

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
    // pair.destroy() // won't work because the pair was already consumed
}
fn closures() {
    helpers::section_title(format!("Closures"));
    println!("Also know as lambdas, closures are functions that capture the enclosing environment, e.g.:");
    println!("\tminimal: |val| val + x;");
    println!("\tmaximal: let thing = |val: i32| -> i32 {{ val +1; val + x }}");
    println!("\tsuper-minimal: || 1;");

    println!("\nCalling a closure is exactly like calling a function, except:");
    println!("\tdon't use the fn keyword and they're nameless, so are either anonymous or assigned to a reference (with `let`)");
    println!("\tinput and return types can be inferred");
    println!("\tinput variable names must be specified");
    println!("\t|| around input parameters instead of ()");
    println!(
        "\tblock delineation ({{ ... }}) only necessary if the closure has more than 1 expression"
    );
    println!("\tclosures have the ability to capture outer environment variables");

    fn function(i: i32) -> i32 {
        i + i
    }

    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;
    let one = || 1;

    let i = 1;
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));
    println!("closure returning 1: {}", one());

    capturing();
    as_input_parameters();
    type_anonymity();
    input_functions();
    as_output_parameters();
    examples_in_std();
}

fn capturing() {
    helpers::section_subtitle(format!("Capturing"));
    println!("Closures are flexible to the max and will try really hard to make the closure work without annotation");
    println!("That means they'll flexibly adapt to the use case, sometimes moving and sometimes borrowing variables");
    println!(
        "There's an order of specificity, basically attempting to limit access when possible:"
    );
    println!("\tby (borrowed) reference: &T");
    println!("\tby (borrowed) mutable reference: &mut T");
    println!("\tby (moved) value: T");
    println!("You can also force the value to be moved with the `move` keyword before the pipes");
    println!("\t");

    println!("\nLet's look at some examples:");

    println!("\n1. &T - reference");
    println!("\tA function like `println!` only needs to have the &str while it operates");
    let color = "green";
    let what_color = || println!("color is: {}", color); // by reference: &T
    what_color();
    what_color();

    println!("\n2. &mut T - mutable reference");
    println!("\tIn the case that you're e.g. incrementing a variable, it all needs to be mutable");
    let mut count = 0; // mutable variable, and mutable closure
    let mut increment = || {
        // by mutable reference: &mut T
        count += 1;
        println!("count is: {}", count);
    };
    increment();
    increment();
    increment();
    increment();
    increment();
    increment();

    println!("\n3. T - moved by value");
    println!("\tmem:::drop requires `T` so this closure can only take by value");
    println!("\tA copy type would copy into the closure, leaving the original untouched");
    println!("A non-copy type must move into the closure itself");
    use std::mem;
    let movable = Box::new(3);
    let consume = || {
        println!("movable: {}", movable);
        mem::drop(movable);
    };

    consume();
    // consume(); // won't work

    println!("\n4. Force move with `move`");
    let haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);
    println!("haystack contains &1: {}", contains(&1));
    println!("haystack contains &4: {}", contains(&4));
    // println!("haystack contains {} elements", haystack.len()); // Won't work - haystack was moved
}

fn as_input_parameters() {
    helpers::section_subtitle(format!("Closures as input parameters"));
    println!("While Rust's closures choose how to capture variables quite well, ambiguity in function declarations is not valid");
    println!("When taking a closure as an input parameter the closure's complete type must be annotated, using these traits:");
    println!("\tFn - captures by reference &T");
    println!("\tFnMut - captures by mutable reference &mut T");
    println!("\tFnOnce - captures by value T");
    println!("These traits are applied prefentially in that order, and the compiler tries to caputer variables in the least restrictive manner");
    println!("So Fn will only capture by reference, but FnOnce will try to capture &T, &mut T and only T if necessary");

    println!("\nSyntax examples:\n\tfn apply<F>(f: F) where F: FnOnce() {{ ... }}\n\tfn apply_to_3<F>(f: F) -> i32 where F: Fn(i32) -> i32 {{ ... }}");
    println!("\n\tfn some_function<T, U>(t: T, u: U) -> i32\n\t\twhere T: Display + Clone,\n\t\t\tU: Clone + Debug {{ ... }}");

    use std::mem;

    fn apply<F>(f: F)
    where
        F: FnOnce(),
    {
        f();
    } // This syntax is fucky too
    fn apply_to_3<F>(f: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(3)
    }
    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();
    // capture greeting by reference and farewell by value
    let diary = || {
        println!("I said {}", greeting); // Requires Fn
        farewell.push_str("!!!"); // Requires FnMut to be captured as mutable reference
        println!("Then I screamed {}", farewell);
        println!("Now I can sleep. Zzzz.");

        mem::drop(farewell); // Manually calling mem::drop requires farewell to e captured by value
    };
    apply(diary);
    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));
}
fn type_anonymity() {
    helpers::section_subtitle(format!("Type Anonymity"));
    println!("Closures necessarily require generics because they succinctly capture variables from enclosing scopes.");
    println!("I understand what those words mean individually, but not in that order.");

    println!("When a closure is defined the compiler creates an anonymous storage structure to store the captured variables");
    println!("The compiler also implements the functionality via one of the traits: Fn, FnMut, or FnOnce");
    println!("This type is assigned to the variable which is stored until calling.");
    println!("Since the type is unknown, any usnage in a function requires generics");
    println!("An unbounded tyoe parameter <T> would not allowed because it is ambiguous, but it can be bounded by on of the traits");

    fn apply<F>(f: F)
    where
        F: Fn(),
    {
        f();
    }
    let x = 7;
    let print = || println!("{}", x);
    apply(print);
}

fn input_functions() {
    helpers::section_subtitle(format!("Input Functions (an aside from closures)"));
    println!("Functions can be used as arguments as well as closures");
    println!("If you define a function that takes a closure as a parameter, any functions that satisfies the closure's bound can be passed");

    fn call_me<F: Fn()>(f: F) {
        f()
    } // takes generic F argument bounded by Fn and calls it
    fn function() {
        println!("I'm a function!");
    }
    let closure = || println!("I'm a closure!");
    call_me(closure);
    call_me(function);
}
fn as_output_parameters() {
    helpers::section_subtitle(format!("Closures as Output Parameters"));
    println!("Since you can accept a closure you should be able to return a closure from a function, right?");
    println!("You can, but you have to Box the return because you can't return a generic type");
    println!("So by using Box you can make the closure concrete and return it");

    println!("\nThe valid traits for returns are slightly different than before");
    println!("\tFn - same as input");
    println!("\tFnMut - same as input");
    println!("\tFnOnce - requires FnBox, but that's unstable, so it can't be used (yet)");
    println!("Additionally the move keyword must be used, which makes sense because otherwise the references would be dropped when the function exited, voiding the closure contents");

    fn create_fn() -> Box<Fn()> {
        let text = "Fn".to_owned();
        Box::new(move || println!("This is a {}", text))
    }
    fn create_fnmut() -> Box<FnMut()> {
        let text = "FnMut".to_owned();
        Box::new(move || println!("This is a {}", text))
    }

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();

    fn_plain();
    fn_mut();
}
fn examples_in_std() {
    helpers::section_subtitle(format!("Here are some examples from the std library:"));

    println!("\n1. Iterator::any");
    println!("Iterator::any is a function that when passed an iterator will return true if any element datisfies the prediate, otherwise false");
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));

    println!("\n2. Iterator::find");
    println!("Iterator::find is a function that when passed an iterator returns the first element that satisfies the predicate (as an Option)");
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();
    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    println!(
        "Find 2 in array2: {:?}",
        array2.into_iter().find(|&&x| x == 2)
    );
}

fn higher_order_functions() {
    helpers::section_title(format!("Higher Order Functions"));
    println!("HOFs are functions which take >0 functions and/or produce a more useful function");
    println!("These are apparently what give Rust it's functional flavour");
    println!("Option and Iterator have more than a few HOFs");

    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    // Imperatively
    let mut acc = 0;
    for n in 0.. {
        let n_squared = n * n;
        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }
    println!("Imperatively: {}", acc);

    // Functionally
    let sum_of_squared_odd_numbers: u32 = (0..)
        .map(|n| n * n) // All natural numbers squared
        .take_while(|&n_squared| n_squared < upper) // below upper limit
        .filter(|&n_squared| is_odd(n_squared)) // that are odd
        .fold(0, |acc, n_squared| acc + n_squared); // sum them
    println!("Functionally: {}", sum_of_squared_odd_numbers);
}

fn diverging_functions() {
    helpers::section_title(format!("Diverging functions"));
    println!("Diverging functions are functions that never return");
    println!("They are marked with !, which is an empty type");
    println!("\tfn foo() -> ! {{\n\t\tpanic!(\"This call never returns.\");\n\t}}");
    fn foo() -> ! {
        panic!("This call never returns.");
    }
    println!("! is event less of a thing than nothing, at least nothing will return ()");
    println!("It's an abstract concept and the way it's explained makes me think it's sort of just background info");
}

pub fn run() {
    helpers::example_title(format!("Example 9: Functions"));
    println!("Functions are declared using the `fn` keyword");
    println!("Function arguments must by type annotated, just like variables");
    println!("If the function returns a variable, the type must be specified after an arrow (->)");
    println!("\nfn run() {{ ... }}");
    println!("\nfn run(variable_1:u64, variable_2: &mut u32) -> bool {{ ... }}");
    println!("It works the same as a function in JS/Python other than the type strictness");

    println!("\nFizzBuzz with functions:");
    fizzbuzz_to(30);
    // fizzbuzz_to(100);
    // fizzbuzz_to(1000);
    // fizzbuzz_to(10000);
    // fizzbuzz_to(1_000_000);
    methods();
    closures();
    higher_order_functions();
    diverging_functions();
}
