from times import cpuTime
from os import paramStr

import tables, strutils

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
    var s = 0

    for i in 0..<r.len:
        var game = r[i]
        var minVals = {
            "red": 0,
            "green": 0,
            "blue": 0
        }.toTable

        for draws in game:
            for color, count in draws:
                minVals[color] = max(minVals[color], count)

        var power = minVals["red"] * minVals["green"] *  minVals["blue"]
        s += power
    return $s


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
