macro_rules! lalrpop_mod_doc {
    ($vis:vis $name:ident) => {
        lalrpop_util::lalrpop_mod!(
            #[allow(clippy::ptr_arg)]
            #[allow(clippy::vec_box)]
            $vis $name);
    }
}

lalrpop_mod_doc!(pub compact);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_nat() {
        assert!(compact::TermParser::new().parse("1").is_ok());
        assert!(compact::TermParser::new().parse("0").is_ok());
        assert!(compact::TermParser::new().parse("23").is_ok());
        assert!(compact::TermParser::new().parse("12345").is_ok());

        assert!(compact::TermParser::new().parse("01").is_err());
        assert!(compact::TermParser::new().parse("0123").is_err());
        assert!(compact::TermParser::new().parse("-1").is_err());
        assert!(compact::TermParser::new().parse("-0123").is_err());
    }
}
