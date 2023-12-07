const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const RaceResults = struct {
    duration: u64,
    record: u64,
};

inline fn isNumber(c: u8) bool {
    return '0' <= c and c <= '9';
}

fn parseNumber(line: []const u8) !u64 {
    var i: usize = 0;

    var number: u64 = 0;
    while (i < line.len) : (i += 1) {
        if (!isNumber(line[i])) continue;
        number = number * 10 + (line[i] - '0');
    }

    return number;
}

fn parseRaceResults(input: []const u8) !RaceResults {
    var line_it = std.mem.splitScalar(u8, input, '\n');
    const duration = try parseNumber(line_it.next().?);
    const record = try parseNumber(line_it.next().?);

    return RaceResults{ .duration = duration, .record = record };
}

fn computeRoots(duration: f64, record: f64) [2]f64 {
    const d = duration * duration - 4 * record;
    const x1 = (duration - std.math.sqrt(d)) / 2.0;
    const x2 = (duration + std.math.sqrt(d)) / 2.0;

    return .{ x1, x2 };
}

fn run(input: [:0]const u8) i64 {
    const race_results = parseRaceResults(input) catch unreachable;

    const roots = computeRoots(@floatFromInt(race_results.duration), @floatFromInt(race_results.record));
    const min = @ceil(roots[0]);
    const max = @floor(roots[1]);

    return @intFromFloat(max - min + 1);
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
    try std.testing.expectEqual(@as(i64, 71503), result);
}
