def to_i_2(obj)
    if obj == "one" or obj == "1"
        return 1
    elsif obj == "two" or obj == "2"
        return 2
    elsif obj == "three" or obj == "3"
        return 3
    elsif obj == "four" or obj == "4"
        return 4
    elsif obj == "five" or obj == "5"
        return 5
    elsif obj == "six" or obj == "6"
        return 6
    elsif obj == "seven" or obj == "7"
        return 7
    elsif obj == "eight" or obj == "8"
        return 8
    elsif obj == "nine" or obj == "9"
        return 9
    end
end

def run(s)
    sum = 0
    for line in s
        matches = line.scan(/(?=([1-9]|one|two|three|four|five|six|seven|eight|nine))/)
        first = to_i_2(matches[0][0])
        last = to_i_2(matches[-1][0])

        sum += first * 10 + last
    end
    return sum
end



starting = Process.clock_gettime(Process::CLOCK_MONOTONIC)
answer = run(ARGV[0].lines)
elapsed = (Process.clock_gettime(Process::CLOCK_MONOTONIC) - starting) * 1000

puts "_duration:#{elapsed}\n#{answer}"
