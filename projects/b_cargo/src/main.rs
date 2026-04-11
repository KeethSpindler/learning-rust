/*
Biggest take-aways from this section is cargo:
- cargo build: compiles the current project
- cargo run: compiles and runs the current project
- cargo check: checks for compilation errors without producing an executable

Also noted in this segment is the command for producing release builds
- cargo build --release: compiles the current project with optimizations for release builds 

I imagine I'll be using these commands a lot in this language
*/

fn main() {
    // I understand that this isn't going to be the best practice for adding strings for outputs and whatnot,
    // but for the time being I'm just trying to get the feel for the println macro and how it interpolates variables
    const GREETING: &str = "Hello, world from Project B!";
    const REMEMBER_STR : &str = "Remember, when using cargo there are three main commands:";
    println!("{}", GREETING);
    println!("{} {build}, {run}, {check}", REMEMBER_STR, build="cargo build", run="cargo run", check="cargo check");
}
