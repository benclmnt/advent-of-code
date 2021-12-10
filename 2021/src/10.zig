const std = @import("std");

const Answer = struct { part1: u32 = 0, part2: u64 = 0 };
const MAX_LINE_LEN = 128;

const Stack = struct {
    next: usize = 0,
    arr: [MAX_LINE_LEN / 2]u8 = .{0} ** (MAX_LINE_LEN / 2),
    fn init(self: *Stack) void {
        self.next = 0;
    }
    fn push(self: *Stack, c: u8) !void {
        if (self.next == MAX_LINE_LEN / 2) return error.IndexOutOfBounds;
        self.arr[self.next] = c;
        self.next += 1;
    }
    fn pop(self: *Stack) !u8 {
        if (self.next == 0) return error.IndexOutOfBounds;
        self.next -= 1;
        return self.arr[self.next];
    }
};

fn corruptedScore(c: u8) u32 {
    return switch (c) {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        else => 0,
    };
}

fn incompleteScore(c: u8) u32 {
    return switch (c) {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        else => 0,
    };
}

fn solve(input: []const u8) Answer {
    var stack = Stack{};
    var part1: u32 = 0;

    var incompletes_len: u8 = 0;
    var incompletes: [128]u64 = undefined;

    var lines = std.mem.tokenize(u8, input, "\r\n");
    outer: while (lines.next()) |line| {
        defer stack.init();
        for (line) |c| {
            switch (c) {
                '(', '[', '{', '<' => stack.push(c) catch unreachable,
                ')', ']', '}', '>' => {
                    var d = stack.pop() catch unreachable;
                    // expected opening pair
                    var expected: u8 = c - @as(u8, (if (c == ')') 1 else 2));
                    if (d != expected) {
                        part1 += corruptedScore(c);
                        continue :outer;
                    }
                },
                else => unreachable,
            }
        }

        var incomplete: u64 = 0;
        while (stack.next > 0) {
            incomplete = incomplete * 5 + incompleteScore((stack.pop() catch unreachable));
        }
        incompletes[incompletes_len] = incomplete;
        incompletes_len += 1;
    }

    std.sort.sort(u64, incompletes[0..incompletes_len], {}, comptime std.sort.asc(u64));
    return Answer{ .part1 = part1, .part2 = incompletes[incompletes_len >> 1] };
}

pub fn main() void {
    const data = @embedFile("../data/10.txt");
    var ans = solve(data);
    std.debug.print("{}\n", .{ans});
}

test "test input" {
    const data = @embedFile("../data/10_example.txt");
    var ans = solve(data);
    try std.testing.expect(ans.part1 == 26397);
    try std.testing.expect(ans.part2 == 288957);
}
