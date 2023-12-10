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

const GridFormatter = struct {
    value: Grid(PipeType),

    pub fn format(f: GridFormatter, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        _ = fmt;
        _ = options;
        for (0..f.value.height) |y| {
            for (0..f.value.width) |x| {
                const t = f.value.at(x, y);
                switch (t) {
                    .start => try writer.writeByte('s'),
                    .vertical => try writer.writeAll("│"),
                    .horizontal => try writer.writeAll("─"),
                    .north_east => try writer.writeAll("└"),
                    .north_west => try writer.writeAll("┘"),
                    .south_east => try writer.writeAll("┌"),
                    .south_west => try writer.writeAll("┐"),
                    .none => try writer.writeByte(' '),
                }
            }
            try writer.writeByte('\n');
        }
    }
};

const Direction = enum {
    north,
    south,
    east,
    west,

    pub inline fn fromDelta(from: Position, to: Position) Direction {
        const from_xi: isize = @intCast(from.x);
        const from_yi: isize = @intCast(from.y);
        const to_xi: isize = @intCast(to.x);
        const to_yi: isize = @intCast(to.y);

        if (to_xi == from_xi + 1 and to_yi == from_yi) {
            return .east;
        } else if (to_xi == from_xi - 1 and to_yi == from_yi) {
            return .west;
        } else if (to_yi == from_yi + 1 and to_xi == from_xi) {
            return .south;
        } else if (to_yi == from_yi - 1 and to_xi == from_xi) {
            return .north;
        } else unreachable;
    }
};

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

fn addSeed(position: Position, seeds: *std.AutoHashMap(Position, void), path_set: *const std.AutoHashMap(Position, void)) !void {
    if (path_set.get(position)) |_| return;
    try seeds.put(position, {});
}

fn makePathGrid(original: Grid(PipeType), path: []const Position) !Grid(PipeType) {
    const data = try allocator.dupe(PipeType, original.data);
    @memset(data, .none);
    for (path) |pos| {
        data[pos.y * original.width + pos.x] = original.at(pos.x, pos.y);
    }
    return Grid(PipeType){ .data = data, .width = original.width, .height = original.height };
}

