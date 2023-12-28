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

proc toggle(c: char): char =
    if c == '.':
        return '#'
    else:
        return '.'

proc run(s: string): string =
    var r = parseInput(s)

    var res = 0

    for idx in 0..<r.len:
        # var pattern = r[idx]
        # By changing 1 position in pattern,
        # symData can change ...

        var symetryCandidates: seq[(bool,int)] = @[]
        for l in 0..<(r[idx].len-1):
            if r[idx].isHorizontalSym(l) == 1:
                symetryCandidates.add (false, l)

        for l in 0..<(r[idx][0].len-1):
            if r[idx].isVerticalSym(l) == 1:
                symetryCandidates.add (true, l)

        var newSymData = (false, -1)

        block inner:
            for i in 0..<r[idx].len:
                for j in 0..<r[idx][0].len:
                    r[idx][i][j] = toggle(r[idx][i][j])

                    for (scType, scPos) in symetryCandidates:
                        if scType and r[idx].isVerticalSym(scPos) == 0:
                            newSymData = (scType, scPos)
                            break inner
                        elif not scType and r[idx].isHorizontalSym(scPos) == 0:
                            newSymData = (scType, scPos)
                            break inner

                    r[idx][i][j] = toggle(r[idx][i][j])  

        var (symDir, symPos) = newSymData

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
