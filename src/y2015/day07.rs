use std::collections::HashMap;
use std::error::Error;

use regex::Regex;

use crate::ParseError;
use crate::util::parse::ParseOps;

#[derive(Debug, Copy, Clone)]
pub enum Op {
    None,
    Const(u64),
    ConstId(usize),
    ConstAnd(u64, usize),
    And(usize, usize),
    Or(usize, usize),
    LShift(usize, usize),
    RShift(usize, usize),
    Not(usize),
}

pub type ParsedInput = (HashMap<String, usize>, Vec<Op>);

pub fn parse(input: &str) -> Result<ParsedInput, Box<dyn Error>> {
    const CONSTANT: &str = r"(?P<constant>[a-z]+|\d+)";
    const AND: &str = r"(?P<lhs_and>[a-z]+|\d+) AND (?P<rhs_and>[a-z]+)";
    const OR: &str = r"(?P<lhs_or>[a-z]+) OR (?P<rhs_or>[a-z]+)";
    const LSHIFT: &str = r"(?P<lhs_lshift>[a-z]+) LSHIFT (?P<rhs_lshift>\d+)";
    const RSHIFT: &str = r"(?P<lhs_rshift>[a-z]+) RSHIFT (?P<rhs_rshift>\d+)";
    const NOT: &str = r"NOT (?P<rhs_not>[a-z]+)";
    const DST_ID: &str = r"(?P<dst_wire>[a-z]+)";

    let pat = format!(r"(?:{CONSTANT}|{AND}|{OR}|{LSHIFT}|{RSHIFT}|{NOT}) -> {DST_ID}");
    let re = Regex::new(&pat).unwrap();

    let mut id_map: HashMap<String, usize> = Default::default();
    let mut state: Vec<Op> = Default::default();

    let mut wire_to_id = |state: &mut Vec<Op>, wire: &str| -> usize {
        let curr_size = id_map.len();
        let id = *id_map.entry(wire.to_owned()).or_insert(curr_size);
        if state.len() <= id {
            state.resize(id + 1, Op::None);
        }
        id
    };

    for caps in re.captures_iter(input) {
        if let Some(constant) = caps.name("constant") {
            let constant_u64: Option<u64> = constant.as_str().parse().ok();
            if let Some(constant_u64) = constant_u64 {
                let dst_id = wire_to_id(&mut state, &caps["dst_wire"]);
                state[dst_id] = Op::Const(constant_u64);
            } else {
                let dst_id = wire_to_id(&mut state, &caps["dst_wire"]);
                let const_id = wire_to_id(&mut state, constant.as_str());
                state[dst_id] = Op::ConstId(const_id);
            }
        } else if let Some(lhs) = caps.name("lhs_and")
            && let Some(rhs) = caps.name("rhs_and")
        {
            let lhs_const: Option<u64> = lhs.as_str().parse().ok();
            if let Some(lhs_const) = lhs_const {
                let dst_id = wire_to_id(&mut state, &caps["dst_wire"]);
                let rhs_id = wire_to_id(&mut state, rhs.as_str());
                state[dst_id] = Op::ConstAnd(lhs_const, rhs_id);
            } else {
                let dst_id = wire_to_id(&mut state, &caps["dst_wire"]);
                let lhs_id = wire_to_id(&mut state, lhs.as_str());
                let rhs_id = wire_to_id(&mut state, rhs.as_str());
                state[dst_id] = Op::And(lhs_id, rhs_id);
            }
        } else if let Some(lhs) = caps.name("lhs_or")
            && let Some(rhs) = caps.name("rhs_or")
        {
            let dst_id = wire_to_id(&mut state, &caps["dst_wire"]);
            let lhs_id = wire_to_id(&mut state, lhs.as_str());
            let rhs_id = wire_to_id(&mut state, rhs.as_str());
            state[dst_id] = Op::Or(lhs_id, rhs_id);
        } else if let Some(lhs) = caps.name("lhs_lshift")
            && let Some(rhs) = caps.name("rhs_lshift")
        {
            let dst_id = wire_to_id(&mut state, &caps["dst_wire"]);
            let lhs_id = wire_to_id(&mut state, lhs.as_str());
            state[dst_id] = Op::LShift(lhs_id, rhs.as_str().unsigned());
        } else if let Some(lhs) = caps.name("lhs_rshift")
            && let Some(rhs) = caps.name("rhs_rshift")
        {
            let dst_id = wire_to_id(&mut state, &caps["dst_wire"]);
            let lhs_id = wire_to_id(&mut state, lhs.as_str());
            state[dst_id] = Op::RShift(lhs_id, rhs.as_str().unsigned());
        } else if let Some(rhs) = caps.name("rhs_not") {
            let dst_id = wire_to_id(&mut state, &caps["dst_wire"]);
            let rhs_id = wire_to_id(&mut state, rhs.as_str());
            state[dst_id] = Op::Not(rhs_id);
        } else {
            return Err(ParseError(format!("Bad input: {}", caps[0].to_owned())).into());
        }
    }

    Ok((id_map, state))
}

fn calc(state: &mut [Op], id: usize) -> u64 {
    let v = match state[id] {
        Op::None => unreachable!(),
        Op::Const(v) => v,
        Op::ConstId(other) => calc(state, other),
        Op::ConstAnd(v, rhs) => v & calc(state, rhs),
        Op::And(lhs, rhs) => calc(state, lhs) & calc(state, rhs),
        Op::Or(lhs, rhs) => calc(state, lhs) | calc(state, rhs),
        Op::LShift(lhs, sh) => calc(state, lhs) << sh,
        Op::RShift(lhs, sh) => calc(state, lhs) >> sh,
        Op::Not(rhs) => !calc(state, rhs),
    };
    state[id] = Op::Const(v);
    v
}

pub fn part1(input: &ParsedInput) -> Option<u64> {
    let (id_map, mut state) = input.to_owned();

    calc(&mut state, id_map["a"]).into()
}

pub fn part2(input: &ParsedInput) -> Option<u64> {
    let (id_map, mut state) = input.to_owned();

    // This recalculates but idc tbh
    state[id_map["b"]] = Op::Const(part1(&input.clone()).unwrap());

    calc(&mut state, id_map["a"]).into()
}
