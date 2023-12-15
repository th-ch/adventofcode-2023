const std = @import("std");

var allocator: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const Pattern = struct {
    rows: []const u32,
    columns: []const u32,
};

fn parsePattern(pattern: []const u8) !Pattern {
    var line_it = std.mem.splitScalar(u8, pattern, '\n');
    var row_list = std.ArrayList(u32).init(allocator);
    var column_count: ?usize = null;
    while (line_it.next()) |line| {
        if (column_count == null) {
            column_count = line.len;
        }
        var current: u32 = 0;
        for (line) |c| {
            current = (current << 1) + @as(u32, if (c == '#') 1 else 0);
        }

        try row_list.append(current);
    }

    const columns = try allocator.alloc(u32, column_count.?);
    @memset(columns, 0);
    for (row_list.items, 0..) |r, i| {
        for (0..columns.len) |j| {
            columns[columns.len - 1 - j] |= ((r >> @as(u5, @intCast(j))) & 0x1) << @as(u5, @intCast(i));
        }
    }

    return Pattern{ .rows = try row_list.toOwnedSlice(), .columns = columns };
}

fn expandReflection(values: []const u32, left: usize, right: usize) bool {
    if (values[left] != values[right]) return false;
    if (left == 0) return true;
    if (right == values.len - 1) return true;
    return expandReflection(values, left - 1, right + 1);
}

fn runPattern(raw_pattern: []const u8) !i64 {
    const pattern = try parsePattern(raw_pattern);

    var result: i64 = 0;
    _ = &result;
    for (0..pattern.columns.len - 1) |i| {
        if (expandReflection(pattern.columns, i, i + 1)) {
            //std.debug.print("Reflection found at column {}\n", .{i});
            return @intCast(i + 1);
        }
    }

    for (0..pattern.rows.len - 1) |i| {
        if (expandReflection(pattern.rows, i, i + 1)) {
            //std.debug.print("Reflection found at row {}\n", .{i});
            return @intCast((i + 1) * 100);
        }
    }

    return result;
}

fn run(input: [:0]const u8) !i64 {
    var result: i64 = 0;
    var pattern_it = std.mem.splitSequence(u8, input, "\n\n");
    while (pattern_it.next()) |raw_pattern| {
        result += try runPattern(raw_pattern);
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
        \\#.##..##.
        \\..#.##.#.
        \\##......#
        \\##......#
        \\..#.##.#.
        \\..##..##.
        \\#.#.##.#.
        \\
        \\#...##..#
        \\#....#..#
        \\..##..###
        \\#####.##.
        \\#####.##.
        \\..##..###
        \\#....#..#
    ;
    const result = try run(input);
    try std.testing.expectEqual(@as(i64, 405), result);
}
