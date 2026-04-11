fn main(){
    /*
    Was curious about why this was a pointer to a string rather than just a string
    I figured we needed to instantiate the string first before we could use the pointer,
    but it turns out when making a literal, Rust automatically allocates the memory.

    When trying to type this as just a string, the compiler threw a fit.
         const greeting: str = "Hello, world from first Rust project!";
         `the size for values of type `str` cannot be known at compilation time`

    I'm guessing unlike Python, Rust needs to know exactly how much memory to
    allocate for a string and won't allow for the continuous resizing that Python
    allows out of the box. 
    */
    const HELLO_WORLD: &str = "Hello, world from first Rust project!";
    const AUTHOR: &str = "Author: Keeth Spindler";
    const DATE: &str = "Date: 2026-04-11";
    println!("{} | {author}, {date}", HELLO_WORLD, author=AUTHOR, date=DATE);
}