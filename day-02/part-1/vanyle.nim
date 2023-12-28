from times import cpuTime
from os import paramStr

import ../../lib/nim/vanyle/speed_utils

const maxCubes = [12,13,14]

proc cToI(c: char): int =
    if c == 'r': return 0
    if c == 'g': return 1
    if c == 'b': return 2

proc run(s: string): string =
    var t: Tokenizer = Tokenizer(s: s, offset: 0)
    var validIds = 0
    var lineIdx = 1

    while not t.atEnd():
        var eol = t.findNext('\n')
        var isGameValid = true

        # Game 1: 19 blue, 12 red; 19 blue, 2 green, 1 red; 13 red, 11 blue
        t.advance(':', eol)
        t.advanceFixed(2) # eat ' :'

        block lineParse:
            while t.offset < eol:
                var eob = t.findNext(';', eol)

                while t.offset < eob:
                    let count = t.eatUnsignedInt()
                    t.advanceFixed(1) # eat space.
                    let color = t.s[t.offset] # store r,g or b.
                    let cIdx = cToI(color)

                    # Process game:
                    if maxCubes[cIdx] < count:
                        isGameValid = false
                        t.offset = eol
                        break lineParse

                    t.advance(',', eob)
                    t.advanceFixed(2) # skip the comma and space

        t.advanceFixed(1) # skip end of line

        if isGameValid:
            validIds += lineIdx

        inc lineIdx

    return $validIds


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
