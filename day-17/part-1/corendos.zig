const std = @import("std");

var allocator: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn Grid(comptime T: type) type {
    return struct {
        width: usize,
        height: usize,
        data: []T,

        pub inline fn at(self: @This(), x: usize, y: usize) T {
            return self.data[y * self.width + x];
        }

        pub inline fn atPtr(self: @This(), x: usize, y: usize) *T {
            return &self.data[y * self.width + x];
        }

        pub fn format(self: @This(), comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
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
}

const Position = struct {
    x: usize,
    y: usize,

    pub inline fn move(self: Position, direction: Direction, step: usize, width: usize, height: usize) ?Position {
        return switch (direction) {
            .up => if (self.y >= step) .{ .x = self.x, .y = self.y - step } else null,
            .down => if (self.y < height - step) .{ .x = self.x, .y = self.y + step } else null,
            .left => if (self.x >= step) .{ .x = self.x - step, .y = self.y } else null,
            .right => if (self.x < width - step) .{ .x = self.x + step, .y = self.y } else null,
        };
    }
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

fn parseGrid(input: []const u8) !Grid(u8) {
    const width = std.mem.indexOfScalar(u8, input, '\n').?;
    const height = std.mem.count(u8, input, "\n") + 1;

    const data = try allocator.alloc(u8, width * height);
    var i: usize = 0;
    for (input) |c| {
        if (c == '\n') continue;
        data[i] = c - '0';
        i += 1;
    }

    return Grid(u8){ .width = width, .height = height, .data = data };
}

const QueueContext = struct {};

const QueueItem = struct {
    cost: usize,
    position: Position,
    vertical: u8,

    pub fn compare(a: QueueItem, b: QueueItem) std.math.Order {
        const cost_order = std.math.order(a.cost, b.cost);
        if (cost_order != .eq) return cost_order;

        const position_x_order = std.math.order(a.position.x, b.position.x);
        if (position_x_order != .eq) return position_x_order;

        const position_y_order = std.math.order(a.position.y, b.position.y);
        if (position_y_order != .eq) return position_y_order;

        return std.math.order(a.vertical, b.vertical);
    }
};

fn run(input: [:0]const u8) !i64 {
    const grid = try parseGrid(input);

    var distance = Grid([2]usize){
        .width = grid.width,
        .height = grid.height,
        .data = b: {
            const data = try allocator.alloc([2]usize, grid.width * grid.height);
            @memset(data, [2]usize{ std.math.maxInt(usize), std.math.maxInt(usize) });
            break :b data;
        },
    };

    var queue = std.PriorityQueue(QueueItem, void, (struct {
        pub fn compare(ctx: void, a: QueueItem, b: QueueItem) std.math.Order {
            _ = ctx;
            return QueueItem.compare(a, b);
        }
    }).compare).init(allocator, {});

    try queue.add(.{ .cost = 0, .position = .{ .x = 0, .y = 0 }, .vertical = 0 });
    try queue.add(.{ .cost = 0, .position = .{ .x = 0, .y = 0 }, .vertical = 1 });

    while (queue.peek()) |u| {
        _ = queue.remove();
        //std.debug.print("Exploring node {}:{} with direction {s} and count {}\n", .{ u.position.x, u.position.y, @tagName(u.direction), u.count });
        if (u.position.x == grid.width - 1 and u.position.y == grid.height - 1) {
            return @intCast(u.cost);
        }

        const new_vertical = 1 - u.vertical;
        if (u.vertical == 1) {
            var cost_up = u.cost;
            var cost_down = u.cost;
            for (1..4) |steps| {
                if (u.position.move(.up, steps, grid.width, grid.height)) |pos_up| {
                    cost_up += grid.at(pos_up.x, pos_up.y);
                    const old_cost = &distance.atPtr(pos_up.x, pos_up.y).*[new_vertical];
                    if (cost_up < old_cost.*) {
                        old_cost.* = cost_up;
                        try queue.add(.{ .cost = cost_up, .position = pos_up, .vertical = new_vertical });
                    }
                }

                if (u.position.move(.down, steps, grid.width, grid.height)) |pos_down| {
                    cost_down += grid.at(pos_down.x, pos_down.y);
                    const old_cost = &distance.atPtr(pos_down.x, pos_down.y).*[new_vertical];
                    if (cost_down < old_cost.*) {
                        old_cost.* = cost_down;
                        try queue.add(.{ .cost = cost_down, .position = pos_down, .vertical = new_vertical });
                    }
                }
            }
        } else {
            var cost_left = u.cost;
            var cost_right = u.cost;
            for (1..4) |steps| {
                if (u.position.move(.left, steps, grid.width, grid.height)) |pos_left| {
                    cost_left += grid.at(pos_left.x, pos_left.y);
                    const old_cost = &distance.atPtr(pos_left.x, pos_left.y).*[new_vertical];
                    if (cost_left < old_cost.*) {
                        old_cost.* = cost_left;
                        try queue.add(.{ .cost = cost_left, .position = pos_left, .vertical = new_vertical });
                    }
                }

                if (u.position.move(.right, steps, grid.width, grid.height)) |pos_right| {
                    cost_right += grid.at(pos_right.x, pos_right.y);
                    const old_cost = &distance.atPtr(pos_right.x, pos_right.y).*[new_vertical];
                    if (cost_right < old_cost.*) {
                        old_cost.* = cost_right;
                        try queue.add(.{ .cost = cost_right, .position = pos_right, .vertical = new_vertical });
                    }
                }
            }
        }
    }

    return 0;
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
    try std.testing.expectEqual(@as(i64, 102), result);
}
