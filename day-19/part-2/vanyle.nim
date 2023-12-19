from times import cpuTime
from os import paramStr

import strutils, tables, parseutils

type Rule = object
    # vari is '0' => always send to loca.
    vari: char # if vari cmpC n, send to loca
    cmpC: char
    n: int
    loca: string

proc cToI(c: char): int {.inline.} =
    if c == 'x': return 0
    if c == 'm': return 1
    if c == 'a': return 2
    if c == 's': return 3
    assert false

proc toString(bytes: openarray[char]): string =
  result = newString(bytes.len)
  copyMem(result[0].addr, bytes[0].unsafeAddr, bytes.len)

iterator arraySplit(s: openarray[char], c: char): auto =
    var i = 0
    var prev = 0
    while i < s.len:
        if s[i] == c:
            yield s.toOpenArray(prev, i - 1)
            prev = i + 1 
        inc i
    yield s.toOpenArray(prev, s.len)


proc indexOf(s: openarray[char], c: char): int =
    for i in 0..<s.len:
        if s[i] == c: return i
    return -1

proc parseInput(s: string): seq[(string, seq[Rule])] = 
    var r1: seq[(string, seq[Rule])] = newSeqOfCap[(string, seq[Rule])](600)

    for l1 in s.arraySplit('\n'):
        if l1.len == 0: break
        var l = l1.toString()
        var idxBracket = l.indexOf('{')

        var workflowName = l.toOpenArray(0, idxBracket-1).toString()
        var instructions = l.toOpenArray(idxBracket+1,l.len-2).toString().split(",")
        
        var ruleList: seq[Rule]
        for ins in instructions:
            var idxColon = ins.indexOf(':')
            if idxColon == -1:
                ruleList.add Rule(vari: '0', loca: ins)
            else:
                var cmpC = '<'
                if '>' in ins:
                    cmpC = '>'

                var n = 0
                discard parseInt(ins.toOpenArray(2, idxColon - 1), n)

                ruleList.add Rule(
                    loca: ins.toOpenArray(idxColon+1, ins.len-1).toString(),
                    cmpC: cmpC,
                    n: n,
                    vari: ins[0]
                )

        r1.add((workflowName, ruleList))

    return r1

proc countLeaves(wt: Table[string, (Rule, Rule)], rule: string, inter: array[4, (int,int)]): int =
    for (a,b) in inter:
        if a > b: return 0

    if rule == "A":
        var res = 1
        for (a,b) in inter: res *= (b-a+1)
        return res
    elif rule == "R":
        return 0

    var (r1, r2) = wt[rule]
    var el = cToI(r1.vari)

    var i1 = inter
    var i2 = inter

    if r1.cmpC == '<':
        i1[el] = (inter[el][0], r1.n-1) # pass
        i2[el] = (r1.n, inter[el][1]) # else
        return countLeaves(wt, r1.loca, i1) + countLeaves(wt, r2.loca, i2)
    else: # r1.cmpC == '>':
        i1[el] = (inter[el][0], r1.n) # else
        i2[el] = (r1.n+1, inter[el][1]) # pass
        return countLeaves(wt, r1.loca, i2) + countLeaves(wt, r2.loca, i1)

proc run(s: string): string =
    var workflows = parseInput(s)

    var workflowTable: Table[string, (Rule, Rule)]
    for (wname, wrule) in workflows:
        # Rule deduplication.
        # The seq is always of length 2.
        for i in 0..<(wrule.len - 1):
            var tmpRule = wname
            if i != 0:
                tmpRule = wname & $i

            var r: Rule = wrule[i]
            var elseClause: Rule = Rule(vari: '0')

            if i < wrule.len-2:
                elseClause.loca = wname & $(i+1)
            else: # i == wrule.len-2
                elseClause.loca = wrule[wrule.len-1].loca

            workflowTable[tmpRule] = (r, elseClause)

    # Count the leaves reaching
    var res = workflowTable.countLeaves("in", [(1,4000),(1,4000),(1,4000),(1,4000)])
    return $res 


var input: string = paramStr(1)

var t0 = cpuTime()
var output = run(input)

echo "_duration:", (cpuTime() - t0) * 1000
echo output