fn run(input: [:0]const u8) !i64 {
    const grid = try parseGrid(input);

    const start_pos = findStartPoint(grid);

    var path_set = std.AutoHashMap(Position, void).init(allocator);
    try path_set.put(start_pos, {});

    var path = std.ArrayList(Position).init(allocator);
    try path.append(start_pos);

    var last_pos = start_pos;
    var next_pos = b: {
        if (start_pos.x > 0) {
            const west_pos = Position{ .x = start_pos.x - 1, .y = start_pos.y };
            const west_pipe = grid.at(west_pos.x, west_pos.y);
            if (west_pipe == .horizontal or west_pipe == .north_east or west_pipe == .south_east) {
                break :b west_pos;
            }
        }

        if (start_pos.x < grid.width - 1) {
            const east_pos = Position{ .x = start_pos.x + 1, .y = start_pos.y };
            const east_pipe = grid.at(east_pos.x, east_pos.y);
            if (east_pipe == .horizontal or east_pipe == .north_west or east_pipe == .south_west) {
                break :b east_pos;
            }
        }

        if (start_pos.y > 0) {
            const north_pos = Position{ .x = start_pos.x, .y = start_pos.y - 1 };
            const north_pipe = grid.at(north_pos.x, north_pos.y);
            if (north_pipe == .vertical or north_pipe == .south_east or north_pipe == .south_west) {
                break :b north_pos;
            }
        }

        if (start_pos.y < grid.height - 1) {
            const south_pos = Position{ .x = start_pos.x, .y = start_pos.y + 1 };
            const south_pipe = grid.at(south_pos.x, south_pos.y);
            if (south_pipe == .vertical or south_pipe == .north_east or south_pipe == .north_west) {
                break :b south_pos;
            }
        }

        unreachable;
    };

    var turns: isize = 0;
    while (true) {
        const pos = next_pos;
        if (pos.x == start_pos.x and pos.y == start_pos.y) break;

        //std.debug.print("Visiting node at {}:{} steps: {}\n", .{ item.pos.x, item.pos.y, item.steps });

        const direction = Direction.fromDelta(last_pos, pos);
        const pipe_type = grid.at(pos.x, pos.y);
        last_pos = pos;
        next_pos = switch (pipe_type) {
            .vertical => b: {
                if (direction == .north) {
                    break :b .{ .x = pos.x, .y = pos.y - 1 };
                } else if (direction == .south) {
                    break :b .{ .x = pos.x, .y = pos.y + 1 };
                } else unreachable;
            },
            .horizontal => b: {
                if (direction == .east) {
                    break :b .{ .x = pos.x + 1, .y = pos.y };
                } else if (direction == .west) {
                    break :b .{ .x = pos.x - 1, .y = pos.y };
                } else unreachable;
            },
            .north_east => b: {
                if (direction == .south) {
                    turns += 1;
                    break :b .{ .x = pos.x + 1, .y = pos.y };
                } else if (direction == .west) {
                    turns -= 1;
                    break :b .{ .x = pos.x, .y = pos.y - 1 };
                } else unreachable;
            },
            .north_west => b: {
                if (direction == .south) {
                    turns -= 1;
                    break :b .{ .x = pos.x - 1, .y = pos.y };
                } else if (direction == .east) {
                    turns += 1;
                    break :b .{ .x = pos.x, .y = pos.y - 1 };
                } else unreachable;
            },
            .south_east => b: {
                if (direction == .north) {
                    turns -= 1;
                    break :b .{ .x = pos.x + 1, .y = pos.y };
                } else if (direction == .west) {
                    turns += 1;
                    break :b .{ .x = pos.x, .y = pos.y + 1 };
                } else unreachable;
            },
            .south_west => b: {
                if (direction == .north) {
                    turns += 1;
                    break :b .{ .x = pos.x - 1, .y = pos.y };
                } else if (direction == .east) {
                    turns -= 1;
                    break :b .{ .x = pos.x, .y = pos.y + 1 };
                } else unreachable;
            },
            else => unreachable,
        };
        try path_set.put(pos, {});
        try path.append(pos);
    }

    var seeds = std.AutoHashMap(Position, void).init(allocator);

    for (1..path.items.len) |i| {
        const from_pos = path.items[i - 1];
        const pos = path.items[i];
        const direction = Direction.fromDelta(from_pos, pos);

        const pipe_type = grid.at(pos.x, pos.y);
        switch (pipe_type) {
            .vertical => {
                if (direction == .north) {
                    if (turns < 0) {
                        try addSeed(.{ .x = pos.x + 1, .y = pos.y }, &seeds, &path_set);
                    } else {
                        try addSeed(.{ .x = pos.x - 1, .y = pos.y }, &seeds, &path_set);
                    }
                } else if (direction == .south) {
                    if (turns < 0) {
                        try addSeed(.{ .x = pos.x - 1, .y = pos.y }, &seeds, &path_set);
                    } else {
                        try addSeed(.{ .x = pos.x + 1, .y = pos.y }, &seeds, &path_set);
                    }
                } else unreachable;
            },
            .horizontal => {
                if (direction == .east) {
                    if (turns < 0) {
                        try addSeed(.{ .x = pos.x, .y = pos.y + 1 }, &seeds, &path_set);
                    } else {
                        try addSeed(.{ .x = pos.x, .y = pos.y - 1 }, &seeds, &path_set);
                    }
                } else if (direction == .west) {
                    if (turns < 0) {
                        try addSeed(.{ .x = pos.x, .y = pos.y - 1 }, &seeds, &path_set);
                    } else {
                        try addSeed(.{ .x = pos.x, .y = pos.y + 1 }, &seeds, &path_set);
                    }
                } else unreachable;
            },
            .north_east => {
                if (direction == .south) {
                    if (turns < 0) {
                        try addSeed(.{ .x = pos.x - 1, .y = pos.y }, &seeds, &path_set);
                        try addSeed(.{ .x = pos.x - 1, .y = pos.y + 1 }, &seeds, &path_set);
                        try addSeed(.{ .x = pos.x, .y = pos.y + 1 }, &seeds, &path_set);
                    } else {
                        try addSeed(.{ .x = pos.x + 1, .y = pos.y - 1 }, &seeds, &path_set);
                    }
                } else if (direction == .west) {
                    if (turns < 0) {
                        try addSeed(.{ .x = pos.x + 1, .y = pos.y - 1 }, &seeds, &path_set);
                    } else {
                        try addSeed(.{ .x = pos.x - 1, .y = pos.y }, &seeds, &path_set);
                        try addSeed(.{ .x = pos.x - 1, .y = pos.y + 1 }, &seeds, &path_set);
                        try addSeed(.{ .x = pos.x, .y = pos.y + 1 }, &seeds, &path_set);
                    }
                } else unreachable;
            },
            .north_west => {
                if (direction == .south) {
                    if (turns < 0) {
                        try addSeed(.{ .x = pos.x - 1, .y = pos.y - 1 }, &seeds, &path_set);
                    } else {
                        try addSeed(.{ .x = pos.x + 1, .y = pos.y }, &seeds, &path_set);
                        try addSeed(.{ .x = pos.x + 1, .y = pos.y + 1 }, &seeds, &path_set);
                        try addSeed(.{ .x = pos.x, .y = pos.y + 1 }, &seeds, &path_set);
                    }
                } else if (direction == .east) {
                    if (turns < 0) {
                        try addSeed(.{ .x = pos.x + 1, .y = pos.y }, &seeds, &path_set);
                        try addSeed(.{ .x = pos.x + 1, .y = pos.y + 1 }, &seeds, &path_set);
                        try addSeed(.{ .x = pos.x, .y = pos.y + 1 }, &seeds, &path_set);
                    } else {
                        try addSeed(.{ .x = pos.x - 1, .y = pos.y - 1 }, &seeds, &path_set);
                    }
                } else unreachable;
            },
            .south_east => {
                if (direction == .north) {
                    if (turns < 0) {
                        try addSeed(.{ .x = pos.x + 1, .y = pos.y + 1 }, &seeds, &path_set);
                    } else {
                        try addSeed(.{ .x = pos.x - 1, .y = pos.y }, &seeds, &path_set);
                        try addSeed(.{ .x = pos.x - 1, .y = pos.y - 1 }, &seeds, &path_set);
                        try addSeed(.{ .x = pos.x, .y = pos.y - 1 }, &seeds, &path_set);
                    }
                } else if (direction == .west) {
                    if (turns < 0) {
                        try addSeed(.{ .x = pos.x - 1, .y = pos.y }, &seeds, &path_set);
                        try addSeed(.{ .x = pos.x - 1, .y = pos.y - 1 }, &seeds, &path_set);
                        try addSeed(.{ .x = pos.x, .y = pos.y - 1 }, &seeds, &path_set);
                    } else {
                        try addSeed(.{ .x = pos.x + 1, .y = pos.y + 1 }, &seeds, &path_set);
                    }
                } else unreachable;
            },
            .south_west => {
                if (direction == .north) {
                    if (turns < 0) {
                        try addSeed(.{ .x = pos.x + 1, .y = pos.y }, &seeds, &path_set);
                        try addSeed(.{ .x = pos.x + 1, .y = pos.y - 1 }, &seeds, &path_set);
                        try addSeed(.{ .x = pos.x, .y = pos.y - 1 }, &seeds, &path_set);
                    } else {
                        try addSeed(.{ .x = pos.x - 1, .y = pos.y + 1 }, &seeds, &path_set);
                    }
                } else if (direction == .east) {
                    if (turns < 0) {
                        try addSeed(.{ .x = pos.x - 1, .y = pos.y + 1 }, &seeds, &path_set);
                    } else {
                        try addSeed(.{ .x = pos.x + 1, .y = pos.y }, &seeds, &path_set);
                        try addSeed(.{ .x = pos.x + 1, .y = pos.y - 1 }, &seeds, &path_set);
                        try addSeed(.{ .x = pos.x, .y = pos.y - 1 }, &seeds, &path_set);
                    }
                } else unreachable;
            },
            else => unreachable,
        }
    }

    var queue = std.fifo.LinearFifo(Position, .Dynamic).init(allocator);
    var it = seeds.iterator();
    while (it.next()) |entry| {
        try queue.writeItem(entry.key_ptr.*);
    }

    var inside_set = std.AutoHashMap(Position, void).init(allocator);

    while (queue.readItem()) |pos| {
        if (path_set.get(pos)) |_| continue;
        if (inside_set.get(pos)) |_| continue;

        try inside_set.put(pos, {});

        if (pos.x > 0) {
            try queue.writeItem(.{ .x = pos.x - 1, .y = pos.y });
        }
        if (pos.x < grid.width - 1) {
            try queue.writeItem(.{ .x = pos.x + 1, .y = pos.y });
        }
        if (pos.y > 0) {
            try queue.writeItem(.{ .x = pos.x, .y = pos.y - 1 });
        }
        if (pos.y < grid.height - 1) {
            try queue.writeItem(.{ .x = pos.x, .y = pos.y + 1 });
        }
    }

    return inside_set.count();
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
        \\...........
        \\.S-------7.
        \\.|F-----7|.
        \\.||.....||.
        \\.||.....||.
        \\.|L-7.F-J|.
        \\.|..|.|..|.
        \\.L--J.L--J.
        \\...........
    ;

    const result = try run(input);
    try std.testing.expectEqual(@as(i64, 4), result);
}

