use std::str::Chars;

macro_rules! val {
    ($stack:expr) => {
        Val {
            stack: $stack,
            index: 0,
        }
    };
}

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Val {
    pub stack: usize,
    pub index: usize,
}

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Vals {
    pub a: Val,
    pub b: Val,
    pub c: Val,
    pub d: Val,
    pub e: Val,
    pub f: Val,
    pub g: Val,
    pub h: Val,
    pub i: Val,
}

impl Vals {
    pub fn new() -> Self {
        Vals { a: val!(0), b: val!(1), c: val!(2), d: val!(3), e: val!(4), f: val!(5), g: val!(6), h: val!(7), i: val!(8) }
    }
}

macro_rules! check_val {
    ($val:expr, $from:expr, $to:expr, $amount:expr) => {
        if $val.stack == $to {
            if $val.index < $amount {
                $val.stack = $from;
                $val.index = $amount - $val.index - 1
            } else {
                $val.index -= $amount;
            }
        } else if $val.stack == $from {
            $val.index += $amount;
        }
    };
}

pub fn direct_parse(string: &mut Chars) -> usize {
    let mut out: usize = 0;
    loop {
        if let Some(chr) = string.nth(0) {
            if chr == ' ' {
                return out;
            }
            out = out * 10 + chr as usize - 48;
        } else {
            return out;
        }
    }
}

pub fn parse(lines: &Vec<&str>) -> Vals {
    let mut values: Vals = Vals::new();
    for line in lines {
        if line.len() == 0 {
            break
        }
        // Get iterator
        let mut c = line.chars();
        // Extract the numbers
        c.nth(4); // 'move'
        let amount = direct_parse(&mut c);
        c.nth(4); // 'from'
        let from = direct_parse(&mut c)-1;
        c.nth(2); // 'to'
        let to = direct_parse(&mut c)-1;

        // Iterate through values
        check_val!(values.a, from, to, amount);
        check_val!(values.b, from, to, amount);
        check_val!(values.c, from, to, amount);
        check_val!(values.d, from, to, amount);
        check_val!(values.e, from, to, amount);
        check_val!(values.f, from, to, amount);
        check_val!(values.g, from, to, amount);
        check_val!(values.h, from, to, amount);
        check_val!(values.i, from, to, amount);
    }
    values
}
