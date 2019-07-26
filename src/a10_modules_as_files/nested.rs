pub fn function() {
    println!("called `my_dir::nested::function()`");
}

#[allow(dead_code)]
fn private_function() {
    println!("called `my_dir::nested::private_function()`");
}