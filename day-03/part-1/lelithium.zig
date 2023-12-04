const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const MATRIX_W: usize = 141; // 140 chars + \n
const MATRIX_H: usize = 140;

// Enable during tests
//const MATRIX_W: usize = 11; // 10 chars + \n
//const MATRIX_H: usize = 10;

fn is_symbol(c: u8) bool {
    return switch (c) {
        '0'...'9', '.', '\n' => false,
        else => true,
    };
}

fn has_symbol_adjacent(input: [:0]const u8, idx: usize) bool {
    const col_idx = @mod(idx, MATRIX_W);
    if (idx >= MATRIX_W) {
        // Check up
        if (is_symbol(input[idx - MATRIX_W])) {
            return true;
        }
        if (col_idx > 0) {
            // Check up-left
            if (is_symbol(input[idx - MATRIX_W - 1])) {
                return true;
            }
        }
        if (col_idx < MATRIX_W - 1) {
            // Check up-right
            if (is_symbol(input[idx - MATRIX_W + 1])) {
                return true;
            }
        }
    }
    if (idx <= (MATRIX_H - 1) * MATRIX_W) {
        // Check below
        if (is_symbol(input[idx + MATRIX_W])) {
            return true;
        }
        if (col_idx > 0) {
            // Check down-left
            if (is_symbol(input[idx + MATRIX_W - 1])) {
                return true;
            }
        }
        if (col_idx < MATRIX_W - 1) {
            // Check down-right
            if (is_symbol(input[idx + MATRIX_W + 1])) {
                return true;
            }
        }
    }
    if (col_idx > 0) {
        // Check to the left
        if (is_symbol(input[idx - 1])) {
            return true;
        }
    }
    if (col_idx < MATRIX_W - 2) {
        // Check to the right
        if (is_symbol(input[idx + 1])) {
            return true;
        }
    }
    return false;
}

fn run(input: [:0]const u8) u64 {
    var idx: usize = 0;
    var part_sum: u64 = 0;
    var number_start_idx: usize = 0;
    var started_parsing: bool = false;
    var number_end_idx: usize = 0;
    var number_valid: bool = false;
    while (idx < input.len) : (idx += 1) {
        const c: u8 = input[idx];
        if (c <= '9' and c >= '0') {
            if (!number_valid) {
                // Only parse if we don't know if the number is valid yet
                number_valid = has_symbol_adjacent(input, idx);
            }
            if (!started_parsing) {
                number_start_idx = idx;
                started_parsing = true;
            }
            number_end_idx = idx;
        } else {
            if (started_parsing and number_valid) {
                // We have a number !
                part_sum += std.fmt.parseInt(u64, input[number_start_idx .. number_end_idx + 1], 10) catch unreachable;
            }
            number_start_idx = 0;
            number_end_idx = 0;
            started_parsing = false;
            number_valid = false;
        }
    }
    return part_sum;
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
    stdout.print("\n", .{}) catch unreachable;
    try std.testing.expect(ans == 4361);
}
