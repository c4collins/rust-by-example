use super::helpers;

pub fn run(){
    helpers::example_title(format!("Example 12: Cargo"));
    println!("`cargo` is the official Rust package management tool");
    println!("It has a lot of useful features to increase the speed and quality of development:");
    println!("\tDependency management and integration with crates.io (The official Rust package registry)");
    println!("\tAwareness of unit tests");
    println!("\tAwareness of benchmarks");
    println!("\nThis example is just a short overview of `cargo` and is mostly just text because cargo is external to a rust program");

    creating_a_project();
    cargo_toml();
    dependencies();
    building_a_project();
    conventions();
    testing();
    build_scripts();
}

fn creating_a_project(){
    helpers::section_title(format!("Creating a project"));
    println!("The first time you'll encounter `cargo` is when creating a new project");
    println!("You can create binaries (programs that run) or libraries (helpers to be included in binaries)");
    println!("You create a new Rust project by running this command, where foo is the name of your project:");
    println!("\tcargo new foo");
    println!("That will create the framework for a binary.  To create a library use:");
    println!("\tcargo new --lib foo");

    println!("\nAfter running either of these commands you'll generate a file structure like this:");
    println!("\tfoo/");
    println!("\t ├── Cargo.toml");
    println!("\t └── src/");
    println!("\t      └── main.rs");
    println!("main.rs - root source file of the new project");
    println!("Cargo.toml - cargo config file for this project (\"foo\")");
}
fn cargo_toml() {
    helpers::section_subtitle(format!("Cargo.toml"));
    println!("The Cargo.toml file contains all of the cargo-related settings for your project");
    println!("It's very similar to the package.json files created by npm");
    println!("When you look inside the Cargo.toml for a new project it will look like this:");
    println!("\t[package]");
    println!("\tname = \"foo\"");
    println!("\tversion = \"0.1.0\"");
    println!("\tauthors = [\"connor\"]");
    println!("\n\t[dependencies]");

    println!("\nThe [package] section sets the descriptive properties of the project, especially for publishing as a crate on crates.io");
    println!("name - The name of the package.  It will be the name of the binary you output when you compile, and the name of your package on crates.io (if you publish it)");
    println!("version - The version of the crate/project.  Uses semantic versioning");
    println!("authors - The list of authors to show when publishing the crate");
    println!("\nThe [dependencies] section allows you to list dependencies for your project");

    println!("\nAll of the available configuration options are available at: https://doc.rust-lang.org/cargo/reference/manifest.html");
}
fn dependencies(){
    helpers::section_subtitle(format!("Dependencies"));
    println!("To add a dependency to our project, we need to look up the version we want to include");
    println!("And then add that dependency to the project's Cargo.toml in the [dependencies] section");
    println!("\t[dependencies]");
    println!("\tcrate_name = \"0.0.0\"");
    println!("\nThere are other ways to add libraries as well, such as from git or from the local filesystem");
    println!("e.g. to included `clap` from crates.io, `rand` from git, and `bar` from local:");
    println!("\t[dependencies]");
    println!("\tclap = \"2.33.0\"");
    println!("\trand = {{ git = \"https://github.com/rust-lang-nursery/rand\" }}");
    println!("\tbar = {{ path = \"../bar\" }}");
}

fn building_a_project(){
    helpers::section_subtitle(format!("Building your project with cargo"));
    println!("`cargo` is more than just a dependency manager.");
    println!("These commands will resolve dependencies, download necessary crates, and rebuilds only what is necessary");

    println!("\nTo build a project, run `cargo build` from anywhere in the project");
    println!("To build and run a project, run `cargo run`");
}

fn conventions(){
    helpers::section_title(format!("More than one Binary"));
    println!("`src/main.rs` is the default binary location");
    println!("If you want more binaries in your project you can add a bin/ dir to src/ and store them there");
    println!("\tsrc/");
    println!("\t ├── main.rs");
    println!("\t └── bin/");
    println!("\t      └── other_bin.rs");
    println!("Then run `cargo build --bin other_bin` to build it or with run to run and build the extra binary");
}

