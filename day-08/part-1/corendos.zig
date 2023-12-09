const std = @import("std");

var allocator: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const Node = [3]u8;

fn parseLine(line: []const u8) struct { Node, Node, Node } {
    return .{ line[0..3].*, line[7..][0..3].*, line[12..][0..3].* };
}

fn run(input: [:0]const u8) i64 {
    var it = std.mem.splitScalar(u8, input, '\n');
    const path = it.next().?;

    _ = it.next().?;

    var map = std.AutoHashMap(Node, [2]Node).init(allocator);
    while (it.next()) |line| {
        const root, const left_node, const right_node = parseLine(line);
        map.put(root, .{ left_node, right_node }) catch unreachable;
    }

    var current_node: Node = "AAA".*;
    var steps: i64 = 0;
    var index: usize = 0;
    while (!std.mem.eql(u8, current_node[0..], "ZZZ")) {
        const next_nodes = map.get(current_node).?;
        current_node = switch (path[index]) {
            'L' => next_nodes[0],
            'R' => next_nodes[1],
            else => unreachable,
        };
        steps += 1;
        index = (index + 1) % path.len;
    }

    return steps;
}

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator); // create memory allocator for strings

    defer arena.deinit(); // clear memory
    allocator = arena.allocator();

    var arg_it = try std.process.argsWithAllocator(allocator);
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
    allocator = arena.allocator();

    const input =
        \\RL
        \\
        \\AAA = (BBB, CCC)
        \\BBB = (DDD, EEE)
        \\CCC = (ZZZ, GGG)
        \\DDD = (DDD, DDD)
        \\EEE = (EEE, EEE)
        \\GGG = (GGG, GGG)
        \\ZZZ = (ZZZ, ZZZ)
    ;
    const result = run(input);
    try std.testing.expectEqual(@as(i64, 2), result);
}
test {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator); // create memory allocator for strings

    defer arena.deinit(); // clear memory
    allocator = arena.allocator();

    const input =
        \\LLR
        \\
        \\AAA = (BBB, BBB)
        \\BBB = (AAA, ZZZ)
        \\ZZZ = (ZZZ, ZZZ)
    ;
    const result = run(input);
    try std.testing.expectEqual(@as(i64, 6), result);
}
