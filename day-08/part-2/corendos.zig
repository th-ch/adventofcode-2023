const std = @import("std");

var allocator: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const Node = [3]u8;

fn parseLine(line: []const u8) struct { Node, Node, Node } {
    return .{ line[0..3].*, line[7..][0..3].*, line[12..][0..3].* };
}

fn lcm(a: u64, b: u64) u64 {
    const temp = @divExact(a, std.math.gcd(a, b));
    return temp * b;
}

fn runForNode(start: Node, path: []const u8, map: *const std.AutoHashMap(Node, [2]Node)) i64 {
    var current_node: Node = start;

    var steps: i64 = 0;
    var index: usize = 0;
    while (current_node[2] != 'Z') {
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

fn run(input: [:0]const u8) i64 {
    var it = std.mem.splitScalar(u8, input, '\n');
    const path = it.next().?;

    _ = it.next().?;

    var map = std.AutoHashMap(Node, [2]Node).init(allocator);
    var starting_nodes = std.ArrayList(Node).init(allocator);
    while (it.next()) |line| {
        const root, const left_node, const right_node = parseLine(line);
        map.put(root, .{ left_node, right_node }) catch unreachable;
        if (root[2] == 'A') {
            starting_nodes.append(root) catch unreachable;
        }
    }

    var result: i64 = 1;

    for (starting_nodes.items) |start| {
        const steps = runForNode(start, path, &map);
        result = @intCast(lcm(@intCast(result), @intCast(steps)));
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
        \\LR
        \\
        \\11A = (11B, XXX)
        \\11B = (XXX, 11Z)
        \\11Z = (11B, XXX)
        \\22A = (22B, XXX)
        \\22B = (22C, 22C)
        \\22C = (22Z, 22Z)
        \\22Z = (22B, 22B)
        \\XXX = (XXX, XXX)
    ;
    const result = run(input);
    try std.testing.expectEqual(@as(i64, 6), result);
}
