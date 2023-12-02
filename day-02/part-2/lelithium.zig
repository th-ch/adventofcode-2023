const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn run(input: [:0]const u8) u64 {
    var lines = std.mem.split(u8, input, "\n");
    var acc: u64 = 0;
    while (lines.next()) |line| {
        var elts = std.mem.tokenize(u8, line, " ");
        _ = elts.next(); // Skip `Game`
        _ = elts.next(); // Skip game id
        // Reset min counters
        var min_r: usize = 0;
        var min_g: usize = 0;
        var min_b: usize = 0;
        while (elts.next()) |count_raw| {
            var count_parsed = std.fmt.parseInt(usize, count_raw, 10) catch unreachable;
            switch (elts.next().?[0]) {
                'g' => {
                    min_g = @max(min_g, count_parsed);
                },
                'r' => {
                    min_r = @max(min_r, count_parsed);
                },
                else => {
                    min_b = @max(min_b, count_parsed);
                },
            }
        }
        acc += min_r * min_g * min_b;
    }

    return acc;
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
        \\Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        \\Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        \\Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        \\Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        \\Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    ;
    const ans = run(input);
    try std.testing.expect(ans == 2286);
}
