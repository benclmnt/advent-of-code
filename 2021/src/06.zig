const std = @import("std");

const State = struct {
    // each fish is represented by the number of days to its next reproduction
    fishes: [9]u64 = .{0} ** 9,

    fn nextDay(self: *State) void {
        const repro = self.fishes[0];
        for (self.fishes) |num_fish, i| {
            if (i == 0) continue;
            self.fishes[i - 1] = num_fish;
        }
        self.fishes[6] += repro;
        self.fishes[8] = repro;
    }

    fn total(self: *State) u64 {
        var count: u64 = 0;
        for (self.fishes) |num_fish| {
            count += num_fish;
        }
        return count;
    }
};

fn partOne(input: []const u8, days: u32) !u64 {
    var state = State{};
    var token = std.mem.tokenize(u8, input, ",\n\r");
    while (token.next()) |t| {
        const fish = try std.fmt.parseInt(u8, t, 10);
        state.fishes[fish] += 1;
    }

    var i: u32 = 0;
    while (i < days) : (i += 1) {
        state.nextDay();
    }
    const total = state.total();
    std.debug.print("{}\n", .{total});
    return total;
}

pub fn main() !void {
    const data = @embedFile("../data/06.txt");
    _ = try partOne(data, 80);
    _ = try partOne(data, 256); // this is part two.
}

test "test input" {
    const data = "3,4,3,1,2";
    try std.testing.expect((try partOne(data, 18)) == 26);
    try std.testing.expect((try partOne(data, 80)) == 5934);
    try std.testing.expect((try partOne(data, 256)) == 26984457539);
}
