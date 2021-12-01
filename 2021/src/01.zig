const std = @import("std");
const stdin = std.io.getStdIn().reader();
const stdout = std.io.getStdOut().writer();

const MAX_INPUT_SIZE = 20;

fn readInput(allocator: *std.mem.Allocator, reader: anytype) !std.ArrayList(u16) {
    var buf: [MAX_INPUT_SIZE]u8 = undefined;
    var list = std.ArrayList(u16).init(allocator);

    while (true) {
        const line = (try reader.readUntilDelimiterOrEof(&buf, '\n')) orelse return list;
        const num = try std.fmt.parseInt(u16, line, 10);
        try list.append(num);
    }
    unreachable;
}

fn part1(input: std.ArrayList(u16)) u16 {
    var a: u16 = std.math.maxInt(u16);
    var count: u16 = 0;
    for (input.items) |x| {
        if (x > a) {
            count += 1;
        }
        a = x;
    }
    return count;
}

fn part2(input: std.ArrayList(u16)) u16 {
    var a: u16 = std.math.maxInt(u16);
    var b: u16 = std.math.maxInt(u16);
    var c: u16 = std.math.maxInt(u16);
    var count: u16 = 0;
    for (input.items) |x| {
        if (x > a) {
            count += 1;
        }
        a = b;
        b = c;
        c = x;
    }
    return count;
}

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = &arena.allocator;

    const input = try readInput(allocator, stdin);
    defer input.deinit();
    // part 1
    const ans1 = part1(input);
    try stdout.print("{}\n", .{ans1});
    // part 2
    const ans2 = part2(input);
    try stdout.print("{}\n", .{ans2});
}

const expect = std.testing.expect;
const test_allocator = std.testing.allocator;

test "test input" {
    var list = std.ArrayList(u16).init(test_allocator);
    defer list.deinit();
    try list.append(199);
    try list.append(200);
    try list.append(208);
    try list.append(210);
    try list.append(200);
    try list.append(207);
    try list.append(240);
    try list.append(269);
    try list.append(260);
    try list.append(263);

    try expect(part1(list) == 7);
    try expect(part2(list) == 5);
}
