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
var memoTable: Table[(string, int16, uint8, uint8), int]
proc possibilities(pattern: string, pslice: int16, hintSlice: uint8, counterSeq: uint8): int =
    # Recursion, my boy!!
    if (pattern, pslice, hintSlice, counterSeq) in memoTable:
        return memoTable[(pattern, pslice, hintSlice, counterSeq)]

    if pslice == -1:  
        if hintSlice > 1:
            return 0
        if hintSlice == 0 and counterSeq == 0:
            return 1
        if hintSlice == 1 and counterSeq == hints[0]:
            return 1
        return 0

    var brokenCounter = counterSeq

    for j in countdown(pslice, 0):
        if pattern[j] == '#':
            inc brokenCounter
        elif pattern[j] == '.':
            if brokenCounter == 0: continue
            if hintSlice == 0: return 0
            if hints[hintSlice-1] == brokenCounter: # ok!
                var r = possibilities(pattern, j-1, hintSlice - 1, 0)
                memoTable[(pattern, pslice, hintSlice, counterSeq)] = r
                return r
            else:
                return 0 # impossible!
        elif pattern[j] == '?':
            var p1 = pattern[0..<j] & '.'
            var c1 = possibilities(p1, j, hintSlice, brokenCounter)
            
            p1[j] = '#'
            var c2 = possibilities(p1, j, hintSlice, brokenCounter)
            
            memoTable[(pattern, pslice, hintSlice, counterSeq)] = c1 + c2
            return c1 + c2

    # make sure the counter and the hint match here.
    return possibilities(pattern, -1, hintSlice, brokenCounter)

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
        var p = possibilities(j[0], (j[0].len-1).int16, j[1].len.uint8, 0)
        res += p

    return $res


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
