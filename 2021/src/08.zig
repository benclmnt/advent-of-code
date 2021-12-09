const std = @import("std");
const StrMap = std.StringHashMap;
const BitSet = std.bit_set.IntegerBitSet;

fn partOne(input: []const u8) u32 {
    var lines = std.mem.tokenize(u8, input, "|\n\r");
    var count: u32 = 0;
    while (lines.next()) |_| { // skip unique patterns
        var tokens = std.mem.tokenize(u8, lines.next().?, " ");
        while (tokens.next()) |t| {
            if (t.len <= 4 or t.len == 7) { // 1, 4, 7, 8
                count += 1;
            }
        }
    }
    std.debug.print("{}\n", .{count});
    return count;
}

//   0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
//  ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....

//   5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
//  gggg    gggg    ....    gggg    gggg

/// getMask represents whether segments a to g are toggled.
/// For example 1 is represented as 0100100 (on my machine)
inline fn getMask(input: []const u8) u7 {
    var bs = BitSet(7).initEmpty();
    for (input) |c| {
        bs.set(c - 'a');
    }
    return bs.mask;
}

fn partTwo(input: []const u8) u32 {
    var lines = std.mem.tokenize(u8, input, "|\n\r");
    var total: u32 = 0;

    while (lines.next()) |patterns| {
        var tokens = std.mem.tokenize(u8, patterns, " ");
        var four: u7 = undefined;
        var seven: u7 = undefined;
        while (tokens.next()) |t| {
            var mask: u7 = getMask(t);
            switch (t.len) {
                3 => seven = mask,
                4 => four = mask,
                else => continue,
            }
        }

        var number: u32 = 0;
        tokens = std.mem.tokenize(u8, lines.next().?, " ");
        while (tokens.next()) |t| {
            var mask: u7 = getMask(t);
            var digit: u8 = switch (t.len) {
                2 => 1,
                3 => 7,
                4 => 4,
                5 => blk: {
                    if (mask & seven == seven) break :blk @as(u8, 3);
                    if (@popCount(u7, mask & four) == 2) break :blk @as(u8, 2);
                    break :blk @as(u8, 5);
                },
                6 => blk: {
                    if (mask & seven != seven) break :blk @as(u8, 6);
                    if (mask & four == four) break :blk @as(u8, 9);
                    break :blk @as(u8, 0);
                },
                7 => 8,
                else => unreachable,
            };
            number = number * 10 + digit;
        }
        total += number;
    }
    std.debug.print("{}\n", .{total});
    return total;
}

pub fn main() void {
    const data = @embedFile("../data/08.txt");
    _ = partOne(data);
    _ = partTwo(data);
}

test "test input" {
    const data = @embedFile("../data/08_example.txt");
    try std.testing.expect(partOne(data) == 26);
    try std.testing.expect(partTwo(data) == 61229);
}
