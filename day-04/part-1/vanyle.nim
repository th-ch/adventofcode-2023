from times import cpuTime
from os import paramStr

import strutils

proc fparseInt(s: string, i: int): int =
    var j = i
    while j < s.len and not s[j].isDigit:
        inc j
    while j < s.len and s[j].isDigit:
        result *= 10
        result += cast[int](s[j]) - cast[int]('0')
        inc j

proc run(s: string): string =
    var points = 0
    var i = 0

    while i < s.len:
        var l = s[i+9..i+115]

        var sh: set[uint8]
        for j in 0..<10:
            let idx = 10 + 3 * j + i
            sh.incl(fparseInt(s, idx).uint8)

        var matches = 0

        for j in 0..<25:
            let idx = 42 + 3 * j + i
            let num = fparseInt(s, idx).uint8
            if num in sh:
                inc matches

        if matches > 0:
            var toAdd = 1
            for i in 1..<matches:
                toAdd *= 2
            points += toAdd

        i += 117

    return $points


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output