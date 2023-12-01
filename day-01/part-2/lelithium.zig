const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const LEN1: usize = 3;
const LEN2: usize = 3;
const LEN3: usize = 5;
const LEN4: usize = 4;
const LEN5: usize = 4;
const LEN6: usize = 3;
const LEN7: usize = 5;
const LEN8: usize = 5;
const LEN9: usize = 4;

// zero doesn't count as per the AOC instructions

// Combos count ! My input contains a lot, which changed the result.
// All combos start with only one letter of the previous digit
// so it's OK to skip the length of the digit minus one
// i.e. if we parse eight, jump to the `t` to check for two, three
// but there's no need to parse `i`, `g`, `h`

fn run(input: [:0]const u8) u64 {
    var acc: u64 = 0;
    var cur: u8 = 0;
    var is_first_digit: bool = true;
    var last_digit: u8 = '0';
    var idx: usize = 0;
    while (idx < input.len) : (idx += 1) {
        // Get current character.
        // Note that we override this with a parsed digit, if found.
        cur = input[idx];
        // On line break, add the last known digit.
        if (cur == '\n') {
            is_first_digit = true;
            acc += last_digit - '0';
            //stdout.print("{c}\n", .{last_digit}) catch unreachable;
            continue;
        }
        // Digit parser
        if (cur == 'e' and idx + LEN8 <= input.len) {
            if (std.mem.eql(u8, input[idx .. idx + LEN8], "eight")) {
                cur = '8';
                idx += LEN8 - 2; // we add 1 in the while loop
            }
        } else if (cur == 'f' and idx + LEN4 <= input.len) {
            if (std.mem.eql(u8, input[idx .. idx + LEN4], "four")) {
                cur = '4';
                idx += LEN4 - 2;
            } else if (std.mem.eql(u8, input[idx .. idx + LEN5], "five")) {
                cur = '5';
                idx += LEN5 - 2;
            }
        } else if (cur == 'n' and idx + LEN9 <= input.len) {
            if (std.mem.eql(u8, input[idx .. idx + LEN9], "nine")) {
                cur = '9';
                idx += LEN9 - 2;
            }
        } else if (cur == 'o' and idx + LEN1 <= input.len) {
            if (std.mem.eql(u8, input[idx .. idx + LEN1], "one")) {
                cur = '1';
                idx += LEN1 - 2;
            }
        } else if (cur == 's') {
            if (idx + LEN7 <= input.len) {
                if (std.mem.eql(u8, input[idx .. idx + LEN6], "six")) {
                    cur = '6';
                    idx += LEN6 - 2;
                }
                if (std.mem.eql(u8, input[idx .. idx + LEN7], "seven")) {
                    cur = '7';
                    idx += LEN7 - 2;
                }
            } else if (idx + LEN6 <= input.len) {
                if (std.mem.eql(u8, input[idx .. idx + LEN6], "six")) {
                    cur = '6';
                    idx += LEN6 - 2;
                }
            }
        } else if (cur == 't') {
            if (idx + LEN3 <= input.len) {
                if (std.mem.eql(u8, input[idx .. idx + LEN2], "two")) {
                    cur = '2';
                    idx += LEN2 - 2;
                } else if (std.mem.eql(u8, input[idx .. idx + LEN3], "three")) {
                    cur = '3';
                    idx += LEN3 - 2;
                }
            } else if (idx + LEN2 <= input.len) {
                if (std.mem.eql(u8, input[idx .. idx + LEN2], "two")) {
                    cur = '2';
                    idx += LEN2 - 2;
                }
            }
        }
        // At this stage, if `cur` isn't a digit, then we're in garbage
        if ((cur < '0') or (cur > '9')) {
            continue;
        }
        // If this is the first digit, add it x10
        if (is_first_digit) {
            //stdout.print("{c}", .{cur}) catch unreachable;
            acc += 10 * (cur - '0');
            is_first_digit = false;
        }
        // Store last seen digit
        last_digit = cur;
    }
    // Final loop iteration
    return acc + last_digit - '0';
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

test "ez" {
    const input =
        \\two1nine
        \\eightwothree
        \\abcone2threexyz
        \\xtwone3four
        \\4nineeightseven2
        \\zoneight234
        \\7pqrstsixteen
    ;
    const ans = run(input);
    try std.testing.expect(ans == 281);
}

test "ez_custom" {
    const input =
        \\onetwo
        \\threefour
        \\fivesix
        \\seveneight
        \\nine0
    ;
    const ans = run(input);
    try std.testing.expect(ans == 12 + 34 + 56 + 78 + 90);
}

test "ez_single_digit" {
    const input =
        \\one
        \\two
        \\three
        \\four
        \\five
        \\six
        \\seven
        \\eight
        \\nine
    ;
    const ans = run(input);
    try std.testing.expect(ans == 11 + 22 + 33 + 44 + 55 + 66 + 77 + 88 + 99);
}

test "ez_shuffle" {
    const input =
        \\one
        \\two
        \\three
        \\four
        \\five
        \\six
        \\seven
        \\eight
        \\nine
    ;
    const ans = run(input);
    try std.testing.expect(ans == 11 + 22 + 33 + 44 + 55 + 66 + 77 + 88 + 99);
}

test "ez_combos" {
    const input =
        \\eightwo
        \\eighthree
        \\fiveight
        \\nineight
        \\oneight
        \\sevenine
        \\threeight
        \\twone
    ;
    const ans = run(input);
    try std.testing.expect(ans == 82 + 83 + 58 + 98 + 18 + 79 + 38 + 21);
}
