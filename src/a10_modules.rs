use super::helpers;

pub fn run() {
    helpers::example_title(format!("Example 10: Modules"));
    println!("Rust provides a powerful module system that can be used to hierarchically split code in logical units and manage visibility");
    println!("A module is a colleciton of items: functions, structs, traits, impl blocks, and other modules");

    module_visibility();
    struct_visibility();
    use_declaration();
    super_and_self();
    file_hierarchy();
}

mod visibility_module {
    fn private_function() {
        println!("called `visibility_module::private_function()`");
    }

    pub fn function() {
        println!("called `visibility_module::function()`");
    }

    pub fn indirect_access() {
        print!("called `visibility_module::indirect_access()`, that\n> ");
        private_function();
    }

    pub mod nested {
        pub fn function() {
            println!("called `visibility_module::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `visibility_module::nested::private_function()`");
        }

        // Public
        pub(in crate) fn public_function_in_my_mod() {
            print!("called `visibility_module::nested::public_function_in_my_mod()`, that\n > ");
            public_function_in_nested()
        }

        // Private
        pub(self) fn public_function_in_nested() {
            println!("called `visibility_module::nested::public_function_in_nested");
        }

        // only in Parent
        pub(super) fn public_function_in_super_mod() {
            println!("called visibility_module::nested::public_function_in_super_mod");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `visibility_module::call_public_funcion_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // only within the current crate
    pub(crate) fn public_function_in_crate() {
        println!("called `visibility_module::public_function_in_crate()");
    }

    // Nested modules follow the same rules for visibility
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `visibility_module::private_nested::function()`");
        }

        // Private parent items will still restrict the visibility of a child item,
        // even if it is declared as visible within a bigger scope.
        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called `visibility_module::private_nested::restricted_function()`");
        }
    }
}

fn visibility_function() {
    println!("called `visibility_function()`");
}

fn module_visibility() {
    helpers::section_title(format!("Module Visibility"));
    println!("Modules allow disambiguation between items that have the same name.");
    println!("They can be private or public, and have private or public members");
    println!("Public items, including those inside nested modules, can be accessed from outside the parent module.");
    println!("Modules can be nested and they follow the same visibility rules\n");
    println!(
        "Private items of a module cannot be directly accessed, even if nested in a public module:"
    );

    visibility_function();
    visibility_module::function();
    visibility_module::indirect_access();
    visibility_module::nested::function();
    visibility_module::call_public_function_in_my_mod();

    // pub(crate) items can be called from anywhere in the same crate
    visibility_module::public_function_in_crate();

    // pub(in path) items can only be called from within the mode specified
    //visibility_module::nested::public_function_in_my_mod();
}

mod struct_module {
    pub struct OpenBox<T> {
        pub contents: T, // public
    } // public
    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        contents: T, // private
    } // public

    impl<T> ClosedBox<T> {
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents: contents }
        }
    }
}

fn struct_visibility() {
    helpers::section_title(format!("Struct Visibility"));
    println!("Structs can also be private or public");
    println!("This privacy is maintained in the goal of encapsulation & only applies outside the mod the struct was defined in");
    println!(
        "This much works as you'd expect: public things are public, private things are private"
    );

    // normal: pub/pub
    let open_box = struct_module::OpenBox {
        contents: "public information",
    };
    println!("The open box contains: {}", open_box.contents);

    // error: pub/priv
    // let closed_box = struct_module::ClosedBox { contents: "classified information" };

    // normal: pub/pub, allows access to private fields
    let _closed_box = struct_module::ClosedBox::new("classified information");

    // error: pub/priv
    //println!("The closed box contains: {}", _closed_box.contents);
    // TODO ^ Try uncommenting this line
}

fn use_declaration() {
    helpers::section_title(format!("The `use` declaration"));
    println!("Similar to Python's `import ... from ... as ...`,\nand especially like JS's destructured `const {{ x1, x2:y }} = import('module-name');`");
    println!(
        "The `use` declaration is used to bind a path to a new name, usually for easier access"
    );
    println!("It looks like this:");
    println!("\tuse crate::deeply::nested::{{ some_fn, ATraitType, and_an_enum }}");
    println!("\tuse crate::deeply::nested::some_fn as deep_nest_fn");
    println!("\tuse crate::deeply::nested::some_fn");
}

fn super_function() {
    println!("called `super_function()`");
}

mod super_cool {
    pub fn function() {
        println!("called `super_cool::function()`");
    }
}

mod super_module {
    fn function() {
        println!("called `super_module::function()`");
    }
    mod cool {
        pub fn function() {
            println!("called `super_module::cool::function()`");
        }
    }
    pub fn indirect_call() {
        print!("called `super_module::indirect_call()`, that\n> ");
        // The `self` keyword refers to the current module scope - in this case `super_module`.
        // Calling `self::function()` and calling `function()` directly both give
        self::function();
        function();
        // We can also use `self` to access another module inside `super_module`:
        self::cool::function();
        // The `super` keyword refers to the parent scope (outside the `super_module` module).
        super::super_function();
        // This will bind to the `super_cool::function` in the *crate* scope.
        // In this case the crate scope is the outermost scope (at main.rs)
        {
            use crate::a10_modules::super_cool::function as root_function;
            root_function();
        }
    }
}

fn super_and_self() {
    helpers::section_title(format!("`super` and `self`"));
    println!("The `super` and `self` keywords can be used in the path to remove ambiguity and prevent unnecessary hardcoding of paths");

    super_module::indirect_call();
}

use super::a10_modules_as_files as my_dir;
fn file_hierarchy() {
    helpers::section_subtitle(format!("File Hierarchy"));
    println!("Modules can be mapped to a file/dir structure");

    fn function() {
        println!("Called function()");
    }
    my_dir::function();
    function();
    my_dir::indirect_access();
    my_dir::nested::function();

    println!("\nThis is the same as the super example, but in different files.");
}
