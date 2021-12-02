const std = @import("std");
const stdout = std.io.getStdOut().writer();
const tokenize = std.mem.tokenize;
const eql = std.mem.eql;
const parseInt = std.fmt.parseInt;

fn part1(input: anytype) !i32 {
    var lines = tokenize(u8, input, "\r\n");
    var x: i32 = 0;
    var y: i32 = 0;
    while (lines.next()) |line| {
        var it = tokenize(u8, line, " ");
        var dir = it.next().?;
        var amt = try parseInt(i32, it.next().?, 10);
        if (eql(u8, dir, "forward")) {
            x += amt;
        } else if (eql(u8, dir, "up")) {
            y -= amt;
        } else if (eql(u8, dir, "down")) {
            y += amt;
        } else {
            unreachable;
        }
    }
    return x * y;
}

fn part2(input: anytype) !i32 {
    var lines = tokenize(u8, input, "\r\n");
    var x: i32 = 0;
    var y: i32 = 0;
    var aim: i32 = 0;
    while (lines.next()) |line| {
        var it = tokenize(u8, line, " ");
        var dir = it.next().?;
        var amt = try parseInt(i32, it.next().?, 10);
        if (eql(u8, dir, "forward")) {
            x += amt;
            y += aim * amt;
        } else if (eql(u8, dir, "up")) {
            aim -= amt;
        } else if (eql(u8, dir, "down")) {
            aim += amt;
        } else {
            unreachable;
        }
    }
    return x * y;
}

pub fn main() !void {
    const data = @embedFile("../data/02.txt");

    var ans1 = try part1(data);
    var ans2 = try part2(data);
    try stdout.print("part1: {}, part2: {}\n", .{ ans1, ans2 });
}

const expect = std.testing.expect;
test "test input" {
    const input =
        \\forward 5
        \\down 5
        \\forward 8
        \\up 3
        \\down 8
        \\forward 2
    ;
    var ans1 = try part1(input);
    var ans2 = try part2(input);
    try expect(ans1 == 150);
    try expect(ans2 == 900);
}
