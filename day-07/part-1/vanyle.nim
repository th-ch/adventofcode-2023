from times import cpuTime
from os import paramStr

import ../../lib/nim/vanyle/speed_utils
import algorithm

const order = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2']

const rorder: array[256, int8] = static:
    var res: array[256, int8]
    for i in 0..<order.len:
        res[cast[int](order[i])] = cast[int8](i+1)
    res

proc fmap(c: char): int =
    ## Map card characters to numbers between 0 and 12, order does not matter.
    ## The joker is number 12, the highest.
    if c == 'T': return 10 - 2
    if c == 'Q': return 11 - 2
    if c == 'K': return 12 - 2
    if c == 'A': return 13 - 2
    if c == 'J': return 14 - 2
    return toDigit(c) - 2

proc profileKind(a: array[5, char]): int8 =
    # Return the win profile of the hand:
    # 5 => 6 , 4 => 5
    # 3,2 => 4, 3 => 3
    # 2,2 => 2 (2 pairs), 2 => 1 (1 pair), 1 => 0 (high card)

    var ct: array[13, int8]
    for i in 0..<a.len:
        inc ct[fmap(a[i])]

    var isThree = false
    var isTwo = false
    for i in 0..<ct.len:
        if ct[i] == 5:
            return 6
        elif ct[i] == 4:
            return 5
        elif ct[i] == 3:
            if isTwo: return 4 # 3,2
            isThree = true 
        elif ct[i] == 2:
            if isThree: return 4 # 3,2
            if isTwo: return 2 # two pairs
            isTwo = true 

    if isThree: return 3
    if isTwo: return 1
    return 0

proc isHighestCardStronger1(a, b: array[5, char]): int =
    for i in 0..<a.len:
        var vA = rorder[cast[int](a[i])]
        var vB = rorder[cast[int](b[i])]
        if vA < vB:
            return 1
        elif vB < vA:
            return -1

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

proc run(s: string): string =
    var r = parseInput(s)

    proc myCmp(a,b: (array[5, char], int16)): int =
        let pa = profileKind(a[0])
        let pb = profileKind(b[0])

        if pa == pb:
            return isHighestCardStronger1(a[0] ,b[0])
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
