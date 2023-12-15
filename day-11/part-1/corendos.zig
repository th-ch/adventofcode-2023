const std = @import("std");

var allocator: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const Position = struct {
    x: usize,
    y: usize,
};

fn run(input: [:0]const u8) !i64 {
    var galaxy_column_lookup = std.ArrayList(bool).init(allocator);
    var galaxy_row_lookup = std.ArrayList(bool).init(allocator);
    var line_it = std.mem.splitScalar(u8, input, '\n');
    var first_line: bool = true;
    while (line_it.next()) |line| {
        var has_galaxy_in_row: bool = false;
        for (line, 0..) |c, i| {
            const has_galaxy_in_column = if (c == '#') true else false;
            if (first_line) {
                try galaxy_column_lookup.append(has_galaxy_in_column);
            } else {
                galaxy_column_lookup.items[i] = galaxy_column_lookup.items[i] or has_galaxy_in_column;
            }

            has_galaxy_in_row = has_galaxy_in_row or has_galaxy_in_column;
        }

        try galaxy_row_lookup.append(has_galaxy_in_row);

        first_line = false;
    }

    var x: usize = 0;
    var y: usize = 0;
    var expanded_x: usize = 0;
    var expanded_y: usize = 0;
    var galaxy_positions = std.ArrayList(Position).init(allocator);

    if (!galaxy_row_lookup.items[y]) {
        expanded_y += 1;
    }

    for (input) |c| {
        if (c == '\n') {
            x = 0;
            expanded_x = 0;
            y += 1;
            expanded_y += 1;
            if (!galaxy_row_lookup.items[y]) {
                expanded_y += 1;
            }
            continue;
        }
        if (!galaxy_column_lookup.items[x]) {
            expanded_x += 1;
        }

        if (c == '#') {
            try galaxy_positions.append(Position{ .x = expanded_x, .y = expanded_y });
        }

        x += 1;
        expanded_x += 1;
    }

    var result: i64 = 0;
    for (0..galaxy_positions.items.len - 1) |i| {
        for (i..galaxy_positions.items.len) |j| {
            const galaxy_a = galaxy_positions.items[i];
            const galaxy_b = galaxy_positions.items[j];
            const distance_x: i64 = @intCast(@abs(@as(i64, @intCast(galaxy_a.x)) - @as(i64, @intCast(galaxy_b.x))));
            const distance_y: i64 = @intCast(@abs(@as(i64, @intCast(galaxy_a.y)) - @as(i64, @intCast(galaxy_b.y))));
            result += distance_x + distance_y;
        }
    }

    return result;
}

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator); // create memory allocator for strings

    defer arena.deinit(); // clear memory
    allocator = arena.allocator();

    var arg_it = try std.process.argsWithAllocator(allocator);
    _ = arg_it.skip(); // skip over exe name
    const input: [:0]const u8 = arg_it.next().?;

    const start: i128 = std.time.nanoTimestamp(); // start time
    const answer = try run(input); // compute answer
    const end: i128 = std.time.nanoTimestamp();
    const elapsed_nano: f64 = @floatFromInt(end - start);
    const elapsed_milli = elapsed_nano / 1_000_000.0;
    try stdout.print("_duration:{d}\n{}\n", .{ elapsed_milli, answer }); // emit actual lines parsed by AOC
}

test {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator); // create memory allocator for strings

    defer arena.deinit(); // clear memory
    allocator = arena.allocator();
    const input =
        \\...#......
        \\.......#..
        \\#.........
        \\..........
        \\......#...
        \\.#........
        \\.........#
        \\..........
        \\.......#..
        \\#...#.....
    ;

    const result = try run(input);
    try std.testing.expectEqual(@as(i64, 374), result);
}
