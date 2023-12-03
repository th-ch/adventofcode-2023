const std = @import("std");

var a: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

inline fn isNumber(c: u8) bool {
    return '0' <= c and c <= '9';
}

const directions = .{
    .{ .x = -1, .y = -1 },
    .{ .x = -1, .y = 0 },
    .{ .x = -1, .y = 1 },

    .{ .x = 0, .y = -1 },
    .{ .x = 0, .y = 1 },

    .{ .x = 1, .y = -1 },
    .{ .x = 1, .y = 0 },
    .{ .x = 1, .y = 1 },
};

const Position = struct {
    x: usize,
    y: usize,
};

const Grid = struct {
    width: usize,
    height: usize,
    data: []const u8,

    pub inline fn at(self: Grid, x: usize, y: usize) u8 {
        return self.data[y * self.width + x];
    }

    pub inline fn isGear(self: Grid, x: isize, y: isize) bool {
        if (x < 0 or y < 0 or x >= self.width or y >= self.height) return false;
        const c = self.at(@intCast(x), @intCast(y));
        return c == '*';
    }

    pub inline fn slice(self: Grid, start: usize, end: usize, y: usize) []const u8 {
        return self.data[y * self.width + start .. y * self.width + end];
    }

    pub fn hasGearAround(self: Grid, start: usize, end: usize, y: usize) ?Position {
        for (start..end) |x| {
            const xi: isize = @intCast(x);
            const yi: isize = @intCast(y);
            inline for (directions) |d| {
                if (self.isGear(xi + d.x, yi + d.y)) {
                    return .{ .x = @intCast(xi + d.x), .y = @intCast(yi + d.y) };
                }
            }
        }
        return null;
    }
};

fn createGrid(input: []const u8) Grid {
    const width = std.mem.indexOfScalar(u8, input, '\n').?;
    const height = std.mem.count(u8, input, "\n") + 1;

    const data = a.alloc(u8, width * height) catch unreachable;
    var i: usize = 0;
    for (input) |c| switch (c) {
        '\n' => {},
        else => {
            data[i] = c;
            i += 1;
        },
    };

    return Grid{ .width = width, .height = height, .data = data };
}

fn run(input: [:0]const u8) i64 {
    const grid = createGrid(input);

    var hashmap = std.AutoHashMap(Position, std.ArrayList(i64)).init(a);

    var result: i64 = 0;

    var y: usize = 0;
    while (y < grid.height) {
        var x: usize = 0;
        while (x < grid.width) {
            const c = grid.at(x, y);
            switch (c) {
                '0'...'9' => {
                    const start = x;
                    x += 1;
                    while (x < grid.width and isNumber(grid.at(x, y))) : (x += 1) {}
                    const end = x;
                    if (grid.hasGearAround(start, end, y)) |gear_position| {
                        const number = std.fmt.parseInt(i64, grid.slice(start, end, y), 10) catch unreachable;
                        const gop = hashmap.getOrPut(gear_position) catch unreachable;
                        if (!gop.found_existing) {
                            gop.value_ptr.* = std.ArrayList(i64).initCapacity(a, 8) catch unreachable;
                        }
                        gop.value_ptr.append(number) catch unreachable;
                    }
                },
                else => x += 1,
            }
        }
        y += 1;
    }

    var it = hashmap.iterator();
    while (it.next()) |entry| {
        const items = entry.value_ptr.items;
        if (items.len == 2) {
            result += items[0] * items[1];
        }
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
        \\467..114..
        \\...*......
        \\..35..633.
        \\......#...
        \\617*......
        \\.....+.58.
        \\..592.....
        \\......755.
        \\...$.*....
        \\.664.598..
    ;
    const result = run(input);
    try std.testing.expectEqual(@as(i64, 467835), result);
}
