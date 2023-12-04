const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const MATRIX_W: usize = 141; // 140 chars + \n
const MATRIX_H: usize = 140;

//Enable during tests
//const MATRIX_W: usize = 11; // 10 chars + \n
//const MATRIX_H: usize = 10;

inline fn is_number(c: u8) bool {
    return c >= '0' and c <= '9';
}

fn scan_number(input: [:0]const u8, idx: usize) u32 {
    // Consts
    const orig_col_idx = @mod(idx, MATRIX_W);
    const row = idx - orig_col_idx;
    // Vars
    var start_idx: usize = idx;
    var end_idx: usize = idx;
    var col_idx = orig_col_idx;
    // Look behind
    while (col_idx >= 0 and is_number(input[row + col_idx])) {
        if (col_idx == 0) {
            start_idx = row + col_idx;
            break;
        }
        col_idx -= 1;
    } else {
        start_idx = row + col_idx + 1;
    }
    col_idx = orig_col_idx;
    // Look ahead
    while (col_idx <= MATRIX_W - 1 and is_number(input[row + col_idx])) {
        if (col_idx == MATRIX_W - 1) {
            break;
        }
        col_idx += 1;
    }
    end_idx = row + col_idx;
    return std.fmt.parseInt(u32, input[start_idx..end_idx], 10) catch unreachable;
}

