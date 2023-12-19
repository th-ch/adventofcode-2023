from times import cpuTime
from os import paramStr

import strutils, re, sequtils, tables

type Rule = object
    # vari is '0' => always send to loca.
    vari: char # if vari cmpC n, send to loca
    cmpC: char
    n: int
    loca: string

# order is always x,m,a,s

proc cToI(c: char): int {.inline.} =
    if c == 'x': return 0
    if c == 'm': return 1
    if c == 'a': return 2
    if c == 's': return 3
    assert false

proc ints*(s: string): seq[int] =
    ## return all integers inside a string
    return re.findAll(s, re"-?\d+").map(parseInt)

proc parseInput(s: string): (seq[(string, seq[Rule])], seq[array[4, int]]) = 
    var sol = s.strip.split("\n\n", 1)
    var lines = sol[0].split("\n")
    var parts = sol[1].split("\n")

    var r1: seq[(string, seq[Rule])]
    var r2: seq[array[4, int]]

    for l in lines:
        var spli = l.split("{", 1)
        var workflowName = spli[0]
        var instructions = spli[1][0..<spli[1].len-1].split(",")
        
        var ruleList: seq[Rule]
        for ins in instructions:
            if ":" notin ins:
                ruleList.add Rule(vari: '0', loca: ins)
            else:
                var insParts = ins.split(":",1)
                var condi = insParts[0]
                var cmpC = '<'
                if '>' in condi:
                    cmpC = '>'

                var condiElements = condi.split(cmpC,1)

                ruleList.add Rule(
                    loca: insParts[1],
                    cmpC: cmpC,
                    n: parseInt(condiElements[1]),
                    vari: condiElements[0][0]
                )

        r1.add((workflowName, ruleList))


    for p in parts:
        var i = ints(p)
        var toadd: array[4, int]
        for j in 0..3:
            toadd[j] = i[j]
        r2.add toadd

    return (r1, r2)

proc run(s: string): string =
    var (workflows, parts) = parseInput(s)

    var workflowTable: Table[string, seq[Rule]]
    for (wname, wrule) in workflows:
        workflowTable[wname] = wrule

    var goodBoys: seq[array[4, int]]

    for p in parts:
        var cworkflow = "in"
        while cworkflow != "A" and cworkflow != "R":
            var rules = workflowTable[cworkflow]

            for r in rules:
                if r.vari == '0':
                    cworkflow = r.loca
                    break

                var rIdx = cToI(r.vari)
                var a = p[rIdx]
                if r.cmpC == '<' and a < r.n:
                    cworkflow = r.loca
                    break
                if r.cmpC == '>' and a > r.n:
                    cworkflow = r.loca
                    break

        if cworkflow == "A":
            goodBoys.add p

    var res = 0
    for p in goodBoys:
        for j in 0..3:
            res += p[j]

    return $res


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
