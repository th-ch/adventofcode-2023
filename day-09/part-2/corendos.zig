const std = @import("std");

var allocator: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn parseLine(line: []const u8) ![]i64 {
    var numbers = try std.ArrayList(i64).initCapacity(allocator, 32);
    var it = std.mem.splitScalar(u8, line, ' ');
    while (it.next()) |raw_number| {
        const number = try std.fmt.parseInt(i64, raw_number, 10);
        try numbers.append(number);
    }

    std.mem.reverse(i64, numbers.items);

    return numbers.toOwnedSlice();
}

fn predict(numbers: []const i64) !i64 {
    const n = numbers.len;

    const raw_derivatives = try allocator.alloc(i64, n * n);
    @memset(raw_derivatives, 0);
    const derivatives = try allocator.alloc([]i64, n);
    for (0..n) |i| {
        derivatives[i] = raw_derivatives[i * n .. (i + 1) * n];
    }

    for (1..n) |i| {
        const number = numbers[i];
        var delta = numbers[i - 1] - number;
        var level: usize = 0;
        var j = i;
        while (j > 0) {
            derivatives[level][j - 1] = delta;
            if (j > 1) {
                delta = derivatives[level][j - 2] - derivatives[level][j - 1];
            }
            j -= 1;
            level += 1;
        }
    }

    for (0..n - 1) |i| {
        const level = n - 1 - i;
        derivatives[level - 1][i + 1] = derivatives[level - 1][i] - derivatives[level][i];
    }

    return numbers[n - 1] - derivatives[0][n - 1];
}

fn run(input: [:0]const u8) !i64 {
    var it = std.mem.splitScalar(u8, input, '\n');
    var result: i64 = 0;
    while (it.next()) |line| {
        const numbers = try parseLine(line);
        const p = try predict(numbers);
        result += p;
        //std.debug.print("Line has {} numbers\n", .{numbers.len});
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
        \\0 3 6 9 12 15
        \\1 3 6 10 15 21
        \\10 13 16 21 30 45
    ;
    const result = try run(input);
    try std.testing.expectEqual(@as(i64, 2), result);
}
