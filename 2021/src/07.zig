const std = @import("std");
var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const min = std.math.min;

const MAX_INPUT_VALUE = 2000;

fn partOne(input: []const u8) !i32 {
    var positions = std.AutoHashMap(i32, i32).init(&gpa.allocator);
    defer positions.deinit();

    var token = std.mem.tokenize(u8, input, ",\n\r");
    var sum: i32 = 0;
    var count: i32 = 0;
    while (token.next()) |t| {
        const pos = try std.fmt.parseInt(i32, t, 10);
        var gop = try positions.getOrPutValue(pos, 0);
        gop.value_ptr.* += 1;

        sum += pos;
        count += 1;
    }

    var min_cost = sum;
    var left: i32 = 0;
    var right: i32 = count - (positions.get(0) orelse 0);

    var i: u16 = 1;
    while (i <= MAX_INPUT_VALUE) : (i += 1) {
        const next = positions.get(i) orelse 0;
        const center = count - left - right;
        min_cost += min(left - right + center, 0);
        left += center;
        right -= next;
    }
    std.debug.print("{}\n", .{min_cost});
    return min_cost;
}

inline fn triangle(x: i32) i32 {
    return (x * (x + 1)) >> 1;
}

fn partTwo(input: []const u8) !i32 {
    var positions = std.AutoArrayHashMap(i32, i32).init(&gpa.allocator);
    defer positions.deinit();

    var token = std.mem.tokenize(u8, input, ",\n\r");
    var sum: i32 = 0;
    while (token.next()) |t| {
        const pos = try std.fmt.parseInt(i32, t, 10);
        var gop = try positions.getOrPutValue(pos, 0);
        gop.value_ptr.* += 1;

        sum += triangle(pos);
    }

    var min_cost = sum;

    var i: u16 = 1;
    while (i <= MAX_INPUT_VALUE) : (i += 1) {
        var it = positions.iterator();
        var cost: i32 = 0;
        while (it.next()) |kv| {
            const k: i32 = try std.math.absInt(kv.key_ptr.* - i);
            const v = kv.value_ptr.*;
            cost += v * triangle(k);
        }
        min_cost = min(min_cost, cost);
    }
    std.debug.print("{}\n", .{min_cost});
    return min_cost;
}

pub fn main() !void {
    const data = @embedFile("../data/07.txt");
    _ = try partOne(data);
    _ = try partTwo(data);
}

test "test input" {
    const data = "16,1,2,0,4,2,7,1,2,14";
    try std.testing.expect(triangle(5) == 15);
    try std.testing.expect((try partOne(data)) == 37);
    try std.testing.expect((try partTwo(data)) == 168);
}
