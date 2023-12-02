const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const red_max: usize = 12;
const green_max: usize = 13;
const blue_max: usize = 14;

fn run(input: [:0]const u8) u64 {
    var lines = std.mem.split(u8, input, "\n");
    var id_acc: u64 = 0;
    var game_id: usize = 1;
    while (lines.next()) |line| : (game_id += 1) {
        var possible: bool = true;
        var elts = std.mem.tokenize(u8, line, " ");
        _ = elts.next(); // Skip `Game`
        // Game IDs are incremental !
        _ = elts.next(); // Skip game id
        //var game_id = std.fmt.parseInt(u32, (game_slice)[0 .. game_slice.len - 1], 10) catch unreachable; // skip `:` and parse game ID
        while (elts.next()) |count_raw| {
            var count_parsed = std.fmt.parseInt(usize, count_raw, 10) catch unreachable;
            if (count_parsed > switch (elts.next().?[0]) {
                'g' => green_max,
                'r' => red_max,
                else => blue_max,
            }) {
                possible = false;
                break;
            }
        }
        if (possible) {
            id_acc += game_id;
        }
    }

    return id_acc;
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
    try std.testing.expect(ans == 8);
}
