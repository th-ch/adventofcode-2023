const std = @import("std");

var allocator: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const CellType = enum {
    empty,
    forward_mirror,
    backward_mirror,
    horizontal_splitter,
    vertical_splitter,

    pub inline fn toChar(self: CellType) u8 {
        return switch (self) {
            .empty => '.',
            .forward_mirror => '/',
            .backward_mirror => '\\',
            .horizontal_splitter => '-',
            .vertical_splitter => '|',
        };
    }
};

fn Grid(comptime T: type) type {
    return struct {
        width: usize,
        height: usize,
        data: []T,

        pub fn at(self: @This(), x: usize, y: usize) T {
            return self.data[y * self.width + x];
        }
    };
}

const MirrorGrid = Grid(CellType);

const Position = struct {
    x: usize,
    y: usize,
};

const Direction = enum {
    up,
    down,
    left,
    right,
};

const QueueEntry = struct {
    position: Position,
    direction: Direction,
};

fn parseGrid(input: []const u8) !MirrorGrid {
    const width = std.mem.indexOfScalar(u8, input, '\n').?;
    const height = std.mem.count(u8, input, "\n") + 1;

    const data = try allocator.alloc(CellType, width * height);
    var i: usize = 0;
    for (input) |c| {
        if (c == '\n') continue;
        data[i] = switch (c) {
            '.' => .empty,
            '/' => .forward_mirror,
            '\\' => .backward_mirror,
            '-' => .horizontal_splitter,
            '|' => .vertical_splitter,
            else => unreachable,
        };
        i += 1;
    }

    return MirrorGrid{ .width = width, .height = height, .data = data };
}

fn nextPosition(position: Position, direction: Direction, grid: MirrorGrid) ?Position {
    return switch (direction) {
        .up => b: {
            if (position.y > 0) break :b .{ .x = position.x, .y = position.y - 1 };
            break :b null;
        },
        .down => b: {
            if (position.y < grid.height - 1) break :b .{ .x = position.x, .y = position.y + 1 };
            break :b null;
        },
        .left => b: {
            if (position.x > 0) break :b .{ .x = position.x - 1, .y = position.y };
            break :b null;
        },
        .right => b: {
            if (position.x < grid.width - 1) break :b .{ .x = position.x + 1, .y = position.y };
            break :b null;
        },
    };
}

fn runFrom(mirror_grid: MirrorGrid, start_position: Position, start_direction: Direction) !i64 {
    var queue = std.ArrayList(QueueEntry).init(allocator);
    var visited = std.AutoHashMap(QueueEntry, void).init(allocator);
    var distinct = std.AutoHashMap(Position, void).init(allocator);

    try queue.append(.{ .position = start_position, .direction = start_direction });

    while (queue.items.len > 0) {
        const item = queue.swapRemove(0);
        var current_position = item.position;
        var current_direction = item.direction;

        while (true) {
            const gop = try visited.getOrPut(.{ .position = current_position, .direction = current_direction });
            if (gop.found_existing) break;

            try distinct.put(current_position, {});

            const cell_type = mirror_grid.at(current_position.x, current_position.y);
            switch (cell_type) {
                .empty => {
                    current_position = nextPosition(current_position, current_direction, mirror_grid) orelse break;
                },
                .forward_mirror => {
                    const new_direction: Direction = switch (current_direction) {
                        .up => .right,
                        .down => .left,
                        .left => .down,
                        .right => .up,
                    };
                    current_position = nextPosition(current_position, new_direction, mirror_grid) orelse break;
                    current_direction = new_direction;
                },
                .backward_mirror => {
                    const new_direction: Direction = switch (current_direction) {
                        .up => .left,
                        .down => .right,
                        .left => .up,
                        .right => .down,
                    };
                    current_position = nextPosition(current_position, new_direction, mirror_grid) orelse break;
                    current_direction = new_direction;
                },
                .horizontal_splitter => {
                    if (current_direction == .left or current_direction == .right) {
                        current_position = nextPosition(current_position, current_direction, mirror_grid) orelse break;
                    } else {
                        if (nextPosition(current_position, .left, mirror_grid)) |left_pos| {
                            try queue.append(.{ .position = left_pos, .direction = .left });
                        }
                        if (nextPosition(current_position, .right, mirror_grid)) |right_pos| {
                            try queue.append(.{ .position = right_pos, .direction = .right });
                        }
                        break;
                    }
                },
                .vertical_splitter => {
                    if (current_direction == .up or current_direction == .down) {
                        current_position = nextPosition(current_position, current_direction, mirror_grid) orelse break;
                    } else {
                        if (nextPosition(current_position, .up, mirror_grid)) |up_pos| {
                            try queue.append(.{ .position = up_pos, .direction = .up });
                        }
                        if (nextPosition(current_position, .down, mirror_grid)) |down_pos| {
                            try queue.append(.{ .position = down_pos, .direction = .down });
                        }
                        break;
                    }
                },
            }
        }
    }

    const energized_cell_count: i64 = @intCast(distinct.count());

    return energized_cell_count;
}

fn run(input: [:0]const u8) !i64 {
    const mirror_grid = try parseGrid(input);
    var result: i64 = 0;

    for (0..mirror_grid.width) |x| {
        const down_result = try runFrom(mirror_grid, .{ .x = x, .y = 0 }, .down);
        const up_result = try runFrom(mirror_grid, .{ .x = x, .y = mirror_grid.height - 1 }, .up);
        result = @max(result, @max(down_result, up_result));
    }

    for (0..mirror_grid.height) |y| {
        const right_result = try runFrom(mirror_grid, .{ .x = 0, .y = y }, .right);
        const left_result = try runFrom(mirror_grid, .{ .x = mirror_grid.width - 1, .y = y }, .left);
        result = @max(result, @max(right_result, left_result));
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
        \\.|...\....
        \\|.-.\.....
        \\.....|-...
        \\........|.
        \\..........
        \\.........\
        \\..../.\\..
        \\.-.-/..|..
        \\.|....-|.\
        \\..//.|....
    ;
    const result = try run(input);
    try std.testing.expectEqual(@as(i64, 51), result);
}
