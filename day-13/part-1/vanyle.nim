from times import cpuTime
from os import paramStr

import strutils, sugar, sequtils

proc parseInput(s: string): seq[seq[string]] = 
    return s.strip.split("\n\n").map(x => x.split("\n"))

proc isHorizontalSym(pattern: seq[string], l: int): int =
    var errCount = 0
    for i in (l+1)..<min(pattern.len,2*l+2):
        var match = 2 * l + 1 - i
        # i: l+1 --> pattern.len : OK
        # match: 0 --> l: OK (and they are disjointed!)

        for j in 0..<pattern[0].len:
            if pattern[i][j] != pattern[match][j]:
                inc errCount
                if errCount >= 2:
                    return errCount
    return errCount

proc isVerticalSym(pattern: seq[string], l: int): int =
    var errCount = 0
    for i in (l+1)..<min(pattern[0].len,2*l+2):
        var match = 2 * l + 1 - i

        for j in 0..<pattern.len:
            if pattern[j][i] != pattern[j][match]:
                inc errCount
                if errCount >= 2:
                    return errCount
    return errCount

proc findSymetry(pattern: seq[string]): (bool, int) =
    for l in 0..<(pattern.len-1):
        if pattern.isHorizontalSym(l) == 0:
            return (false, l)

    for l in 0..<(pattern[0].len-1):
        if pattern.isVerticalSym(l) == 0:
            return (true, l)

    return (false, -1)

proc run(s: string): string =
    var r = parseInput(s)
    var res = 0

    for pattern in r:
        var (symDir, symPos) = findSymetry(pattern)
        if symDir: # vertical
            res += (symPos+1)
        else:
            res += 100 * (symPos+1)

    return $res


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
