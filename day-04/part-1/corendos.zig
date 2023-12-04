const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in
const stderr = std.io.getStdErr().writer();

fn runLine(line: []const u8) !i64 {
    var winning_numbers = [_]u8{0} ** 100;
    var result: i64 = 0;
    var it = std.mem.splitScalar(u8, line, ' ');

    // Skip garbage
    _ = it.next();
    while (it.next()) |t| {
        if (t.len != 0) break;
    }

    var winning_number_done = false;

    while (it.next()) |token| {
        if (token.len == 0) continue;
        if (std.mem.eql(u8, token, "|")) {
            winning_number_done = true;
        } else {
            const number = try std.fmt.parseInt(u8, token, 10);
            if (!winning_number_done) {
                winning_numbers[number] = 1;
            } else {
                if (winning_numbers[number] == 1) {
                    if (result == 0) {
                        result = 1;
                    } else {
                        result = result * 2;
                    }
                }
            }
        }
    }

    return result;
}

fn run(input: [:0]const u8) i64 {
    var result: i64 = 0;

    var line_it = std.mem.splitScalar(u8, input, '\n');
    while (line_it.next()) |line| {
        result += runLine(line) catch unreachable;
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
        \\Card  1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        \\Card  2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        \\Card  3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        \\Card  4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        \\Card  5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        \\Card  6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    ;
    const result = run(input);
    try std.testing.expectEqual(@as(i64, 13), result);
}
