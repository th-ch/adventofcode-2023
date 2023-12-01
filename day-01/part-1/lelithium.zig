const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn run(input: [:0]const u8) u64 {
    var acc: u64 = 0;
    var first: bool = true;
    var last: u8 = '0';
    for (input) |elt| {
        if (elt == '\n') {
            first = true;
            acc += last - '0';
            continue;
        }
        if ((elt < '0') or (elt > '9')) {
            continue;
        }
        if (first) {
            acc += 10 * (elt - '0');
            first = false;
        }
        last = elt;
    }
    return acc + last - '0';
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
        \\1abc2
        \\pqr3stu8vwx
        \\a1b2c3d4e5f
        \\treb7uchet
    ;
    const ans = run(input);
    try std.testing.expect(ans == 142);
}
