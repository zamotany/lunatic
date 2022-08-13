mod expressions;
mod parser;
mod parser_utils;
mod parsing_error;

pub use parser::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{debug_visitor, scanner};

    fn expect_source_to_equal_ast(source: &str, expected: &str) {
        let mut scanner = scanner::Scanner::new(source);
        let tokens = scanner.scan_tokens().unwrap();
        let parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();
        let debug_visitor = debug_visitor::DebugVisitor;
        let output = ast.visit(&debug_visitor);
        assert_eq!(&output[..], expected);
    }

    #[test]
    fn should_parse_expressions() {
        expect_source_to_equal_ast(
            "true or (false and true) and true",
            "[or l=`true` r=[and l=([and l=`false` r=`true`]) r=`true`]]",
        );
        expect_source_to_equal_ast(
            "true and false and true",
            "[and l=`true` r=[and l=`false` r=`true`]]",
        );
        expect_source_to_equal_ast(
            "true or false and true and true",
            "[or l=`true` r=[and l=`false` r=[and l=`true` r=`true`]]]",
        );
        expect_source_to_equal_ast(
            "true or false or false and true",
            "[or l=`true` r=[or l=`false` r=[and l=`false` r=`true`]]]",
        );
        expect_source_to_equal_ast("1 >= 2 or 3", "[or l=[>= l=`1` r=`2`] r=`3`]");
        expect_source_to_equal_ast("false or 1 > 2", "[or l=`false` r=[> l=`1` r=`2`]]");
        expect_source_to_equal_ast("1 >= 2 and 3", "[and l=[>= l=`1` r=`2`] r=`3`]");
        expect_source_to_equal_ast(
            "false and 5 >= 5 or 11 < 10",
            "[or l=[and l=`false` r=[>= l=`5` r=`5`]] r=[< l=`11` r=`10`]]",
        );
        expect_source_to_equal_ast(
            "2 == 2 ^ 1 or true",
            "[or l=[== l=`2` r=[^ l=`2` r=`1`]] r=`true`]",
        );
        expect_source_to_equal_ast("not true or true", "[or l=[not r=`true`] r=`true`]");
        expect_source_to_equal_ast(
            "2 / 2 == 1 and true",
            "[and l=[== l=[/ l=`2` r=`2`] r=`1`] r=`true`]",
        );
        expect_source_to_equal_ast(
            "2 - 1 == 1 and true",
            "[and l=[== l=[- l=`2` r=`1`] r=`1`] r=`true`]",
        );
        expect_source_to_equal_ast(
            "'hello' .. 'world' ~= 0 or true",
            "[or l=[~= l=[.. l=`'hello'` r=`'world'`] r=`0`] r=`true`]",
        );
        expect_source_to_equal_ast(
            "2 << 2 == 8 or true",
            "[or l=[== l=[<< l=`2` r=`2`] r=`8`] r=`true`]",
        );
        expect_source_to_equal_ast(
            "1 & 1 == 3 or true",
            "[or l=[== l=[& l=`1` r=`1`] r=`3`] r=`true`]",
        );
        expect_source_to_equal_ast(
            "1 ~ 1 == 3 or true",
            "[or l=[== l=[~ l=`1` r=`1`] r=`3`] r=`true`]",
        );
        expect_source_to_equal_ast(
            "1 | 1 == 3 or true",
            "[or l=[== l=[| l=`1` r=`1`] r=`3`] r=`true`]",
        );
        expect_source_to_equal_ast(
            "(1 ~= 2 or (true or 2 << 1 == 4)) and false and (true or false)",
            "[and l=([or l=[~= l=`1` r=`2`] r=([or l=`true` r=[== l=[<< l=`2` r=`1`] r=`4`]])]) r=[and l=`false` r=([or l=`true` r=`false`])]]"
        );
        expect_source_to_equal_ast(
            "{ foo = 1, bar = 2 } and true",
            "[and l=Tc[`foo`=`1` `bar`=`2` ] r=`true`]",
        );
        expect_source_to_equal_ast(
            "foo.baz or foo[1 + 2].baz",
            "[or l=foo.baz r=foo[[+ l=`1` r=`2`]].baz]",
        );
    }

    #[test]
    fn should_parse_table_constructor() {
        expect_source_to_equal_ast("{ foo = 1, }", "Tc[`foo`=`1` ]");
        expect_source_to_equal_ast("{ 123 }", "Tc[?=`123` ]");
        expect_source_to_equal_ast("{ }", "Tc[]");
        expect_source_to_equal_ast("{ 'foo' }", "Tc[?=`'foo'` ]");
        expect_source_to_equal_ast(
            "{ ['fo'..'o'] = 'bar' }",
            "Tc[[.. l=`'fo'` r=`'o'`]=`'bar'` ]",
        );
        expect_source_to_equal_ast("{ [1 + 2] = 'bar' }", "Tc[[+ l=`1` r=`2`]=`'bar'` ]");
        expect_source_to_equal_ast("{ 1, 2, 3 }", "Tc[?=`1` ?=`2` ?=`3` ]");
        expect_source_to_equal_ast("{ foo = 1, bar = 2; }", "Tc[`foo`=`1` `bar`=`2` ]");
        expect_source_to_equal_ast(
            "{ [1 + 2] = 'bar', ['1'..'2'] = 'foo' }",
            "Tc[[+ l=`1` r=`2`]=`'bar'` [.. l=`'1'` r=`'2'`]=`'foo'` ]",
        );
        expect_source_to_equal_ast(
            "{ 'hello'; [1 + 2] = 'bar'; foo = 1; }",
            "Tc[?=`'hello'` [+ l=`1` r=`2`]=`'bar'` `foo`=`1` ]",
        );
        expect_source_to_equal_ast(
            "{ foo = { bar = 1 }, baz = {} }",
            "Tc[`foo`=Tc[`bar`=`1` ] `baz`=Tc[] ]",
        );
    }

    #[test]
    fn should_parse_variables() {
        expect_source_to_equal_ast("foo", "foo");
        expect_source_to_equal_ast("foo == true", "[== l=foo r=`true`]");
        expect_source_to_equal_ast("foo.bar", "foo.bar");
        expect_source_to_equal_ast("(foo).bar", "(foo).bar");
        expect_source_to_equal_ast("(foo or baz).bar", "([or l=foo r=baz]).bar");
        expect_source_to_equal_ast("foo.baz.bar", "foo.baz.bar");
        expect_source_to_equal_ast("foo.baz.bar.qoo", "foo.baz.bar.qoo");
        expect_source_to_equal_ast("(foo or baz).bar.qoo", "([or l=foo r=baz]).bar.qoo");
        expect_source_to_equal_ast("foo[bar]", "foo[bar]");
        expect_source_to_equal_ast("foo.baz[bar]", "foo.baz[bar]");
        expect_source_to_equal_ast("foo[1 + 2]", "foo[[+ l=`1` r=`2`]]");
        expect_source_to_equal_ast("foo.baz[1 + 2]", "foo.baz[[+ l=`1` r=`2`]]");
        expect_source_to_equal_ast("foo[1 + 2][3 + 4]", "foo[[+ l=`1` r=`2`]][[+ l=`3` r=`4`]]");
        expect_source_to_equal_ast("foo[1 + 2].bar", "foo[[+ l=`1` r=`2`]].bar");
    }
}
