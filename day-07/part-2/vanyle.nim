from times import cpuTime
from os import paramStr

import ../../lib/nim/vanyle/speed_utils
import algorithm

proc fmap(c: char): int =
    ## Map card characters to numbers between 0 and 12, order does not matter.
    ## The joker is number 12, the highest.
    if c == 'T': return 10 - 2
    if c == 'Q': return 11 - 2
    if c == 'K': return 12 - 2
    if c == 'A': return 13 - 2
    if c == 'J': return 14 - 2
    return toDigit(c) - 2

proc parseInput(s: string): seq[(array[5, char], int16)] = 
    result = newSeqOfCap[(array[5, char], int16)](1000)
    var t = Tokenizer(s:s,offset:0)

    while t.offset < s.len:
        var a: array[5, char]
        for i in 0..<5:
            a[i] = t.s[t.offset]
            t.advanceFixed(1)
        
        t.advanceFixed(1)
        var i = t.eatUnsignedInt().int16
        result.add((a, i))

        t.advanceFixed(1)

const order2 = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J']

const rorder2: array[256, int8] = static:
    var res: array[256, int8]
    for i in 0..<order2.len:
        res[cast[int](order2[i])] = cast[int8](i)
    res

proc isHighestCardStronger2(a, b: array[5, char]): bool =
    for i in 0..<a.len:
        var vA = rorder2[cast[int](a[i])]
        var vB = rorder2[cast[int](b[i])]
        if vA < vB:
            return true
        elif vB < vA:
            return false

proc profileKind2(a: array[5, char]): int8 =
    # Same as profileKind, but takes jokers into account properly.
    var ct: array[13, int8]
    for i in 0..<a.len:
        inc ct[fmap(a[i])]

    var jcount = ct[fmap('J')]

    var values: set[0..5]
    for i in 0..<(ct.len-1):
        if ct[i] != 0:
            values.incl (ct[i] + jcount)

    if 5 in values: return 6
    if 4 in values: return 5
    if 3 in values:
        if values.len == 2:
            return 4
        else:
            return 3
    if values.len == 4:
        return 2
    if 2 in values:
        return 1
    return 0

proc run(s: string): string =
    var r = parseInput(s)

    proc myCmp(a,b: (array[5, char], int16)): int =
        let pa = profileKind2(a[0])
        let pb = profileKind2(b[0])

        if pa == pb:
            if isHighestCardStronger2(a[0] ,b[0]):
                return 1
            else:
                return -1
        else:
            if pa > pb:
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
