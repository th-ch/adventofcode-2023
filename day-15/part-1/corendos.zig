const std = @import("std");

var allocator: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

fn computeHash(input: []const u8) i64 {
    var state: u16 = 0;
    for (input) |c| {
        state += @as(u16, @intCast(c));
        state *= 17;
        state = state % 256;
    }

    return @intCast(state);
}

fn run(input: [:0]const u8) !i64 {
    var result: i64 = 0;
    var it = std.mem.splitScalar(u8, input, ',');
    while (it.next()) |to_hash| {
        result += computeHash(to_hash);
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
    const input =
        \\rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
    ;
    const result = try run(input);
    try std.testing.expectEqual(@as(i64, 1320), result);
}