test {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator); // create memory allocator for strings

    defer arena.deinit(); // clear memory
    allocator = arena.allocator();

    const input =
        \\.F----7F7F7F7F-7....
        \\.|F--7||||||||FJ....
        \\.||.FJ||||||||L7....
        \\FJL7L7LJLJ||LJ.L-7..
        \\L--J.L7...LJS7F-7L7.
        \\....F-J..F7FJ|L7L7L7
        \\....L7.F7||L7|.L7L7|
        \\.....|FJLJ|FJ|F7|.LJ
        \\....FJL-7.||.||||...
        \\....L---J.LJ.LJLJ...
    ;

    const result = try run(input);
    try std.testing.expectEqual(@as(i64, 8), result);
}

test {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator); // create memory allocator for strings

    defer arena.deinit(); // clear memory
    allocator = arena.allocator();

    const input =
        \\FF7FSF7F7F7F7F7F---7
        \\L|LJ||||||||||||F--J
        \\FL-7LJLJ||||||LJL-77
        \\F--JF--7||LJLJ7F7FJ-
        \\L---JF-JLJ.||-FJLJJ7
        \\|F|F-JF---7F7-L7L|7|
        \\|FFJF7L7F-JF7|JL---7
        \\7-L-JL7||F7|L7F-7F7|
        \\L.L7LFJ|||||FJL7||LJ
        \\L7JLJL-JLJLJL--JLJ.L
    ;

    const result = try run(input);
    try std.testing.expectEqual(@as(i64, 10), result);
}
