const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const GameInfo = struct {
    red_count: i64 = 0,
    green_count: i64 = 0,
    blue_count: i64 = 0,
};

fn parseGameId(s: []const u8) i64 {
    var it = std.mem.splitScalar(u8, s, ' ');
    _ = it.next();
    return std.fmt.parseInt(i64, it.next().?, 10) catch unreachable;
}

fn parseSetInfo(s: []const u8) GameInfo {
    var it = std.mem.splitScalar(u8, s, ',');
    var result = GameInfo{};
    while (it.next()) |cube_str| {
        const trimmed_cube_str = std.mem.trim(u8, cube_str, " ");

        var cube_it = std.mem.splitScalar(u8, trimmed_cube_str, ' ');
        const cube_count = std.fmt.parseInt(i64, cube_it.next().?, 10) catch unreachable;
        const cube_color_str = cube_it.next().?;
        if (std.mem.eql(u8, cube_color_str, "red")) {
            result.red_count = cube_count;
        } else if (std.mem.eql(u8, cube_color_str, "green")) {
            result.green_count = cube_count;
        } else if (std.mem.eql(u8, cube_color_str, "blue")) {
            result.blue_count = cube_count;
        } else unreachable;
    }

    return result;
}

fn runLine(line: []const u8) struct { id: i64, info: GameInfo } {
    var it = std.mem.splitScalar(u8, line, ':');
    const game_id = parseGameId(it.next().?);

    var result = GameInfo{};

    var set_it = std.mem.splitScalar(u8, it.rest(), ';');
    while (set_it.next()) |set_str| {
        const set_info = parseSetInfo(set_str);
        result.red_count = @max(result.red_count, set_info.red_count);
        result.green_count = @max(result.green_count, set_info.green_count);
        result.blue_count = @max(result.blue_count, set_info.blue_count);
    }

    return .{ .id = game_id, .info = result };
}

fn run(input: [:0]const u8) i64 {
    var it = std.mem.splitScalar(u8, input, '\n');
    var result: i64 = 0;
    while (it.next()) |line| {
        const line_result = runLine(line);
        result += line_result.info.red_count * line_result.info.green_count * line_result.info.blue_count;
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
    const input =
        \\Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        \\Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        \\Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        \\Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        \\Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    ;
    const result = run(input);
    try std.testing.expectEqual(@as(i64, 2286), result);
}
