const std = @import("std");
const builtin = @import("builtin");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const SCRATCH_LEN: usize = if (builtin.is_test) 5 else 10;
const WINNERS_LEN: usize = if (builtin.is_test) 8 else 25;
const CARD_OFFSET: usize = if (builtin.is_test) 8 else 10; // `Card 123: ` or `Card 1: `
const SCRATCH_OFFSET: usize = SCRATCH_LEN * 3 + 2; // 3 per scratch number + `| `
const SKIP_OFFSET: usize = CARD_OFFSET + (SCRATCH_LEN + WINNERS_LEN) * 3 + 2;

inline fn parse_2_digits(input: []const u8) u8 {
    if (input[0] == ' ') {
        return input[1] - '0';
    }
    return (input[0] - '0') * 10 + (input[1] - '0');
}

fn run(input: [:0]const u8) u64 {
    var won: u64 = 0;
    var idx: usize = CARD_OFFSET;
    var scratch = [_]u8{0} ** SCRATCH_LEN;
    var winners = [_]u8{0} ** WINNERS_LEN;
    while (idx < input.len) : (idx += SKIP_OFFSET) {
        for (0..SCRATCH_LEN) |i| {
            scratch[i] = parse_2_digits(input[(idx + (i * 3))..(idx + 2 + (i * 3))]);
        }
        for (0..WINNERS_LEN) |i| {
            winners[i] = parse_2_digits(input[(idx + SCRATCH_OFFSET + (i * 3))..(idx + SCRATCH_OFFSET + (i * 3) + 2)]);
        }
        var matches: u8 = 0;
        scratch: for (scratch) |s| {
            for (winners) |w| {
                if (w == s) {
                    matches += 1;
                    continue :scratch;
                }
            }
        }
        if (matches != 0) {
            won += std.math.pow(u64, 2, matches - 1);
        }
    }
    return won;
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
    const elapsed_nano = @as(f64, @floatFromInt(end - start));
    const elapsed_milli = elapsed_nano / 1_000_000.0;
    try stdout.print("_duration:{d}\n{}\n", .{ elapsed_milli, answer }); // emit actual lines parsed by AOC
}

test "aoc" {
    const input =
        \\Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        \\Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        \\Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        \\Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        \\Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        \\Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    ;
    stdout.print("\n", .{}) catch unreachable;
    const ans = run(input);
    try std.testing.expect(ans == 13);
}
