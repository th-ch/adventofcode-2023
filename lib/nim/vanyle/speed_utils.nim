# Utility functions for Nim
# Mainly used to quickly parse strings without having to do copies.

iterator fastSplit*(s: openarray[char], c: char): auto =
    var i = 0
    var prev = 0
    while i < s.len:
        if s[i] == c:
            yield s.toOpenArray(prev, i-1)
            prev = i+1
        inc i
    yield s.toOpenArray(prev, s.len-1)

proc toString*(s: openarray[char]): string =
    result = newString(s.len)
    copyMem(result[0].addr, s[0].addr, s.len)

proc isDigit*(c: char): bool {.inline.} =
    return c <= '9' and '0' <= c

proc toDigit*(c: char): int {.inline.} =
    return cast[int](c) - cast[int]('0')

type Tokenizer* = object
    s*: string
    offset*: int

proc isEqual*(a: openarray[char], b: static[string]): bool {.inline.} =
    # Don't check for length as b is static, so we can assume that a was correctly sliced.
    for i in 0..<b.len: # notice we use b here as b is know at compile time so the loop can be unrolled.
        if a[i] != b[i]: return false
    return true

proc advance*(t: var Tokenizer, c: char, until: int = int.high) =
    while t.offset < until and t.s[t.offset] != c:
        inc t.offset

proc advance*(t: var Tokenizer, s: static[string], until: int = int.high) =
    while t.offset < until and not isEqual(t.s.toOpenArray(t.offset, t.offset+s.len-1), s):
        inc t.offset

proc findNext*(t: Tokenizer, c: char, until: int = int.high): int =
    result = t.offset
    while result < until and result < t.s.len and t.s[result] != c:
        inc result

proc findNext*(t: Tokenizer, s: static[string], until: int = int.high): int =
    result = t.offset
    while result < until and result < (t.s.len - s.len + 1) and not isEqual(t.s.toOpenArray(result, result+s.len-1), s):
        inc result

proc advanceFixed*(t: var Tokenizer, i: int) {.inline.} =
    t.offset += i

proc atEnd*(t: Tokenizer): bool {.inline.} = return t.offset >= t.s.len

proc eatUnsignedInt*(t: var Tokenizer): int =
    while true:
        let c = t.s[t.offset]
        if not isDigit(c):
            return result
        var d = toDigit(c)
        result *= 10
        result += d
        inc t.offset

proc ints*(s: openarray[char], cap: int = 3): seq[int] =
    ## return all integers inside a string, quickly.
    ## Handle negative numbers (at a speed cost.)
    result = newSeqOfCap[int](cap)
    var p = 0
    var nflag = 1
    var isP = false

    for i in 0..<s.len:
        if s[i] == '-':
            nflag = -1
        elif isDigit(s[i]):
            isP = true
            p *= 10
            p += toDigit(s[i]) * nflag
        else:
            if isP:
                result.add(p)
                p = 0
                isP = false
            nflag = 1
    
    if isP:
        result.add p
