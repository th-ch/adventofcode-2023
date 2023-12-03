from times import cpuTime
from os import paramStr

import strutils, tables

var maxCubes = {
    "red": 12,
    "green": 13,
    "blue": 14
}.toTable

proc parseInput(s: string): seq[seq[Table[string, int]]] = 
    var l = s.strip().split("\n")

    for line in l:
        var game = line.split(":")[1]
        var draws = game.strip.split(";")
        var cgame: seq[Table[string,int]]
        for d in draws:
            var cc = d.strip.split(",")
            var tt: Table[string, int]
            for el in cc:
                var pair = el.strip.split(" ",2)
                tt[pair[1]] = parseInt(pair[0])
            cgame.add(tt)
        result.add cgame


proc run(s: string): string =
    var r = parseInput(s)

    var validIds = 0
    for i in 0..<r.len:
        var game = r[i]
        var isGameValid = true
        for draws in game:
            var isDrawValid = true
            for color, count in draws:
                if maxCubes[color] < count:
                    isDrawValid = false
                    break
            if not isDrawValid:
                isGameValid = false
                break
        if isGameValid:
            validIds += (i+1)

    return $validIds


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
