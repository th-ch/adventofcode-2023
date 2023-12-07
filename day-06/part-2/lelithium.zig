const std = @import("std");
const builtin = @import("builtin");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const ACCUMULATOR_SIZE = 50;

inline fn get_solution_count(t: u64, d: u64) u64 {
    const half_t: f64 = @as(f64, @floatFromInt(t)) / 2.0;
    const half_delta: f64 = @sqrt(half_t * half_t - @as(f64, @floatFromInt(d)));
    return @intFromFloat(@ceil(half_t + half_delta) - 1 - @floor((half_t - half_delta)));
}

fn run(input: [:0]const u8) u64 {
    // Parse times
    var time: u64 = 0;
    var idx: usize = 10; // Skip `Time:    `
    var number_acc = [_]u8{0} ** ACCUMULATOR_SIZE;
    var number_len: u8 = 0;
    // Scan through number
    while (idx < input.len) : (idx += 1) {
        if (input[idx] == ' ') {
            continue;
        } else if (input[idx] == '\n') {
            time = std.fmt.parseInt(u64, number_acc[0..number_len], 10) catch unreachable;
            break;
        }
        number_acc[number_len] = input[idx];
        number_len += 1;
    }
    idx += 11; // Skip `Distance: `
    // Parse distance
    var distance: u64 = 0;
    number_acc = [_]u8{0} ** ACCUMULATOR_SIZE;
    number_len = 0;
    // Scan through number
    while (idx < input.len) : (idx += 1) {
        if (input[idx] == ' ') {
            continue;
        } else if (input[idx] == '\n') {
            distance = std.fmt.parseInt(u64, number_acc[0..number_len], 10) catch unreachable;
            break;
        }
        number_acc[number_len] = input[idx];
        number_len += 1;
    } else {
        distance = std.fmt.parseInt(u64, number_acc[0..number_len], 10) catch unreachable;
    }
    return get_solution_count(time, distance);
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

test "aoc" {
    const input =
        \\Time:      7  15   30
        \\Distance:  9  40  200
    ;
    stdout.print("\n", .{}) catch unreachable;
    const ans = run(input);
    try std.testing.expect(ans == 71503);
}
