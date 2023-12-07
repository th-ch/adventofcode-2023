const std = @import("std");

var allocator: std.mem.Allocator = undefined;
const stdout = std.io.getStdOut().writer(); //prepare stdout to write in

const CardType = enum {
    @"2",
    @"3",
    @"4",
    @"5",
    @"6",
    @"7",
    @"8",
    @"9",
    t,
    j,
    q,
    k,
    a,

    pub const count = std.meta.tags(CardType).len;

    pub fn from(c: u8) CardType {
        return switch (c) {
            'A' => .a,
            'K' => .k,
            'Q' => .q,
            'J' => .j,
            'T' => .t,
            '9' => .@"9",
            '8' => .@"8",
            '7' => .@"7",
            '6' => .@"6",
            '5' => .@"5",
            '4' => .@"4",
            '3' => .@"3",
            '2' => .@"2",
            else => unreachable,
        };
    }
};

const HandType = enum {
    high_card,
    pair,
    two_pair,
    three_of_a_kind,
    full_house,
    four_of_a_kind,
    five_of_a_kind,
};

const Hand = struct {
    cards: [5]CardType,
    type: HandType,
    bid: i64,

    pub fn lessThan(a: Hand, b: Hand) bool {
        const a_type_value = @intFromEnum(a.type);
        const b_type_value = @intFromEnum(b.type);
        return if (a_type_value < b_type_value)
            true
        else if (a_type_value > b_type_value)
            false
        else b: {
            break :b for (a.cards, b.cards) |a_card, b_card| {
                const a_card_value = @intFromEnum(a_card);
                const b_card_value = @intFromEnum(b_card);
                if (a_card_value < b_card_value) break true;
                if (a_card_value > b_card_value) break false;
            } else unreachable;
        };
    }
};

fn computeCardsAndType(raw_cards: []const u8) struct { [5]CardType, HandType } {
    std.debug.assert(raw_cards.len == 5);

    var card_count = [_]u8{0} ** CardType.count;
    var current_type: HandType = .high_card;
    var cards: [5]CardType = undefined;
    for (raw_cards, 0..) |c, i| {
        const card = CardType.from(c);
        const card_index = @intFromEnum(card);
        const old_card_count = card_count[card_index];
        switch (current_type) {
            .five_of_a_kind => {},
            .four_of_a_kind => {
                if (old_card_count == 4) {
                    current_type = .five_of_a_kind;
                }
            },
            .full_house => {},
            .three_of_a_kind => {
                if (old_card_count == 1) {
                    current_type = .full_house;
                } else if (old_card_count == 3) {
                    current_type = .four_of_a_kind;
                }
            },
            .two_pair => {
                if (old_card_count == 2) {
                    current_type = .full_house;
                }
            },
            .pair => {
                if (old_card_count == 1) {
                    current_type = .two_pair;
                } else if (old_card_count == 2) {
                    current_type = .three_of_a_kind;
                }
            },
            .high_card => {
                if (old_card_count == 1) {
                    current_type = .pair;
                }
            },
        }
        card_count[card_index] += 1;
        cards[i] = card;
    }

    return .{ cards, current_type };
}

fn parseHand(line: []const u8) !Hand {
    var it = std.mem.splitScalar(u8, line, ' ');
    const cards, const hand_type = computeCardsAndType(it.next().?);
    const bid = try std.fmt.parseInt(i64, it.next().?, 10);

    return Hand{ .cards = cards, .type = hand_type, .bid = bid };
}

fn run(input: [:0]const u8) i64 {
    var hands = std.ArrayList(Hand).init(allocator);
    var line_it = std.mem.splitScalar(u8, input, '\n');
    while (line_it.next()) |line| {
        const hand = parseHand(line) catch unreachable;
        hands.append(hand) catch unreachable;
    }

    std.sort.heap(Hand, hands.items, {}, comptime (struct {
        fn lessThan(_: void, a: Hand, b: Hand) bool {
            return a.lessThan(b);
        }
    }).lessThan);

    var result: i64 = 0;
    for (hands.items, 0..) |hand, i| {
        result += @as(i64, @intCast(i + 1)) * hand.bid;
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
        \\32T3K 765
        \\T55J5 684
        \\KK677 28
        \\KTJJT 220
        \\QQQJA 483
    ;

    const result = run(input);
    try std.testing.expectEqual(@as(i64, 6440), result);
}
