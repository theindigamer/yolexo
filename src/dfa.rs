use std::fmt::Display;
use std::io::Write;
use std::vec::Vec;
use regex_syntax::hir::Hir;

type Letter = u8;

struct Pred {
    EQ(Letter),
    NE(Letter),
    Betw(Letter, Letter), // Invariant LHS < RHS
    Cat(Vec<Box<Pred>>),
    Alt(Vec<Box<Pred>>),
}

type StateIdx = u32;

struct State {
    jumps: Vec<(Pred, Option<StateIdx>)>,
}

pub struct DFA<T> {
    pub trans_table: Vec<State>,
}

pub fn nullable(h: &Hir) -> bool {
    is_match_empty(h)
}

pub fn firstpos(h: &Hir) -> Set<Pred> {
    vec![]
}

pub fn lastpos(h: &Hir) -> Set<Pred> {
    vec![]
}

pub fn followpos(p: Position) -> Vec<Position> {
    vec![p]
}

pub fn char_to_pred(c: char) -> Pred {
}

pub fn to_predicate(cur: ClassUnicodeRange) -> Pred {

}
