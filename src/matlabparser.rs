// Generated from src/matlab.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::matlablistener::*;
use super::matlabvisitor::*;

use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const T__0:isize=1; 
		pub const T__1:isize=2; 
		pub const T__2:isize=3; 
		pub const T__3:isize=4; 
		pub const T__4:isize=5; 
		pub const T__5:isize=6; 
		pub const T__6:isize=7; 
		pub const T__7:isize=8; 
		pub const T__8:isize=9; 
		pub const T__9:isize=10; 
		pub const T__10:isize=11; 
		pub const T__11:isize=12; 
		pub const T__12:isize=13; 
		pub const T__13:isize=14; 
		pub const T__14:isize=15; 
		pub const T__15:isize=16; 
		pub const T__16:isize=17; 
		pub const T__17:isize=18; 
		pub const T__18:isize=19; 
		pub const ARRAYMUL:isize=20; 
		pub const ARRAYDIV:isize=21; 
		pub const ARRAYRDIV:isize=22; 
		pub const ARRAYPOW:isize=23; 
		pub const BREAK:isize=24; 
		pub const RETURN:isize=25; 
		pub const FUNCTION:isize=26; 
		pub const FOR:isize=27; 
		pub const WHILE:isize=28; 
		pub const END:isize=29; 
		pub const GLOBAL:isize=30; 
		pub const IF:isize=31; 
		pub const CLEAR:isize=32; 
		pub const ELSE:isize=33; 
		pub const ELSEIF:isize=34; 
		pub const LE_OP:isize=35; 
		pub const GE_OP:isize=36; 
		pub const EQ_OP:isize=37; 
		pub const NE_OP:isize=38; 
		pub const TRANSPOSE:isize=39; 
		pub const NCTRANSPOSE:isize=40; 
		pub const STRING_LITERAL:isize=41; 
		pub const IDENTIFIER:isize=42; 
		pub const CONSTANT:isize=43; 
		pub const CR:isize=44; 
		pub const WS:isize=45;
	pub const RULE_primary_expression:usize = 0; 
	pub const RULE_postfix_expression:usize = 1; 
	pub const RULE_index_expression:usize = 2; 
	pub const RULE_index_expression_list:usize = 3; 
	pub const RULE_array_expression:usize = 4; 
	pub const RULE_unary_expression:usize = 5; 
	pub const RULE_unary_operator:usize = 6; 
	pub const RULE_multiplicative_expression:usize = 7; 
	pub const RULE_additive_expression:usize = 8; 
	pub const RULE_relational_expression:usize = 9; 
	pub const RULE_equality_expression:usize = 10; 
	pub const RULE_and_expression:usize = 11; 
	pub const RULE_or_expression:usize = 12; 
	pub const RULE_expression:usize = 13; 
	pub const RULE_assignment_expression:usize = 14; 
	pub const RULE_eostmt:usize = 15; 
	pub const RULE_statement:usize = 16; 
	pub const RULE_statement_list:usize = 17; 
	pub const RULE_identifier_list:usize = 18; 
	pub const RULE_global_statement:usize = 19; 
	pub const RULE_clear_statement:usize = 20; 
	pub const RULE_expression_statement:usize = 21; 
	pub const RULE_assignment_statement:usize = 22; 
	pub const RULE_array_element:usize = 23; 
	pub const RULE_array_list:usize = 24; 
	pub const RULE_selection_statement:usize = 25; 
	pub const RULE_elseif_clause:usize = 26; 
	pub const RULE_iteration_statement:usize = 27; 
	pub const RULE_jump_statement:usize = 28; 
	pub const RULE_translation_unit:usize = 29; 
	pub const RULE_func_ident_list:usize = 30; 
	pub const RULE_func_return_list:usize = 31; 
	pub const RULE_function_declare_lhs:usize = 32; 
	pub const RULE_function_declare:usize = 33;
	pub const ruleNames: [&'static str; 34] =  [
		"primary_expression", "postfix_expression", "index_expression", "index_expression_list", 
		"array_expression", "unary_expression", "unary_operator", "multiplicative_expression", 
		"additive_expression", "relational_expression", "equality_expression", 
		"and_expression", "or_expression", "expression", "assignment_expression", 
		"eostmt", "statement", "statement_list", "identifier_list", "global_statement", 
		"clear_statement", "expression_statement", "assignment_statement", "array_element", 
		"array_list", "selection_statement", "elseif_clause", "iteration_statement", 
		"jump_statement", "translation_unit", "func_ident_list", "func_return_list", 
		"function_declare_lhs", "function_declare"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;41] = [
		None, Some("'('"), Some("')'"), Some("'['"), Some("']'"), Some("':'"), 
		Some("','"), Some("'+'"), Some("'-'"), Some("'~'"), Some("'*'"), Some("'/'"), 
		Some("'\\'"), Some("'^'"), Some("'<'"), Some("'>'"), Some("'&'"), Some("'|'"), 
		Some("'='"), Some("';'"), Some("'.*'"), Some("'.\\'"), Some("'./'"), Some("'.^'"), 
		Some("'break'"), Some("'return'"), Some("'function'"), Some("'for'"), 
		Some("'while'"), Some("'end'"), Some("'global'"), Some("'if'"), Some("'clear'"), 
		Some("'else'"), Some("'elseif'"), Some("'<='"), Some("'>='"), Some("'=='"), 
		Some("'~='"), Some("'transpose'"), Some("'.''")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;46]  = [
		None, None, None, None, None, None, None, None, None, None, None, None, 
		None, None, None, None, None, None, None, None, Some("ARRAYMUL"), Some("ARRAYDIV"), 
		Some("ARRAYRDIV"), Some("ARRAYPOW"), Some("BREAK"), Some("RETURN"), Some("FUNCTION"), 
		Some("FOR"), Some("WHILE"), Some("END"), Some("GLOBAL"), Some("IF"), Some("CLEAR"), 
		Some("ELSE"), Some("ELSEIF"), Some("LE_OP"), Some("GE_OP"), Some("EQ_OP"), 
		Some("NE_OP"), Some("TRANSPOSE"), Some("NCTRANSPOSE"), Some("STRING_LITERAL"), 
		Some("IDENTIFIER"), Some("CONSTANT"), Some("CR"), Some("WS")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,matlabParserExt, I, matlabParserContextType , dyn matlabListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type matlabTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, matlabParserContextType , dyn matlabListener<'input> + 'a>;

/// Parser for matlab grammar
pub struct matlabParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","2");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				matlabParserExt{
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> matlabParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> matlabParser<'input, I, DefaultErrorStrategy<'input,matlabParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for matlabParser
pub trait matlabParserContext<'input>:
	for<'x> Listenable<dyn matlabListener<'input> + 'x > + 
	for<'x> Visitable<dyn matlabVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=matlabParserContextType>
{}

impl<'input, 'x, T> VisitableDyn<T> for dyn matlabParserContext<'input> + 'input
where
    T: matlabVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn matlabVisitor<'input> + 'x))
    }
}

impl<'input> matlabParserContext<'input> for TerminalNode<'input,matlabParserContextType> {}
impl<'input> matlabParserContext<'input> for ErrorNode<'input,matlabParserContextType> {}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn matlabParserContext<'input> + 'input{}

#[antlr_rust::impl_tid]
impl<'input> antlr_rust::TidAble<'input> for dyn matlabListener<'input> + 'input{}

pub struct matlabParserContextType;
antlr_rust::type_id!{matlabParserContextType}

impl<'input> ParserNodeType<'input> for matlabParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn matlabParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct matlabParserExt{
}

impl matlabParserExt{
}


