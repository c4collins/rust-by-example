pub fn example_title (title: String) {
    let length = title.len() + 4;
    println!("\n\t/{:*<1$}\\", "", length);
    println!("\t|{: ^1$}|", title, length);
    println!("\t\\{:*<1$}/\n", "", length);
}

pub fn section_title (title: String) {
    let length = title.len() + 4;
    println!("\n\t/{:-<1$}\\", "", length);
    println!("\t|{: ^1$}|", title, length);
    println!("\t\\{:-<1$}/\n", "", length);
}

pub fn section_subtitle (title: String) {
    let length = title.len();
    println!("\n\t{: ^1$}", title, length);
    println!("\t{:=<1$}\n", "", length);
}