fn testing(){
    helpers::section_title(format!("Testing"));
    println!("Rust has firt-class support for unit and integration testing");
    println!("I'm not going to get into how to write tests here, but just how to store and run them");
    println!("Organizationally, Rust projects keep their tests in a `tests/` dir that's sibling to `src/`");
    println!("\tfoo/");
    println!("\t ├── Cargo.toml");
    println!("\t └── src/");
    println!("\t      └── main.rs");
    println!("\t └── tests/");
    println!("\t      └── a_test.rs");
    println!("\t      └── another_test.rs");
    println!("\t      └── another.rs");

    println!("To run all the tests, simply run `cargo test`");
    println!("To run a single test, just name it like `cargo test another_test`");
    println!("The name you give is actually a pattern match, so `cargo test ano` would run `another` and `another_test`");
    println!("\nBe aware: Tests are run concurrently, so they shouldn't conflict or race with one another");

}

fn build_scripts(){
    helpers::section_title(format!("Build Scripts"));
    println!("Sometimes the automated build from cargo is not enough.");
    println!("Maybe you need some prerequities installed, like code generation or including same native code that needs to be compiled");
    println!("To solve this problem you can write a build script for cargo to run");

    println!("\nTo add a build script to your package you can either create a file called `build.rs` in the project directory or update the Cargo.toml with:");
    println!("\t[package]");
    println!("\t...");
    println!("\tbuild = \"build.rs\"");
    println!("That would do the same as the default behaviour");
    
    println!("\nThe build script is a Rust file that is compiled and then invoked prior to compiling anything else in the package.");
    println!("It is generally used to ensure all build prerequisites are met before building the project/crate.");

    println!("\nThe build script outputs to stdout and the output will also be logged to `target/debug/build/<pkg>/output`");
    println!("Lines prefixed with `cargo:` will be interpreted by cargo directly and can be used to define parameters for the packages compilation");

    build_script_env_vars();
}

fn build_script_env_vars(){
    helpers::section_subtitle(format!("Build script environment variables"));
    println!("The build script environment contains many useful variables:");
    println!("\tCARGO - path to the cargo binary performing the build");
    println!("\tCARGO_MANIFEST_DIR - directory containing the manifest for the package, and the directory the build script is run in");
    println!("\tCARGO_MANIFEST_LINKS - The manifest `links` value");
    println!("\tCARGO_FEATURE_<name> - For each activated feature of the package being built, there will be an envvar where `<name>` is the feature name uppercased and `-` changed to `_`");
    println!("\tCARGO_CFG_<cfg> - For each configuration option of the package being built, there will be an envvar with the name rules as CARGO_FEATURE_<name> plus Booleans only exist if they're true");
    println!("\t\tConfigurations with multiple values are returned joined with a comma");
    println!("\tOUT_DIR - the folder in which the output will be placed.  It's in the `build/` dir, but is unique for each package");
    println!("\tTARGET - the target triple being compiled for");
    println!("\tHOST - the host triple of the rust compiler");
    println!("\tNUM_JOBS - top-level parallelism (probably no longer needed)");
    println!("\tOPT_LEVEL, DEBUG - values of the corresponsing variables for the profile being built");
    println!("\tPROFILE - `release` or `debug`");
    println!("\tDEP_<name>_<key> - Build script links");
    println!("\tRUSTC, RUSTDOC - Compiler and Documentation Genrator that Cargo is using");
    println!("\tRUSTC_LINKER - path to the linker binary that Cargo has resolved to use for the current target, if specified");

    println!("\nTo use environment varibles in Rust you do it like this:");
    println!("\tuse std::env;");
    println!("\tlet var_name = env::var(\"ENV_VAR_NAME\").unwrap();");
}
