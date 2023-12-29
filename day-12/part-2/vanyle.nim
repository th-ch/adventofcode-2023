from times import cpuTime
from os import paramStr

import tables, strutils, sequtils, sugar, hashes

func parseInput(s: string): seq[(string, seq[uint8])] = 
    let lines = s.strip.splitLines
    for i in lines:
        var s = i.split(" ",2)

        result.add((
            s[0],
            s[1].split(",").map(x => x.parseInt().uint8)
        ))


var hints: seq[uint8]
var memoTable: Table[(int16, uint8), int]
proc possibilities(pattern: string, pslice: int16, hintSlice: uint8): int =
    if hintSlice == 0:
        for c in pattern.toOpenArray(0, pslice):
            if c == '#': return 0
        return 1
    
    if pslice == -1:
        return 0
    
    if (pslice, hintSlice) in memoTable:
        return memoTable[(pslice, hintSlice)]

    let hint = hints[hintSlice-1].int
    var res = 0

    for i in countdown(pslice.int, hint - 1):
        var isMatch = true
        for i in (i - hint + 1)..<(i + 1):
            isMatch = isMatch and not (pattern[i] == '.')
        isMatch = isMatch and not (i - hint + 1 > 0 and pattern[i - hint] == '#')
        isMatch = isMatch and not (i + 1 < pattern.len and pattern[i + 1] == '#')

        if isMatch:
            res += possibilities(pattern, (i - hint - 1).int16, hintSlice - 1)

        if pattern[i] == '#':
            break

    memoTable[(pslice, hintSlice)] = res
    return res

proc duplicate(pattern: string, hints: seq[uint8]): (string, seq[uint8]) =
    var patfive = ""
    var hintfive: seq[uint8] = @[]
    for i in 0..<5:
        patfive &= pattern
        if i != 4:
            patfive.add "?"
        hintfive = hintfive.concat(hints)

    return (patfive, hintfive)

proc run(s: string): string =
    var r = parseInput(s)
    var res = 0

    for i in r:
        var j = duplicate(i[0], i[1])
        hints = j[1]
        memoTable.clear()
        var p = possibilities(j[0], (j[0].len-1).int16, j[1].len.uint8)
        res += p

    return $res


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
