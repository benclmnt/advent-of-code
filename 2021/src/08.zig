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

/// deduce patterns from input and output the translation from the patterns seen.
fn deduce(input: []const u8) [10]u7 {
    var translation: [10]u7 = undefined; // translate digits -> mask
    var len_5: [3]u7 = undefined; // 2, 3, 5
    var len_6: [3]u7 = undefined; // 0, 6, 9

    {
        var tokens = std.mem.tokenize(u8, input, " ");
        var i: u8 = 0;
        var j: u8 = 0;
        var k: u8 = 0;
        while (tokens.next()) |t| : (i += 1) {
            var mask = getMask(t);
            switch (t.len) {
                2 => translation[1] = mask,
                3 => translation[7] = mask,
                4 => translation[4] = mask,
                5 => {
                    len_5[j] = mask;
                    j += 1;
                },
                6 => {
                    len_6[k] = mask;
                    k += 1;
                },
                7 => translation[8] = mask,
                else => unreachable,
            }
        }
    }

    for (len_6) |i| {
        if (@popCount(u7, translation[1] ^ translation[7] | translation[4] ^ i) == 1) {
            translation[9] = i;
        } else if (i & translation[1] != translation[1]) {
            translation[6] = i;
        } else {
            translation[0] = i;
        }
    }

    for (len_5) |i| {
        if (i & translation[1] == translation[1]) {
            translation[3] = i;
        } else if (@popCount(u7, translation[9] ^ translation[1] ^ i) == 1) {
            translation[5] = i;
        } else {
            translation[2] = i;
        }
    }

    return translation;
}

fn translate(input: []const u8, translation: [10]u7) u32 {
    var tokens = std.mem.tokenize(u8, input, " ");
    var digits: [4]u16 = undefined;

    var i: u8 = 0;
    while (tokens.next()) |t| : (i += 1) {
        var mask = getMask(t);
        for (translation) |tr, j| {
            if (tr == mask) {
                digits[i] = @intCast(u16, j);
                break;
            }
        }
    }
    return 1000 * digits[0] + 100 * digits[1] + 10 * digits[2] + digits[3];
}

fn partTwo(input: []const u8) u32 {
    var lines = std.mem.tokenize(u8, input, "|\n\r");
    var total: u32 = 0;

    while (lines.next()) |patterns| {
        var translation = deduce(patterns);
        total += translate(lines.next().?, translation);
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
