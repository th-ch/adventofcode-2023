const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn parseSeeds(line: []const u8) ![]Range {
    var seeds = std.ArrayList(Range).init(a);
    var it = std.mem.splitScalar(u8, line, ' ');
    _ = it.next();

    while (it.next()) |raw_start_number| {
        const start_number = try std.fmt.parseInt(i64, raw_start_number, 10);
        const length = try std.fmt.parseInt(i64, it.next().?, 10);
        try seeds.append(Range{ .start = start_number, .end = start_number + length });
    }

    return seeds.toOwnedSlice();
}

const Range = struct {
    start: i64,
    end: i64,
};

const RangeMapping = struct {
    destination: i64,
    source: i64,
    length: i64,
};

const Map = struct {
    mappings: []const RangeMapping,
};

fn parseMap(block: []const u8) !Map {
    var it = std.mem.splitScalar(u8, block, '\n');
    _ = it.next();

    var mappings = std.ArrayList(RangeMapping).init(a);

    while (it.next()) |line| {
        var number_it = std.mem.splitScalar(u8, line, ' ');
        const destination = try std.fmt.parseInt(i64, number_it.next().?, 10);
        const source = try std.fmt.parseInt(i64, number_it.next().?, 10);
        const length = try std.fmt.parseInt(i64, number_it.next().?, 10);

        try mappings.append(RangeMapping{ .destination = destination, .source = source, .length = length });
    }

    return Map{ .mappings = try mappings.toOwnedSlice() };
}

fn run(input: [:0]const u8) i64 {
    var it = std.mem.splitSequence(u8, input, "\n\n");
    const seeds = parseSeeds(it.next().?) catch unreachable;
    var maps = std.ArrayList(Map).init(a);

    while (it.next()) |map_block| {
        const map = parseMap(map_block) catch unreachable;
        maps.append(map) catch unreachable;
    }

    var ranges = std.ArrayList(Range).init(a);
    ranges.appendSlice(seeds) catch unreachable;
    var temp_ranges = std.ArrayList(Range).init(a);

    for (maps.items) |map| {
        while (ranges.items.len > 0) {
            const range = ranges.pop();
            var mapped = false;

            for (map.mappings) |mapping| {
                const mapping_range = Range{ .start = mapping.source, .end = mapping.source + mapping.length };
                const delta = mapping.destination - mapping.source;

                if (range.start >= mapping_range.start and range.end <= mapping_range.end) {
                    temp_ranges.append(Range{ .start = range.start + delta, .end = range.end + delta }) catch unreachable;
                    mapped = true;
                    break;
                }

                if (range.start >= mapping_range.start and range.start < mapping_range.end) {
                    const range1 = Range{ .start = range.start, .end = mapping_range.end };
                    const range2 = Range{ .start = mapping_range.end, .end = range.end };

                    temp_ranges.append(Range{ .start = range1.start + delta, .end = range1.end + delta }) catch unreachable;
                    ranges.append(range2) catch unreachable;
                    mapped = true;
                    break;
                }

                if (range.end > mapping_range.start and range.end <= mapping_range.end) {
                    const range1 = Range{ .start = range.start, .end = mapping_range.start };
                    const range2 = Range{ .start = mapping_range.start, .end = range.end };

                    ranges.append(range1) catch unreachable;
                    temp_ranges.append(Range{ .start = range2.start + delta, .end = range2.end + delta }) catch unreachable;
                    mapped = true;
                    break;
                }

                if (range.start < mapping_range.start and range.end > mapping_range.end) {
                    const range1 = Range{ .start = range.start, .end = mapping_range.start };
                    const range2 = Range{ .start = mapping_range.start, .end = mapping_range.end };
                    const range3 = Range{ .start = mapping_range.end, .end = range.end };

                    ranges.append(range1) catch unreachable;
                    ranges.append(range3) catch unreachable;
                    temp_ranges.append(Range{ .start = range2.start + delta, .end = range2.end + delta }) catch unreachable;
                    mapped = true;
                    break;
                }
            }

            if (!mapped) {
                temp_ranges.append(range) catch unreachable;
            }
        }

        ranges.appendSlice(temp_ranges.items) catch unreachable;
        temp_ranges.clearRetainingCapacity();
    }

    var result: i64 = std.math.maxInt(i64);

    for (ranges.items) |r| {
        result = @min(result, r.start);
    }

    return result;
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

test {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator); // create memory allocator for strings

    defer arena.deinit(); // clear memory
    a = arena.allocator();
    const input =
        \\seeds: 79 14 55 13
        \\
        \\seed-to-soil map:
        \\50 98 2
        \\52 50 48
        \\
        \\soil-to-fertilizer map:
        \\0 15 37
        \\37 52 2
        \\39 0 15
        \\
        \\fertilizer-to-water map:
        \\49 53 8
        \\0 11 42
        \\42 0 7
        \\57 7 4
        \\
        \\water-to-light map:
        \\88 18 7
        \\18 25 70
        \\
        \\light-to-temperature map:
        \\45 77 23
        \\81 45 19
        \\68 64 13
        \\
        \\temperature-to-humidity map:
        \\0 69 1
        \\1 0 69
        \\
        \\humidity-to-location map:
        \\60 56 37
        \\56 93 4
    ;

    const result = run(input);
    try std.testing.expectEqual(@as(i64, 46), result);
}
