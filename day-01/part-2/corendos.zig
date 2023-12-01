const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn runLine(line: []const u8) i64 {
    var first_digit: ?u8 = null;
    var last_digit: ?u8 = null;

    for (line, 0..) |c, i| {
        const maybe_digit: ?u8 = switch (c) {
            '0'...'9' => c - '0',
            'o' => if (i + 3 <= line.len and std.mem.eql(u8, line[i .. i + 3], "one"))
                1
            else
                null,
            't' => if (i + 3 <= line.len and std.mem.eql(u8, line[i .. i + 3], "two"))
                2
            else if (i + 5 <= line.len and std.mem.eql(u8, line[i .. i + 5], "three"))
                3
            else
                null,
            'f' => if (i + 4 <= line.len and std.mem.eql(u8, line[i .. i + 4], "four"))
                4
            else if (i + 4 <= line.len and std.mem.eql(u8, line[i .. i + 4], "five"))
                5
            else
                null,
            's' => if (i + 3 <= line.len and std.mem.eql(u8, line[i .. i + 3], "six"))
                6
            else if (i + 5 <= line.len and std.mem.eql(u8, line[i .. i + 5], "seven"))
                7
            else
                null,
            'e' => if (i + 5 <= line.len and std.mem.eql(u8, line[i .. i + 5], "eight"))
                8
            else
                null,
            'n' => if (i + 4 <= line.len and std.mem.eql(u8, line[i .. i + 4], "nine"))
                9
            else
                null,
            else => null,
        };

        if (maybe_digit) |digit| {
            if (first_digit == null) {
                first_digit = digit;
            }
            last_digit = digit;
        }
    }
    return first_digit.? * 10 + last_digit.?;
}

fn run(input: [:0]const u8) i64 {
    var line_it = std.mem.splitScalar(u8, input, '\n');
    var result: i64 = 0;

    while (line_it.next()) |line| {
        result += runLine(line);
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
    const input =
        \\two1nine
        \\eightwothree
        \\abcone2threexyz
        \\xtwone3four
        \\4nineeightseven2
        \\zoneight234
        \\7pqrstsixteen
    ;
    const result = run(input);
    try std.testing.expectEqual(@as(i64, 281), result);
}
