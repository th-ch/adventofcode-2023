const std = @import("std");

var allocator: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const Category = enum {
    x,
    m,
    a,
    s,

    pub inline fn fromChar(c: u8) Category {
        return switch (c) {
            'x' => .x,
            'm' => .m,
            'a' => .a,
            's' => .s,
            else => unreachable,
        };
    }
};

const ComparisonRule = struct {
    category: Category,
    order: std.math.CompareOperator,
    value: i64,
    target: []const u8,
};

const UnconditionalRule = struct {
    target: []const u8,
};

const RuleType = enum {
    comparison,
    unconditional,
};

const Rule = union(RuleType) {
    comparison: ComparisonRule,
    unconditional: UnconditionalRule,
};

const Workflow = struct {
    name: []const u8,
    rules: []Rule,
};

const Part = struct {
    x: usize,
    m: usize,
    a: usize,
    s: usize,

    pub inline fn getCategoryValue(self: Part, category: Category) usize {
        return switch (category) {
            .x => self.x,
            .m => self.m,
            .a => self.a,
            .s => self.s,
        };
    }
};

fn orderFromChar(c: u8) std.math.CompareOperator {
    return switch (c) {
        '>' => .gt,
        '<' => .lt,
        else => unreachable,
    };
}

fn parseRules(raw_rules: []const u8) ![]Rule {
    var rule_list = std.ArrayList(Rule).init(allocator);
    var it = std.mem.splitScalar(u8, raw_rules, ',');
    while (it.next()) |raw_rule| {
        const maybe_colon_index = std.mem.indexOfScalar(u8, raw_rule, ':');
        const rule = if (maybe_colon_index) |colon_index| b: {
            const category = Category.fromChar(raw_rule[0]);
            const order = orderFromChar(raw_rule[1]);
            const value = try std.fmt.parseInt(i64, raw_rule[2..colon_index], 10);
            const target = raw_rule[colon_index + 1 ..];
            break :b Rule{ .comparison = .{ .category = category, .order = order, .value = value, .target = target } };
        } else Rule{ .unconditional = .{ .target = raw_rule } };
        try rule_list.append(rule);
    }

    return rule_list.toOwnedSlice();
}

fn parseWorkflow(raw_workflow: []const u8) !Workflow {
    const bracket_index = std.mem.indexOfScalar(u8, raw_workflow, '{').?;
    const workflow_name = raw_workflow[0..bracket_index];
    const raw_rules = raw_workflow[bracket_index + 1 .. raw_workflow.len - 1];
    const rules = try parseRules(raw_rules);

    return Workflow{ .name = workflow_name, .rules = rules };
}

fn parseWorkflows(raw_workflows: []const u8) !std.StringHashMap(Workflow) {
    var map = std.StringHashMap(Workflow).init(allocator);
    var it = std.mem.splitScalar(u8, raw_workflows, '\n');
    while (it.next()) |raw_workflow| {
        const workflow = try parseWorkflow(raw_workflow);
        try map.put(workflow.name, workflow);
    }

    return map;
}

