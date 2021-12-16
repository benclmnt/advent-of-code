const std = @import("std");
const min = std.math.min;
const max = std.math.max;
const Map = std.AutoHashMap;
var gpa = std.heap.GeneralPurposeAllocator(.{}){};
const FIELD_SIZE = 1000;

const State = struct {
    field: Map([2]u16, u2) = undefined,
    count: u32 = 0,

    fn init(self: *State, allocator: std.mem.Allocator) void {
        self.field = Map([2]u16, u2).init(allocator);
    }

    fn deinit(self: *State) void {
        self.field.deinit();
    }

    inline fn update(self: *State, i: u16, j: u16) !void {
        var gop = try self.field.getOrPutValue(.{ i, j }, 0);

        switch (gop.value_ptr.*) {
            2, 3 => return,
            1 => self.count += 1,
            0 => {},
        }
        gop.value_ptr.* += 1;
    }

    fn horizontal(self: *State, start: [2]u16, end: [2]u16) !void {
        if (start[0] == end[0]) {
            var j = min(start[1], end[1]);
            while (j <= max(start[1], end[1])) : (j += 1) {
                try self.update(start[0], j);
            }
        }
    }

    fn vertical(self: *State, start: [2]u16, end: [2]u16) !void {
        if (start[1] == end[1]) {
            var i = min(start[0], end[0]);
            while (i <= max(start[0], end[0])) : (i += 1) {
                try self.update(i, start[1]);
            }
        }
    }

    fn diagonal(self: *State, start: [2]u16, end: [2]u16) !void {
        if ((start[0] == end[0]) or (start[1] == end[1])) return;
        if (start[0] > end[0]) return self.diagonal(end, start);
        var i = start[0];
        var j: i16 = @intCast(i16, start[1]);
        var delta: i8 = if (start[1] < end[1]) 1 else -1;
        while (i <= end[0]) : ({
            i += 1;
            j += delta;
        }) {
            try self.update(i, @intCast(u16, j));
        }
    }
};

fn partOne(input: []const u8) !u32 {
    var coords = std.mem.tokenize(u8, input, "\n ->");
    var state = State{};
    state.init(gpa.allocator());
    defer state.deinit();

    while (coords.next()) |s| {
        const start = try parseCoord(s);
        const end = try parseCoord(coords.next().?);

        try state.horizontal(start, end);
        try state.vertical(start, end);
    }
    std.debug.print("{}\n", .{state.count});
    return state.count;
}

fn partTwo(input: []const u8) !u32 {
    var coords = std.mem.tokenize(u8, input, "\n ->");
    var state = State{};
    state.init(gpa.allocator());
    defer state.deinit();

    while (coords.next()) |s| {
        const start = try parseCoord(s);
        const end = try parseCoord(coords.next().?);

        try state.horizontal(start, end);
        try state.vertical(start, end);
        // only add this line for part 2.
        try state.diagonal(start, end);
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
