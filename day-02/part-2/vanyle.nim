from times import cpuTime
from os import paramStr

import ../../lib/nim/vanyle/speed_utils

proc cToI(c: char): int =
    if c == 'r': return 0
    if c == 'g': return 1
    if c == 'b': return 2
    assert false

template parseInput(s: string, gameStart: untyped, inGame: untyped, gameEnd: untyped): untyped = 
    var t: Tokenizer = Tokenizer(s: s, offset: 0)

    while not t.atEnd():
        var eol = t.findNext('\n')

        t.advance(':', eol)
        t.advanceFixed(2) # eat ' :'

        gameStart

        while t.offset < eol:
            # Game 1: 19 blue, 12 red; 19 blue, 2 green, 1 red; 13 red, 11 blue

            var eob = t.findNext(';', eol)

            while t.offset < eob:
                var count {.inject.} = t.eatUnsignedInt()
                t.advanceFixed(1) # eat space.
                var color {.inject.} = t.s[t.offset] # store r,g or b.
                
                inGame

                t.advance(',', eob)
                t.advanceFixed(2) # skip the comma and space

        gameEnd

        t.advanceFixed(1) # skip end of line

proc run(s: string): string =
    var res = 0
    var minVals = [0, 0, 0]

    parseInput s:
        minVals = [0, 0, 0]
    do:
        let cidx = cToI(color)
        minVals[cidx] = max(minVals[cidx], count)
    do:
        var power = minVals[0] * minVals[1] *  minVals[2]
        res += power

    return $res


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
