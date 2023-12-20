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
    value: usize,
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
            const value = try std.fmt.parseInt(usize, raw_rule[2..colon_index], 10);
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

fn parsePart(raw_part: []const u8) !Part {
    var it = std.mem.splitScalar(u8, raw_part[1 .. raw_part.len - 1], ',');
    const x = try std.fmt.parseInt(usize, it.next().?[2..], 10);
    const m = try std.fmt.parseInt(usize, it.next().?[2..], 10);
    const a = try std.fmt.parseInt(usize, it.next().?[2..], 10);
    const s = try std.fmt.parseInt(usize, it.next().?[2..], 10);
    return Part{ .x = x, .m = m, .a = a, .s = s };
}

fn run(input: [:0]const u8) !i64 {
    var it = std.mem.splitSequence(u8, input, "\n\n");
    const raw_workflows = it.next().?;
    const workflow_map = try parseWorkflows(raw_workflows);

    var accepted_parts = std.ArrayList(Part).init(allocator);

    const raw_parts = it.next().?;
    var raw_part_it = std.mem.splitScalar(u8, raw_parts, '\n');
    while (raw_part_it.next()) |raw_part| {
        const part = try parsePart(raw_part);
        var current_workflow_name: []const u8 = "in";

        //std.debug.print("\n{}:", .{part});
        while (true) {
            //std.debug.print(" {s}", .{current_workflow_name});
            if (std.mem.eql(u8, current_workflow_name, "A")) {
                try accepted_parts.append(part);
                break;
            } else if (std.mem.eql(u8, current_workflow_name, "R")) {
                break;
            }
            //std.debug.print(" ->", .{});

            const current_workflow = workflow_map.getPtr(current_workflow_name).?;

            for (current_workflow.rules) |rule| {
                switch (rule) {
                    .comparison => |comparison| {
                        if (std.math.compare(part.getCategoryValue(comparison.category), comparison.order, comparison.value)) {
                            current_workflow_name = comparison.target;
                            break;
                        }
                    },
                    .unconditional => |unconditional| {
                        current_workflow_name = unconditional.target;
                        break;
                    },
                }
            } else unreachable;
        }
    }
    //std.debug.print("\n", .{});

    var result: i64 = 0;
    for (accepted_parts.items) |p| {
        result += @intCast(p.x + p.m + p.a + p.s);
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
    try std.testing.expectEqual(@as(i64, 19114), result);
}
