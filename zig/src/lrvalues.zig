const expect = @import("std").testing.expect;
const mem = @import("std").mem;

test "local variable" {
    var x: i32 = 1;
    const y = 2;

    x = x + y;

    expect(x == 3);
}

fn do_assign_var(r: *i32, v: i32) void {
    r.* = v;
}

test "function variable" {
    var x: i32 = 1;
    const y = 2;

    do_assign_var(&x, x + y);

    expect(x == 3);
}

test "local array" {
    var x = [_]i32{ 1, 1 };
    const y = 2;

    x[0] = x[0] + y;
    expect(mem.eql(i32, &x, &[_]i32{ 3, 1 }));
}

fn do_assign_array(r: []i32, v: i32) void {
    r[0] = v;
}

test "function array" {
    var x = [_]i32{ 1, 1 };
    const y = 2;

    do_assign_array(x[0..], x[0] + y);
    expect(mem.eql(i32, &x, &[_]i32{ 3, 1 }));
}

const S = struct {
    i: i32,
    b: bool,
};

test "local struct" {
    var x = S{ .i = 1, .b = true };
    const y = 2;
    x.i = x.i + y;

    expect(x.i == 3);
    expect(x.b == true);
}

fn do_assign_struct(r: *S, v: i32) void {
    r.i = v;
}

test "function struct" {
    var x = S{ .i = 1, .b = true };
    const y = 2;

    do_assign_struct(&x, x.i + y);

    expect(x.i == 3);
    expect(x.b == true);
}
