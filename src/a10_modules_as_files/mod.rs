mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called `my_dir::function()`");
}

fn private_function() {
    println!("called `my_dir::private_function()`");
}

pub fn indirect_access() {
    print!("called `my_dir::indirect_access()`, that\n> ");

    private_function();
}