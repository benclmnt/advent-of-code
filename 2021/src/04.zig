const std = @import("std");

const State = struct {
    order_len: u8 = 0,
    order: [128]u8 = undefined,
    boards: [128]Bingo = undefined,
    boards_won: [128]bool = .{false} ** 128,
    board_len: u8 = 0,
    boards_left: u8 = undefined,
};

const Bingo = struct {
    board: [5][5]u8 = undefined,
    marked: [5][5]bool = .{.{false} ** 5} ** 5,

    fn mark(self: *Bingo, n: u8) void {
        for (self.board) |row, i| {
            for (row) |cell, j| {
                if (cell == n) {
                    self.marked[i][j] = true;
                    return;
                }
            }
        }
    }
    fn countUnmarked(self: *const Bingo) u32 {
        var sum: u32 = 0;
        for (self.board) |row, i| {
            for (row) |cell, j| {
                if (!self.marked[i][j]) {
                    sum += cell;
                }
            }
        }
        return sum;
    }

    fn checkHorizontal(self: *const Bingo) bool {
        var i: u8 = 0;
        outer: while (i < 5) : (i += 1) {
            var j: u8 = 0;
            while (j < 5) : (j += 1) {
                if (!self.marked[i][j]) continue :outer;
            }
            return true;
        }
        return false;
    }

    fn checkVertical(self: *const Bingo) bool {
        var j: u8 = 0;
        outer: while (j < 5) : (j += 1) {
            var i: u8 = 0;
            while (i < 5) : (i += 1) {
                if (!self.marked[i][j]) continue :outer;
            }
            return true;
        }
        return false;
    }

    fn checkWin(self: *const Bingo) bool {
        return self.checkHorizontal() or self.checkVertical();
    }
};

fn parseInput(input: []const u8) !State {
    var state = State{};
    var lines = std.mem.tokenize(u8, input, "\r\n");

    var orders = std.mem.tokenize(u8, lines.next().?, ",");
    var order_len: u8 = 0;
    while (orders.next()) |order| : (order_len += 1) {
        state.order[order_len] = std.fmt.parseInt(u8, order, 10) catch unreachable;
    }
    state.order_len = order_len;

    var board_len: u8 = 0;
    while (parseBoard(&lines)) |board| : (board_len += 1) {
        state.boards[board_len] = board;
    }
    state.board_len = board_len;
    state.boards_left = board_len;
    return state;
}

fn parseBoard(lines: *std.mem.TokenIterator(u8)) ?Bingo {
    var bingo = Bingo{};
    var i: u8 = 0;
    while (i < 5) : (i += 1) {
        if (lines.next()) |line| {
            var cells = std.mem.tokenize(u8, line, " ");
            var j: u8 = 0;
            while (cells.next()) |cell| : (j += 1) {
                bingo.board[i][j] = std.fmt.parseInt(u8, cell, 10) catch unreachable;
            }
        } else return null;
    }
    return bingo;
}

fn partOne(state: *State) u32 {
    for (state.order) |order, i| {
        if (i >= state.order_len) break;
        for (state.boards) |*bingo, j| {
            if (j >= state.board_len) break;
            bingo.mark(order);
            if (bingo.checkWin()) {
                std.debug.print("{} {}\n", .{ order, bingo.countUnmarked() });
                return bingo.countUnmarked() * order;
            }
        }
    }
    unreachable;
}

fn partTwo(state: *State) u32 {
    for (state.order) |order, i| {
        if (i >= state.order_len) break;
        for (state.boards) |*bingo, j| {
            if (j >= state.board_len) break;
            if (state.boards_won[j]) continue;
            bingo.mark(order);
            if (bingo.checkWin()) {
                state.boards_won[j] = true;
                state.boards_left -= 1;
            }
            if (state.boards_left == 0) {
                std.debug.print("{} {}\n", .{ order, bingo.countUnmarked() });
                return bingo.countUnmarked() * order;
            }
        }
    }
    unreachable;
}

pub fn main() !void {
    const data = @embedFile("../data/04.txt");
    var state = try parseInput(data);
    var ans1 = partOne(&state);
    state = try parseInput(data);
    var ans2 = partTwo(&state);
    std.debug.print("{} {}\n", .{ ans1, ans2 });
}

test "test input" {
    const data = @embedFile("../data/04_example.txt");
    var state = try parseInput(data);
    var ans1 = partOne(&state);
    state = try parseInput(data);
    var ans2 = partTwo(&state);
    std.debug.print("{} {}\n", .{ ans1, ans2 });
    try std.testing.expect(ans1 == 4512);
    try std.testing.expect(ans2 == 1924);
}
