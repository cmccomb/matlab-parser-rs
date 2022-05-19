#![feature(try_blocks)]
#![feature(specialization)]
#![feature(coerce_unsized)]

mod csvlexer;
mod csvlistener;
mod csvparser;
mod csvvisitor;

mod matlablexer;

use std::borrow::Cow;
use std::fmt::Write;
use std::io::Read;
use std::iter::FromIterator;

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::int_stream::IntStream;
use antlr_rust::lexer::Lexer;

use crate::csvlexer::*;
use crate::csvlistener::*;
use crate::csvparser::CSVParser;
use antlr_rust::token::{Token, TOKEN_EOF};
use antlr_rust::token_factory::{ArenaCommonFactory, OwningTokenFactory};
use antlr_rust::token_stream::UnbufferedTokenStream;
use antlr_rust::tree::{
    ParseTree, ParseTreeListener, ParseTreeVisitor, ParseTreeWalker, TerminalNode, Tree,
    VisitChildren, Visitable,
};
use antlr_rust::InputStream;

use crate::csvparser::{
    CSVParserContext, CSVParserContextType, CsvFileContext, HdrContext, RowContext,
};
use crate::csvvisitor::CSVVisitor;

use matlablexer::matlabLexer;

#[test]
fn lexer_test_matlab() {
    println!("test started lexer_test_matlab");
    let tf = matlablexer::LocalTokenFactory::default();
    // let tf = ArenaCommonFactory::default();
    let mut _lexer = matlabLexer::new_with_token_factory(
        InputStream::new("a = ones(21, 21)".into()),
        // Box::new(UTF16InputStream::from_str("V123,V2\nd1,d222","".into())),
        &tf,
    );
    let mut token_source = UnbufferedTokenStream::new_buffered(_lexer);
    let mut token_source_iter = token_source.token_iter();
    assert_eq!(
        token_source_iter.next().unwrap().to_string(),
        "[@0,0:0='a',<42>,1:0]"
    );
    assert_eq!(
        token_source_iter.next().unwrap().to_string(),
        "[@1,2:2='=',<18>,1:2]"
    );
    assert_eq!(
        token_source_iter.next().unwrap().to_string(),
        "[@2,4:7='ones',<42>,1:4]"
    );
    assert_eq!(
        token_source_iter.next().unwrap().to_string(),
        "[@3,8:8='(',<1>,1:8]"
    );
    assert_eq!(
        token_source_iter.next().unwrap().to_string(),
        "[@4,9:10='21',<43>,1:9]"
    );
    assert_eq!(
        token_source_iter.next().unwrap().to_string(),
        "[@5,11:11=',',<6>,1:11]"
    );
    assert_eq!(
        token_source_iter.next().unwrap().to_string(),
        "[@6,13:14='21',<43>,1:13]"
    );
    assert_eq!(
        token_source_iter.next().unwrap().to_string(),
        "[@7,15:15=')',<2>,1:15]"
    );
    assert_eq!(
        token_source_iter.next().unwrap().to_string(),
        "[@8,16:15='<EOF>',<-1>,1:16]"
    );
    assert!(token_source_iter.next().is_none());
}

mod matlablistener;
mod matlabparser;
use matlabparser::{matlabParserContext, matlabParserContextType};

struct Listener {}

impl<'input> ParseTreeListener<'input, matlabParserContextType> for Listener {
    fn enter_every_rule(&mut self, ctx: &dyn matlabParserContext<'input>) {
        println!(
            "rule entered {}",
            matlabparser::ruleNames
                .get(ctx.get_rule_index())
                .unwrap_or(&"error")
        )
    }
}

impl<'input> matlablistener::matlabListener<'input> for Listener {}

#[test]
fn parser_test_csv() {
    let tf = matlablexer::LocalTokenFactory::default();
    let mut _lexer =
        matlablexer::matlabLexer::new_with_token_factory(InputStream::new("a=2; b=2;".into()), &tf);
    let token_source = CommonTokenStream::new(_lexer);
    let mut parser = matlabparser::matlabParser::new(token_source);
    parser.add_parse_listener(Box::new(Listener {}));
    let result = parser.assignment_expression();
    assert!(result.is_ok());
    // assert_eq!(
    //     result.unwrap().to_string_tree(&*parser),
    //     "(csvFile (hdr (row (field V123) , (field V2) \\n)) (row (field d1) , (field d2) \\n))"
    // );
}

mod matlabvisitor;
use matlabvisitor::matlabVisitor;

struct MymatlabVisitor<'i, T>(Vec<&'i str>, T);

impl<T> VisitChildren<'i, matlabParserContextType> for MymatlabVisitor<'i, T> {
    fn visit_children_inner(&mut self, node: &antlr_rust::parser::Type) {
        todo!()
    }
}

impl<'i, T> ParseTreeVisitor<'i, matlabParserContextType> for MymatlabVisitor<'i, T> {
    fn visit_terminal(&mut self, node: &TerminalNode<'i, matlabParserContextType>) {
        if node.symbol.get_token_type() == matlabparser::TEXT {
            if let Cow::Borrowed(s) = node.symbol.text {
                self.0.push(s);
            }
        }
    }
}
//
// use csvparser::RowContextAttrs;
// // use antlr_rust::rule_context::CustomRuleContext;
// use std::borrow::Cow;
// use std::rc::Rc;
//
// impl<'i, T> CSVVisitor<'i> for MyCSVVisitor<'i, T> {
//     fn visit_hdr(&mut self, _ctx: &HdrContext<'i>) {}
//
//     fn visit_row(&mut self, ctx: &RowContext<'i>) {
//         if ctx.field_all().len() > 1 {
//             self.visit_children(ctx)
//         }
//     }
// }
//
// // tests zero-copy parsing with non static visitor
// #[test]
// fn test_visitor() {
//     fn parse<'a>(tf: &'a ArenaCommonFactory<'a>) -> Rc<CsvFileContext<'a>> {
//         let mut _lexer =
//             CSVLexer::new_with_token_factory(InputStream::new("h1,h2\nd1,d2\nd3\n".into()), tf);
//         let token_source = CommonTokenStream::new(_lexer);
//         let mut parser = CSVParser::new(token_source);
//         let result = parser.csvFile().expect("parsed unsuccessfully");
//
//         let mut test = 5;
//         let mut visitor = MyCSVVisitor(Vec::new(), &mut test);
//         result.accept(&mut visitor);
//         assert_eq!(visitor.0, vec!["d1", "d2"]);
//
//         result
//     }
//     let tf = ArenaCommonFactory::default();
//
//     let _result = parse(&tf);
// }
