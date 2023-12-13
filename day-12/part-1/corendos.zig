const std = @import("std");

var allocator: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn parseGroups(str: []const u8) ![]u8 {
    var list = try std.ArrayList(u8).initCapacity(allocator, 10);
    var it = std.mem.splitScalar(u8, str, ',');
    while (it.next()) |raw_number| {
        const number = try std.fmt.parseInt(u8, raw_number, 10);
        try list.append(number);
    }

    return list.toOwnedSlice();
}

fn checkFit(condition_string: []const u8, start: usize, end: usize) bool {
    var can_fit = for (condition_string[start..end]) |c| {
        if (c != '#' and c != '?') break false;
    } else true;

    if (start > 0) {
        can_fit = can_fit and condition_string[start - 1] != '#';
    }

    if (end < condition_string.len) {
        can_fit = can_fit and condition_string[end] != '#';
    }

    return can_fit;
}

fn arrangementCountRec(dps: []i64, condition_string: []const u8, groups: []const u8, start: usize, end: usize, group_index: usize) i64 {
    // We have no consecutive groups to place, this is a valid arrangement if there is no '#' in the remaining condition string.
    if (group_index == groups.len) {
        return for (condition_string[start..end]) |c| {
            if (c == '#') break 0;
        } else 1;
    }
    // We have no space left to put consecutive groups, this is not a valid arrangement.
    if (start == end) return 0;

    const dp = dps[groups.len * start + group_index];
    if (dp != 0) return dp - 1;

    const current_group_size = groups[group_index];
    var arrangement_count: i64 = 0;
    if (end - start >= current_group_size) {
        for (start..end - current_group_size + 1) |i| {
            const can_group_fit = checkFit(condition_string, i, i + current_group_size);
            if (can_group_fit) {
                const next_index = @min(end, i + current_group_size + 1);
                arrangement_count += arrangementCountRec(dps, condition_string, groups, next_index, end, group_index + 1);
            } else {
                // Nothing to do there.
            }

            // If the current character is a '#', we can't continue after otherwise, we would have more groups than required.
            if (condition_string[i] == '#') break;
        }
    } else {
        // There is not enough space left to place the current group.
    }

    dps[groups.len * start + group_index] = arrangement_count + 1;

    return arrangement_count;
}

fn runLine(line: []const u8) !i64 {
    var it = std.mem.splitScalar(u8, line, ' ');
    const condition_string = it.next().?;
    const groups = try parseGroups(it.next().?);

    const dps = try allocator.alloc(i64, condition_string.len * groups.len);
    @memset(dps, 0);

    const result = arrangementCountRec(dps, condition_string, groups, 0, condition_string.len, 0);
    return result;
}

test {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator); // create memory allocator for strings

    defer arena.deinit(); // clear memory
    allocator = arena.allocator();

    const arrangement_count = try runLine("#.?. 1");
    try std.testing.expectEqual(@as(i64, 1), arrangement_count);
}

fn run(input: [:0]const u8) !i64 {
    var result: i64 = 0;
    var line_it = std.mem.splitScalar(u8, input, '\n');
    while (line_it.next()) |line| {
        const arrangement_count = try runLine(line);
        result += arrangement_count;
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
        \\???.### 1,1,3
        \\.??..??...?##. 1,1,3
        \\?#?#?#?#?#?#?#? 1,3,1,6
        \\????.#...#... 4,1,1
        \\????.######..#####. 1,6,5
        \\?###???????? 3,2,1
    ;
    const result = try run(input);
    try std.testing.expectEqual(@as(i64, 21), result);
}
