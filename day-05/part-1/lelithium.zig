const std = @import("std");
const builtin = @import("builtin");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const SEED_COUNT: usize = if (builtin.is_test) 4 else 20; // Fixed
const MAP_COUNT: usize = if (builtin.is_test) 4 else 250; // Empirical
const RANGE_COUNT: usize = if (builtin.is_test) 4 else 50; // Empirical

const Range = struct {
    dest_start: u64,
    source_start: u64,
    range: u64,
    inline fn map_seed(self: *const Range, seed: u64) ?u64 {
        if (seed < self.source_start or seed > self.source_start + self.range) {
            return null;
        }
        return seed - self.source_start + self.dest_start;
    }
};

fn run(input: [:0]const u8) u64 {

    // Split all blocks
    var blocks = std.mem.split(u8, input, "\n\n");
    // Parse seeds
    var seeds = [_]u64{0} ** SEED_COUNT;
    var max_seed_idx: u8 = 0;
    var seed_it = std.mem.split(u8, blocks.next().?, " ");
    _ = seed_it.next(); // Skip `seeds:`
    while (seed_it.next()) |seed_raw| : (max_seed_idx += 1) {
        seeds[max_seed_idx] = std.fmt.parseInt(u64, seed_raw, 10) catch unreachable;
    }

    //stdout.print("Parsed seeds {any}\n", .{seeds}) catch unreachable;

    // Parse maps
    while (blocks.next()) |map| {
        // Store map ranges
        var map_ranges = [_]?Range{null} ** 50;
        var map_range_count: u8 = 0;
        // Split lines
        var lines = std.mem.split(u8, map, "\n");
        _ = lines.next(); // Skip `x-to-y map:`
        // For each line
        while (lines.next()) |line| : (map_range_count += 1) {
            // Split on spaces
            var range_it = std.mem.split(u8, line, " ");
            // Create new range and save it
            map_ranges[map_range_count] = Range{
                .dest_start = std.fmt.parseInt(u64, range_it.next().?, 10) catch unreachable,
                .source_start = std.fmt.parseInt(u64, range_it.next().?, 10) catch unreachable,
                .range = std.fmt.parseInt(u64, range_it.next().?, 10) catch unreachable,
            };
        }
        for (seeds[0..max_seed_idx], 0..) |seed, seed_idx| {
            //stdout.print("Evaluating seed {}\n", .{seed}) catch unreachable;
            ranges: for (map_ranges[0..map_range_count]) |range| {
                //stdout.print("\tEvaluating range [{}, {}, {}]\n", .{ range.?.dest_start, range.?.source_start, range.?.range }) catch unreachable;
                if (range.?.map_seed(seed)) |val| {
                    //stdout.print("\t\tSeed is in range ! Got value {}\n", .{val}) catch unreachable;
                    // Seed in range !
                    seeds[seed_idx] = val;
                    break :ranges;
                }
            } else {
                // Seed not in map ranges - use seed value instead
                //stdout.print("\t\tSeed not in range - keeping {}\n", .{seed}) catch unreachable;
                seeds[seed_idx] = seed;
            }
        }
        //stdout.print("Current seed status {any}\n\n", .{seeds}) catch unreachable;
    }
    // Store output
    var lowest: u64 = std.math.maxInt(u64);
    for (seeds[0..max_seed_idx]) |seed| {
        lowest = @min(lowest, seed);
    }
    return lowest;
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

test "aoc" {
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
    stdout.print("\n", .{}) catch unreachable;
    const ans = run(input);
    try std.testing.expect(ans == 35);
}
