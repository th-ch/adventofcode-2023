const std = @import("std");

var allocator: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn Grid(comptime T: type) type {
    return struct {
        const Self = @This();
        data: []T,
        width: usize,
        height: usize,

        pub inline fn at(self: Self, x: usize, y: usize) T {
            return self.data[y * self.width + x];
        }
    };
}

const PipeType = enum {
    start,
    vertical,
    horizontal,
    north_east,
    north_west,
    south_east,
    south_west,
    none,

    pub inline fn fromChar(c: u8) PipeType {
        return switch (c) {
            'S' => .start,
            '|' => .vertical,
            '-' => .horizontal,
            'L' => .north_east,
            'J' => .north_west,
            '7' => .south_west,
            'F' => .south_east,
            '.' => .none,
            else => unreachable,
        };
    }

    pub inline fn canConnectToNorth(t: PipeType) bool {
        return switch (t) {
            .start => true,
            .vertical => true,
            .horizontal => false,
            .north_east => true,
            .north_west => true,
            .south_east => false,
            .south_west => false,
            .none => false,
        };
    }

    pub inline fn canConnectToSouth(t: PipeType) bool {
        return switch (t) {
            .start => true,
            .vertical => true,
            .horizontal => false,
            .north_east => false,
            .north_west => false,
            .south_east => true,
            .south_west => true,
            .none => false,
        };
    }

    pub inline fn canConnectToWest(t: PipeType) bool {
        return switch (t) {
            .start => true,
            .vertical => false,
            .horizontal => true,
            .north_east => false,
            .north_west => true,
            .south_east => false,
            .south_west => true,
            .none => false,
        };
    }

    pub inline fn canConnectToEast(t: PipeType) bool {
        return switch (t) {
            .start => true,
            .vertical => false,
            .horizontal => true,
            .north_east => true,
            .north_west => false,
            .south_east => true,
            .south_west => false,
            .none => false,
        };
    }
};

const Position = struct {
    x: usize,
    y: usize,
};

fn parseGrid(input: []const u8) !Grid(PipeType) {
    const width = std.mem.indexOfScalar(u8, input, '\n').?;
    const height = std.mem.count(u8, input, "\n") + 1;

    const data = allocator.alloc(PipeType, width * height) catch unreachable;
    var i: usize = 0;
    for (input) |c| switch (c) {
        '\n' => {},
        else => {
            data[i] = PipeType.fromChar(c);
            i += 1;
        },
    };

    return Grid(PipeType){ .width = width, .height = height, .data = data };
}

fn findStartPoint(grid: Grid(PipeType)) Position {
    const index = std.mem.indexOfScalar(PipeType, grid.data, .start).?;
    const y: usize = index / grid.width;
    const x: usize = index % grid.width;

    return .{ .x = x, .y = y };
}

const QueueItem = struct {
    pos: Position,
    steps: i64,
};

fn run(input: [:0]const u8) !i64 {
    const grid = try parseGrid(input);
    const start = findStartPoint(grid);
    var set = std.AutoArrayHashMap(Position, void).init(allocator);
    var queue = std.fifo.LinearFifo(QueueItem, std.fifo.LinearFifoBufferType.Dynamic).init(allocator);
    try queue.writeItem(QueueItem{ .pos = start, .steps = 0 });

    var max_steps: i64 = 0;
    while (queue.readItem()) |item| {
        if (set.get(item.pos)) |_| continue;

        try set.put(item.pos, {});

        //std.debug.print("Visiting node at {}:{} steps: {}\n", .{ item.pos.x, item.pos.y, item.steps });
        max_steps = @max(max_steps, item.steps);

        const pipe_type = grid.at(item.pos.x, item.pos.y);

        if (pipe_type.canConnectToWest() and item.pos.x > 0) {
            const west_pos = Position{ .x = item.pos.x - 1, .y = item.pos.y };
            const west_pipe = grid.at(west_pos.x, west_pos.y);
            if (west_pipe == .horizontal or west_pipe == .north_east or west_pipe == .south_east) {
                try queue.writeItem(QueueItem{ .pos = west_pos, .steps = item.steps + 1 });
            }
        }

        if (pipe_type.canConnectToEast() and item.pos.x < grid.width - 1) {
            const east_pos = Position{ .x = item.pos.x + 1, .y = item.pos.y };
            const east_pipe = grid.at(east_pos.x, east_pos.y);
            if (east_pipe == .horizontal or east_pipe == .north_west or east_pipe == .south_west) {
                try queue.writeItem(QueueItem{ .pos = east_pos, .steps = item.steps + 1 });
            }
        }

        if (pipe_type.canConnectToNorth() and item.pos.y > 0) {
            const north_pos = Position{ .x = item.pos.x, .y = item.pos.y - 1 };
            const north_pipe = grid.at(north_pos.x, north_pos.y);
            if (north_pipe == .vertical or north_pipe == .south_east or north_pipe == .south_west) {
                try queue.writeItem(QueueItem{ .pos = north_pos, .steps = item.steps + 1 });
            }
        }

        if (pipe_type.canConnectToSouth() and item.pos.y < grid.height - 1) {
            const south_pos = Position{ .x = item.pos.x, .y = item.pos.y + 1 };
            const south_pipe = grid.at(south_pos.x, south_pos.y);
            if (south_pipe == .vertical or south_pipe == .north_east or south_pipe == .north_west) {
                try queue.writeItem(QueueItem{ .pos = south_pos, .steps = item.steps + 1 });
            }
        }
    }
    return max_steps;
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
        \\.....
        \\.S-7.
        \\.|.|.
        \\.L-J.
        \\.....
    ;

    const result = try run(input);
    try std.testing.expectEqual(@as(i64, 4), result);
}

test {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator); // create memory allocator for strings

    defer arena.deinit(); // clear memory
    allocator = arena.allocator();

    const input =
        \\..F7.
        \\.FJ|.
        \\SJ.L7
        \\|F--J
        \\LJ...
    ;

    const result = try run(input);
    try std.testing.expectEqual(@as(i64, 8), result);
}