const PartRange = struct {
    min_x: i64 = 1,
    max_x: i64 = 4000,
    min_m: i64 = 1,
    max_m: i64 = 4000,
    min_a: i64 = 1,
    max_a: i64 = 4000,
    min_s: i64 = 1,
    max_s: i64 = 4000,

    pub inline fn isValid(self: PartRange) bool {
        return self.min_x <= self.max_x and
            self.min_m <= self.max_m and
            self.min_a <= self.max_a and
            self.min_s <= self.max_s;
    }

    pub inline fn combination(self: PartRange) i64 {
        std.debug.assert(self.isValid());
        return (self.max_x - self.min_x + 1) * (self.max_m - self.min_m + 1) * (self.max_a - self.min_a + 1) * (self.max_s - self.min_s + 1);
    }

    pub fn applyCondition(self: PartRange, category: Category, order: std.math.CompareOperator, value: i64) ?PartRange {
        var new_part_range = self;
        switch (category) {
            .x => {
                const new_min, const new_max = if (order == .lt) .{ self.min_x, @min(self.max_x, value - 1) } else if (order == .gt) .{ @max(self.min_x, value + 1), self.max_x } else unreachable;
                new_part_range.min_x = new_min;
                new_part_range.max_x = new_max;
            },
            .m => {
                const new_min, const new_max = if (order == .lt) .{ self.min_m, @min(self.max_m, value - 1) } else if (order == .gt) .{ @max(self.min_m, value + 1), self.max_m } else unreachable;
                new_part_range.min_m = new_min;
                new_part_range.max_m = new_max;
            },
            .a => {
                const new_min, const new_max = if (order == .lt) .{ self.min_a, @min(self.max_a, value - 1) } else if (order == .gt) .{ @max(self.min_a, value + 1), self.max_a } else unreachable;
                new_part_range.min_a = new_min;
                new_part_range.max_a = new_max;
            },
            .s => {
                const new_min, const new_max = if (order == .lt) .{ self.min_s, @min(self.max_s, value - 1) } else if (order == .gt) .{ @max(self.min_s, value + 1), self.max_s } else unreachable;
                new_part_range.min_s = new_min;
                new_part_range.max_s = new_max;
            },
        }

        return if (new_part_range.isValid()) new_part_range else null;
    }

    pub fn applyReverseCondition(self: PartRange, category: Category, order: std.math.CompareOperator, value: i64) ?PartRange {
        const reverse_order: std.math.CompareOperator = if (order == .lt) .gt else if (order == .gt) .lt else unreachable;
        const reverse_value: i64 = if (order == .lt) value - 1 else if (order == .gt) value + 1 else unreachable;
        return self.applyCondition(category, reverse_order, reverse_value);
    }

    pub fn format(self: PartRange, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        _ = fmt;
        _ = options;
        try writer.print("{s}{{ .x = [{};{}], .m = [{};{}], .a = [{};{}], .s = [{};{}] }}", .{
            @typeName(PartRange),
            self.min_x,
            self.max_x,
            self.min_m,
            self.max_m,
            self.min_a,
            self.max_a,
            self.min_s,
            self.max_s,
        });
    }
};

fn count(part_range: PartRange, workflow_name: []const u8, map: std.StringHashMap(Workflow)) i64 {
    //std.debug.print("{}\n", .{part_range});
    if (std.mem.eql(u8, workflow_name, "A")) {
        return part_range.combination();
    } else if (std.mem.eql(u8, workflow_name, "R")) {
        return 0;
    }

    const workflow = map.get(workflow_name).?;

    var result: i64 = 0;
    var current_part_range = part_range;

    for (workflow.rules) |rule| {
        switch (rule) {
            .unconditional => |u| {
                result += count(current_part_range, u.target, map);
            },
            .comparison => |c| {
                // If applying the condition of this rule result in a still valid part range, we recurse.
                if (current_part_range.applyCondition(c.category, c.order, c.value)) |next_part_range| {
                    result += count(next_part_range, c.target, map);
                }

                // We apply the reverse condition for the parts that weren't selected by the rule. If it results in an invalid
                // part range, we can early stop the loop.
                current_part_range = current_part_range.applyReverseCondition(c.category, c.order, c.value) orelse break;
            },
        }
    }

    return result;
}

fn run(input: [:0]const u8) !i64 {
    var it = std.mem.splitSequence(u8, input, "\n\n");
    const raw_workflows = it.next().?;
    const workflow_map = try parseWorkflows(raw_workflows);

    std.debug.print("\n", .{});

    const initial_part_range = PartRange{};
    const result = count(initial_part_range, "in", workflow_map);

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
        \\px{a<2006:qkq,m>2090:A,rfg}
        \\pv{a>1716:R,A}
        \\lnx{m>1548:A,A}
        \\rfg{s<537:gd,x>2440:R,A}
        \\qs{s>3448:A,lnx}
        \\qkq{x<1416:A,crn}
        \\crn{x>2662:A,R}
        \\in{s<1351:px,qqz}
        \\qqz{s>2770:qs,m<1801:hdj,R}
        \\gd{a>3333:R,R}
        \\hdj{m>838:A,pv}
        \\
        \\{x=787,m=2655,a=1222,s=2876}
        \\{x=1679,m=44,a=2067,s=496}
        \\{x=2036,m=264,a=79,s=2244}
        \\{x=2461,m=1339,a=466,s=291}
        \\{x=2127,m=1623,a=2188,s=1013}
    ;
    const result = try run(input);
    try std.testing.expectEqual(@as(i64, 167409079868000), result);
}