impl<'input> TokenAware<'input> for matlabParserExt{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for matlabParserExt{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for matlabParserExt{
	fn get_grammar_file_name(&self) -> & str{ "matlab.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn matlabParserContext<'input> + 'input)>, rule_index: isize, pred_index: isize,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					1 => matlabParser::<'input,I,_>::postfix_expression_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					3 => matlabParser::<'input,I,_>::index_expression_list_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					7 => matlabParser::<'input,I,_>::multiplicative_expression_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					8 => matlabParser::<'input,I,_>::additive_expression_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					9 => matlabParser::<'input,I,_>::relational_expression_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					10 => matlabParser::<'input,I,_>::equality_expression_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					11 => matlabParser::<'input,I,_>::and_expression_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					12 => matlabParser::<'input,I,_>::or_expression_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					13 => matlabParser::<'input,I,_>::expression_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					17 => matlabParser::<'input,I,_>::statement_list_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					18 => matlabParser::<'input,I,_>::identifier_list_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					24 => matlabParser::<'input,I,_>::array_list_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					26 => matlabParser::<'input,I,_>::elseif_clause_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					30 => matlabParser::<'input,I,_>::func_ident_list_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> matlabParser<'input, I, DefaultErrorStrategy<'input,matlabParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn postfix_expression_sempred(_localctx: Option<&Postfix_expressionContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 2)
				}
				1=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn index_expression_list_sempred(_localctx: Option<&Index_expression_listContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				2=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn multiplicative_expression_sempred(_localctx: Option<&Multiplicative_expressionContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				3=>{
					recog.precpred(None, 8)
				}
				4=>{
					recog.precpred(None, 7)
				}
				5=>{
					recog.precpred(None, 6)
				}
				6=>{
					recog.precpred(None, 5)
				}
				7=>{
					recog.precpred(None, 4)
				}
				8=>{
					recog.precpred(None, 3)
				}
				9=>{
					recog.precpred(None, 2)
				}
				10=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn additive_expression_sempred(_localctx: Option<&Additive_expressionContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				11=>{
					recog.precpred(None, 2)
				}
				12=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn relational_expression_sempred(_localctx: Option<&Relational_expressionContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				13=>{
					recog.precpred(None, 4)
				}
				14=>{
					recog.precpred(None, 3)
				}
				15=>{
					recog.precpred(None, 2)
				}
				16=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn equality_expression_sempred(_localctx: Option<&Equality_expressionContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				17=>{
					recog.precpred(None, 2)
				}
				18=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn and_expression_sempred(_localctx: Option<&And_expressionContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				19=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn or_expression_sempred(_localctx: Option<&Or_expressionContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				20=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn expression_sempred(_localctx: Option<&ExpressionContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				21=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn statement_list_sempred(_localctx: Option<&Statement_listContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				22=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn identifier_list_sempred(_localctx: Option<&Identifier_listContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				23=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn array_list_sempred(_localctx: Option<&Array_listContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				24=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn elseif_clause_sempred(_localctx: Option<&Elseif_clauseContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				25=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn func_ident_list_sempred(_localctx: Option<&Func_ident_listContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				26=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
}
//------------------- primary_expression ----------------
pub type Primary_expressionContextAll<'input> = Primary_expressionContext<'input>;


pub type Primary_expressionContext<'input> = BaseParserRuleContext<'input,Primary_expressionContextExt<'input>>;

#[derive(Clone)]
pub struct Primary_expressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for Primary_expressionContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for Primary_expressionContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_primary_expression(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_primary_expression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for Primary_expressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_primary_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for Primary_expressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_primary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_primary_expression }
}
antlr_rust::type_id!{Primary_expressionContextExt<'a>}

impl<'input> Primary_expressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Primary_expressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Primary_expressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Primary_expressionContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<Primary_expressionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
/// Retrieves first TerminalNode corresponding to token CONSTANT
/// Returns `None` if there is no child corresponding to token CONSTANT
fn CONSTANT(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(CONSTANT, 0)
}
/// Retrieves first TerminalNode corresponding to token STRING_LITERAL
/// Returns `None` if there is no child corresponding to token STRING_LITERAL
fn STRING_LITERAL(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(STRING_LITERAL, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn array_list(&self) -> Option<Rc<Array_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Primary_expressionContextAttrs<'input> for Primary_expressionContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn primary_expression(&mut self,)
	-> Result<Rc<Primary_expressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Primary_expressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_primary_expression);
        let mut _localctx: Rc<Primary_expressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(81);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(0,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(68);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(69);
					recog.base.match_token(CONSTANT,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(70);
					recog.base.match_token(STRING_LITERAL,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(71);
					recog.base.match_token(T__0,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(72);
					recog.expression_rec(0)?;

					recog.base.set_state(73);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					recog.base.set_state(75);
					recog.base.match_token(T__2,&mut recog.err_handler)?;

					recog.base.set_state(76);
					recog.base.match_token(T__3,&mut recog.err_handler)?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					recog.base.set_state(77);
					recog.base.match_token(T__2,&mut recog.err_handler)?;

					/*InvokeRule array_list*/
					recog.base.set_state(78);
					recog.array_list_rec(0)?;

					recog.base.set_state(79);
					recog.base.match_token(T__3,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- postfix_expression ----------------
pub type Postfix_expressionContextAll<'input> = Postfix_expressionContext<'input>;


pub type Postfix_expressionContext<'input> = BaseParserRuleContext<'input,Postfix_expressionContextExt<'input>>;

#[derive(Clone)]
pub struct Postfix_expressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for Postfix_expressionContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for Postfix_expressionContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_postfix_expression(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_postfix_expression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for Postfix_expressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_postfix_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for Postfix_expressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_postfix_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_postfix_expression }
}
antlr_rust::type_id!{Postfix_expressionContextExt<'a>}

impl<'input> Postfix_expressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Postfix_expressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Postfix_expressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Postfix_expressionContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<Postfix_expressionContextExt<'input>>{

fn primary_expression(&self) -> Option<Rc<Primary_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn array_expression(&self) -> Option<Rc<Array_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn postfix_expression(&self) -> Option<Rc<Postfix_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token TRANSPOSE
/// Returns `None` if there is no child corresponding to token TRANSPOSE
fn TRANSPOSE(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(TRANSPOSE, 0)
}
/// Retrieves first TerminalNode corresponding to token NCTRANSPOSE
/// Returns `None` if there is no child corresponding to token NCTRANSPOSE
fn NCTRANSPOSE(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(NCTRANSPOSE, 0)
}

}

impl<'input> Postfix_expressionContextAttrs<'input> for Postfix_expressionContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  postfix_expression(&mut self,)
	-> Result<Rc<Postfix_expressionContextAll<'input>>,ANTLRError> {
		self.postfix_expression_rec(0)
	}

	fn postfix_expression_rec(&mut self, _p: isize)
	-> Result<Rc<Postfix_expressionContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Postfix_expressionContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 2, RULE_postfix_expression, _p);
	    let mut _localctx: Rc<Postfix_expressionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 2;
		let result: Result<(), ANTLRError> = try {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(86);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(1,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule primary_expression*/
					recog.base.set_state(84);
					recog.primary_expression()?;

					}
				}
			,
				2 =>{
					{
					/*InvokeRule array_expression*/
					recog.base.set_state(85);
					recog.array_expression()?;

					}
				}

				_ => {}
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(94);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(3,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(92);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(2,&mut recog.base)? {
						1 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = Postfix_expressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_postfix_expression);
							_localctx = tmp;
							recog.base.set_state(88);
							if !({recog.precpred(None, 2)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
							}
							recog.base.set_state(89);
							recog.base.match_token(TRANSPOSE,&mut recog.err_handler)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = Postfix_expressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_postfix_expression);
							_localctx = tmp;
							recog.base.set_state(90);
							if !({recog.precpred(None, 1)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
							}
							recog.base.set_state(91);
							recog.base.match_token(NCTRANSPOSE,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(96);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(3,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- index_expression ----------------
pub type Index_expressionContextAll<'input> = Index_expressionContext<'input>;


pub type Index_expressionContext<'input> = BaseParserRuleContext<'input,Index_expressionContextExt<'input>>;

#[derive(Clone)]
pub struct Index_expressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for Index_expressionContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for Index_expressionContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_index_expression(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_index_expression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for Index_expressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_index_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for Index_expressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_index_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_index_expression }
}
antlr_rust::type_id!{Index_expressionContextExt<'a>}

impl<'input> Index_expressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Index_expressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Index_expressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Index_expressionContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<Index_expressionContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Index_expressionContextAttrs<'input> for Index_expressionContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn index_expression(&mut self,)
	-> Result<Rc<Index_expressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Index_expressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_index_expression);
        let mut _localctx: Rc<Index_expressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(99);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__4 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(97);
					recog.base.match_token(T__4,&mut recog.err_handler)?;

					}
				}

			 T__0 | T__2 | T__6 | T__7 | T__8 | STRING_LITERAL | IDENTIFIER | CONSTANT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule expression*/
					recog.base.set_state(98);
					recog.expression_rec(0)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- index_expression_list ----------------
pub type Index_expression_listContextAll<'input> = Index_expression_listContext<'input>;


pub type Index_expression_listContext<'input> = BaseParserRuleContext<'input,Index_expression_listContextExt<'input>>;

#[derive(Clone)]
pub struct Index_expression_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for Index_expression_listContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for Index_expression_listContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_index_expression_list(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_index_expression_list(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for Index_expression_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_index_expression_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Index_expression_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_index_expression_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_index_expression_list }
}
antlr_rust::type_id!{Index_expression_listContextExt<'a>}

impl<'input> Index_expression_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Index_expression_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Index_expression_listContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Index_expression_listContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<Index_expression_listContextExt<'input>>{

fn index_expression(&self) -> Option<Rc<Index_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn index_expression_list(&self) -> Option<Rc<Index_expression_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Index_expression_listContextAttrs<'input> for Index_expression_listContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  index_expression_list(&mut self,)
	-> Result<Rc<Index_expression_listContextAll<'input>>,ANTLRError> {
		self.index_expression_list_rec(0)
	}

	fn index_expression_list_rec(&mut self, _p: isize)
	-> Result<Rc<Index_expression_listContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Index_expression_listContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 6, RULE_index_expression_list, _p);
	    let mut _localctx: Rc<Index_expression_listContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 6;
		let result: Result<(), ANTLRError> = try {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule index_expression*/
			recog.base.set_state(102);
			recog.index_expression()?;

			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(109);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(5,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Index_expression_listContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_index_expression_list);
					_localctx = tmp;
					recog.base.set_state(104);
					if !({recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(105);
					recog.base.match_token(T__5,&mut recog.err_handler)?;

					/*InvokeRule index_expression*/
					recog.base.set_state(106);
					recog.index_expression()?;

					}
					} 
				}
				recog.base.set_state(111);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(5,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- array_expression ----------------
pub type Array_expressionContextAll<'input> = Array_expressionContext<'input>;


pub type Array_expressionContext<'input> = BaseParserRuleContext<'input,Array_expressionContextExt<'input>>;

#[derive(Clone)]
pub struct Array_expressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for Array_expressionContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for Array_expressionContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_array_expression(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_array_expression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for Array_expressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_array_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for Array_expressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_array_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_array_expression }
}
antlr_rust::type_id!{Array_expressionContextExt<'a>}

impl<'input> Array_expressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Array_expressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Array_expressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Array_expressionContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<Array_expressionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn index_expression_list(&self) -> Option<Rc<Index_expression_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Array_expressionContextAttrs<'input> for Array_expressionContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn array_expression(&mut self,)
	-> Result<Rc<Array_expressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Array_expressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_array_expression);
        let mut _localctx: Rc<Array_expressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(112);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			recog.base.set_state(113);
			recog.base.match_token(T__0,&mut recog.err_handler)?;

			/*InvokeRule index_expression_list*/
			recog.base.set_state(114);
			recog.index_expression_list_rec(0)?;

			recog.base.set_state(115);
			recog.base.match_token(T__1,&mut recog.err_handler)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- unary_expression ----------------
pub type Unary_expressionContextAll<'input> = Unary_expressionContext<'input>;


pub type Unary_expressionContext<'input> = BaseParserRuleContext<'input,Unary_expressionContextExt<'input>>;

#[derive(Clone)]
pub struct Unary_expressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for Unary_expressionContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for Unary_expressionContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_unary_expression(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_unary_expression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for Unary_expressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_unary_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for Unary_expressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unary_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unary_expression }
}
antlr_rust::type_id!{Unary_expressionContextExt<'a>}

impl<'input> Unary_expressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Unary_expressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Unary_expressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Unary_expressionContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<Unary_expressionContextExt<'input>>{

fn postfix_expression(&self) -> Option<Rc<Postfix_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn unary_operator(&self) -> Option<Rc<Unary_operatorContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Unary_expressionContextAttrs<'input> for Unary_expressionContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn unary_expression(&mut self,)
	-> Result<Rc<Unary_expressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Unary_expressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_unary_expression);
        let mut _localctx: Rc<Unary_expressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(121);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__0 | T__2 | STRING_LITERAL | IDENTIFIER | CONSTANT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule postfix_expression*/
					recog.base.set_state(117);
					recog.postfix_expression_rec(0)?;

					}
				}

			 T__6 | T__7 | T__8 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule unary_operator*/
					recog.base.set_state(118);
					recog.unary_operator()?;

					/*InvokeRule postfix_expression*/
					recog.base.set_state(119);
					recog.postfix_expression_rec(0)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- unary_operator ----------------
pub type Unary_operatorContextAll<'input> = Unary_operatorContext<'input>;


pub type Unary_operatorContext<'input> = BaseParserRuleContext<'input,Unary_operatorContextExt<'input>>;

#[derive(Clone)]
pub struct Unary_operatorContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for Unary_operatorContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for Unary_operatorContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_unary_operator(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_unary_operator(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for Unary_operatorContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_unary_operator(self);
	}
}

impl<'input> CustomRuleContext<'input> for Unary_operatorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unary_operator }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unary_operator }
}
antlr_rust::type_id!{Unary_operatorContextExt<'a>}

impl<'input> Unary_operatorContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Unary_operatorContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Unary_operatorContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Unary_operatorContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<Unary_operatorContextExt<'input>>{


}

impl<'input> Unary_operatorContextAttrs<'input> for Unary_operatorContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn unary_operator(&mut self,)
	-> Result<Rc<Unary_operatorContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Unary_operatorContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_unary_operator);
        let mut _localctx: Rc<Unary_operatorContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(123);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__6) | (1usize << T__7) | (1usize << T__8))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- multiplicative_expression ----------------
pub type Multiplicative_expressionContextAll<'input> = Multiplicative_expressionContext<'input>;


pub type Multiplicative_expressionContext<'input> = BaseParserRuleContext<'input,Multiplicative_expressionContextExt<'input>>;

#[derive(Clone)]
pub struct Multiplicative_expressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for Multiplicative_expressionContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for Multiplicative_expressionContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_multiplicative_expression(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_multiplicative_expression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for Multiplicative_expressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_multiplicative_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for Multiplicative_expressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_multiplicative_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_multiplicative_expression }
}
antlr_rust::type_id!{Multiplicative_expressionContextExt<'a>}

impl<'input> Multiplicative_expressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Multiplicative_expressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Multiplicative_expressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Multiplicative_expressionContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<Multiplicative_expressionContextExt<'input>>{

fn unary_expression(&self) -> Option<Rc<Unary_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn multiplicative_expression(&self) -> Option<Rc<Multiplicative_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ARRAYMUL
/// Returns `None` if there is no child corresponding to token ARRAYMUL
fn ARRAYMUL(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(ARRAYMUL, 0)
}
/// Retrieves first TerminalNode corresponding to token ARRAYDIV
/// Returns `None` if there is no child corresponding to token ARRAYDIV
fn ARRAYDIV(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(ARRAYDIV, 0)
}
/// Retrieves first TerminalNode corresponding to token ARRAYRDIV
/// Returns `None` if there is no child corresponding to token ARRAYRDIV
fn ARRAYRDIV(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(ARRAYRDIV, 0)
}
/// Retrieves first TerminalNode corresponding to token ARRAYPOW
/// Returns `None` if there is no child corresponding to token ARRAYPOW
fn ARRAYPOW(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(ARRAYPOW, 0)
}

}

impl<'input> Multiplicative_expressionContextAttrs<'input> for Multiplicative_expressionContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  multiplicative_expression(&mut self,)
	-> Result<Rc<Multiplicative_expressionContextAll<'input>>,ANTLRError> {
		self.multiplicative_expression_rec(0)
	}

	fn multiplicative_expression_rec(&mut self, _p: isize)
	-> Result<Rc<Multiplicative_expressionContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Multiplicative_expressionContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 14, RULE_multiplicative_expression, _p);
	    let mut _localctx: Rc<Multiplicative_expressionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 14;
		let result: Result<(), ANTLRError> = try {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule unary_expression*/
			recog.base.set_state(126);
			recog.unary_expression()?;

			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(154);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(8,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(152);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(7,&mut recog.base)? {
						1 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = Multiplicative_expressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_multiplicative_expression);
							_localctx = tmp;
							recog.base.set_state(128);
							if !({recog.precpred(None, 8)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 8)".to_owned()), None))?;
							}
							recog.base.set_state(129);
							recog.base.match_token(T__9,&mut recog.err_handler)?;

							/*InvokeRule unary_expression*/
							recog.base.set_state(130);
							recog.unary_expression()?;

							}
						}
					,
						2 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = Multiplicative_expressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_multiplicative_expression);
							_localctx = tmp;
							recog.base.set_state(131);
							if !({recog.precpred(None, 7)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 7)".to_owned()), None))?;
							}
							recog.base.set_state(132);
							recog.base.match_token(T__10,&mut recog.err_handler)?;

							/*InvokeRule unary_expression*/
							recog.base.set_state(133);
							recog.unary_expression()?;

							}
						}
					,
						3 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = Multiplicative_expressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_multiplicative_expression);
							_localctx = tmp;
							recog.base.set_state(134);
							if !({recog.precpred(None, 6)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 6)".to_owned()), None))?;
							}
							recog.base.set_state(135);
							recog.base.match_token(T__11,&mut recog.err_handler)?;

							/*InvokeRule unary_expression*/
							recog.base.set_state(136);
							recog.unary_expression()?;

							}
						}
					,
						4 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = Multiplicative_expressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_multiplicative_expression);
							_localctx = tmp;
							recog.base.set_state(137);
							if !({recog.precpred(None, 5)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 5)".to_owned()), None))?;
							}
							recog.base.set_state(138);
							recog.base.match_token(T__12,&mut recog.err_handler)?;

							/*InvokeRule unary_expression*/
							recog.base.set_state(139);
							recog.unary_expression()?;

							}
						}
					,
						5 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = Multiplicative_expressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_multiplicative_expression);
							_localctx = tmp;
							recog.base.set_state(140);
							if !({recog.precpred(None, 4)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
							}
							recog.base.set_state(141);
							recog.base.match_token(ARRAYMUL,&mut recog.err_handler)?;

							/*InvokeRule unary_expression*/
							recog.base.set_state(142);
							recog.unary_expression()?;

							}
						}
					,
						6 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = Multiplicative_expressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_multiplicative_expression);
							_localctx = tmp;
							recog.base.set_state(143);
							if !({recog.precpred(None, 3)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
							}
							recog.base.set_state(144);
							recog.base.match_token(ARRAYDIV,&mut recog.err_handler)?;

							/*InvokeRule unary_expression*/
							recog.base.set_state(145);
							recog.unary_expression()?;

							}
						}
					,
						7 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = Multiplicative_expressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_multiplicative_expression);
							_localctx = tmp;
							recog.base.set_state(146);
							if !({recog.precpred(None, 2)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
							}
							recog.base.set_state(147);
							recog.base.match_token(ARRAYRDIV,&mut recog.err_handler)?;

							/*InvokeRule unary_expression*/
							recog.base.set_state(148);
							recog.unary_expression()?;

							}
						}
					,
						8 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = Multiplicative_expressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_multiplicative_expression);
							_localctx = tmp;
							recog.base.set_state(149);
							if !({recog.precpred(None, 1)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
							}
							recog.base.set_state(150);
							recog.base.match_token(ARRAYPOW,&mut recog.err_handler)?;

							/*InvokeRule unary_expression*/
							recog.base.set_state(151);
							recog.unary_expression()?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(156);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(8,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- additive_expression ----------------
pub type Additive_expressionContextAll<'input> = Additive_expressionContext<'input>;


pub type Additive_expressionContext<'input> = BaseParserRuleContext<'input,Additive_expressionContextExt<'input>>;

#[derive(Clone)]
pub struct Additive_expressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for Additive_expressionContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for Additive_expressionContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_additive_expression(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_additive_expression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for Additive_expressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_additive_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for Additive_expressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_additive_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_additive_expression }
}
antlr_rust::type_id!{Additive_expressionContextExt<'a>}

impl<'input> Additive_expressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Additive_expressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Additive_expressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Additive_expressionContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<Additive_expressionContextExt<'input>>{

fn multiplicative_expression(&self) -> Option<Rc<Multiplicative_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn additive_expression(&self) -> Option<Rc<Additive_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Additive_expressionContextAttrs<'input> for Additive_expressionContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  additive_expression(&mut self,)
	-> Result<Rc<Additive_expressionContextAll<'input>>,ANTLRError> {
		self.additive_expression_rec(0)
	}

	fn additive_expression_rec(&mut self, _p: isize)
	-> Result<Rc<Additive_expressionContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Additive_expressionContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 16, RULE_additive_expression, _p);
	    let mut _localctx: Rc<Additive_expressionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 16;
		let result: Result<(), ANTLRError> = try {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule multiplicative_expression*/
			recog.base.set_state(158);
			recog.multiplicative_expression_rec(0)?;

			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(168);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(10,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(166);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(9,&mut recog.base)? {
						1 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = Additive_expressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_additive_expression);
							_localctx = tmp;
							recog.base.set_state(160);
							if !({recog.precpred(None, 2)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
							}
							recog.base.set_state(161);
							recog.base.match_token(T__6,&mut recog.err_handler)?;

							/*InvokeRule multiplicative_expression*/
							recog.base.set_state(162);
							recog.multiplicative_expression_rec(0)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = Additive_expressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_additive_expression);
							_localctx = tmp;
							recog.base.set_state(163);
							if !({recog.precpred(None, 1)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
							}
							recog.base.set_state(164);
							recog.base.match_token(T__7,&mut recog.err_handler)?;

							/*InvokeRule multiplicative_expression*/
							recog.base.set_state(165);
							recog.multiplicative_expression_rec(0)?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(170);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(10,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- relational_expression ----------------
pub type Relational_expressionContextAll<'input> = Relational_expressionContext<'input>;


pub type Relational_expressionContext<'input> = BaseParserRuleContext<'input,Relational_expressionContextExt<'input>>;

#[derive(Clone)]
pub struct Relational_expressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for Relational_expressionContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for Relational_expressionContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_relational_expression(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_relational_expression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for Relational_expressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_relational_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for Relational_expressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_relational_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_relational_expression }
}
antlr_rust::type_id!{Relational_expressionContextExt<'a>}

impl<'input> Relational_expressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Relational_expressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Relational_expressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Relational_expressionContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<Relational_expressionContextExt<'input>>{

fn additive_expression(&self) -> Option<Rc<Additive_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn relational_expression(&self) -> Option<Rc<Relational_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LE_OP
/// Returns `None` if there is no child corresponding to token LE_OP
fn LE_OP(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(LE_OP, 0)
}
/// Retrieves first TerminalNode corresponding to token GE_OP
/// Returns `None` if there is no child corresponding to token GE_OP
fn GE_OP(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(GE_OP, 0)
}

}

impl<'input> Relational_expressionContextAttrs<'input> for Relational_expressionContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  relational_expression(&mut self,)
	-> Result<Rc<Relational_expressionContextAll<'input>>,ANTLRError> {
		self.relational_expression_rec(0)
	}

	fn relational_expression_rec(&mut self, _p: isize)
	-> Result<Rc<Relational_expressionContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Relational_expressionContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 18, RULE_relational_expression, _p);
	    let mut _localctx: Rc<Relational_expressionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 18;
		let result: Result<(), ANTLRError> = try {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule additive_expression*/
			recog.base.set_state(172);
			recog.additive_expression_rec(0)?;

			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(188);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(12,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(186);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(11,&mut recog.base)? {
						1 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = Relational_expressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_relational_expression);
							_localctx = tmp;
							recog.base.set_state(174);
							if !({recog.precpred(None, 4)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
							}
							recog.base.set_state(175);
							recog.base.match_token(T__13,&mut recog.err_handler)?;

							/*InvokeRule additive_expression*/
							recog.base.set_state(176);
							recog.additive_expression_rec(0)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = Relational_expressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_relational_expression);
							_localctx = tmp;
							recog.base.set_state(177);
							if !({recog.precpred(None, 3)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
							}
							recog.base.set_state(178);
							recog.base.match_token(T__14,&mut recog.err_handler)?;

							/*InvokeRule additive_expression*/
							recog.base.set_state(179);
							recog.additive_expression_rec(0)?;

							}
						}
					,
						3 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = Relational_expressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_relational_expression);
							_localctx = tmp;
							recog.base.set_state(180);
							if !({recog.precpred(None, 2)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
							}
							recog.base.set_state(181);
							recog.base.match_token(LE_OP,&mut recog.err_handler)?;

							/*InvokeRule additive_expression*/
							recog.base.set_state(182);
							recog.additive_expression_rec(0)?;

							}
						}
					,
						4 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = Relational_expressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_relational_expression);
							_localctx = tmp;
							recog.base.set_state(183);
							if !({recog.precpred(None, 1)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
							}
							recog.base.set_state(184);
							recog.base.match_token(GE_OP,&mut recog.err_handler)?;

							/*InvokeRule additive_expression*/
							recog.base.set_state(185);
							recog.additive_expression_rec(0)?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(190);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(12,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- equality_expression ----------------
pub type Equality_expressionContextAll<'input> = Equality_expressionContext<'input>;


pub type Equality_expressionContext<'input> = BaseParserRuleContext<'input,Equality_expressionContextExt<'input>>;

#[derive(Clone)]
pub struct Equality_expressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for Equality_expressionContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for Equality_expressionContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_equality_expression(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_equality_expression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for Equality_expressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_equality_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for Equality_expressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_equality_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_equality_expression }
}
antlr_rust::type_id!{Equality_expressionContextExt<'a>}

impl<'input> Equality_expressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Equality_expressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Equality_expressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Equality_expressionContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<Equality_expressionContextExt<'input>>{

fn relational_expression(&self) -> Option<Rc<Relational_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn equality_expression(&self) -> Option<Rc<Equality_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token EQ_OP
/// Returns `None` if there is no child corresponding to token EQ_OP
fn EQ_OP(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(EQ_OP, 0)
}
/// Retrieves first TerminalNode corresponding to token NE_OP
/// Returns `None` if there is no child corresponding to token NE_OP
fn NE_OP(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(NE_OP, 0)
}

}

impl<'input> Equality_expressionContextAttrs<'input> for Equality_expressionContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  equality_expression(&mut self,)
	-> Result<Rc<Equality_expressionContextAll<'input>>,ANTLRError> {
		self.equality_expression_rec(0)
	}

	fn equality_expression_rec(&mut self, _p: isize)
	-> Result<Rc<Equality_expressionContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Equality_expressionContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 20, RULE_equality_expression, _p);
	    let mut _localctx: Rc<Equality_expressionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 20;
		let result: Result<(), ANTLRError> = try {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule relational_expression*/
			recog.base.set_state(192);
			recog.relational_expression_rec(0)?;

			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(202);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(14,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(200);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(13,&mut recog.base)? {
						1 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = Equality_expressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_equality_expression);
							_localctx = tmp;
							recog.base.set_state(194);
							if !({recog.precpred(None, 2)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
							}
							recog.base.set_state(195);
							recog.base.match_token(EQ_OP,&mut recog.err_handler)?;

							/*InvokeRule relational_expression*/
							recog.base.set_state(196);
							recog.relational_expression_rec(0)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleAltStartAction*/
							let mut tmp = Equality_expressionContextExt::new(_parentctx.clone(), _parentState);
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_equality_expression);
							_localctx = tmp;
							recog.base.set_state(197);
							if !({recog.precpred(None, 1)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
							}
							recog.base.set_state(198);
							recog.base.match_token(NE_OP,&mut recog.err_handler)?;

							/*InvokeRule relational_expression*/
							recog.base.set_state(199);
							recog.relational_expression_rec(0)?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(204);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(14,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- and_expression ----------------
pub type And_expressionContextAll<'input> = And_expressionContext<'input>;


pub type And_expressionContext<'input> = BaseParserRuleContext<'input,And_expressionContextExt<'input>>;

#[derive(Clone)]
pub struct And_expressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for And_expressionContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for And_expressionContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_and_expression(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_and_expression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for And_expressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_and_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for And_expressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_and_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_and_expression }
}
antlr_rust::type_id!{And_expressionContextExt<'a>}

impl<'input> And_expressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<And_expressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,And_expressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait And_expressionContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<And_expressionContextExt<'input>>{

fn equality_expression(&self) -> Option<Rc<Equality_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn and_expression(&self) -> Option<Rc<And_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> And_expressionContextAttrs<'input> for And_expressionContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  and_expression(&mut self,)
	-> Result<Rc<And_expressionContextAll<'input>>,ANTLRError> {
		self.and_expression_rec(0)
	}

	fn and_expression_rec(&mut self, _p: isize)
	-> Result<Rc<And_expressionContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = And_expressionContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 22, RULE_and_expression, _p);
	    let mut _localctx: Rc<And_expressionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 22;
		let result: Result<(), ANTLRError> = try {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule equality_expression*/
			recog.base.set_state(206);
			recog.equality_expression_rec(0)?;

			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(213);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(15,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = And_expressionContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_and_expression);
					_localctx = tmp;
					recog.base.set_state(208);
					if !({recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(209);
					recog.base.match_token(T__15,&mut recog.err_handler)?;

					/*InvokeRule equality_expression*/
					recog.base.set_state(210);
					recog.equality_expression_rec(0)?;

					}
					} 
				}
				recog.base.set_state(215);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(15,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- or_expression ----------------
pub type Or_expressionContextAll<'input> = Or_expressionContext<'input>;


pub type Or_expressionContext<'input> = BaseParserRuleContext<'input,Or_expressionContextExt<'input>>;

#[derive(Clone)]
pub struct Or_expressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for Or_expressionContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for Or_expressionContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_or_expression(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_or_expression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for Or_expressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_or_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for Or_expressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_or_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_or_expression }
}
antlr_rust::type_id!{Or_expressionContextExt<'a>}

impl<'input> Or_expressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Or_expressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Or_expressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Or_expressionContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<Or_expressionContextExt<'input>>{

fn and_expression(&self) -> Option<Rc<And_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn or_expression(&self) -> Option<Rc<Or_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Or_expressionContextAttrs<'input> for Or_expressionContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  or_expression(&mut self,)
	-> Result<Rc<Or_expressionContextAll<'input>>,ANTLRError> {
		self.or_expression_rec(0)
	}

	fn or_expression_rec(&mut self, _p: isize)
	-> Result<Rc<Or_expressionContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Or_expressionContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 24, RULE_or_expression, _p);
	    let mut _localctx: Rc<Or_expressionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 24;
		let result: Result<(), ANTLRError> = try {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule and_expression*/
			recog.base.set_state(217);
			recog.and_expression_rec(0)?;

			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(224);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(16,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Or_expressionContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_or_expression);
					_localctx = tmp;
					recog.base.set_state(219);
					if !({recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(220);
					recog.base.match_token(T__16,&mut recog.err_handler)?;

					/*InvokeRule and_expression*/
					recog.base.set_state(221);
					recog.and_expression_rec(0)?;

					}
					} 
				}
				recog.base.set_state(226);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(16,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- expression ----------------
pub type ExpressionContextAll<'input> = ExpressionContext<'input>;


pub type ExpressionContext<'input> = BaseParserRuleContext<'input,ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for ExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for ExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_expression(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_expression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for ExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr_rust::type_id!{ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExpressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait ExpressionContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<ExpressionContextExt<'input>>{

fn or_expression(&self) -> Option<Rc<Or_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  expression(&mut self,)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		self.expression_rec(0)
	}

	fn expression_rec(&mut self, _p: isize)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 26, RULE_expression, _p);
	    let mut _localctx: Rc<ExpressionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 26;
		let result: Result<(), ANTLRError> = try {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule or_expression*/
			recog.base.set_state(228);
			recog.or_expression_rec(0)?;

			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(235);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(17,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = ExpressionContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression);
					_localctx = tmp;
					recog.base.set_state(230);
					if !({recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(231);
					recog.base.match_token(T__4,&mut recog.err_handler)?;

					/*InvokeRule or_expression*/
					recog.base.set_state(232);
					recog.or_expression_rec(0)?;

					}
					} 
				}
				recog.base.set_state(237);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(17,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- assignment_expression ----------------
pub type Assignment_expressionContextAll<'input> = Assignment_expressionContext<'input>;


pub type Assignment_expressionContext<'input> = BaseParserRuleContext<'input,Assignment_expressionContextExt<'input>>;

#[derive(Clone)]
pub struct Assignment_expressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for Assignment_expressionContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for Assignment_expressionContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_assignment_expression(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_assignment_expression(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for Assignment_expressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_assignment_expression(self);
	}
}

impl<'input> CustomRuleContext<'input> for Assignment_expressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assignment_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assignment_expression }
}
antlr_rust::type_id!{Assignment_expressionContextExt<'a>}

impl<'input> Assignment_expressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Assignment_expressionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Assignment_expressionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Assignment_expressionContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<Assignment_expressionContextExt<'input>>{

fn postfix_expression(&self) -> Option<Rc<Postfix_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Assignment_expressionContextAttrs<'input> for Assignment_expressionContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assignment_expression(&mut self,)
	-> Result<Rc<Assignment_expressionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Assignment_expressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_assignment_expression);
        let mut _localctx: Rc<Assignment_expressionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule postfix_expression*/
			recog.base.set_state(238);
			recog.postfix_expression_rec(0)?;

			recog.base.set_state(239);
			recog.base.match_token(T__17,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(240);
			recog.expression_rec(0)?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- eostmt ----------------
pub type EostmtContextAll<'input> = EostmtContext<'input>;


pub type EostmtContext<'input> = BaseParserRuleContext<'input,EostmtContextExt<'input>>;

#[derive(Clone)]
pub struct EostmtContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for EostmtContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for EostmtContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_eostmt(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_eostmt(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for EostmtContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_eostmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for EostmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eostmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eostmt }
}
antlr_rust::type_id!{EostmtContextExt<'a>}

impl<'input> EostmtContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EostmtContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EostmtContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EostmtContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<EostmtContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CR
/// Returns `None` if there is no child corresponding to token CR
fn CR(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(CR, 0)
}

}

impl<'input> EostmtContextAttrs<'input> for EostmtContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn eostmt(&mut self,)
	-> Result<Rc<EostmtContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EostmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_eostmt);
        let mut _localctx: Rc<EostmtContextAll> = _localctx;
		let mut _la: isize;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(242);
			_la = recog.base.input.la(1);
			if { !((((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << T__5) | (1usize << T__18) | (1usize << CR))) != 0)) } {
				recog.err_handler.recover_inline(&mut recog.base)?;

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- statement ----------------
pub type StatementContextAll<'input> = StatementContext<'input>;


pub type StatementContext<'input> = BaseParserRuleContext<'input,StatementContextExt<'input>>;

#[derive(Clone)]
pub struct StatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for StatementContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for StatementContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_statement(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_statement(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for StatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_statement(self);
	}
}

impl<'input> CustomRuleContext<'input> for StatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}
antlr_rust::type_id!{StatementContextExt<'a>}

impl<'input> StatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StatementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StatementContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<StatementContextExt<'input>>{

fn global_statement(&self) -> Option<Rc<Global_statementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn clear_statement(&self) -> Option<Rc<Clear_statementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn assignment_statement(&self) -> Option<Rc<Assignment_statementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression_statement(&self) -> Option<Rc<Expression_statementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn selection_statement(&self) -> Option<Rc<Selection_statementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn iteration_statement(&self) -> Option<Rc<Iteration_statementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn jump_statement(&self) -> Option<Rc<Jump_statementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> StatementContextAttrs<'input> for StatementContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn statement(&mut self,)
	-> Result<Rc<StatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_statement);
        let mut _localctx: Rc<StatementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(251);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(18,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule global_statement*/
					recog.base.set_state(244);
					recog.global_statement()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule clear_statement*/
					recog.base.set_state(245);
					recog.clear_statement()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					/*InvokeRule assignment_statement*/
					recog.base.set_state(246);
					recog.assignment_statement()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					/*InvokeRule expression_statement*/
					recog.base.set_state(247);
					recog.expression_statement()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5);
					recog.base.enter_outer_alt(None, 5);
					{
					/*InvokeRule selection_statement*/
					recog.base.set_state(248);
					recog.selection_statement()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6);
					recog.base.enter_outer_alt(None, 6);
					{
					/*InvokeRule iteration_statement*/
					recog.base.set_state(249);
					recog.iteration_statement()?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7);
					recog.base.enter_outer_alt(None, 7);
					{
					/*InvokeRule jump_statement*/
					recog.base.set_state(250);
					recog.jump_statement()?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- statement_list ----------------
pub type Statement_listContextAll<'input> = Statement_listContext<'input>;


pub type Statement_listContext<'input> = BaseParserRuleContext<'input,Statement_listContextExt<'input>>;

#[derive(Clone)]
pub struct Statement_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for Statement_listContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for Statement_listContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_statement_list(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_statement_list(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for Statement_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_statement_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Statement_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement_list }
}
antlr_rust::type_id!{Statement_listContextExt<'a>}

impl<'input> Statement_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Statement_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Statement_listContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Statement_listContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<Statement_listContextExt<'input>>{

fn statement(&self) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn statement_list(&self) -> Option<Rc<Statement_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Statement_listContextAttrs<'input> for Statement_listContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  statement_list(&mut self,)
	-> Result<Rc<Statement_listContextAll<'input>>,ANTLRError> {
		self.statement_list_rec(0)
	}

	fn statement_list_rec(&mut self, _p: isize)
	-> Result<Rc<Statement_listContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Statement_listContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 34, RULE_statement_list, _p);
	    let mut _localctx: Rc<Statement_listContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 34;
		let result: Result<(), ANTLRError> = try {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule statement*/
			recog.base.set_state(254);
			recog.statement()?;

			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(260);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(19,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Statement_listContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_statement_list);
					_localctx = tmp;
					recog.base.set_state(256);
					if !({recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					/*InvokeRule statement*/
					recog.base.set_state(257);
					recog.statement()?;

					}
					} 
				}
				recog.base.set_state(262);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(19,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- identifier_list ----------------
pub type Identifier_listContextAll<'input> = Identifier_listContext<'input>;


pub type Identifier_listContext<'input> = BaseParserRuleContext<'input,Identifier_listContextExt<'input>>;

#[derive(Clone)]
pub struct Identifier_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for Identifier_listContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for Identifier_listContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_identifier_list(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_identifier_list(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for Identifier_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_identifier_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Identifier_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_identifier_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_identifier_list }
}
antlr_rust::type_id!{Identifier_listContextExt<'a>}

impl<'input> Identifier_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Identifier_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Identifier_listContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Identifier_listContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<Identifier_listContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn identifier_list(&self) -> Option<Rc<Identifier_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Identifier_listContextAttrs<'input> for Identifier_listContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  identifier_list(&mut self,)
	-> Result<Rc<Identifier_listContextAll<'input>>,ANTLRError> {
		self.identifier_list_rec(0)
	}

	fn identifier_list_rec(&mut self, _p: isize)
	-> Result<Rc<Identifier_listContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Identifier_listContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 36, RULE_identifier_list, _p);
	    let mut _localctx: Rc<Identifier_listContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 36;
		let result: Result<(), ANTLRError> = try {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			recog.base.set_state(264);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(270);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(20,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Identifier_listContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_identifier_list);
					_localctx = tmp;
					recog.base.set_state(266);
					if !({recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(267);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
					} 
				}
				recog.base.set_state(272);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(20,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- global_statement ----------------
pub type Global_statementContextAll<'input> = Global_statementContext<'input>;


pub type Global_statementContext<'input> = BaseParserRuleContext<'input,Global_statementContextExt<'input>>;

#[derive(Clone)]
pub struct Global_statementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for Global_statementContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for Global_statementContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_global_statement(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_global_statement(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for Global_statementContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_global_statement(self);
	}
}

impl<'input> CustomRuleContext<'input> for Global_statementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_global_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_global_statement }
}
antlr_rust::type_id!{Global_statementContextExt<'a>}

impl<'input> Global_statementContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Global_statementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Global_statementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Global_statementContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<Global_statementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token GLOBAL
/// Returns `None` if there is no child corresponding to token GLOBAL
fn GLOBAL(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(GLOBAL, 0)
}
fn identifier_list(&self) -> Option<Rc<Identifier_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn eostmt(&self) -> Option<Rc<EostmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Global_statementContextAttrs<'input> for Global_statementContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn global_statement(&mut self,)
	-> Result<Rc<Global_statementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Global_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_global_statement);
        let mut _localctx: Rc<Global_statementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(273);
			recog.base.match_token(GLOBAL,&mut recog.err_handler)?;

			/*InvokeRule identifier_list*/
			recog.base.set_state(274);
			recog.identifier_list_rec(0)?;

			/*InvokeRule eostmt*/
			recog.base.set_state(275);
			recog.eostmt()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- clear_statement ----------------
pub type Clear_statementContextAll<'input> = Clear_statementContext<'input>;


pub type Clear_statementContext<'input> = BaseParserRuleContext<'input,Clear_statementContextExt<'input>>;

#[derive(Clone)]
pub struct Clear_statementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for Clear_statementContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for Clear_statementContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_clear_statement(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_clear_statement(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for Clear_statementContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_clear_statement(self);
	}
}

impl<'input> CustomRuleContext<'input> for Clear_statementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_clear_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_clear_statement }
}
antlr_rust::type_id!{Clear_statementContextExt<'a>}

impl<'input> Clear_statementContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Clear_statementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Clear_statementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Clear_statementContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<Clear_statementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token CLEAR
/// Returns `None` if there is no child corresponding to token CLEAR
fn CLEAR(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(CLEAR, 0)
}
fn identifier_list(&self) -> Option<Rc<Identifier_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn eostmt(&self) -> Option<Rc<EostmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Clear_statementContextAttrs<'input> for Clear_statementContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn clear_statement(&mut self,)
	-> Result<Rc<Clear_statementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Clear_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_clear_statement);
        let mut _localctx: Rc<Clear_statementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(277);
			recog.base.match_token(CLEAR,&mut recog.err_handler)?;

			/*InvokeRule identifier_list*/
			recog.base.set_state(278);
			recog.identifier_list_rec(0)?;

			/*InvokeRule eostmt*/
			recog.base.set_state(279);
			recog.eostmt()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expression_statement ----------------
pub type Expression_statementContextAll<'input> = Expression_statementContext<'input>;


pub type Expression_statementContext<'input> = BaseParserRuleContext<'input,Expression_statementContextExt<'input>>;

#[derive(Clone)]
pub struct Expression_statementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for Expression_statementContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for Expression_statementContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_expression_statement(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_expression_statement(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for Expression_statementContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_expression_statement(self);
	}
}

impl<'input> CustomRuleContext<'input> for Expression_statementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression_statement }
}
antlr_rust::type_id!{Expression_statementContextExt<'a>}

impl<'input> Expression_statementContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Expression_statementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Expression_statementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Expression_statementContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<Expression_statementContextExt<'input>>{

fn eostmt(&self) -> Option<Rc<EostmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Expression_statementContextAttrs<'input> for Expression_statementContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn expression_statement(&mut self,)
	-> Result<Rc<Expression_statementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Expression_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_expression_statement);
        let mut _localctx: Rc<Expression_statementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(285);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__5 | T__18 | CR 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule eostmt*/
					recog.base.set_state(281);
					recog.eostmt()?;

					}
				}

			 T__0 | T__2 | T__6 | T__7 | T__8 | STRING_LITERAL | IDENTIFIER | CONSTANT 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule expression*/
					recog.base.set_state(282);
					recog.expression_rec(0)?;

					/*InvokeRule eostmt*/
					recog.base.set_state(283);
					recog.eostmt()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- assignment_statement ----------------
pub type Assignment_statementContextAll<'input> = Assignment_statementContext<'input>;


pub type Assignment_statementContext<'input> = BaseParserRuleContext<'input,Assignment_statementContextExt<'input>>;

#[derive(Clone)]
pub struct Assignment_statementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for Assignment_statementContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for Assignment_statementContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_assignment_statement(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_assignment_statement(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for Assignment_statementContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_assignment_statement(self);
	}
}

impl<'input> CustomRuleContext<'input> for Assignment_statementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assignment_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assignment_statement }
}
antlr_rust::type_id!{Assignment_statementContextExt<'a>}

impl<'input> Assignment_statementContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Assignment_statementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Assignment_statementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Assignment_statementContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<Assignment_statementContextExt<'input>>{

fn assignment_expression(&self) -> Option<Rc<Assignment_expressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn eostmt(&self) -> Option<Rc<EostmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Assignment_statementContextAttrs<'input> for Assignment_statementContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assignment_statement(&mut self,)
	-> Result<Rc<Assignment_statementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Assignment_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_assignment_statement);
        let mut _localctx: Rc<Assignment_statementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule assignment_expression*/
			recog.base.set_state(287);
			recog.assignment_expression()?;

			/*InvokeRule eostmt*/
			recog.base.set_state(288);
			recog.eostmt()?;

			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- array_element ----------------
pub type Array_elementContextAll<'input> = Array_elementContext<'input>;


pub type Array_elementContext<'input> = BaseParserRuleContext<'input,Array_elementContextExt<'input>>;

#[derive(Clone)]
pub struct Array_elementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for Array_elementContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for Array_elementContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_array_element(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_array_element(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for Array_elementContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_array_element(self);
	}
}

impl<'input> CustomRuleContext<'input> for Array_elementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_array_element }
	//fn type_rule_index() -> usize where Self: Sized { RULE_array_element }
}
antlr_rust::type_id!{Array_elementContextExt<'a>}

impl<'input> Array_elementContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Array_elementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Array_elementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Array_elementContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<Array_elementContextExt<'input>>{

fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression_statement(&self) -> Option<Rc<Expression_statementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Array_elementContextAttrs<'input> for Array_elementContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn array_element(&mut self,)
	-> Result<Rc<Array_elementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Array_elementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_array_element);
        let mut _localctx: Rc<Array_elementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(292);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(22,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule expression*/
					recog.base.set_state(290);
					recog.expression_rec(0)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule expression_statement*/
					recog.base.set_state(291);
					recog.expression_statement()?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- array_list ----------------
pub type Array_listContextAll<'input> = Array_listContext<'input>;


pub type Array_listContext<'input> = BaseParserRuleContext<'input,Array_listContextExt<'input>>;

#[derive(Clone)]
pub struct Array_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for Array_listContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for Array_listContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_array_list(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_array_list(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for Array_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_array_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Array_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_array_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_array_list }
}
antlr_rust::type_id!{Array_listContextExt<'a>}

impl<'input> Array_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Array_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Array_listContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Array_listContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<Array_listContextExt<'input>>{

fn array_element(&self) -> Option<Rc<Array_elementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn array_list(&self) -> Option<Rc<Array_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Array_listContextAttrs<'input> for Array_listContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  array_list(&mut self,)
	-> Result<Rc<Array_listContextAll<'input>>,ANTLRError> {
		self.array_list_rec(0)
	}

	fn array_list_rec(&mut self, _p: isize)
	-> Result<Rc<Array_listContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Array_listContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 48, RULE_array_list, _p);
	    let mut _localctx: Rc<Array_listContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 48;
		let result: Result<(), ANTLRError> = try {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			/*InvokeRule array_element*/
			recog.base.set_state(295);
			recog.array_element()?;

			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(301);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(23,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Array_listContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_array_list);
					_localctx = tmp;
					recog.base.set_state(297);
					if !({recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					/*InvokeRule array_element*/
					recog.base.set_state(298);
					recog.array_element()?;

					}
					} 
				}
				recog.base.set_state(303);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(23,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- selection_statement ----------------
pub type Selection_statementContextAll<'input> = Selection_statementContext<'input>;


pub type Selection_statementContext<'input> = BaseParserRuleContext<'input,Selection_statementContextExt<'input>>;

#[derive(Clone)]
pub struct Selection_statementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for Selection_statementContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for Selection_statementContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_selection_statement(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_selection_statement(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for Selection_statementContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_selection_statement(self);
	}
}

impl<'input> CustomRuleContext<'input> for Selection_statementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_selection_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_selection_statement }
}
antlr_rust::type_id!{Selection_statementContextExt<'a>}

impl<'input> Selection_statementContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Selection_statementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Selection_statementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Selection_statementContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<Selection_statementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IF
/// Returns `None` if there is no child corresponding to token IF
fn IF(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(IF, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn statement_list_all(&self) ->  Vec<Rc<Statement_listContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement_list(&self, i: usize) -> Option<Rc<Statement_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(END, 0)
}
fn eostmt(&self) -> Option<Rc<EostmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token ELSE
/// Returns `None` if there is no child corresponding to token ELSE
fn ELSE(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(ELSE, 0)
}
fn elseif_clause(&self) -> Option<Rc<Elseif_clauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Selection_statementContextAttrs<'input> for Selection_statementContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn selection_statement(&mut self,)
	-> Result<Rc<Selection_statementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Selection_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_selection_statement);
        let mut _localctx: Rc<Selection_statementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(334);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(24,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(304);
					recog.base.match_token(IF,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(305);
					recog.expression_rec(0)?;

					/*InvokeRule statement_list*/
					recog.base.set_state(306);
					recog.statement_list_rec(0)?;

					recog.base.set_state(307);
					recog.base.match_token(END,&mut recog.err_handler)?;

					/*InvokeRule eostmt*/
					recog.base.set_state(308);
					recog.eostmt()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(310);
					recog.base.match_token(IF,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(311);
					recog.expression_rec(0)?;

					/*InvokeRule statement_list*/
					recog.base.set_state(312);
					recog.statement_list_rec(0)?;

					recog.base.set_state(313);
					recog.base.match_token(ELSE,&mut recog.err_handler)?;

					/*InvokeRule statement_list*/
					recog.base.set_state(314);
					recog.statement_list_rec(0)?;

					recog.base.set_state(315);
					recog.base.match_token(END,&mut recog.err_handler)?;

					/*InvokeRule eostmt*/
					recog.base.set_state(316);
					recog.eostmt()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(318);
					recog.base.match_token(IF,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(319);
					recog.expression_rec(0)?;

					/*InvokeRule statement_list*/
					recog.base.set_state(320);
					recog.statement_list_rec(0)?;

					/*InvokeRule elseif_clause*/
					recog.base.set_state(321);
					recog.elseif_clause_rec(0)?;

					recog.base.set_state(322);
					recog.base.match_token(END,&mut recog.err_handler)?;

					/*InvokeRule eostmt*/
					recog.base.set_state(323);
					recog.eostmt()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4);
					recog.base.enter_outer_alt(None, 4);
					{
					recog.base.set_state(325);
					recog.base.match_token(IF,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(326);
					recog.expression_rec(0)?;

					/*InvokeRule statement_list*/
					recog.base.set_state(327);
					recog.statement_list_rec(0)?;

					/*InvokeRule elseif_clause*/
					recog.base.set_state(328);
					recog.elseif_clause_rec(0)?;

					recog.base.set_state(329);
					recog.base.match_token(ELSE,&mut recog.err_handler)?;

					/*InvokeRule statement_list*/
					recog.base.set_state(330);
					recog.statement_list_rec(0)?;

					recog.base.set_state(331);
					recog.base.match_token(END,&mut recog.err_handler)?;

					/*InvokeRule eostmt*/
					recog.base.set_state(332);
					recog.eostmt()?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- elseif_clause ----------------
pub type Elseif_clauseContextAll<'input> = Elseif_clauseContext<'input>;


pub type Elseif_clauseContext<'input> = BaseParserRuleContext<'input,Elseif_clauseContextExt<'input>>;

#[derive(Clone)]
pub struct Elseif_clauseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for Elseif_clauseContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for Elseif_clauseContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_elseif_clause(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_elseif_clause(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for Elseif_clauseContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_elseif_clause(self);
	}
}

impl<'input> CustomRuleContext<'input> for Elseif_clauseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_elseif_clause }
	//fn type_rule_index() -> usize where Self: Sized { RULE_elseif_clause }
}
antlr_rust::type_id!{Elseif_clauseContextExt<'a>}

impl<'input> Elseif_clauseContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Elseif_clauseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Elseif_clauseContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Elseif_clauseContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<Elseif_clauseContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ELSEIF
/// Returns `None` if there is no child corresponding to token ELSEIF
fn ELSEIF(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(ELSEIF, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn statement_list(&self) -> Option<Rc<Statement_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn elseif_clause(&self) -> Option<Rc<Elseif_clauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Elseif_clauseContextAttrs<'input> for Elseif_clauseContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  elseif_clause(&mut self,)
	-> Result<Rc<Elseif_clauseContextAll<'input>>,ANTLRError> {
		self.elseif_clause_rec(0)
	}

	fn elseif_clause_rec(&mut self, _p: isize)
	-> Result<Rc<Elseif_clauseContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Elseif_clauseContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 52, RULE_elseif_clause, _p);
	    let mut _localctx: Rc<Elseif_clauseContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 52;
		let result: Result<(), ANTLRError> = try {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			recog.base.set_state(337);
			recog.base.match_token(ELSEIF,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(338);
			recog.expression_rec(0)?;

			/*InvokeRule statement_list*/
			recog.base.set_state(339);
			recog.statement_list_rec(0)?;

			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(348);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(25,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Elseif_clauseContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_elseif_clause);
					_localctx = tmp;
					recog.base.set_state(341);
					if !({recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(342);
					recog.base.match_token(ELSEIF,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(343);
					recog.expression_rec(0)?;

					/*InvokeRule statement_list*/
					recog.base.set_state(344);
					recog.statement_list_rec(0)?;

					}
					} 
				}
				recog.base.set_state(350);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(25,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- iteration_statement ----------------
pub type Iteration_statementContextAll<'input> = Iteration_statementContext<'input>;


pub type Iteration_statementContext<'input> = BaseParserRuleContext<'input,Iteration_statementContextExt<'input>>;

#[derive(Clone)]
pub struct Iteration_statementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for Iteration_statementContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for Iteration_statementContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_iteration_statement(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_iteration_statement(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for Iteration_statementContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_iteration_statement(self);
	}
}

impl<'input> CustomRuleContext<'input> for Iteration_statementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_iteration_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_iteration_statement }
}
antlr_rust::type_id!{Iteration_statementContextExt<'a>}

impl<'input> Iteration_statementContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Iteration_statementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Iteration_statementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Iteration_statementContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<Iteration_statementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token WHILE
/// Returns `None` if there is no child corresponding to token WHILE
fn WHILE(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(WHILE, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn statement_list(&self) -> Option<Rc<Statement_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token END
/// Returns `None` if there is no child corresponding to token END
fn END(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(END, 0)
}
fn eostmt(&self) -> Option<Rc<EostmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token FOR
/// Returns `None` if there is no child corresponding to token FOR
fn FOR(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(FOR, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}

}

impl<'input> Iteration_statementContextAttrs<'input> for Iteration_statementContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn iteration_statement(&mut self,)
	-> Result<Rc<Iteration_statementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Iteration_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_iteration_statement);
        let mut _localctx: Rc<Iteration_statementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(375);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(26,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(351);
					recog.base.match_token(WHILE,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(352);
					recog.expression_rec(0)?;

					/*InvokeRule statement_list*/
					recog.base.set_state(353);
					recog.statement_list_rec(0)?;

					recog.base.set_state(354);
					recog.base.match_token(END,&mut recog.err_handler)?;

					/*InvokeRule eostmt*/
					recog.base.set_state(355);
					recog.eostmt()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(357);
					recog.base.match_token(FOR,&mut recog.err_handler)?;

					recog.base.set_state(358);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(359);
					recog.base.match_token(T__17,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(360);
					recog.expression_rec(0)?;

					/*InvokeRule statement_list*/
					recog.base.set_state(361);
					recog.statement_list_rec(0)?;

					recog.base.set_state(362);
					recog.base.match_token(END,&mut recog.err_handler)?;

					/*InvokeRule eostmt*/
					recog.base.set_state(363);
					recog.eostmt()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(365);
					recog.base.match_token(FOR,&mut recog.err_handler)?;

					recog.base.set_state(366);
					recog.base.match_token(T__0,&mut recog.err_handler)?;

					recog.base.set_state(367);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(368);
					recog.base.match_token(T__17,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(369);
					recog.expression_rec(0)?;

					recog.base.set_state(370);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					/*InvokeRule statement_list*/
					recog.base.set_state(371);
					recog.statement_list_rec(0)?;

					recog.base.set_state(372);
					recog.base.match_token(END,&mut recog.err_handler)?;

					/*InvokeRule eostmt*/
					recog.base.set_state(373);
					recog.eostmt()?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- jump_statement ----------------
pub type Jump_statementContextAll<'input> = Jump_statementContext<'input>;


pub type Jump_statementContext<'input> = BaseParserRuleContext<'input,Jump_statementContextExt<'input>>;

#[derive(Clone)]
pub struct Jump_statementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for Jump_statementContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for Jump_statementContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_jump_statement(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_jump_statement(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for Jump_statementContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_jump_statement(self);
	}
}

impl<'input> CustomRuleContext<'input> for Jump_statementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_jump_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_jump_statement }
}
antlr_rust::type_id!{Jump_statementContextExt<'a>}

impl<'input> Jump_statementContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Jump_statementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Jump_statementContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Jump_statementContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<Jump_statementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token BREAK
/// Returns `None` if there is no child corresponding to token BREAK
fn BREAK(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(BREAK, 0)
}
fn eostmt(&self) -> Option<Rc<EostmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token RETURN
/// Returns `None` if there is no child corresponding to token RETURN
fn RETURN(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(RETURN, 0)
}

}

impl<'input> Jump_statementContextAttrs<'input> for Jump_statementContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn jump_statement(&mut self,)
	-> Result<Rc<Jump_statementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Jump_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_jump_statement);
        let mut _localctx: Rc<Jump_statementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(381);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 BREAK 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(377);
					recog.base.match_token(BREAK,&mut recog.err_handler)?;

					/*InvokeRule eostmt*/
					recog.base.set_state(378);
					recog.eostmt()?;

					}
				}

			 RETURN 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(379);
					recog.base.match_token(RETURN,&mut recog.err_handler)?;

					/*InvokeRule eostmt*/
					recog.base.set_state(380);
					recog.eostmt()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- translation_unit ----------------
pub type Translation_unitContextAll<'input> = Translation_unitContext<'input>;


pub type Translation_unitContext<'input> = BaseParserRuleContext<'input,Translation_unitContextExt<'input>>;

#[derive(Clone)]
pub struct Translation_unitContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for Translation_unitContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for Translation_unitContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_translation_unit(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_translation_unit(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for Translation_unitContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_translation_unit(self);
	}
}

impl<'input> CustomRuleContext<'input> for Translation_unitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_translation_unit }
	//fn type_rule_index() -> usize where Self: Sized { RULE_translation_unit }
}
antlr_rust::type_id!{Translation_unitContextExt<'a>}

impl<'input> Translation_unitContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Translation_unitContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Translation_unitContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Translation_unitContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<Translation_unitContextExt<'input>>{

fn statement_list(&self) -> Option<Rc<Statement_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token FUNCTION
/// Returns `None` if there is no child corresponding to token FUNCTION
fn FUNCTION(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(FUNCTION, 0)
}
fn function_declare(&self) -> Option<Rc<Function_declareContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn eostmt(&self) -> Option<Rc<EostmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Translation_unitContextAttrs<'input> for Translation_unitContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn translation_unit(&mut self,)
	-> Result<Rc<Translation_unitContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Translation_unitContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_translation_unit);
        let mut _localctx: Rc<Translation_unitContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(389);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 T__0 | T__2 | T__5 | T__6 | T__7 | T__8 | T__18 | BREAK | RETURN | FOR |
			 WHILE | GLOBAL | IF | CLEAR | STRING_LITERAL | IDENTIFIER | CONSTANT |
			 CR 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule statement_list*/
					recog.base.set_state(383);
					recog.statement_list_rec(0)?;

					}
				}

			 FUNCTION 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(384);
					recog.base.match_token(FUNCTION,&mut recog.err_handler)?;

					/*InvokeRule function_declare*/
					recog.base.set_state(385);
					recog.function_declare()?;

					/*InvokeRule eostmt*/
					recog.base.set_state(386);
					recog.eostmt()?;

					/*InvokeRule statement_list*/
					recog.base.set_state(387);
					recog.statement_list_rec(0)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- func_ident_list ----------------
pub type Func_ident_listContextAll<'input> = Func_ident_listContext<'input>;


pub type Func_ident_listContext<'input> = BaseParserRuleContext<'input,Func_ident_listContextExt<'input>>;

#[derive(Clone)]
pub struct Func_ident_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for Func_ident_listContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for Func_ident_listContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_func_ident_list(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_func_ident_list(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for Func_ident_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_func_ident_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Func_ident_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_func_ident_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_func_ident_list }
}
antlr_rust::type_id!{Func_ident_listContextExt<'a>}

impl<'input> Func_ident_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Func_ident_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Func_ident_listContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Func_ident_listContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<Func_ident_listContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn func_ident_list(&self) -> Option<Rc<Func_ident_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Func_ident_listContextAttrs<'input> for Func_ident_listContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  func_ident_list(&mut self,)
	-> Result<Rc<Func_ident_listContextAll<'input>>,ANTLRError> {
		self.func_ident_list_rec(0)
	}

	fn func_ident_list_rec(&mut self, _p: isize)
	-> Result<Rc<Func_ident_listContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = Func_ident_listContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 60, RULE_func_ident_list, _p);
	    let mut _localctx: Rc<Func_ident_listContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 60;
		let result: Result<(), ANTLRError> = try {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			{
			recog.base.set_state(392);
			recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(399);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(29,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					{
					/*recRuleAltStartAction*/
					let mut tmp = Func_ident_listContextExt::new(_parentctx.clone(), _parentState);
					recog.push_new_recursion_context(tmp.clone(), _startState, RULE_func_ident_list);
					_localctx = tmp;
					recog.base.set_state(394);
					if !({recog.precpred(None, 1)}) {
						Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
					}
					recog.base.set_state(395);
					recog.base.match_token(T__5,&mut recog.err_handler)?;

					recog.base.set_state(396);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
					} 
				}
				recog.base.set_state(401);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(29,&mut recog.base)?;
			}
			}
		};
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- func_return_list ----------------
pub type Func_return_listContextAll<'input> = Func_return_listContext<'input>;


pub type Func_return_listContext<'input> = BaseParserRuleContext<'input,Func_return_listContextExt<'input>>;

#[derive(Clone)]
pub struct Func_return_listContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for Func_return_listContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for Func_return_listContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_func_return_list(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_func_return_list(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for Func_return_listContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_func_return_list(self);
	}
}

impl<'input> CustomRuleContext<'input> for Func_return_listContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_func_return_list }
	//fn type_rule_index() -> usize where Self: Sized { RULE_func_return_list }
}
antlr_rust::type_id!{Func_return_listContextExt<'a>}

impl<'input> Func_return_listContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Func_return_listContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Func_return_listContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Func_return_listContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<Func_return_listContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn func_ident_list(&self) -> Option<Rc<Func_ident_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Func_return_listContextAttrs<'input> for Func_return_listContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn func_return_list(&mut self,)
	-> Result<Rc<Func_return_listContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Func_return_listContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_func_return_list);
        let mut _localctx: Rc<Func_return_listContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(407);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IDENTIFIER 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(402);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}

			 T__2 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(403);
					recog.base.match_token(T__2,&mut recog.err_handler)?;

					/*InvokeRule func_ident_list*/
					recog.base.set_state(404);
					recog.func_ident_list_rec(0)?;

					recog.base.set_state(405);
					recog.base.match_token(T__3,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- function_declare_lhs ----------------
pub type Function_declare_lhsContextAll<'input> = Function_declare_lhsContext<'input>;


pub type Function_declare_lhsContext<'input> = BaseParserRuleContext<'input,Function_declare_lhsContextExt<'input>>;

#[derive(Clone)]
pub struct Function_declare_lhsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for Function_declare_lhsContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for Function_declare_lhsContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_function_declare_lhs(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_function_declare_lhs(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for Function_declare_lhsContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_function_declare_lhs(self);
	}
}

impl<'input> CustomRuleContext<'input> for Function_declare_lhsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_function_declare_lhs }
	//fn type_rule_index() -> usize where Self: Sized { RULE_function_declare_lhs }
}
antlr_rust::type_id!{Function_declare_lhsContextExt<'a>}

impl<'input> Function_declare_lhsContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Function_declare_lhsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Function_declare_lhsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Function_declare_lhsContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<Function_declare_lhsContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token IDENTIFIER
/// Returns `None` if there is no child corresponding to token IDENTIFIER
fn IDENTIFIER(&self) -> Option<Rc<TerminalNode<'input,matlabParserContextType>>> where Self:Sized{
	self.get_token(IDENTIFIER, 0)
}
fn func_ident_list(&self) -> Option<Rc<Func_ident_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Function_declare_lhsContextAttrs<'input> for Function_declare_lhsContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn function_declare_lhs(&mut self,)
	-> Result<Rc<Function_declare_lhsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Function_declare_lhsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_function_declare_lhs);
        let mut _localctx: Rc<Function_declare_lhsContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(418);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(31,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(409);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(410);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(411);
					recog.base.match_token(T__0,&mut recog.err_handler)?;

					recog.base.set_state(412);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3);
					recog.base.enter_outer_alt(None, 3);
					{
					recog.base.set_state(413);
					recog.base.match_token(IDENTIFIER,&mut recog.err_handler)?;

					recog.base.set_state(414);
					recog.base.match_token(T__0,&mut recog.err_handler)?;

					/*InvokeRule func_ident_list*/
					recog.base.set_state(415);
					recog.func_ident_list_rec(0)?;

					recog.base.set_state(416);
					recog.base.match_token(T__1,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- function_declare ----------------
pub type Function_declareContextAll<'input> = Function_declareContext<'input>;


pub type Function_declareContext<'input> = BaseParserRuleContext<'input,Function_declareContextExt<'input>>;

#[derive(Clone)]
pub struct Function_declareContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> matlabParserContext<'input> for Function_declareContext<'input>{}

impl<'input,'a> Listenable<dyn matlabListener<'input> + 'a> for Function_declareContext<'input>{
	fn enter(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_function_declare(self);
	}
	fn exit(&self,listener: &mut (dyn matlabListener<'input> + 'a)) {
		listener.exit_function_declare(self);
		listener.exit_every_rule(self);
	}
}

impl<'input,'a> Visitable<dyn matlabVisitor<'input> + 'a> for Function_declareContext<'input>{
	fn accept(&self,visitor: &mut (dyn matlabVisitor<'input> + 'a)) {
		visitor.visit_function_declare(self);
	}
}

