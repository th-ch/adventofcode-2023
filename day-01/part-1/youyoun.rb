def is_number?(obj)
    return obj.ord >= 48 && obj.ord <= 57
end

def run(s)
    sum = 0
    for line in s
        first, last = nil, nil
        first_idx = nil
        line.each_char do |c|
            if is_number?(c)
                if first.nil?
                    first = c.to_i
                end
                last = c.to_i
            end
        end
        sum += first * 10 + last
    end
    return sum
end


starting = Process.clock_gettime(Process::CLOCK_MONOTONIC)
answer = run(ARGV[0].lines)
elapsed = (Process.clock_gettime(Process::CLOCK_MONOTONIC) - starting) * 1000

puts "_duration:#{elapsed}\n#{answer}"