fn run(input: [:0]const u8) u64 {
    var idx: usize = 0;
    var gear_sum: u64 = 0;
    while (idx < input.len) : (idx += 1) {
        const col_idx = @mod(idx, MATRIX_W);
        if (input[idx] == '*') {
            var num1: u32 = 0;
            var num2: u32 = 0;
            // Try to find adjacent numbers
            // Check if we can look up
            if (idx >= MATRIX_W) {
                // Can we check up-left ?
                if (col_idx > 0) {
                    // Check up-left
                    if (is_number(input[idx - MATRIX_W - 1])) {
                        // First iteration, so num1 has to be 0
                        num1 = scan_number(input, idx - MATRIX_W - 1);
                    }
                }
                // Check up only if we haven't already scanned it
                if (is_number(input[idx - MATRIX_W]) and num1 == 0) {
                    num1 = scan_number(input, idx - MATRIX_W);
                }
                // Check up-right only if there's no digit up
                if (col_idx < MATRIX_W - 1 and !is_number(input[idx - MATRIX_W]) and is_number(input[idx - MATRIX_W + 1])) {
                    if (num1 == 0) {
                        num1 = scan_number(input, idx - MATRIX_W + 1);
                    } else {
                        num2 = scan_number(input, idx - MATRIX_W + 1);
                        gear_sum += num1 * num2;
                        continue;
                    }
                }
            }
            // Can we check left ?
            if (col_idx > 0) {
                // Check to the left
                if (is_number(input[idx - 1])) {
                    if (num1 == 0) {
                        num1 = scan_number(input, idx - 1);
                    } else {
                        num2 = scan_number(input, idx - 1);
                        gear_sum += num1 * num2;
                        continue;
                    }
                }
            }
            // Can we check right ?
            if (col_idx < MATRIX_W - 1) {
                // Check to the right
                if (is_number(input[idx + 1])) {
                    if (num1 == 0) {
                        num1 = scan_number(input, idx + 1);
                    } else {
                        num2 = scan_number(input, idx + 1);
                        gear_sum += num1 * num2;
                        continue;
                    }
                }
            }
            // Can we check down ?
            if (idx <= (MATRIX_H - 1) * MATRIX_W) {
                // Can we check down-left ?
                if (col_idx > 0) {
                    // Check down-left
                    if (is_number(input[idx + MATRIX_W - 1])) {
                        if (num1 == 0) {
                            num1 = scan_number(input, idx + MATRIX_W - 1);
                        } else {
                            num2 = scan_number(input, idx + MATRIX_W - 1);
                            gear_sum += num1 * num2;
                            continue;
                        }
                    } else {
                        // Edge-case: check down
                        if (is_number(input[idx + MATRIX_W])) {
                            if (num1 == 0) {
                                num1 = scan_number(input, idx + MATRIX_W);
                            } else {
                                num2 = scan_number(input, idx + MATRIX_W);
                                gear_sum += num1 * num2;
                                continue;
                            }
                        }
                    }
                } else if (is_number(input[idx + MATRIX_W])) {
                    // Check down
                    if (num1 == 0) {
                        num1 = scan_number(input, idx + MATRIX_W);
                    } else {
                        num2 = scan_number(input, idx + MATRIX_W);
                        gear_sum += num1 * num2;
                        continue;
                    }
                }
                // Check up-right only if there's no digit up
                if (col_idx < MATRIX_W - 1) {
                    // Split if to avoid going over input.len
                    if (!is_number(input[idx + MATRIX_W]) and is_number(input[idx + MATRIX_W + 1])) {
                        if (num1 == 0) {
                            num1 = scan_number(input, idx + MATRIX_W + 1);
                        } else {
                            num2 = scan_number(input, idx + MATRIX_W + 1);
                            gear_sum += num1 * num2;
                            continue;
                        }
                    }
                }
            }
            // Cog with a single number. Do nothing.
        }
    }
    return gear_sum;
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

test "scan_number" {
    const input =
        \\.467..114.
        \\123....456
        \\..10......
    ;
    stdout.print("\n", .{}) catch unreachable;
    try std.testing.expect(scan_number(input, 1) == 467);
    try std.testing.expect(scan_number(input, 2) == 467);
    try std.testing.expect(scan_number(input, 3) == 467);

    try std.testing.expect(scan_number(input, 6) == 114);
    try std.testing.expect(scan_number(input, 7) == 114);
    try std.testing.expect(scan_number(input, 8) == 114);

    try std.testing.expect(scan_number(input, 11) == 123);
    try std.testing.expect(scan_number(input, 12) == 123);
    try std.testing.expect(scan_number(input, 13) == 123);

    try std.testing.expect(scan_number(input, 18) == 456);
    try std.testing.expect(scan_number(input, 19) == 456);
    try std.testing.expect(scan_number(input, 20) == 456);

    try std.testing.expect(scan_number(input, 24) == 10);
    try std.testing.expect(scan_number(input, 25) == 10);
}

test "synthetic_single" {
    stdout.print("\n", .{}) catch unreachable;
    const input_1 =
        \\100.100...
        \\...*......
    ;
    try std.testing.expect(run(input_1) == 100 * 100);

    const input_2 =
        \\100.......
        \\100*......
    ;
    try std.testing.expect(run(input_2) == 100 * 100);

    const input_3 =
        \\...100....
        \\100*......
    ;
    try std.testing.expect(run(input_3) == 100 * 100);

    const input_4 =
        \\....100...
        \\100*......
    ;
    try std.testing.expect(run(input_4) == 100 * 100);

    const input_5 =
        \\100*100...
    ;
    try std.testing.expect(run(input_5) == 100 * 100);

    const input_6 =
        \\.100......
        \\...*100...
    ;
    try std.testing.expect(run(input_6) == 100 * 100);

    const input_7 =
        \\...*100...
        \\.100......
    ;
    try std.testing.expect(run(input_7) == 100 * 100);

    const input_8 =
        \\*100......
        \\1.........
    ;
    try std.testing.expect(run(input_8) == 100);

    const input_9 =
        \\.*100.....
        \\.1........
    ;
    try std.testing.expect(run(input_9) == 100);

    const input_10 =
        \\.....100*.
        \\.........1
    ;
    try std.testing.expect(run(input_10) == 100);

    const input_11 =
        \\100.......
        \\...*......
        \\..10......
    ;
    try std.testing.expect(run(input_11) == 1000);
}

test "synthetic_many" {
    stdout.print("\n", .{}) catch unreachable;
    const input_1 =
        \\100.100...
        \\...*..*100
    ;
    try std.testing.expect(run(input_1) == 100 * 100 + 100 * 100);
}

test "ez" {
    const input =
        \\467..114..
        \\...*......
        \\..35..633.
        \\......#...
        \\617*......
        \\.....+.58.
        \\..592.....
        \\......755.
        \\...$.*....
        \\.664.598..
    ;
    stdout.print("\n", .{}) catch unreachable;
    const ans = run(input);
    try std.testing.expect(ans == 467835);
}