impl<'input> CustomRuleContext<'input> for Function_declareContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = matlabParserContextType;
	fn get_rule_index(&self) -> usize { RULE_function_declare }
	//fn type_rule_index() -> usize where Self: Sized { RULE_function_declare }
}
antlr_rust::type_id!{Function_declareContextExt<'a>}

impl<'input> Function_declareContextExt<'input>{
	fn new(parent: Option<Rc<dyn matlabParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<Function_declareContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Function_declareContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait Function_declareContextAttrs<'input>: matlabParserContext<'input> + BorrowMut<Function_declareContextExt<'input>>{

fn function_declare_lhs(&self) -> Option<Rc<Function_declare_lhsContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn func_return_list(&self) -> Option<Rc<Func_return_listContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> Function_declareContextAttrs<'input> for Function_declareContext<'input>{}

impl<'input, I, H> matlabParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn function_declare(&mut self,)
	-> Result<Rc<Function_declareContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Function_declareContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_function_declare);
        let mut _localctx: Rc<Function_declareContextAll> = _localctx;
		let result: Result<(), ANTLRError> = try {

			recog.base.set_state(425);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(32,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					/*InvokeRule function_declare_lhs*/
					recog.base.set_state(420);
					recog.function_declare_lhs()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					/*InvokeRule func_return_list*/
					recog.base.set_state(421);
					recog.func_return_list()?;

					recog.base.set_state(422);
					recog.base.match_token(T__17,&mut recog.err_handler)?;

					/*InvokeRule function_declare_lhs*/
					recog.base.set_state(423);
					recog.function_declare_lhs()?;

					}
				}

				_ => {}
			}
		};
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x2f\u{1ae}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\x20\x09\
	\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x03\x02\x03\x02\x03\
	\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\x02\x03\
	\x02\x03\x02\x05\x02\x54\x0a\x02\x03\x03\x03\x03\x03\x03\x05\x03\x59\x0a\
	\x03\x03\x03\x03\x03\x03\x03\x03\x03\x07\x03\x5f\x0a\x03\x0c\x03\x0e\x03\
	\x62\x0b\x03\x03\x04\x03\x04\x05\x04\x66\x0a\x04\x03\x05\x03\x05\x03\x05\
	\x03\x05\x03\x05\x03\x05\x07\x05\x6e\x0a\x05\x0c\x05\x0e\x05\x71\x0b\x05\
	\x03\x06\x03\x06\x03\x06\x03\x06\x03\x06\x03\x07\x03\x07\x03\x07\x03\x07\
	\x05\x07\x7c\x0a\x07\x03\x08\x03\x08\x03\x09\x03\x09\x03\x09\x03\x09\x03\
	\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\
	\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\x09\x03\
	\x09\x03\x09\x03\x09\x03\x09\x03\x09\x07\x09\u{9b}\x0a\x09\x0c\x09\x0e\x09\
	\u{9e}\x0b\x09\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\
	\x0a\x03\x0a\x07\x0a\u{a9}\x0a\x0a\x0c\x0a\x0e\x0a\u{ac}\x0b\x0a\x03\x0b\
	\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\
	\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x07\x0b\u{bd}\x0a\x0b\x0c\x0b\x0e\
	\x0b\u{c0}\x0b\x0b\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\
	\x03\x0c\x03\x0c\x07\x0c\u{cb}\x0a\x0c\x0c\x0c\x0e\x0c\u{ce}\x0b\x0c\x03\
	\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x07\x0d\u{d6}\x0a\x0d\x0c\x0d\
	\x0e\x0d\u{d9}\x0b\x0d\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x07\
	\x0e\u{e1}\x0a\x0e\x0c\x0e\x0e\x0e\u{e4}\x0b\x0e\x03\x0f\x03\x0f\x03\x0f\
	\x03\x0f\x03\x0f\x03\x0f\x07\x0f\u{ec}\x0a\x0f\x0c\x0f\x0e\x0f\u{ef}\x0b\
	\x0f\x03\x10\x03\x10\x03\x10\x03\x10\x03\x11\x03\x11\x03\x12\x03\x12\x03\
	\x12\x03\x12\x03\x12\x03\x12\x03\x12\x05\x12\u{fe}\x0a\x12\x03\x13\x03\x13\
	\x03\x13\x03\x13\x03\x13\x07\x13\u{105}\x0a\x13\x0c\x13\x0e\x13\u{108}\x0b\
	\x13\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x07\x14\u{10f}\x0a\x14\x0c\
	\x14\x0e\x14\u{112}\x0b\x14\x03\x15\x03\x15\x03\x15\x03\x15\x03\x16\x03\
	\x16\x03\x16\x03\x16\x03\x17\x03\x17\x03\x17\x03\x17\x05\x17\u{120}\x0a\
	\x17\x03\x18\x03\x18\x03\x18\x03\x19\x03\x19\x05\x19\u{127}\x0a\x19\x03\
	\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x07\x1a\u{12e}\x0a\x1a\x0c\x1a\x0e\
	\x1a\u{131}\x0b\x1a\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\
	\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\
	\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\
	\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x05\x1b\u{151}\x0a\x1b\x03\
	\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x03\
	\x1c\x07\x1c\u{15d}\x0a\x1c\x0c\x1c\x0e\x1c\u{160}\x0b\x1c\x03\x1d\x03\x1d\
	\x03\x1d\x03\x1d\x03\x1d\x03\x1d\x03\x1d\x03\x1d\x03\x1d\x03\x1d\x03\x1d\
	\x03\x1d\x03\x1d\x03\x1d\x03\x1d\x03\x1d\x03\x1d\x03\x1d\x03\x1d\x03\x1d\
	\x03\x1d\x03\x1d\x03\x1d\x03\x1d\x05\x1d\u{17a}\x0a\x1d\x03\x1e\x03\x1e\
	\x03\x1e\x03\x1e\x05\x1e\u{180}\x0a\x1e\x03\x1f\x03\x1f\x03\x1f\x03\x1f\
	\x03\x1f\x03\x1f\x05\x1f\u{188}\x0a\x1f\x03\x20\x03\x20\x03\x20\x03\x20\
	\x03\x20\x03\x20\x07\x20\u{190}\x0a\x20\x0c\x20\x0e\x20\u{193}\x0b\x20\x03\
	\x21\x03\x21\x03\x21\x03\x21\x03\x21\x05\x21\u{19a}\x0a\x21\x03\x22\x03\
	\x22\x03\x22\x03\x22\x03\x22\x03\x22\x03\x22\x03\x22\x03\x22\x05\x22\u{1a5}\
	\x0a\x22\x03\x23\x03\x23\x03\x23\x03\x23\x03\x23\x05\x23\u{1ac}\x0a\x23\
	\x03\x23\x02\x10\x04\x08\x10\x12\x14\x16\x18\x1a\x1c\x24\x26\x32\x36\x3e\
	\x24\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\
	\x24\x26\x28\x2a\x2c\x2e\x30\x32\x34\x36\x38\x3a\x3c\x3e\x40\x42\x44\x02\
	\x04\x03\x02\x09\x0b\x05\x02\x08\x08\x15\x15\x2e\x2e\x02\u{1c1}\x02\x53\
	\x03\x02\x02\x02\x04\x58\x03\x02\x02\x02\x06\x65\x03\x02\x02\x02\x08\x67\
	\x03\x02\x02\x02\x0a\x72\x03\x02\x02\x02\x0c\x7b\x03\x02\x02\x02\x0e\x7d\
	\x03\x02\x02\x02\x10\x7f\x03\x02\x02\x02\x12\u{9f}\x03\x02\x02\x02\x14\u{ad}\
	\x03\x02\x02\x02\x16\u{c1}\x03\x02\x02\x02\x18\u{cf}\x03\x02\x02\x02\x1a\
	\u{da}\x03\x02\x02\x02\x1c\u{e5}\x03\x02\x02\x02\x1e\u{f0}\x03\x02\x02\x02\
	\x20\u{f4}\x03\x02\x02\x02\x22\u{fd}\x03\x02\x02\x02\x24\u{ff}\x03\x02\x02\
	\x02\x26\u{109}\x03\x02\x02\x02\x28\u{113}\x03\x02\x02\x02\x2a\u{117}\x03\
	\x02\x02\x02\x2c\u{11f}\x03\x02\x02\x02\x2e\u{121}\x03\x02\x02\x02\x30\u{126}\
	\x03\x02\x02\x02\x32\u{128}\x03\x02\x02\x02\x34\u{150}\x03\x02\x02\x02\x36\
	\u{152}\x03\x02\x02\x02\x38\u{179}\x03\x02\x02\x02\x3a\u{17f}\x03\x02\x02\
	\x02\x3c\u{187}\x03\x02\x02\x02\x3e\u{189}\x03\x02\x02\x02\x40\u{199}\x03\
	\x02\x02\x02\x42\u{1a4}\x03\x02\x02\x02\x44\u{1ab}\x03\x02\x02\x02\x46\x54\
	\x07\x2c\x02\x02\x47\x54\x07\x2d\x02\x02\x48\x54\x07\x2b\x02\x02\x49\x4a\
	\x07\x03\x02\x02\x4a\x4b\x05\x1c\x0f\x02\x4b\x4c\x07\x04\x02\x02\x4c\x54\
	\x03\x02\x02\x02\x4d\x4e\x07\x05\x02\x02\x4e\x54\x07\x06\x02\x02\x4f\x50\
	\x07\x05\x02\x02\x50\x51\x05\x32\x1a\x02\x51\x52\x07\x06\x02\x02\x52\x54\
	\x03\x02\x02\x02\x53\x46\x03\x02\x02\x02\x53\x47\x03\x02\x02\x02\x53\x48\
	\x03\x02\x02\x02\x53\x49\x03\x02\x02\x02\x53\x4d\x03\x02\x02\x02\x53\x4f\
	\x03\x02\x02\x02\x54\x03\x03\x02\x02\x02\x55\x56\x08\x03\x01\x02\x56\x59\
	\x05\x02\x02\x02\x57\x59\x05\x0a\x06\x02\x58\x55\x03\x02\x02\x02\x58\x57\
	\x03\x02\x02\x02\x59\x60\x03\x02\x02\x02\x5a\x5b\x0c\x04\x02\x02\x5b\x5f\
	\x07\x29\x02\x02\x5c\x5d\x0c\x03\x02\x02\x5d\x5f\x07\x2a\x02\x02\x5e\x5a\
	\x03\x02\x02\x02\x5e\x5c\x03\x02\x02\x02\x5f\x62\x03\x02\x02\x02\x60\x5e\
	\x03\x02\x02\x02\x60\x61\x03\x02\x02\x02\x61\x05\x03\x02\x02\x02\x62\x60\
	\x03\x02\x02\x02\x63\x66\x07\x07\x02\x02\x64\x66\x05\x1c\x0f\x02\x65\x63\
	\x03\x02\x02\x02\x65\x64\x03\x02\x02\x02\x66\x07\x03\x02\x02\x02\x67\x68\
	\x08\x05\x01\x02\x68\x69\x05\x06\x04\x02\x69\x6f\x03\x02\x02\x02\x6a\x6b\
	\x0c\x03\x02\x02\x6b\x6c\x07\x08\x02\x02\x6c\x6e\x05\x06\x04\x02\x6d\x6a\
	\x03\x02\x02\x02\x6e\x71\x03\x02\x02\x02\x6f\x6d\x03\x02\x02\x02\x6f\x70\
	\x03\x02\x02\x02\x70\x09\x03\x02\x02\x02\x71\x6f\x03\x02\x02\x02\x72\x73\
	\x07\x2c\x02\x02\x73\x74\x07\x03\x02\x02\x74\x75\x05\x08\x05\x02\x75\x76\
	\x07\x04\x02\x02\x76\x0b\x03\x02\x02\x02\x77\x7c\x05\x04\x03\x02\x78\x79\
	\x05\x0e\x08\x02\x79\x7a\x05\x04\x03\x02\x7a\x7c\x03\x02\x02\x02\x7b\x77\
	\x03\x02\x02\x02\x7b\x78\x03\x02\x02\x02\x7c\x0d\x03\x02\x02\x02\x7d\x7e\
	\x09\x02\x02\x02\x7e\x0f\x03\x02\x02\x02\x7f\u{80}\x08\x09\x01\x02\u{80}\
	\u{81}\x05\x0c\x07\x02\u{81}\u{9c}\x03\x02\x02\x02\u{82}\u{83}\x0c\x0a\x02\
	\x02\u{83}\u{84}\x07\x0c\x02\x02\u{84}\u{9b}\x05\x0c\x07\x02\u{85}\u{86}\
	\x0c\x09\x02\x02\u{86}\u{87}\x07\x0d\x02\x02\u{87}\u{9b}\x05\x0c\x07\x02\
	\u{88}\u{89}\x0c\x08\x02\x02\u{89}\u{8a}\x07\x0e\x02\x02\u{8a}\u{9b}\x05\
	\x0c\x07\x02\u{8b}\u{8c}\x0c\x07\x02\x02\u{8c}\u{8d}\x07\x0f\x02\x02\u{8d}\
	\u{9b}\x05\x0c\x07\x02\u{8e}\u{8f}\x0c\x06\x02\x02\u{8f}\u{90}\x07\x16\x02\
	\x02\u{90}\u{9b}\x05\x0c\x07\x02\u{91}\u{92}\x0c\x05\x02\x02\u{92}\u{93}\
	\x07\x17\x02\x02\u{93}\u{9b}\x05\x0c\x07\x02\u{94}\u{95}\x0c\x04\x02\x02\
	\u{95}\u{96}\x07\x18\x02\x02\u{96}\u{9b}\x05\x0c\x07\x02\u{97}\u{98}\x0c\
	\x03\x02\x02\u{98}\u{99}\x07\x19\x02\x02\u{99}\u{9b}\x05\x0c\x07\x02\u{9a}\
	\u{82}\x03\x02\x02\x02\u{9a}\u{85}\x03\x02\x02\x02\u{9a}\u{88}\x03\x02\x02\
	\x02\u{9a}\u{8b}\x03\x02\x02\x02\u{9a}\u{8e}\x03\x02\x02\x02\u{9a}\u{91}\
	\x03\x02\x02\x02\u{9a}\u{94}\x03\x02\x02\x02\u{9a}\u{97}\x03\x02\x02\x02\
	\u{9b}\u{9e}\x03\x02\x02\x02\u{9c}\u{9a}\x03\x02\x02\x02\u{9c}\u{9d}\x03\
	\x02\x02\x02\u{9d}\x11\x03\x02\x02\x02\u{9e}\u{9c}\x03\x02\x02\x02\u{9f}\
	\u{a0}\x08\x0a\x01\x02\u{a0}\u{a1}\x05\x10\x09\x02\u{a1}\u{aa}\x03\x02\x02\
	\x02\u{a2}\u{a3}\x0c\x04\x02\x02\u{a3}\u{a4}\x07\x09\x02\x02\u{a4}\u{a9}\
	\x05\x10\x09\x02\u{a5}\u{a6}\x0c\x03\x02\x02\u{a6}\u{a7}\x07\x0a\x02\x02\
	\u{a7}\u{a9}\x05\x10\x09\x02\u{a8}\u{a2}\x03\x02\x02\x02\u{a8}\u{a5}\x03\
	\x02\x02\x02\u{a9}\u{ac}\x03\x02\x02\x02\u{aa}\u{a8}\x03\x02\x02\x02\u{aa}\
	\u{ab}\x03\x02\x02\x02\u{ab}\x13\x03\x02\x02\x02\u{ac}\u{aa}\x03\x02\x02\
	\x02\u{ad}\u{ae}\x08\x0b\x01\x02\u{ae}\u{af}\x05\x12\x0a\x02\u{af}\u{be}\
	\x03\x02\x02\x02\u{b0}\u{b1}\x0c\x06\x02\x02\u{b1}\u{b2}\x07\x10\x02\x02\
	\u{b2}\u{bd}\x05\x12\x0a\x02\u{b3}\u{b4}\x0c\x05\x02\x02\u{b4}\u{b5}\x07\
	\x11\x02\x02\u{b5}\u{bd}\x05\x12\x0a\x02\u{b6}\u{b7}\x0c\x04\x02\x02\u{b7}\
	\u{b8}\x07\x25\x02\x02\u{b8}\u{bd}\x05\x12\x0a\x02\u{b9}\u{ba}\x0c\x03\x02\
	\x02\u{ba}\u{bb}\x07\x26\x02\x02\u{bb}\u{bd}\x05\x12\x0a\x02\u{bc}\u{b0}\
	\x03\x02\x02\x02\u{bc}\u{b3}\x03\x02\x02\x02\u{bc}\u{b6}\x03\x02\x02\x02\
	\u{bc}\u{b9}\x03\x02\x02\x02\u{bd}\u{c0}\x03\x02\x02\x02\u{be}\u{bc}\x03\
	\x02\x02\x02\u{be}\u{bf}\x03\x02\x02\x02\u{bf}\x15\x03\x02\x02\x02\u{c0}\
	\u{be}\x03\x02\x02\x02\u{c1}\u{c2}\x08\x0c\x01\x02\u{c2}\u{c3}\x05\x14\x0b\
	\x02\u{c3}\u{cc}\x03\x02\x02\x02\u{c4}\u{c5}\x0c\x04\x02\x02\u{c5}\u{c6}\
	\x07\x27\x02\x02\u{c6}\u{cb}\x05\x14\x0b\x02\u{c7}\u{c8}\x0c\x03\x02\x02\
	\u{c8}\u{c9}\x07\x28\x02\x02\u{c9}\u{cb}\x05\x14\x0b\x02\u{ca}\u{c4}\x03\
	\x02\x02\x02\u{ca}\u{c7}\x03\x02\x02\x02\u{cb}\u{ce}\x03\x02\x02\x02\u{cc}\
	\u{ca}\x03\x02\x02\x02\u{cc}\u{cd}\x03\x02\x02\x02\u{cd}\x17\x03\x02\x02\
	\x02\u{ce}\u{cc}\x03\x02\x02\x02\u{cf}\u{d0}\x08\x0d\x01\x02\u{d0}\u{d1}\
	\x05\x16\x0c\x02\u{d1}\u{d7}\x03\x02\x02\x02\u{d2}\u{d3}\x0c\x03\x02\x02\
	\u{d3}\u{d4}\x07\x12\x02\x02\u{d4}\u{d6}\x05\x16\x0c\x02\u{d5}\u{d2}\x03\
	\x02\x02\x02\u{d6}\u{d9}\x03\x02\x02\x02\u{d7}\u{d5}\x03\x02\x02\x02\u{d7}\
	\u{d8}\x03\x02\x02\x02\u{d8}\x19\x03\x02\x02\x02\u{d9}\u{d7}\x03\x02\x02\
	\x02\u{da}\u{db}\x08\x0e\x01\x02\u{db}\u{dc}\x05\x18\x0d\x02\u{dc}\u{e2}\
	\x03\x02\x02\x02\u{dd}\u{de}\x0c\x03\x02\x02\u{de}\u{df}\x07\x13\x02\x02\
	\u{df}\u{e1}\x05\x18\x0d\x02\u{e0}\u{dd}\x03\x02\x02\x02\u{e1}\u{e4}\x03\
	\x02\x02\x02\u{e2}\u{e0}\x03\x02\x02\x02\u{e2}\u{e3}\x03\x02\x02\x02\u{e3}\
	\x1b\x03\x02\x02\x02\u{e4}\u{e2}\x03\x02\x02\x02\u{e5}\u{e6}\x08\x0f\x01\
	\x02\u{e6}\u{e7}\x05\x1a\x0e\x02\u{e7}\u{ed}\x03\x02\x02\x02\u{e8}\u{e9}\
	\x0c\x03\x02\x02\u{e9}\u{ea}\x07\x07\x02\x02\u{ea}\u{ec}\x05\x1a\x0e\x02\
	\u{eb}\u{e8}\x03\x02\x02\x02\u{ec}\u{ef}\x03\x02\x02\x02\u{ed}\u{eb}\x03\
	\x02\x02\x02\u{ed}\u{ee}\x03\x02\x02\x02\u{ee}\x1d\x03\x02\x02\x02\u{ef}\
	\u{ed}\x03\x02\x02\x02\u{f0}\u{f1}\x05\x04\x03\x02\u{f1}\u{f2}\x07\x14\x02\
	\x02\u{f2}\u{f3}\x05\x1c\x0f\x02\u{f3}\x1f\x03\x02\x02\x02\u{f4}\u{f5}\x09\
	\x03\x02\x02\u{f5}\x21\x03\x02\x02\x02\u{f6}\u{fe}\x05\x28\x15\x02\u{f7}\
	\u{fe}\x05\x2a\x16\x02\u{f8}\u{fe}\x05\x2e\x18\x02\u{f9}\u{fe}\x05\x2c\x17\
	\x02\u{fa}\u{fe}\x05\x34\x1b\x02\u{fb}\u{fe}\x05\x38\x1d\x02\u{fc}\u{fe}\
	\x05\x3a\x1e\x02\u{fd}\u{f6}\x03\x02\x02\x02\u{fd}\u{f7}\x03\x02\x02\x02\
	\u{fd}\u{f8}\x03\x02\x02\x02\u{fd}\u{f9}\x03\x02\x02\x02\u{fd}\u{fa}\x03\
	\x02\x02\x02\u{fd}\u{fb}\x03\x02\x02\x02\u{fd}\u{fc}\x03\x02\x02\x02\u{fe}\
	\x23\x03\x02\x02\x02\u{ff}\u{100}\x08\x13\x01\x02\u{100}\u{101}\x05\x22\
	\x12\x02\u{101}\u{106}\x03\x02\x02\x02\u{102}\u{103}\x0c\x03\x02\x02\u{103}\
	\u{105}\x05\x22\x12\x02\u{104}\u{102}\x03\x02\x02\x02\u{105}\u{108}\x03\
	\x02\x02\x02\u{106}\u{104}\x03\x02\x02\x02\u{106}\u{107}\x03\x02\x02\x02\
	\u{107}\x25\x03\x02\x02\x02\u{108}\u{106}\x03\x02\x02\x02\u{109}\u{10a}\
	\x08\x14\x01\x02\u{10a}\u{10b}\x07\x2c\x02\x02\u{10b}\u{110}\x03\x02\x02\
	\x02\u{10c}\u{10d}\x0c\x03\x02\x02\u{10d}\u{10f}\x07\x2c\x02\x02\u{10e}\
	\u{10c}\x03\x02\x02\x02\u{10f}\u{112}\x03\x02\x02\x02\u{110}\u{10e}\x03\
	\x02\x02\x02\u{110}\u{111}\x03\x02\x02\x02\u{111}\x27\x03\x02\x02\x02\u{112}\
	\u{110}\x03\x02\x02\x02\u{113}\u{114}\x07\x20\x02\x02\u{114}\u{115}\x05\
	\x26\x14\x02\u{115}\u{116}\x05\x20\x11\x02\u{116}\x29\x03\x02\x02\x02\u{117}\
	\u{118}\x07\x22\x02\x02\u{118}\u{119}\x05\x26\x14\x02\u{119}\u{11a}\x05\
	\x20\x11\x02\u{11a}\x2b\x03\x02\x02\x02\u{11b}\u{120}\x05\x20\x11\x02\u{11c}\
	\u{11d}\x05\x1c\x0f\x02\u{11d}\u{11e}\x05\x20\x11\x02\u{11e}\u{120}\x03\
	\x02\x02\x02\u{11f}\u{11b}\x03\x02\x02\x02\u{11f}\u{11c}\x03\x02\x02\x02\
	\u{120}\x2d\x03\x02\x02\x02\u{121}\u{122}\x05\x1e\x10\x02\u{122}\u{123}\
	\x05\x20\x11\x02\u{123}\x2f\x03\x02\x02\x02\u{124}\u{127}\x05\x1c\x0f\x02\
	\u{125}\u{127}\x05\x2c\x17\x02\u{126}\u{124}\x03\x02\x02\x02\u{126}\u{125}\
	\x03\x02\x02\x02\u{127}\x31\x03\x02\x02\x02\u{128}\u{129}\x08\x1a\x01\x02\
	\u{129}\u{12a}\x05\x30\x19\x02\u{12a}\u{12f}\x03\x02\x02\x02\u{12b}\u{12c}\
	\x0c\x03\x02\x02\u{12c}\u{12e}\x05\x30\x19\x02\u{12d}\u{12b}\x03\x02\x02\
	\x02\u{12e}\u{131}\x03\x02\x02\x02\u{12f}\u{12d}\x03\x02\x02\x02\u{12f}\
	\u{130}\x03\x02\x02\x02\u{130}\x33\x03\x02\x02\x02\u{131}\u{12f}\x03\x02\
	\x02\x02\u{132}\u{133}\x07\x21\x02\x02\u{133}\u{134}\x05\x1c\x0f\x02\u{134}\
	\u{135}\x05\x24\x13\x02\u{135}\u{136}\x07\x1f\x02\x02\u{136}\u{137}\x05\
	\x20\x11\x02\u{137}\u{151}\x03\x02\x02\x02\u{138}\u{139}\x07\x21\x02\x02\
	\u{139}\u{13a}\x05\x1c\x0f\x02\u{13a}\u{13b}\x05\x24\x13\x02\u{13b}\u{13c}\
	\x07\x23\x02\x02\u{13c}\u{13d}\x05\x24\x13\x02\u{13d}\u{13e}\x07\x1f\x02\
	\x02\u{13e}\u{13f}\x05\x20\x11\x02\u{13f}\u{151}\x03\x02\x02\x02\u{140}\
	\u{141}\x07\x21\x02\x02\u{141}\u{142}\x05\x1c\x0f\x02\u{142}\u{143}\x05\
	\x24\x13\x02\u{143}\u{144}\x05\x36\x1c\x02\u{144}\u{145}\x07\x1f\x02\x02\
	\u{145}\u{146}\x05\x20\x11\x02\u{146}\u{151}\x03\x02\x02\x02\u{147}\u{148}\
	\x07\x21\x02\x02\u{148}\u{149}\x05\x1c\x0f\x02\u{149}\u{14a}\x05\x24\x13\
	\x02\u{14a}\u{14b}\x05\x36\x1c\x02\u{14b}\u{14c}\x07\x23\x02\x02\u{14c}\
	\u{14d}\x05\x24\x13\x02\u{14d}\u{14e}\x07\x1f\x02\x02\u{14e}\u{14f}\x05\
	\x20\x11\x02\u{14f}\u{151}\x03\x02\x02\x02\u{150}\u{132}\x03\x02\x02\x02\
	\u{150}\u{138}\x03\x02\x02\x02\u{150}\u{140}\x03\x02\x02\x02\u{150}\u{147}\
	\x03\x02\x02\x02\u{151}\x35\x03\x02\x02\x02\u{152}\u{153}\x08\x1c\x01\x02\
	\u{153}\u{154}\x07\x24\x02\x02\u{154}\u{155}\x05\x1c\x0f\x02\u{155}\u{156}\
	\x05\x24\x13\x02\u{156}\u{15e}\x03\x02\x02\x02\u{157}\u{158}\x0c\x03\x02\
	\x02\u{158}\u{159}\x07\x24\x02\x02\u{159}\u{15a}\x05\x1c\x0f\x02\u{15a}\
	\u{15b}\x05\x24\x13\x02\u{15b}\u{15d}\x03\x02\x02\x02\u{15c}\u{157}\x03\
	\x02\x02\x02\u{15d}\u{160}\x03\x02\x02\x02\u{15e}\u{15c}\x03\x02\x02\x02\
	\u{15e}\u{15f}\x03\x02\x02\x02\u{15f}\x37\x03\x02\x02\x02\u{160}\u{15e}\
	\x03\x02\x02\x02\u{161}\u{162}\x07\x1e\x02\x02\u{162}\u{163}\x05\x1c\x0f\
	\x02\u{163}\u{164}\x05\x24\x13\x02\u{164}\u{165}\x07\x1f\x02\x02\u{165}\
	\u{166}\x05\x20\x11\x02\u{166}\u{17a}\x03\x02\x02\x02\u{167}\u{168}\x07\
	\x1d\x02\x02\u{168}\u{169}\x07\x2c\x02\x02\u{169}\u{16a}\x07\x14\x02\x02\
	\u{16a}\u{16b}\x05\x1c\x0f\x02\u{16b}\u{16c}\x05\x24\x13\x02\u{16c}\u{16d}\
	\x07\x1f\x02\x02\u{16d}\u{16e}\x05\x20\x11\x02\u{16e}\u{17a}\x03\x02\x02\
	\x02\u{16f}\u{170}\x07\x1d\x02\x02\u{170}\u{171}\x07\x03\x02\x02\u{171}\
	\u{172}\x07\x2c\x02\x02\u{172}\u{173}\x07\x14\x02\x02\u{173}\u{174}\x05\
	\x1c\x0f\x02\u{174}\u{175}\x07\x04\x02\x02\u{175}\u{176}\x05\x24\x13\x02\
	\u{176}\u{177}\x07\x1f\x02\x02\u{177}\u{178}\x05\x20\x11\x02\u{178}\u{17a}\
	\x03\x02\x02\x02\u{179}\u{161}\x03\x02\x02\x02\u{179}\u{167}\x03\x02\x02\
	\x02\u{179}\u{16f}\x03\x02\x02\x02\u{17a}\x39\x03\x02\x02\x02\u{17b}\u{17c}\
	\x07\x1a\x02\x02\u{17c}\u{180}\x05\x20\x11\x02\u{17d}\u{17e}\x07\x1b\x02\
	\x02\u{17e}\u{180}\x05\x20\x11\x02\u{17f}\u{17b}\x03\x02\x02\x02\u{17f}\
	\u{17d}\x03\x02\x02\x02\u{180}\x3b\x03\x02\x02\x02\u{181}\u{188}\x05\x24\
	\x13\x02\u{182}\u{183}\x07\x1c\x02\x02\u{183}\u{184}\x05\x44\x23\x02\u{184}\
	\u{185}\x05\x20\x11\x02\u{185}\u{186}\x05\x24\x13\x02\u{186}\u{188}\x03\
	\x02\x02\x02\u{187}\u{181}\x03\x02\x02\x02\u{187}\u{182}\x03\x02\x02\x02\
	\u{188}\x3d\x03\x02\x02\x02\u{189}\u{18a}\x08\x20\x01\x02\u{18a}\u{18b}\
	\x07\x2c\x02\x02\u{18b}\u{191}\x03\x02\x02\x02\u{18c}\u{18d}\x0c\x03\x02\
	\x02\u{18d}\u{18e}\x07\x08\x02\x02\u{18e}\u{190}\x07\x2c\x02\x02\u{18f}\
	\u{18c}\x03\x02\x02\x02\u{190}\u{193}\x03\x02\x02\x02\u{191}\u{18f}\x03\
	\x02\x02\x02\u{191}\u{192}\x03\x02\x02\x02\u{192}\x3f\x03\x02\x02\x02\u{193}\
	\u{191}\x03\x02\x02\x02\u{194}\u{19a}\x07\x2c\x02\x02\u{195}\u{196}\x07\
	\x05\x02\x02\u{196}\u{197}\x05\x3e\x20\x02\u{197}\u{198}\x07\x06\x02\x02\
	\u{198}\u{19a}\x03\x02\x02\x02\u{199}\u{194}\x03\x02\x02\x02\u{199}\u{195}\
	\x03\x02\x02\x02\u{19a}\x41\x03\x02\x02\x02\u{19b}\u{1a5}\x07\x2c\x02\x02\
	\u{19c}\u{19d}\x07\x2c\x02\x02\u{19d}\u{19e}\x07\x03\x02\x02\u{19e}\u{1a5}\
	\x07\x04\x02\x02\u{19f}\u{1a0}\x07\x2c\x02\x02\u{1a0}\u{1a1}\x07\x03\x02\
	\x02\u{1a1}\u{1a2}\x05\x3e\x20\x02\u{1a2}\u{1a3}\x07\x04\x02\x02\u{1a3}\
	\u{1a5}\x03\x02\x02\x02\u{1a4}\u{19b}\x03\x02\x02\x02\u{1a4}\u{19c}\x03\
	\x02\x02\x02\u{1a4}\u{19f}\x03\x02\x02\x02\u{1a5}\x43\x03\x02\x02\x02\u{1a6}\
	\u{1ac}\x05\x42\x22\x02\u{1a7}\u{1a8}\x05\x40\x21\x02\u{1a8}\u{1a9}\x07\
	\x14\x02\x02\u{1a9}\u{1aa}\x05\x42\x22\x02\u{1aa}\u{1ac}\x03\x02\x02\x02\
	\u{1ab}\u{1a6}\x03\x02\x02\x02\u{1ab}\u{1a7}\x03\x02\x02\x02\u{1ac}\x45\
	\x03\x02\x02\x02\x23\x53\x58\x5e\x60\x65\x6f\x7b\u{9a}\u{9c}\u{a8}\u{aa}\
	\u{bc}\u{be}\u{ca}\u{cc}\u{d7}\u{e2}\u{ed}\u{fd}\u{106}\u{110}\u{11f}\u{126}\
	\u{12f}\u{150}\u{15e}\u{179}\u{17f}\u{187}\u{191}\u{199}\u{1a4}\u{1ab}";

