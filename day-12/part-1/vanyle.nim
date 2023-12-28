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


type LinkedString = object
    c: char
    h: Hash
    n: int32 # index to next element.

var memory: seq[LinkedString]

proc addAtEnd(ll: int32, c: char): int32 =
    var lh = hashes.hash(0)
    if ll != -1:
        lh = memory[ll].h

    var ls = LinkedString(
        c: c,
        n: ll,
        h: lh !& hash(c)
    )
    memory.add(ls)
    return memory.len.int32 - 1

proc toLinkedString(s: string): int32 =
    var current = -1.int32
    for i in s:
        current = current.addAtEnd(i)
    return current

proc hash(ll: int): Hash =
    if ll == -1: return hash(0)
    return memory[ll].h

var hints: seq[uint8]
var memoTable: Table[(int32, uint8, uint8), int]
proc possibilities(pattern: int32, hintSlice: uint8, counterSeq: uint8): int =
    # Recursion, my boy!!
    if (pattern, hintSlice, counterSeq) in memoTable:
        return memoTable[(pattern, hintSlice, counterSeq)]

    if pattern == -1:  
        if hintSlice > 1:
            return 0
        if hintSlice == 0 and counterSeq == 0:
            return 1
        if hintSlice == 1 and counterSeq == hints[0]:
            return 1
        return 0

    var brokenCounter = counterSeq
    var pp = pattern

    while pp != -1:
        var el = memory[pp]
        if el.c == '#':
            inc brokenCounter
        elif el.c == '.':
            if brokenCounter == 0:
                pp = el.n
                continue
            if hintSlice == 0:
                return 0
            if hints[hintSlice-1] == brokenCounter: # ok!
                var r = possibilities(el.n, hintSlice - 1, 0)
                memoTable[(pattern, hintSlice, counterSeq)] = r
                return r
            else:
                return 0 # impossible!
        elif el.c == '?':
            var p1 = el.n.addAtEnd('.')
            var c1 = possibilities(p1, hintSlice, brokenCounter)
            
            var p2 = el.n.addAtEnd('#')
            var c2 = possibilities(p2, hintSlice, brokenCounter)
            
            memoTable[(pattern, hintSlice, counterSeq)] = c1 + c2
            return c1 + c2
        pp = el.n

    # make sure the counter and the hint match here.
    return possibilities(-1, hintSlice, brokenCounter)

proc run(s: string): string =
    var r = parseInput(s)
    var res = 0
    memory = newSeqOfCap[LinkedString](3000)
    
    for i in r:
        hints = i[1]
        memoTable.clear()
        memory.setLen(0)
        var p = possibilities(i[0].toLinkedString(), i[1].len.uint8, 0)
        res += p
    return $res


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
