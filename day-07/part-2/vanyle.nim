from times import cpuTime
from os import paramStr

import ../../lib/nim/vanyle/speed_utils
import algorithm

proc parseInput(s: string): seq[(array[5, char], int16, int16)] = 
    result = newSeqOfCap[(array[5, char], int16, int16)](1000)
    var t = Tokenizer(s:s,offset:0)
    var lineIdx = 0.int16

    while t.offset < s.len:
        var a: array[5, char]
        for i in 0..<5:
            a[i] = t.s[t.offset]
            t.advanceFixed(1)
        
        t.advanceFixed(1)
        var i = t.eatUnsignedInt().int16
        result.add((a, i, lineIdx))
        inc lineIdx

        t.advanceFixed(1)

const order2 = ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A']
const rorder2: array[256, int8] = static:
    var res: array[256, int8]
    for i in 0..<order2.len:
        res[cast[int](order2[i])] = cast[int8](i)
    res

type HandTypes = enum
    HighCard = 0,
    APair = 1,
    TwoPairs = 2,
    ThreeOfKind = 3,
    FullHouse = 4,
    FourOfKind = 5,
    FiveOfKind = 6

proc fmap(c: char): int =
    # Map card characters to numbers between 0 and 12, order does not matter.
    # The joker is number 12, the highest.
    if c == 'T': return 10 - 2
    if c == 'Q': return 11 - 2
    if c == 'K': return 12 - 2
    if c == 'A': return 13 - 2
    if c == 'J': return 14 - 2
    return toDigit(c) - 2

proc highCardValue2(a: array[5, char]): int =
    # Represent the value of the highest card as an int for fast cmp.
    # Return a value between 0 and 13^5 - 1
    for i in 0..<a.len:
        result *= order2.len
        result += rorder2[cast[int](a[i])]
    # max value: 13^5 = 371293

proc profileKind2(a: array[5, char]): HandTypes =
    # Same as profileKind, but takes jokers into account properly.
    var ct: array[13, int8]
    for i in 0..<a.len:
        inc ct[fmap(a[i])]

    var jcount = ct[fmap('J')]

    var values: set[0..5]
    var twoCount = 0
    for i in 0..<(ct.len-1):
        if ct[i] != 0:
            values.incl (ct[i] + jcount)
            if ct[i] == 2:
                inc twoCount

    values.incl jcount

    if 5 in values: return FiveOfKind
    if 4 in values: return FourOfKind
    if 3 in values:
        if twoCount > 1 or (2 in values and jcount == 0):
            return FullHouse
        else:
            return ThreeOfKind
    if jcount == 0 and twoCount == 2: # J > 0 => 2 pairs are impossible.
        return TwoPairs
    if 2 in values:
        return APair
    return HighCard

proc run(s: string): string =
    var r = parseInput(s)

    var valueCache = newSeqOfCap[int32](r.len)
    for i in r:
        let pa = profileKind2(i[0]).int32 * 371293 # 13 ^ 5  
        let vh = highCardValue2(i[0]).int32
        valueCache.add(pa + vh)

    proc myCmp(a,b: (array[5, char], int16, int16)): int =
        if valueCache[a[2]] > valueCache[b[2]]:
            return 1
        else:
            return -1

    r.sort(myCmp)

    var score = 0
    for i in 0..<r.len:
        score += (i+1) * r[i][1]

    return $score


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
