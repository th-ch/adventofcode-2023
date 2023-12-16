const std = @import("std");

var allocator: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const OperationType = enum {
    remove,
    change,
};

const Operation = union(OperationType) {
    remove: void,
    change: u8,
};

const Label = struct {
    storage: [16]u8,
    len: usize,

    pub inline fn slice(self: Label) []const u8 {
        return self.storage[0..self.len];
    }

    pub inline fn fromSlice(s: []const u8) Label {
        var label: Label = undefined;
        @memcpy(label.storage[0..s.len], s);
        label.len = s.len;
        return label;
    }
};

const Lens = struct {
    label: Label,
    focal_length: u8,
};

const Hashmap = struct {
    slots: [256]std.ArrayList(Lens),

    pub fn init() Hashmap {
        var slots: [256]std.ArrayList(Lens) = undefined;
        for (slots[0..]) |*s| {
            s.* = std.ArrayList(Lens).init(allocator);
        }
        return Hashmap{ .slots = slots };
    }

    pub fn computeFocusingPower(self: Hashmap) i64 {
        var result: i64 = 0;
        for (self.slots, 1..) |s, box_number| {
            for (s.items, 1..) |lens, slot_number| {
                const focal_length: usize = lens.focal_length;
                result += @intCast(box_number * slot_number * focal_length);
            }
        }

        return result;
    }

    pub fn format(self: Hashmap, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        _ = fmt;
        _ = options;
        for (self.slots, 0..) |s, i| {
            if (s.items.len != 0) {
                try writer.print("Box {}:", .{i});
                for (s.items) |item| {
                    try writer.print(" [{s} {}]", .{ item.label.slice(), item.focal_length });
                }
                try writer.writeByte('\n');
            }
        }
    }
};

fn computeHash(input: []const u8) u8 {
    var state: u16 = 0;
    for (input) |c| {
        state += @as(u16, @intCast(c));
        state *= 17;
        state = state % 256;
    }

    return @intCast(state);
}

fn parseInstruction(input: []const u8) struct { label: []const u8, operation: Operation } {
    for (input, 0..) |c, i| {
        if (c == '-') {
            return .{ .label = input[0..i], .operation = .{ .remove = {} } };
        } else if (c == '=') {
            const focal_length = std.fmt.parseInt(u8, input[i + 1 ..], 10) catch unreachable;
            return .{ .label = input[0..i], .operation = .{ .change = focal_length } };
        }
    }
    unreachable;
}

fn run(input: [:0]const u8) !i64 {
    var hashmap = Hashmap.init();
    var it = std.mem.splitScalar(u8, input, ',');
    while (it.next()) |raw_instruction| {
        const parse_result = parseInstruction(raw_instruction);
        const hash = computeHash(parse_result.label);
        const slot = &hashmap.slots[@intCast(hash)];

        const operation = parse_result.operation;

        switch (operation) {
            .remove => {
                const maybe_index = for (slot.items, 0..) |item, i| {
                    if (std.mem.eql(u8, item.label.slice(), parse_result.label)) break i;
                } else null;

                if (maybe_index) |index| {
                    _ = slot.orderedRemove(index);
                }
            },
            .change => |focal_length| {
                const maybe_index = for (slot.items, 0..) |item, i| {
                    if (std.mem.eql(u8, item.label.slice(), parse_result.label)) break i;
                } else null;

                if (maybe_index) |index| {
                    slot.items[index].focal_length = focal_length;
                } else {
                    try slot.append(Lens{ .label = Label.fromSlice(parse_result.label), .focal_length = focal_length });
                }
            },
        }
    }

    //std.debug.print("\n{}\n", .{hashmap});

    const focusing_power = hashmap.computeFocusingPower();
    return focusing_power;
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
        \\rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
    ;
    const result = try run(input);
    try std.testing.expectEqual(@as(i64, 145), result);
}
