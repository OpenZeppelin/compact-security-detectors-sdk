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
#[rustfmt::skip]
mod tests {
    use midnight_security_rules_sdk::ast::{expression::Expression, literal::Literal};

    use super::*;

    #[test]
    fn test_parse_nat() {
        let l = compact::TermParser::new().parse("1").unwrap();
        if let midnight_security_rules_sdk::ast::program::CompactNode::Expression(
            Expression::Literal(Literal::Nat(n)),
        ) = l
        {
            assert_eq!(n.value, 1);
            assert_eq!(n.location.start, 0);
            assert_eq!(n.location.end, 1);
        } else {
            panic!("Wrong Nat `1` parsing");
        }

        let l = compact::TermParser::new().parse("0").unwrap();
        if let midnight_security_rules_sdk::ast::program::CompactNode::Expression(
            Expression::Literal(Literal::Nat(n)),
        ) = l
        {
            assert_eq!(n.value, 0);
            assert_eq!(n.location.start, 0);
            assert_eq!(n.location.end, 1);
        } else {
            panic!("Wrong Nat `0` parsing");
        }

        let l = compact::TermParser::new().parse("123").unwrap();
        if let midnight_security_rules_sdk::ast::program::CompactNode::Expression(
            Expression::Literal(Literal::Nat(n)),
        ) = l
        {
            assert_eq!(n.value, 123);
            assert_eq!(n.location.start, 0);
            assert_eq!(n.location.end, 3);
        } else {
            panic!("Wrong Nat `123` parsing");
        }

        let l = compact::TermParser::new().parse("12345").unwrap();
        if let midnight_security_rules_sdk::ast::program::CompactNode::Expression(
            Expression::Literal(Literal::Nat(n)),
        ) = l
        {
            assert_eq!(n.value, 12345);
            assert_eq!(n.location.start, 0);
            assert_eq!(n.location.end, 5);
        } else {
            panic!("Wrong Nat `12345` parsing");
        }

        assert!(compact::TermParser::new().parse("01").is_err());
        assert!(compact::TermParser::new().parse("0123").is_err());
        assert!(compact::TermParser::new().parse("-1").is_err());
        assert!(compact::TermParser::new().parse("-0123").is_err());
    }

    #[test]
    fn test_parse_version() {
        assert!(compact::TermParser::new().parse("1.2.3").is_ok());
        assert!(compact::TermParser::new().parse("1.2").is_ok());
        assert!(compact::TermParser::new().parse("1").is_ok());
        assert!(compact::TermParser::new()
            .parse(
                "
            1.2.3"
            )
            .is_ok());

        assert!(compact::TermParser::new().parse("1.2.").is_err());
        assert!(compact::TermParser::new().parse("1.").is_err());
        assert!(compact::TermParser::new().parse("1.2.3.").is_err());
    }

    #[test]
    fn test_parse_bool() {
        assert!(compact::TermParser::new().parse("true").is_ok());
        assert!(compact::TermParser::new().parse("false").is_ok());
        assert!(compact::TermParser::new().parse("True").is_err());
        assert!(compact::TermParser::new().parse("False").is_err());
    }

    #[test]
    fn test_parse_str() {
        assert!(compact::TermParser::new().parse("\"\"").is_ok());
        assert!(compact::TermParser::new().parse("\"hello\"").is_ok());
        assert!(compact::TermParser::new().parse("\"hello world\"").is_ok());
        assert!(compact::TermParser::new().parse("\"hello world!\"").is_ok());
        assert!(compact::TermParser::new().parse("\"hello world!\"").is_ok());
        assert!(compact::TermParser::new().parse("\"hello world!\"").is_ok());
        assert!(compact::TermParser::new().parse("\"hello world!\"").is_ok());
        assert!(compact::TermParser::new().parse("\"hello world!\"").is_ok());
        assert!(compact::TermParser::new().parse("\"hello world!\"").is_ok());
        assert!(compact::TermParser::new().parse("\"hello world!\"").is_ok());
        assert!(compact::TermParser::new().parse("\"hello world!\"").is_ok());
        assert!(compact::TermParser::new().parse("\"hello world!\"").is_ok());
        assert!(compact::TermParser::new().parse("\"hello world!\"").is_ok());

        assert!(compact::TermParser::new().parse("\"hello world!").is_err());
        assert!(compact::TermParser::new().parse("hello world!\"").is_err());
        assert!(compact::TermParser::new().parse("hello world!").is_err());
    }

