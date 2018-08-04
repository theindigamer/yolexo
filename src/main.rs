extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("parse.pest");

#[derive(Parser)]
#[grammar = "parse.pest"]
struct YolexoParser;

fn main() {
    let pairs = YolexoParser::parse(Rule::ident_list, "lexer! {   }")
        .unwrap_or_else(|e| panic!("{}", e));
    let pairs = YolexoParser

    // Because ident_list is silent, the iterator will contain idents
    for pair in pairs {

        let span = pair.clone().into_span();
        // A pair is a combination of the rule which matched and a span of input
        println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", span);
        println!("Text:    {}", span.as_str());

        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
            let inner_span = inner_pair.clone().into_span();
            match inner_pair.as_rule() {
                Rule::letter => println!("Letter:  {}", inner_span.as_str()),
                Rule::digit => println!("Digit:   {}", inner_span.as_str()),
                _ => unreachable!(),
            };
        }
    }
}
