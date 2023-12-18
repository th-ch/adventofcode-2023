const std = @import("std");

var allocator: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const Grid = struct {
    width: usize,
    height: usize,
    data: []u8,

    pub inline fn at(self: Grid, x: usize, y: usize) u8 {
        return self.data[y * self.width + x];
    }

    pub fn format(self: Grid, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        _ = fmt;
        _ = options;
        for (0..self.height) |y| {
            for (0..self.width) |x| {
                try writer.print("{}", .{self.data[y * self.width + x]});
            }
            try writer.writeByte('\n');
        }
    }
};

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

const Vertex = struct {
    position: Position,
    direction: Direction,
    count: usize,
};

fn parseGrid(input: []const u8) !Grid {
    const width = std.mem.indexOfScalar(u8, input, '\n').?;
    const height = std.mem.count(u8, input, "\n") + 1;

    const data = try allocator.alloc(u8, width * height);
    var i: usize = 0;
    for (input) |c| {
        if (c == '\n') continue;
        data[i] = c - '0';
        i += 1;
    }

    return Grid{ .width = width, .height = height, .data = data };
}

fn getNeighbors(vertex: Vertex, grid: Grid) [3]?Vertex {
    var result = [_]?Vertex{null} ** 3;

    switch (vertex.direction) {
        .up => {
            if (vertex.count >= 3 and vertex.position.x > 0) {
                result[0] = Vertex{
                    .position = .{ .x = vertex.position.x - 1, .y = vertex.position.y },
                    .direction = .left,
                    .count = 0,
                };
            }

            if (vertex.count >= 3 and vertex.position.x < grid.width - 1) {
                result[1] = Vertex{
                    .position = .{ .x = vertex.position.x + 1, .y = vertex.position.y },
                    .direction = .right,
                    .count = 0,
                };
            }

            if (vertex.count < 9 and vertex.position.y > 0) {
                result[2] = Vertex{
                    .position = .{ .x = vertex.position.x, .y = vertex.position.y - 1 },
                    .direction = .up,
                    .count = vertex.count + 1,
                };
            }
        },
        .down => {
            if (vertex.count >= 3 and vertex.position.x > 0) {
                result[0] = Vertex{
                    .position = .{ .x = vertex.position.x - 1, .y = vertex.position.y },
                    .direction = .left,
                    .count = 0,
                };
            }

            if (vertex.count >= 3 and vertex.position.x < grid.width - 1) {
                result[1] = Vertex{
                    .position = .{ .x = vertex.position.x + 1, .y = vertex.position.y },
                    .direction = .right,
                    .count = 0,
                };
            }

            if (vertex.count < 9 and vertex.position.y < grid.height - 1) {
                result[2] = Vertex{
                    .position = .{ .x = vertex.position.x, .y = vertex.position.y + 1 },
                    .direction = .down,
                    .count = vertex.count + 1,
                };
            }
        },
        .left => {
            if (vertex.count >= 3 and vertex.position.y > 0) {
                result[0] = Vertex{
                    .position = .{ .x = vertex.position.x, .y = vertex.position.y - 1 },
                    .direction = .up,
                    .count = 0,
                };
            }

            if (vertex.count >= 3 and vertex.position.y < grid.height - 1) {
                result[1] = Vertex{
                    .position = .{ .x = vertex.position.x, .y = vertex.position.y + 1 },
                    .direction = .down,
                    .count = 0,
                };
            }

            if (vertex.count < 9 and vertex.position.x > 0) {
                result[2] = Vertex{
                    .position = .{ .x = vertex.position.x - 1, .y = vertex.position.y },
                    .direction = .left,
                    .count = vertex.count + 1,
                };
            }
        },
        .right => {
            if (vertex.count >= 3 and vertex.position.y > 0) {
                result[0] = Vertex{
                    .position = .{ .x = vertex.position.x, .y = vertex.position.y - 1 },
                    .direction = .up,
                    .count = 0,
                };
            }

            if (vertex.count >= 3 and vertex.position.y < grid.height - 1) {
                result[1] = Vertex{
                    .position = .{ .x = vertex.position.x, .y = vertex.position.y + 1 },
                    .direction = .down,
                    .count = 0,
                };
            }

            if (vertex.count < 9 and vertex.position.x < grid.width - 1) {
                result[2] = Vertex{
                    .position = .{ .x = vertex.position.x + 1, .y = vertex.position.y },
                    .direction = .right,
                    .count = vertex.count + 1,
                };
            }
        },
    }

    return result;
}
const QueueContext = struct {
    distance: *const std.AutoHashMap(Vertex, usize),

    pub fn compare(self: QueueContext, a: Vertex, b: Vertex) std.math.Order {
        const a_priority = self.distance.get(a) orelse std.math.maxInt(usize);
        const b_priority = self.distance.get(b) orelse std.math.maxInt(usize);

        return std.math.order(a_priority, b_priority);
    }
};

fn run(input: [:0]const u8) !i64 {
    const grid = try parseGrid(input);

    var distance = std.AutoHashMap(Vertex, usize).init(allocator);
    var queue = std.PriorityQueue(Vertex, QueueContext, QueueContext.compare).init(allocator, .{ .distance = &distance });

    const source1 = Vertex{ .position = .{ .x = 0, .y = 0 }, .direction = .right, .count = 0 };
    try distance.put(source1, 0);
    try queue.add(source1);
    const source2 = Vertex{ .position = .{ .x = 0, .y = 0 }, .direction = .down, .count = 0 };
    try distance.put(source2, 0);
    try queue.add(source2);

    while (queue.peek()) |u| {
        _ = queue.remove();
        //std.debug.print("Exploring node {}:{} with direction {s} and count {}\n", .{ u.position.x, u.position.y, @tagName(u.direction), u.count });
        const d = distance.get(u).?;
        if (u.position.x == grid.width - 1 and u.position.y == grid.height - 1 and u.count >= 3) {
            return @intCast(d);
        }

        const neighbors = getNeighbors(u, grid);
        for (neighbors) |maybe_n| if (maybe_n) |n| {
            const alt = d + @as(usize, grid.at(n.position.x, n.position.y));
            if (alt < distance.get(n) orelse std.math.maxInt(usize)) {
                try distance.put(n, alt);
                try queue.add(n);
            }
        };
    }

    var result: i64 = std.math.maxInt(i64);
    var it = distance.iterator();
    while (it.next()) |entry| {
        if (entry.key_ptr.position.x == grid.width - 1 and entry.key_ptr.position.y == grid.height - 1 and entry.key_ptr.count >= 3) {
            std.debug.print("Distance for {s} and count {}: {}\n", .{ @tagName(entry.key_ptr.direction), entry.key_ptr.count, entry.value_ptr.* });
            result = @min(result, @as(i64, @intCast(entry.value_ptr.*)));
        }
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
        \\2413432311323
        \\3215453535623
        \\3255245654254
        \\3446585845452
        \\4546657867536
        \\1438598798454
        \\4457876987766
        \\3637877979653
        \\4654967986887
        \\4564679986453
        \\1224686865563
        \\2546548887735
        \\4322674655533
    ;
    const result = try run(input);
    try std.testing.expectEqual(@as(i64, 94), result);
}

test {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator); // create memory allocator for strings

    defer arena.deinit(); // clear memory
    allocator = arena.allocator();

    const input =
        \\111111111111
        \\999999999991
        \\999999999991
        \\999999999991
        \\999999999991
    ;
    const result = try run(input);
    try std.testing.expectEqual(@as(i64, 71), result);
}
