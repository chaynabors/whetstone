// Comment
// Hello I'm a comment

// Assignment
let x = 10;
std::assert(x, 10);

// Print
let y = "hello"
std::assert(y, "hello");
std::print(y);

// Comparison
if y == "hello" then std::print("goodbye");

// Scope
{
    let y = "goodbye";
    std::assert(y, "goodbye");
}
std::assert(y, "hello");

// Function
fn hello_world() {
    std::print("hello_world");
}
hello_world();

// Function arg
fn print(str) {
    std::print(str);
}

// Function return
fn zero() -> zero {
    // Notice no semicolon here
    0
}

let zero = zero();

// Function return multiple
fn zero_one() -> [zero, one] {
    [0, 1]
}
let [zero, one] = zero_one();
let zero_one = zero_one();

// Optional return syntax
fn one() -> [] {
    return 1;
}
let one = one();

// Scope evaluation
let yes = {
    let no = false;
    !no
};