    #[test]
    fn test_identifier() {
        assert!(compact::TermParser::new().parse("a").is_ok());
        assert!(compact::TermParser::new().parse("a1").is_ok());
        assert!(compact::TermParser::new().parse("a_1").is_ok());
        assert!(compact::TermParser::new().parse("a_1_").is_ok());
        assert!(compact::TermParser::new().parse("a_1_2").is_ok());
        assert!(compact::TermParser::new().parse("a_1_2_").is_ok());
        assert!(compact::TermParser::new().parse("a_1_2_3").is_ok());
        assert!(compact::TermParser::new().parse("a_1_2_3_").is_ok());
        assert!(compact::TermParser::new().parse("a_1_2_3_4").is_ok());
        assert!(compact::TermParser::new().parse("a_1_2_3_4_").is_ok());
        assert!(compact::TermParser::new().parse("a_1_2_3_4_5").is_ok());
        assert!(compact::TermParser::new().parse("a_1_2_3_4_5_").is_ok());
        assert!(compact::TermParser::new().parse("a_1_2_3_4_5_6").is_ok());
        assert!(compact::TermParser::new().parse("a_1_2_3_4_5_6_").is_ok());
        assert!(compact::TermParser::new().parse("a_1_2_3_4_5_6_7").is_ok());
        assert!(compact::TermParser::new().parse("a_1_2_3_4_5_6_7_").is_ok());
        assert!(compact::TermParser::new()
            .parse("a_1_2_3_4_5_6_7_8")
            .is_ok());
        assert!(compact::TermParser::new()
            .parse("a_1_2_3_4_5_6_7_8_")
            .is_ok());
        assert!(compact::TermParser::new()
            .parse("a_1_2_3_4_5_6_7_8_9")
            .is_ok());
        assert!(compact::TermParser::new()
            .parse(
                "
            a_1_2_3_4_5_6_7_8_9"
            )
            .is_ok());
    }

    #[test]
    fn test_pragma() {
        assert!(compact::TermParser::new().parse("pragma language_version >= 1.2.3;").is_ok());
        assert!(compact::TermParser::new().parse("pragma language_version > 1.2.3;").is_ok());
        assert!(compact::TermParser::new().parse("pragma language_version = 1.2.3;").is_ok());
        assert!(compact::TermParser::new().parse("pragma language_version >= ;").is_err());
        assert!(compact::TermParser::new().parse("pragma language_version > ;").is_err());
        assert!(compact::TermParser::new().parse("pragma language_version == ;").is_err());
        assert!(compact::TermParser::new().parse("pragma language_version < ;").is_err());
        assert!(compact::TermParser::new().parse("pragma language_version =< ;").is_err());
        assert!(compact::TermParser::new().parse("pragma language_version >= 1.2.3").is_err());
        assert!(compact::TermParser::new().parse("pragma language_version > 1.2.3").is_err());
        assert!(compact::TermParser::new().parse("pragma language_version == 1.2.3").is_err());
        assert!(compact::TermParser::new().parse("pragma language_version < 1.2.3").is_err());
        assert!(compact::TermParser::new().parse("pragma language_version =< 1.2.3").is_err());
        assert!(compact::TermParser::new().parse("pragma language_version 1.2.3;").is_err());
        assert!(compact::TermParser::new().parse("pragma language_version >= 1.2.3.4;").is_err());
        assert!(compact::TermParser::new().parse("pragma language_version >= 1..2.3;").is_err());
        assert!(compact::TermParser::new().parse("pragma language_version >= 1.2..3;").is_err());
        assert!(compact::TermParser::new().parse("pragma language_version >= 1.2.3.;").is_err());
    }

    #[test]
    fn test_import() {
        assert!(compact::TermParser::new().parse("import a;").is_ok());
        assert!(compact::TermParser::new().parse("import a_1;").is_ok());
        assert!(compact::TermParser::new().parse("import \"../../Test\";").is_ok());
        assert!(compact::TermParser::new().parse("import ;").is_err());
        assert!(compact::TermParser::new().parse("import 1;").is_err());
        assert!(compact::TermParser::new().parse("import a b;").is_err());
        assert!(compact::TermParser::new().parse("import \"../../Test").is_err());
        assert!(compact::TermParser::new().parse("import ../../Test\";").is_err());
    }

    #[test]
    fn test_export() {
        assert!(compact::TermParser::new().parse("export { a }").is_ok());
        assert!(compact::TermParser::new().parse("export { a, b }").is_ok());
        assert!(compact::TermParser::new().parse("export { a, b, c }").is_ok());
        assert!(compact::TermParser::new().parse("export { a };").is_ok());
        assert!(compact::TermParser::new().parse("export { a, b };").is_ok());
        assert!(compact::TermParser::new().parse("export { a, b, c };").is_ok());
        assert!(compact::TermParser::new().parse("export { ;").is_err());
        assert!(compact::TermParser::new().parse("export { a, ;").is_err());
        assert!(compact::TermParser::new().parse("export { a b }").is_err());
        assert!(compact::TermParser::new().parse("export { a, b c }").is_err());
        assert!(compact::TermParser::new().parse("export { a, b, c, }").is_err());
    }

    #[test]
    fn test_ledger() {
        assert!(compact::TermParser::new().parse("ledger test : Boolean;").is_ok());
    }
}
