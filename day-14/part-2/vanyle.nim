from times import cpuTime
from os import paramStr

import strutils, tables

proc parseInput(s: string): seq[string] = 
    return s.strip.split("\n")

proc tiltNorth(s: var seq[string]) =
    for i in 1..<s.len:
        for j in 0..<s[i].len:
            var pi = i
            if s[pi][j] == 'O':
                while pi-1 >= 0 and s[pi-1][j] == '.':
                    s[pi-1][j] = 'O'
                    s[pi][j] = '.'
                    dec pi

proc tiltWest(s: var seq[string]) =
    for i in 0..<s.len:
        for j in 1..<s[i].len:
            var pj = j
            if s[i][j] == 'O':
                while pj-1 >= 0 and s[i][pj-1] == '.':
                    s[i][pj-1] = 'O'
                    s[i][pj] = '.'
                    dec pj

proc tiltSouth(s: var seq[string]) =
    for i in countdown(s.len-2, 0):
        for j in 0..<s[i].len:
            var pi = i
            if s[i][j] == 'O':
                while pi+1 < s.len and s[pi+1][j] == '.':
                    s[pi+1][j] = 'O'
                    s[pi][j] = '.'
                    inc pi

proc tiltEast(s: var seq[string]) =
    let wi = s[0].len
    for i in 0..<s.len:
        for j in countdown(s[0].len-2, 0):
            var pj = j
            if s[i][j] == 'O':
                while pj < wi and s[i][pj+1] == '.':
                    s[i][pj+1] = 'O'
                    s[i][pj] = '.'
                    inc pj

var cycleFinder: Table[seq[string], int]
var callCounter = 0
proc cycle(s: var seq[string]): int =
    if s in cycleFinder:
        result = callCounter - cycleFinder[s]
    else:
        result = -1
    cycleFinder[s] = callCounter

    s.tiltNorth()
    s.tiltWest()
    s.tiltSouth()
    s.tiltEast()
    
    inc callCounter

proc run(s: string): string =
    var r = parseInput(s)
    var cycleLength = 0

    for _ in 0..<1000000000:
        var c = cycle(r)
        if c != -1:
            cycleLength = c
            break

    var remaining = 1000000000 - callCounter
    # how many transformation are left to apply?
    var toApply = remaining mod cycleLength

    for _ in 0..<toApply:
        discard cycle(r)

    var res = 0
    for i in 0..<r.len:
        for j in 0..<r[i].len:
            if r[i][j] == 'O':
                res += r.len - i

    return $res


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
