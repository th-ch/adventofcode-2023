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

fn expandReflection(values: []const u32, left: usize, right: usize, had_change: bool) bool {
    const need_change = if (values[left] != values[right]) b: {
        // If the values are different and we already produces a change, this is not a valid reflection.
        if (had_change) {
            return false;
        } else {
            // If there are more than 1 bit flip required, it can't be a valid reflection.
            const diff = values[left] ^ values[right];
            if (@popCount(diff) != 1) {
                return false;
            }
            // We continue the check assuming we flip the current value.
            break :b true;
        }
    } else had_change; // We propagate the change if the two current values are equal.
    //std.debug.print("Comparing {} with {} need change: {}\n", .{ left, right, need_change });
    if (left == 0) return need_change;
    if (right == values.len - 1) return need_change;
    return expandReflection(values, left - 1, right + 1, need_change);
}

fn runPattern(raw_pattern: []const u8) !i64 {
    const pattern = try parsePattern(raw_pattern);

    for (0..pattern.columns.len - 1) |i| {
        //std.debug.print("Comparing column {} with {}\n", .{ i, i + 1 });
        if (expandReflection(pattern.columns, i, i + 1, false)) {
            //std.debug.print("Reflection found at column {}\n", .{i});
            return @intCast(i + 1);
        }
    }

    for (0..pattern.rows.len - 1) |i| {
        //std.debug.print("Comparing row {} with {}\n", .{ i, i + 1 });
        if (expandReflection(pattern.rows, i, i + 1, false)) {
            //std.debug.print("Reflection found at row {}\n", .{i});
            return @intCast((i + 1) * 100);
        }
    }

    unreachable;
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
    try std.testing.expectEqual(@as(i64, 400), result);
}
