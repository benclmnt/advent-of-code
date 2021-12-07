const std = @import("std");
const min = std.math.min;
const max = std.math.max;
const FIELD_SIZE = 1000;

const State = struct {
    field: [FIELD_SIZE][FIELD_SIZE]u2 = .{.{0} ** FIELD_SIZE} ** FIELD_SIZE,
    count: u32 = 0,

    inline fn update(self: *State, i: u16, j: u16) void {
        switch (self.field[i][j]) {
            2, 3 => return,
            1 => self.count += 1,
            0 => {},
        }
        self.field[i][j] += 1;
    }

    fn horizontal(self: *State, start: [2]u16, end: [2]u16) void {
        if (start[0] == end[0]) {
            var j = min(start[1], end[1]);
            while (j <= max(start[1], end[1])) : (j += 1) {
                self.update(start[0], j);
            }
        }
    }

    fn vertical(self: *State, start: [2]u16, end: [2]u16) void {
        if (start[1] == end[1]) {
            var i = min(start[0], end[0]);
            while (i <= max(start[0], end[0])) : (i += 1) {
                self.update(i, start[1]);
            }
        }
    }

    fn diagonal(self: *State, start: [2]u16, end: [2]u16) void {
        if ((start[0] == end[0]) or (start[1] == end[1])) return;
        if (start[0] > end[0]) return self.diagonal(end, start);
        var i = start[0];
        var j: i16 = @intCast(i16, start[1]);
        var delta: i8 = if (start[1] < end[1]) 1 else -1;
        while (i <= end[0]) : ({
            i += 1;
            j += delta;
        }) {
            self.update(i, @intCast(u16, j));
        }
    }
};

fn partOne(input: []const u8) !u32 {
    var coords = std.mem.tokenize(u8, input, "\n ->");
    var state = State{};

    while (coords.next()) |s| {
        const start = try parseCoord(s);
        const end = try parseCoord(coords.next().?);

        state.horizontal(start, end);
        state.vertical(start, end);
    }
    std.debug.print("{}\n", .{state.count});
    return state.count;
}

fn partTwo(input: []const u8) !u32 {
    var coords = std.mem.tokenize(u8, input, "\n ->");
    var state = State{};

    while (coords.next()) |s| {
        const start = try parseCoord(s);
        const end = try parseCoord(coords.next().?);

        state.horizontal(start, end);
        state.vertical(start, end);
        // only add this line for part 2.
        state.diagonal(start, end);
    }
    std.debug.print("{}\n", .{state.count});
    return state.count;
}

fn parseCoord(input: []const u8) ![2]u16 {
    var tokens = std.mem.tokenize(u8, input, ",");
    var coord: [2]u16 = undefined;
    coord[0] = try std.fmt.parseInt(u16, tokens.next().?, 10);
    coord[1] = try std.fmt.parseInt(u16, tokens.next().?, 10);
    return coord;
}

pub fn main() !void {
    const data = @embedFile("../data/05.txt");
    _ = try partOne(data);
    _ = try partTwo(data);
}

test "test input" {
    const data = @embedFile("../data/05_example.txt");
    var ans1 = try partOne(data);
    try std.testing.expect(ans1 == 5);
    var ans2 = try partTwo(data);
    try std.testing.expect(ans2 == 12);
}
