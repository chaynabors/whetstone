// Comment
// Hello I'm a comment

// Assignment
x = 10;
assert x == 10;

// Print
y = "hello"
assert y == "hello";

// Comparison
if y == "hello" then std::print("goodbye");

// Scope
{
    y = "goodbye";
    assert y == "goodbye";
}
assert y == "hello";

// Function
fn hello_world() {
    print "hello_world";
}

hello_world();

// Function arg
fn print(str) {
    print str;
}

// Function return
fn zero() -> zero {
    // Notice no semicolon here
    0
}

zero = zero();

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
