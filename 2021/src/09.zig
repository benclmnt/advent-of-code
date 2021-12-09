const std = @import("std");
const mem = std.mem;
const BitSet = std.bit_set.StaticBitSet;

const BOARD_SIZE = 128;

const Answer = struct { part1: u32 = 0, part2: u32 = 0 };

// takes in the top left corner of the 3x3
inline fn lowest(
    i: u8,
    j: u8,
    board: *[BOARD_SIZE][BOARD_SIZE]u8,
) bool {
    return board[i + 1][j + 1] < board[i][j + 1] and board[i + 1][j + 1] < board[i + 2][j + 1] and board[i + 1][j + 1] < board[i + 1][j] and board[i + 1][j + 1] < board[i + 1][j + 2];
}

const TopThree = struct {
    a: u32 = 0,
    b: u32 = 0,
    c: u32 = 0,
    fn add(self: *TopThree, d: u32) void {
        if (d > self.a) {
            self.c = self.b;
            self.b = self.a;
            self.a = d;
        } else if (d > self.b) {
            self.c = self.b;
            self.b = d;
        } else if (d > self.c) {
            self.c = d;
        }
    }
};

fn dfs(
    i: usize,
    j: usize,
    board: *[BOARD_SIZE][BOARD_SIZE]u8,
    bs: *BitSet(BOARD_SIZE * BOARD_SIZE),
) u32 {
    if (i < 0 or i >= BOARD_SIZE or j < 0 or j >= BOARD_SIZE) return 0;
    if (board[i][j] >= '9') return 0; // barrier

    var idx: u64 = @intCast(u64, (i + 1) * BOARD_SIZE + j + 1);
    if (bs.isSet(idx)) return 0;
    bs.set(idx);
    return 1 + dfs(i + 1, j, board, bs) + dfs(i - 1, j, board, bs) + dfs(i, j + 1, board, bs) + dfs(i, j - 1, board, bs);
}

fn solve(input: []const u8) Answer {
    var part1: u32 = 0;
    var topThree = TopThree{};

    var board: [BOARD_SIZE][BOARD_SIZE]u8 = .{.{std.math.maxInt(u8)} ** BOARD_SIZE} ** BOARD_SIZE;
    var width: usize = 0;
    var height: usize = 0;

    var lines = std.mem.tokenize(u8, input, "\n");
    while (lines.next()) |line| : (height += 1) {
        width = line.len;
        // give padding to each row.
        std.mem.copy(u8, board[height + 1][1 .. width + 1], line);
    }

    var i: u8 = 0;
    while (i < height) : (i += 1) {
        var j: u8 = 0;
        while (j < width) : (j += 1) {
            if (lowest(i, j, &board)) {
                part1 += (std.fmt.charToDigit(board[i + 1][j + 1], 10) catch unreachable) + 1;
                var bs = BitSet(BOARD_SIZE * BOARD_SIZE).initEmpty();
                var basin = dfs(i + 1, j + 1, &board, &bs);
                topThree.add(basin);
            }
        }
    }
    var part2 = topThree.a * topThree.b * topThree.c;
    std.debug.print("{} {}\n", .{ part1, part2 });
    return Answer{ .part1 = part1, .part2 = part2 };
}

pub fn main() void {
    const data = @embedFile("../data/09.txt");
    _ = solve(data);
}

test "test input" {
    const data = @embedFile("../data/09_example.txt");
    const ans = solve(data);
    try std.testing.expect(ans.part1 == 15);
    try std.testing.expect(ans.part2 == 1134);
}
