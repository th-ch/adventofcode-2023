const std = @import("std");
const builtin = @import("builtin");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const RACE_COUNT = if (builtin.is_test) 3 else 4;

inline fn parse_2_digits(input: []const u8) u32 {
    // Quickly parse `99`, `9 ` and `9\n` formats
    if (input[1] == ' ' or input[1] == '\n') {
        return input[0] - '0';
    }
    return (input[0] - '0') * 10 + (input[1] - '0');
}

inline fn get_solution_count(t: u32, d: u32) u32 {
    const half_t: f32 = @as(f32, @floatFromInt(t)) / 2.0;
    const half_delta: f32 = @sqrt(half_t * half_t - @as(f32, @floatFromInt(d)));
    return @intFromFloat(@ceil(half_t + half_delta) - 1 - @floor((half_t - half_delta)));
}

fn run(input: [:0]const u8) u64 {
    // Parse times
    var times = [_]u32{0} ** RACE_COUNT;
    var last_idx: usize = 10; // Skip `Time:    `
    for (0..RACE_COUNT) |race_id| {
        while (last_idx < input.len) : (last_idx += 1) {
            if (input[last_idx] != ' ') {
                times[race_id] = parse_2_digits(input[last_idx .. last_idx + 2]);
                last_idx += 2;
                break;
            }
        }
    }
    last_idx += 11; // Skip `Distance: `

    // Parse distances
    var number_start: usize = 0;
    var distances = [_]u32{0} ** RACE_COUNT;
    for (0..RACE_COUNT) |race_id| {
        while (last_idx < input.len) : (last_idx += 1) {
            if (input[last_idx] != ' ') {
                if (number_start == 0) {
                    number_start = last_idx;
                }
            } else if (number_start != 0) {
                distances[race_id] = std.fmt.parseInt(u16, input[number_start..last_idx], 10) catch unreachable;
                number_start = 0;
                break;
            }
        } else {
            distances[RACE_COUNT - 1] = std.fmt.parseInt(u16, input[number_start..last_idx], 10) catch unreachable;
        }
    }

    // Compute output
    var out: u64 = 1;
    for (0..RACE_COUNT) |race_id| {
        out *= get_solution_count(times[race_id], distances[race_id]);
    }
    return out;
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
    try std.testing.expect(ans == 288);
}
