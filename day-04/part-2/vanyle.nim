from times import cpuTime
from os import paramStr

import sequtils, strutils

proc fparseInt(s: string, i: int): int =
    var j = i
    while j < s.len and not s[j].isDigit:
        inc j
    while j < s.len and s[j].isDigit:
        result *= 10
        result += cast[int](s[j]) - cast[int]('0')
        inc j

proc run(s: string): string =
    var wTable: seq[int] = newSeq[int](s.len div 117 + 1)
    var cardsOwned: seq[int] = newSeq[int](wTable.len)
    
    var i = 0
    var idx = 0
    while i < s.len:
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

        wTable[idx] = matches # card i makes you win 'matches' cards
        i += 117
        inc idx

    for i in 0..<cardsOwned.len:
        cardsOwned[i] = 1

    for i in 0..<cardsOwned.len:
        var w = wTable[i]
        for j in (i+1) ..< (i + 1 + w):
            cardsOwned[j] += cardsOwned[i]   

    return $cardsOwned.foldl(a + b)


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
