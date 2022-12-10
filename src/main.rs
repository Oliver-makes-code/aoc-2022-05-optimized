use std::time::Instant;

mod instr;

macro_rules! get_val {
    ($val:expr, $stack:expr, $idx:expr) => {
        &$stack[$idx[$val.stack].unwrap()+$val.index].chars().nth($val.stack*4+1).unwrap().to_string()
    };
}

macro_rules! check_stack {
    ($line:expr, $n:expr, $stack:expr, $idx:expr, $count:expr, $i:expr) => {
        if  $line.nth($n).unwrap() != ' ' && $stack[$idx].is_none() {
            $stack[$idx] = Some($i);
            $count += 1;
        }
    };
}

fn main() {
    let inital = Instant::now();
    let mut now = Instant::now();
    println!("Reading input");
    // Read file at compile
    let mut input = include_str!("../day5.txt").lines().collect::<Vec<_>>();
    // Reverse input to read commands backwards
    input.reverse();
    println!("Elapsed: {:.2?}\n", now.elapsed());
    now = Instant::now();
    println!("Parsing instructions");
    let values = instr::parse(&input);
    println!("Elapsed: {:.2?}\n", now.elapsed());
    now = Instant::now();
    println!("Extracting values");
    let mut start: Vec<Option<usize>> = vec![None,None,None,None,None,None,None,None,None];
    let mut num = 0;
    // Re-reverse to parse stack
    input.reverse();
    // Iterate through lines
    let mut idx = 0;
    while num != 9 {
        let mut line = input[idx].chars();
        let mut stack_num = 0;
        // Get first char, set value if not space
        check_stack!(line, 0, start, stack_num, num, idx);
        stack_num += 1;
        check_stack!(line, 3, start, stack_num, num, idx);
        stack_num += 1;
        check_stack!(line, 3, start, stack_num, num, idx);
        stack_num += 1;
        check_stack!(line, 3, start, stack_num, num, idx);
        stack_num += 1;
        check_stack!(line, 3, start, stack_num, num, idx);
        stack_num += 1;
        check_stack!(line, 3, start, stack_num, num, idx);
        stack_num += 1;
        check_stack!(line, 3, start, stack_num, num, idx);
        stack_num += 1;
        check_stack!(line, 3, start, stack_num, num, idx);
        stack_num += 1;
        check_stack!(line, 3, start, stack_num, num, idx);
        idx += 1;
    }
    let mut out = "".to_string();
    out += get_val!(values.a, input, start);
    out += get_val!(values.b, input, start);
    out += get_val!(values.c, input, start);
    out += get_val!(values.d, input, start);
    out += get_val!(values.e, input, start);
    out += get_val!(values.f, input, start);
    out += get_val!(values.g, input, start);
    out += get_val!(values.h, input, start);
    out += get_val!(values.i, input, start);
    println!("Elapsed: {:.2?}\n", now.elapsed());
    println!("Result: {}\n", out);
    println!("Total: {:.2?}", inital.elapsed());
}
