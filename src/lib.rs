extern crate regex_syntax;

#[cfg(test)]
mod tests {
    extern crate regex_syntax;
    use regex_syntax::ast::parse::Parser as P;
    use regex_syntax::hir::translate::Translator;

    #[test]
    fn it_works() {
        let a = P::new().parse(r"a|b");
        println!("{:?}", a);
        assert_eq!(1 + 2, 4);
    }
}
