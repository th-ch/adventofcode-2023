from times import cpuTime
from os import paramStr

import tables, ../../lib/nim/vanyle/speed_utils

proc parseUnsignedInt*(s: openarray[char]): int =
    var i = 0
    while i < s.len and isDigit(s[i]):
        result *= 10
        result += toDigit(s[i])
        inc i

proc hashS(s: openarray[char]): int =
    var h = 0
    for j in 0..<s.len:
        var ascii = cast[int](s[j])
        h += ascii
        h = h * 17
        h = h mod 256
    return h

proc run(s: string): string =
    var boxes: array[256, OrderedTable[string, int]]

    for ll in s.fastSplit(','):
        var i = 0
        while ll[i] != '-' and ll[i] != '=':
            inc i

        var label = ll.toOpenArray(0, i-1).toString()
        var boxId = hashS(label)

        if ll[i] == '=':
            var focLength = parseUnsignedInt(ll.toOpenArray(i+1, ll.len-1))
            boxes[boxId][label] = focLength
        elif ll[i] == '-':
            boxes[boxId].del(label)

    var res = 0
    for b in 0..<256:
        var i = 1
        for k,v in boxes[b]:
            var power = (b+1) * i * v
            res += power
            inc i

    return $res


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
