use super::helpers;

fn mutability(){
    helpers::section_title(format!("Mutability"));
    println!("Variable bindings are static unless the mut modifier is used during declaration");

    println!("\nWith:");
    let mut mutable_binding = 1;
    println!("\tlet mut mutable_binding = 1;");
    println!("Before mutation: {}", mutable_binding);
    println!("And then:");
    mutable_binding += 1;
    println!("\tmutable_binding += 1;");
    println!("After mutation: {}", mutable_binding);
}

fn scope_and_shadowing() {
    helpers::section_title(format!("Scope and Shadowing"));
    println!("Variable bindings have a scope and are constrained to the block {{ ... }}");
    println!("Variable shadowing is allowed.");

    println!("\nWith:");
    println!("\tlet long_lived_binding = 1;");
    println!("\t{{\n\t\tlet short_lived_binding = 2;\n\t\tlet long_lived_binding = 5;\n\t}}");

    let long_lived_binding = 1;
    {
        let short_lived_binding = 2;
        let long_lived_binding = 5;
        println!("inner:: short: {}, long: {}", short_lived_binding, long_lived_binding);
    }
    println!("outer:: long: {}", long_lived_binding);
    println!("And you can shadow in the same scope too:");
    println!("\tlet long_lived_binding = 'a';");
    let long_lived_binding = 'a';
    println!("outer:: long: {}", long_lived_binding);
}

pub fn run() {
    helpers::example_title(format!("Example {}: {}", 4, "Variable Bindings"));
    println!("Rust is more safe because of static typing, but annotation can be minimized");
    println!("Because the annotation/type can often be inferred by the contents");
    println!("The inferred type is then enforced by the compiler, restricting unsafe usages");

    println!("\nValues (e.g. literals) can be bound to variables using let");
    let an_int = 1u32;
    let a_bool = true;
    let unit = ();
    let int_copy = an_int;

    println!("\nWith:");
    println!("\tlet an_int = 1u32;");
    println!("\tlet a_bool = true;");
    println!("\tlet unit = ();");
    println!("\tlet int_copy = an_int;");

    println!("int_copy: {:?}", int_copy);
    println!("a_bool: {:?}", a_bool);
    println!("unit: {:?}", unit);

    println!("\nUnused variables will raise a warning in the compiler - silence it by prepending a _");
    println!("e.g _unused_var = 0;");

    println!("\nVariables can be declared without setting a value (e.g. let x;)");
    println!("This is rarely done because it can lead to uninitialized variables, but it's there");

    mutability();
    scope_and_shadowing();
}