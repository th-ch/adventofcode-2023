const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const RaceResults = struct {
    durations: []u64,
    record: []u64,
};

inline fn isNumber(c: u8) bool {
    return '0' <= c and c <= '9';
}

fn parseNumbers(line: []const u8) ![]u64 {
    var it = std.mem.splitScalar(u8, line, ' ');

    var numbers = std.ArrayList(u64).init(a);
    while (it.next()) |token| {
        if (token.len == 0 or !isNumber(token[0])) continue;
        const number = try std.fmt.parseInt(u64, token, 10);
        try numbers.append(number);
    }

    return try numbers.toOwnedSlice();
}

fn parseRaceResults(input: []const u8) !RaceResults {
    var line_it = std.mem.splitScalar(u8, input, '\n');
    const durations = try parseNumbers(line_it.next().?);
    const record = try parseNumbers(line_it.next().?);

    return RaceResults{ .durations = durations, .record = record };
}

fn computeDistanceTravelled(duration: u64, holding_duration: u64) u64 {
    std.debug.assert(holding_duration <= duration);
    const speed = holding_duration;
    const distance_travelled = (duration - holding_duration) * speed;
    return distance_travelled;
}

fn run(input: [:0]const u8) i64 {
    const race_results = parseRaceResults(input) catch unreachable;

    var result: i64 = 1;
    for (race_results.durations, race_results.record) |duration, record| {
        var winning_ways: i64 = 0;
        for (0..duration) |holding_duration| {
            const distance_travelled = computeDistanceTravelled(duration, holding_duration);
            if (distance_travelled > record) {
                winning_ways += 1;
            }
        }
        result *= winning_ways;
    }

    return result;
}

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator); // create memory allocator for strings

    defer arena.deinit(); // clear memory
    a = arena.allocator();

    var arg_it = try std.process.argsWithAllocator(a);
    _ = arg_it.skip(); // skip over exe name
    const input: [:0]const u8 = arg_it.next().?;

    const start: i128 = std.time.nanoTimestamp(); // start time
    const answer = run(input); // compute answer
    const end: i128 = std.time.nanoTimestamp();
    const elapsed_nano: f64 = @floatFromInt(end - start);
    const elapsed_milli = elapsed_nano / 1_000_000.0;
    try stdout.print("_duration:{d}\n{}\n", .{ elapsed_milli, answer }); // emit actual lines parsed by AOC
}

test {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator); // create memory allocator for strings

    defer arena.deinit(); // clear memory
    a = arena.allocator();

    const input =
        \\Time:      7  15   30
        \\Distance:  9  40  200
    ;

    const result = run(input);
    try std.testing.expectEqual(@as(i64, 288), result);
}
