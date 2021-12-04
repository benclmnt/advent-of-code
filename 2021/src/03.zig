const std = @import("std");

fn partOne(input: []const u8) !u32 {
    var zeros: [16]u16 = .{0} ** 16;
    var ones = [_]u16{0} ** 16;
    var len: usize = 0;

    var lines = std.mem.tokenize(u8, input, "\r\n");
    while (lines.next()) |line| {
        len = line.len;
        for (line) |bit, i| {
            if (bit == '0') {
                zeros[i] += 1;
            } else if (bit == '1') {
                ones[i] += 1;
            }
        }
    }

    var gamma: u32 = 0;
    var epsilon: u32 = 0;

    for (zeros) |cnt, i| {
        if (i >= len) break;

        const mask = @shlExact(@as(u32, 1), @intCast(u4, len - 1 - i));
        if (cnt < ones[i]) gamma |= mask else epsilon |= mask;
    }

    return gamma * epsilon;
}

pub fn main() !void {
    const data = @embedFile("../data/03.txt");
    const ans1 = try partOne(data);
    std.debug.print("{} {}", .{ ans1, 0 });
}

test "test input" {
    const data = @embedFile("../data/03_example.txt");
    const ans1 = try partOne(data);
    try std.testing.expect(ans1 == 198);
}
