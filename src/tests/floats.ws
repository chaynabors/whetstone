a = 0.0;
assert a == 0.0;

a = 0.;
assert a == 0.0;

a = .0;
assert a == .0;

a = 1.0;
b = 10.0;
assert a + b == 11.0;
assert a - b == -9.0;
assert a * b == 10.0;
assert a / b == 0.1;
