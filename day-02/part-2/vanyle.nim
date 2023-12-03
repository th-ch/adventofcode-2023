from times import cpuTime
from os import paramStr

import strutils

proc run(s: string): string =
    var r = 0

    for line in s.split("\n"):
        var game = line.split(": ")[1]
        var minVals = [0,0,0]
        for d in game.split("; "):
            for el in d.split(", "):
                var pair = el.split(" ",2)
                if pair[1][0] == 'r': minVals[0] = max(minVals[0], parseInt(pair[0]))
                if pair[1][0] == 'g': minVals[1] = max(minVals[1], parseInt(pair[0]))
                if pair[1][0] == 'b': minVals[2] = max(minVals[2], parseInt(pair[0]))
        let power = minVals[0] * minVals[1] *  minVals[2]
        r += power

    return $r


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
