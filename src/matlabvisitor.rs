#![allow(nonstandard_style)]
// Generated from src/matlab.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor};
use super::matlabparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link matlabParser}.
 */
pub trait matlabVisitor<'input>: ParseTreeVisitor<'input,matlabParserContextType>{
	/**
	 * Visit a parse tree produced by {@link matlabParser#primary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_primary_expression(&mut self, ctx: &Primary_expressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#postfix_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_postfix_expression(&mut self, ctx: &Postfix_expressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#index_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_index_expression(&mut self, ctx: &Index_expressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#index_expression_list}.
	 * @param ctx the parse tree
	 */
	fn visit_index_expression_list(&mut self, ctx: &Index_expression_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#array_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_array_expression(&mut self, ctx: &Array_expressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#unary_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_unary_expression(&mut self, ctx: &Unary_expressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#unary_operator}.
	 * @param ctx the parse tree
	 */
	fn visit_unary_operator(&mut self, ctx: &Unary_operatorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#multiplicative_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_multiplicative_expression(&mut self, ctx: &Multiplicative_expressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#additive_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_additive_expression(&mut self, ctx: &Additive_expressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#relational_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_relational_expression(&mut self, ctx: &Relational_expressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#equality_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_equality_expression(&mut self, ctx: &Equality_expressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#and_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_and_expression(&mut self, ctx: &And_expressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#or_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_or_expression(&mut self, ctx: &Or_expressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expression(&mut self, ctx: &ExpressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#assignment_expression}.
	 * @param ctx the parse tree
	 */
	fn visit_assignment_expression(&mut self, ctx: &Assignment_expressionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#eostmt}.
	 * @param ctx the parse tree
	 */
	fn visit_eostmt(&mut self, ctx: &EostmtContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_statement(&mut self, ctx: &StatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#statement_list}.
	 * @param ctx the parse tree
	 */
	fn visit_statement_list(&mut self, ctx: &Statement_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#identifier_list}.
	 * @param ctx the parse tree
	 */
	fn visit_identifier_list(&mut self, ctx: &Identifier_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#global_statement}.
	 * @param ctx the parse tree
	 */
	fn visit_global_statement(&mut self, ctx: &Global_statementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#clear_statement}.
	 * @param ctx the parse tree
	 */
	fn visit_clear_statement(&mut self, ctx: &Clear_statementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#expression_statement}.
	 * @param ctx the parse tree
	 */
	fn visit_expression_statement(&mut self, ctx: &Expression_statementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#assignment_statement}.
	 * @param ctx the parse tree
	 */
	fn visit_assignment_statement(&mut self, ctx: &Assignment_statementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#array_element}.
	 * @param ctx the parse tree
	 */
	fn visit_array_element(&mut self, ctx: &Array_elementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#array_list}.
	 * @param ctx the parse tree
	 */
	fn visit_array_list(&mut self, ctx: &Array_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#selection_statement}.
	 * @param ctx the parse tree
	 */
	fn visit_selection_statement(&mut self, ctx: &Selection_statementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#elseif_clause}.
	 * @param ctx the parse tree
	 */
	fn visit_elseif_clause(&mut self, ctx: &Elseif_clauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#iteration_statement}.
	 * @param ctx the parse tree
	 */
	fn visit_iteration_statement(&mut self, ctx: &Iteration_statementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#jump_statement}.
	 * @param ctx the parse tree
	 */
	fn visit_jump_statement(&mut self, ctx: &Jump_statementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#translation_unit}.
	 * @param ctx the parse tree
	 */
	fn visit_translation_unit(&mut self, ctx: &Translation_unitContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#func_ident_list}.
	 * @param ctx the parse tree
	 */
	fn visit_func_ident_list(&mut self, ctx: &Func_ident_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#func_return_list}.
	 * @param ctx the parse tree
	 */
	fn visit_func_return_list(&mut self, ctx: &Func_return_listContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#function_declare_lhs}.
	 * @param ctx the parse tree
	 */
	fn visit_function_declare_lhs(&mut self, ctx: &Function_declare_lhsContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link matlabParser#function_declare}.
	 * @param ctx the parse tree
	 */
	fn visit_function_declare(&mut self, ctx: &Function_declareContext<'input>) { self.visit_children(ctx) }


}