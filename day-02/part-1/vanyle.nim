from times import cpuTime
from os import paramStr

import strutils

# red = 0, green = 1, blue = 2
var maxCubes = [12,13,14]

proc parseInput(s: string): seq[seq[array[3, int]]] = 
    var l = s.split("\n")

    for line in l:
        var game = line.split(": ")[1]
        var draws = game.split("; ")
        var cgame: seq[array[3,int]]
        for d in draws:
            var cc = d.strip.split(", ")
            var tt: array[3,int] = [0,0,0]
            for el in cc:
                var pair = el.split(" ",2)
                if pair[1][0] == 'r': tt[0] = parseInt(pair[0])
                if pair[1][0] == 'g': tt[1] = parseInt(pair[0])
                if pair[1][0] == 'b': tt[2] = parseInt(pair[0])
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
