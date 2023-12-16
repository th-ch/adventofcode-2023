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

    pub inline fn atPtr(self: Grid, x: usize, y: usize) *u8 {
        return &self.data[y * self.width + x];
    }

    pub inline fn move(self: Grid, from_x: usize, from_y: usize, to_x: usize, to_y: usize) void {
        const temp = self.at(from_x, from_y);
        self.atPtr(from_x, from_y).* = '.';
        self.atPtr(to_x, to_y).* = temp;
    }

    pub fn findRoundRock(self: Grid, column_index: usize, start_row_index: usize) ?usize {
        return for (start_row_index + 1..self.height) |row_index| {
            const c = self.at(column_index, row_index);
            // We have found a round rock
            if (c == 'O') break row_index;
            // There is a static rock blocking the way, we can abort the search.
            if (c == '#') break null;
        } else null;
    }

    pub fn format(self: Grid, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        _ = fmt;
        _ = options;
        for (0..self.height) |y| {
            try writer.print("{s}\n", .{self.data[y * self.width .. (y + 1) * self.width]});
        }
    }
};

const GridContext = struct {
    pub fn hash(ctx: GridContext, grid: Grid) u64 {
        _ = ctx;
        return std.hash.Wyhash.hash(0, grid.data);
    }

    pub fn eql(ctx: GridContext, a: Grid, b: Grid) bool {
        _ = ctx;
        return std.mem.eql(u8, a.data, b.data);
    }
};

fn parseGrid(input: []const u8) !Grid {
    const width = std.mem.indexOfScalar(u8, input, '\n').?;
    const height = std.mem.count(u8, input, "\n") + 1;

    const data = try allocator.alloc(u8, width * height);

    var index: usize = 0;
    for (input) |c| {
        if (c == '\n') continue;
        data[index] = c;

        index += 1;
    }

    return Grid{ .width = width, .height = height, .data = data };
}

fn moveRockNorth(grid: Grid) void {
    for (0..grid.width) |column_index| {
        var last_obstacle_index: usize = 0;
        for (0..grid.height) |row_index| {
            const c = grid.at(column_index, row_index);
            if (c == '#') {
                last_obstacle_index = row_index + 1;
            } else if (c == 'O') {
                grid.move(column_index, row_index, column_index, last_obstacle_index);
                last_obstacle_index = last_obstacle_index + 1;
            }
        }
    }
}

fn moveRockSouth(grid: Grid) void {
    for (0..grid.width) |column_index| {
        var last_obstacle_index: usize = grid.height;
        for (0..grid.height) |inverse_row_index| {
            const row_index = grid.height - 1 - inverse_row_index;
            const c = grid.at(column_index, row_index);
            if (c == '#') {
                last_obstacle_index = row_index;
            } else if (c == 'O') {
                grid.move(column_index, row_index, column_index, last_obstacle_index - 1);
                last_obstacle_index = last_obstacle_index - 1;
            }
        }
    }
}

fn moveRockWest(grid: Grid) void {
    for (0..grid.height) |row_index| {
        var last_obstacle_index: usize = 0;
        for (0..grid.width) |column_index| {
            const c = grid.at(column_index, row_index);
            if (c == '#') {
                last_obstacle_index = column_index + 1;
            } else if (c == 'O') {
                grid.move(column_index, row_index, last_obstacle_index, row_index);
                last_obstacle_index = last_obstacle_index + 1;
            }
        }
    }
}

fn moveRockEast(grid: Grid) void {
    for (0..grid.height) |row_index| {
        var last_obstacle_index: usize = grid.width;
        for (0..grid.width) |inverse_column_index| {
            const column_index = grid.width - 1 - inverse_column_index;
            const c = grid.at(column_index, row_index);
            if (c == '#') {
                last_obstacle_index = column_index;
            } else if (c == 'O') {
                grid.move(column_index, row_index, last_obstacle_index - 1, row_index);
                last_obstacle_index = last_obstacle_index - 1;
            }
        }
    }
}

fn cycle(grid: Grid) void {
    moveRockNorth(grid);
    moveRockWest(grid);
    moveRockSouth(grid);
    moveRockEast(grid);
}

fn computeLoad(grid: Grid) i64 {
    var result: i64 = 0;
    for (0..grid.height) |y| {
        const factor: i64 = @intCast(grid.height - y);
        const round_rock_count = std.mem.count(u8, grid.data[y * grid.width .. (y + 1) * grid.width], "O");
        result += factor * @as(i64, @intCast(round_rock_count));
    }
    return result;
}

fn run(input: [:0]const u8) !i64 {
    const grid = try parseGrid(input);
    // This map contains the association between a given grid and at which iteration it was obtained.
    var grid_iteration_map = std.HashMap(Grid, usize, GridContext, std.hash_map.default_max_load_percentage).init(allocator);
    // This array contains all the grid we've seen. The index of a grid is the iteration index at which it was observed.
    var grid_seen = std.ArrayList(Grid).init(allocator);
    {
        const new_grid = Grid{ .width = grid.width, .height = grid.height, .data = try allocator.dupe(u8, grid.data) };
        try grid_iteration_map.put(new_grid, 0);
        try grid_seen.append(new_grid);
    }

    var cycle_start: usize = 0;
    var cycle_length: usize = 0;
    for (1..1_000_000_000) |i| {
        cycle(grid);

        const gop = try grid_iteration_map.getOrPut(grid);
        if (!gop.found_existing) {
            // If the grid has not been observed yet, we add it to the map and array.
            const new_grid = Grid{ .width = grid.width, .height = grid.height, .data = try allocator.dupe(u8, grid.data) };
            gop.key_ptr.* = new_grid;
            gop.value_ptr.* = i;
            try grid_seen.append(new_grid);
        } else {
            // Otherwise, we compute the index at which the cycle start and we compute the size of the cycle.
            cycle_start = gop.value_ptr.*;
            cycle_length = i - gop.value_ptr.*;
            break;
        }
    }

    // Since we know there is a cycle:
    // * We skip the first iterations before the cycle
    // * Then we compute how many times we go through the cycle
    // * We derive the equivalent iteration index to retrieve the grid we're at after 1000000000 iterations.
    const equivalent_iteration_index = (1_000_000_000 - cycle_start) % cycle_length + cycle_start;
    const load = computeLoad(grid_seen.items[equivalent_iteration_index]);
    return load;
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
        \\O....#....
        \\O.OO#....#
        \\.....##...
        \\OO.#O....O
        \\.O.....O#.
        \\O.#..O.#.#
        \\..O..#O..O
        \\.......O..
        \\#....###..
        \\#OO..#....
    ;
    const result = try run(input);
    try std.testing.expectEqual(@as(i64, 64), result);
}
