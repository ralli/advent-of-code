function parse_input(input)
    instructions, rest = split(input, "\n\n")
    lines = split(rest, "\n")
    nodes = [tuple([match.match for match in eachmatch(r"[0-9A-Z]+", line)]...) for line in lines]
    return (instructions=instructions, nodes=nodes)
end

function part1(instructions, nodes)::Int
    node_dict = Dict([(from, (left, right)) for (from, left, right) in nodes])
    i = 0
    current = "AAA"
    num_steps = 0
    while true
        if current == "ZZZ"
            break
        end
        instruction = instructions[i+1]
        (left, right) = node_dict[current]
        next = if instruction == 'L'
            left
        else
            right
        end
        # println(current, " ", instruction, " ", next)
        current = next
        i = i + 1
        if i >= length(instructions)
            i = 0
        end
        num_steps += 1
    end

    return num_steps
end

function path_length(instructions, nodes, start)::Int
    node_dict = Dict([(from, (left, right)) for (from, left, right) in nodes])
    i = 0
    current = start
    num_steps = 0
    while true
        if current[lastindex(current)] == 'Z'
            break
        end
        instruction = instructions[i+1]
        (left, right) = node_dict[current]
        next = if instruction == 'L'
            left
        else
            right
        end
        # println(current, " ", instruction, " ", next)
        current = next
        i = i + 1
        if i >= length(instructions)
            i = 0
        end
        num_steps += 1
    end

    return num_steps
end

function part2(instructions, nodes)
    starts = [from for (from, _, _) in nodes if from[lastindex(from)] == 'A']
    path_lengths = [path_length(instructions, nodes, start) for start in starts]
    result = 1
    for v in path_lengths
        result = lcm(result, v)
    end
    return result
end

filename = "day8.txt"
content = read(filename, String)
(instructions, nodes) = parse_input(content)
result = part1(instructions, nodes)
println(result)
result = part2(instructions, nodes)
println(result)