const std = @import("std");

var allocator: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const Direction = enum {
    up,
    down,
    right,
    left,

    pub inline fn fromChar(c: u8) Direction {
        return switch (c) {
            '3' => .up,
            '1' => .down,
            '0' => .right,
            '2' => .left,
            else => unreachable,
        };
    }
};

const Color = packed struct {
    r: u8 = 0,
    g: u8 = 0,
    b: u8 = 0,
};

const Trench = struct {
    direction: Direction,
    distance: i64,
    color: Color = .{},
};

const Position = struct {
    x: i64,
    y: i64,

    pub fn move(self: Position, direction: Direction, distance: i64) Position {
        return switch (direction) {
            .up => Position{ .x = self.x, .y = self.y - distance },
            .down => Position{ .x = self.x, .y = self.y + distance },
            .left => Position{ .x = self.x - distance, .y = self.y },
            .right => Position{ .x = self.x + distance, .y = self.y },
        };
    }
};

fn parseLine(line: []const u8) !Trench {
    var it = std.mem.splitScalar(u8, line, ' ');
    _ = it.next();
    _ = it.next();
    const hex = it.next().?[2..8];
    const distance = try std.fmt.parseInt(i64, hex[0 .. hex.len - 1], 16);
    const direction = Direction.fromChar(hex[hex.len - 1]);

    return Trench{ .direction = direction, .distance = distance };
}

fn run(input: [:0]const u8) !i64 {
    var line_it = std.mem.splitScalar(u8, input, '\n');
    var previous: ?Position = null;
    var area: i64 = 0;
    var perimeter: i64 = 0;
    while (line_it.next()) |line| {
        const trench = try parseLine(line);
        perimeter += trench.distance;
        if (previous == null) {
            previous = .{ .x = 0, .y = 0 };
            continue;
        }
        const current = previous.?.move(trench.direction, trench.distance);
        area += previous.?.x * current.y - previous.?.y * current.x;
        previous = current;
    }

    return @divExact(area, 2) + @divExact(perimeter, 2) + 1;
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
        \\R 6 (#70c710)
        \\D 5 (#0dc571)
        \\L 2 (#5713f0)
        \\D 2 (#d2c081)
        \\R 2 (#59c680)
        \\D 2 (#411b91)
        \\L 5 (#8ceee2)
        \\U 2 (#caa173)
        \\L 1 (#1b58a2)
        \\U 2 (#caa171)
        \\R 2 (#7807d2)
        \\U 3 (#a77fa3)
        \\L 2 (#015232)
        \\U 2 (#7a21e3)
    ;
    const result = try run(input);
    try std.testing.expectEqual(@as(i64, 952408144115), result);
}
