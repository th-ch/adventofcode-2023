const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn run(input: [:0]const u8) i64 {
    var line_it = std.mem.splitScalar(u8, input, '\n');
    var result: i64 = 0;
    while (line_it.next()) |line| {
        var first_digit: ?u8 = null;
        var last_digit: ?u8 = null;
        for (line) |c| switch (c) {
            '0'...'9' => {
                if (first_digit == null) {
                    first_digit = c - '0';
                }
                last_digit = c - '0';
            },
            else => {},
        };
        result += first_digit.? * 10 + last_digit.?;
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
        \\1abc2
        \\pqr3stu8vwx
        \\a1b2c3d4e5f
        \\treb7uchet
    ;
    const result = run(input);
    try std.testing.expectEqual(@as(i64, 142), result);
}
