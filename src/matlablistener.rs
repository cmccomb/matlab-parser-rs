#![allow(nonstandard_style)]
// Generated from src/matlab.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::matlabparser::*;

pub trait matlabListener<'input> : ParseTreeListener<'input,matlabParserContextType>{

/**
 * Enter a parse tree produced by {@link matlabParser#primary_expression}.
 * @param ctx the parse tree
 */
fn enter_primary_expression(&mut self, _ctx: &Primary_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#primary_expression}.
 * @param ctx the parse tree
 */
fn exit_primary_expression(&mut self, _ctx: &Primary_expressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#postfix_expression}.
 * @param ctx the parse tree
 */
fn enter_postfix_expression(&mut self, _ctx: &Postfix_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#postfix_expression}.
 * @param ctx the parse tree
 */
fn exit_postfix_expression(&mut self, _ctx: &Postfix_expressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#index_expression}.
 * @param ctx the parse tree
 */
fn enter_index_expression(&mut self, _ctx: &Index_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#index_expression}.
 * @param ctx the parse tree
 */
fn exit_index_expression(&mut self, _ctx: &Index_expressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#index_expression_list}.
 * @param ctx the parse tree
 */
fn enter_index_expression_list(&mut self, _ctx: &Index_expression_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#index_expression_list}.
 * @param ctx the parse tree
 */
fn exit_index_expression_list(&mut self, _ctx: &Index_expression_listContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#array_expression}.
 * @param ctx the parse tree
 */
fn enter_array_expression(&mut self, _ctx: &Array_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#array_expression}.
 * @param ctx the parse tree
 */
fn exit_array_expression(&mut self, _ctx: &Array_expressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#unary_expression}.
 * @param ctx the parse tree
 */
fn enter_unary_expression(&mut self, _ctx: &Unary_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#unary_expression}.
 * @param ctx the parse tree
 */
fn exit_unary_expression(&mut self, _ctx: &Unary_expressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#unary_operator}.
 * @param ctx the parse tree
 */
fn enter_unary_operator(&mut self, _ctx: &Unary_operatorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#unary_operator}.
 * @param ctx the parse tree
 */
fn exit_unary_operator(&mut self, _ctx: &Unary_operatorContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#multiplicative_expression}.
 * @param ctx the parse tree
 */
fn enter_multiplicative_expression(&mut self, _ctx: &Multiplicative_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#multiplicative_expression}.
 * @param ctx the parse tree
 */
fn exit_multiplicative_expression(&mut self, _ctx: &Multiplicative_expressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#additive_expression}.
 * @param ctx the parse tree
 */
fn enter_additive_expression(&mut self, _ctx: &Additive_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#additive_expression}.
 * @param ctx the parse tree
 */
fn exit_additive_expression(&mut self, _ctx: &Additive_expressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#relational_expression}.
 * @param ctx the parse tree
 */
fn enter_relational_expression(&mut self, _ctx: &Relational_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#relational_expression}.
 * @param ctx the parse tree
 */
fn exit_relational_expression(&mut self, _ctx: &Relational_expressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#equality_expression}.
 * @param ctx the parse tree
 */
fn enter_equality_expression(&mut self, _ctx: &Equality_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#equality_expression}.
 * @param ctx the parse tree
 */
fn exit_equality_expression(&mut self, _ctx: &Equality_expressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#and_expression}.
 * @param ctx the parse tree
 */
fn enter_and_expression(&mut self, _ctx: &And_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#and_expression}.
 * @param ctx the parse tree
 */
fn exit_and_expression(&mut self, _ctx: &And_expressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#or_expression}.
 * @param ctx the parse tree
 */
fn enter_or_expression(&mut self, _ctx: &Or_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#or_expression}.
 * @param ctx the parse tree
 */
fn exit_or_expression(&mut self, _ctx: &Or_expressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expression(&mut self, _ctx: &ExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expression(&mut self, _ctx: &ExpressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#assignment_expression}.
 * @param ctx the parse tree
 */
fn enter_assignment_expression(&mut self, _ctx: &Assignment_expressionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#assignment_expression}.
 * @param ctx the parse tree
 */
fn exit_assignment_expression(&mut self, _ctx: &Assignment_expressionContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#eostmt}.
 * @param ctx the parse tree
 */
fn enter_eostmt(&mut self, _ctx: &EostmtContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#eostmt}.
 * @param ctx the parse tree
 */
fn exit_eostmt(&mut self, _ctx: &EostmtContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#statement}.
 * @param ctx the parse tree
 */
fn enter_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#statement}.
 * @param ctx the parse tree
 */
fn exit_statement(&mut self, _ctx: &StatementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#statement_list}.
 * @param ctx the parse tree
 */
fn enter_statement_list(&mut self, _ctx: &Statement_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#statement_list}.
 * @param ctx the parse tree
 */
fn exit_statement_list(&mut self, _ctx: &Statement_listContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#identifier_list}.
 * @param ctx the parse tree
 */
fn enter_identifier_list(&mut self, _ctx: &Identifier_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#identifier_list}.
 * @param ctx the parse tree
 */
fn exit_identifier_list(&mut self, _ctx: &Identifier_listContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#global_statement}.
 * @param ctx the parse tree
 */
fn enter_global_statement(&mut self, _ctx: &Global_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#global_statement}.
 * @param ctx the parse tree
 */
fn exit_global_statement(&mut self, _ctx: &Global_statementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#clear_statement}.
 * @param ctx the parse tree
 */
fn enter_clear_statement(&mut self, _ctx: &Clear_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#clear_statement}.
 * @param ctx the parse tree
 */
fn exit_clear_statement(&mut self, _ctx: &Clear_statementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#expression_statement}.
 * @param ctx the parse tree
 */
fn enter_expression_statement(&mut self, _ctx: &Expression_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#expression_statement}.
 * @param ctx the parse tree
 */
fn exit_expression_statement(&mut self, _ctx: &Expression_statementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#assignment_statement}.
 * @param ctx the parse tree
 */
fn enter_assignment_statement(&mut self, _ctx: &Assignment_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#assignment_statement}.
 * @param ctx the parse tree
 */
fn exit_assignment_statement(&mut self, _ctx: &Assignment_statementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#array_element}.
 * @param ctx the parse tree
 */
fn enter_array_element(&mut self, _ctx: &Array_elementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#array_element}.
 * @param ctx the parse tree
 */
fn exit_array_element(&mut self, _ctx: &Array_elementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#array_list}.
 * @param ctx the parse tree
 */
fn enter_array_list(&mut self, _ctx: &Array_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#array_list}.
 * @param ctx the parse tree
 */
fn exit_array_list(&mut self, _ctx: &Array_listContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#selection_statement}.
 * @param ctx the parse tree
 */
fn enter_selection_statement(&mut self, _ctx: &Selection_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#selection_statement}.
 * @param ctx the parse tree
 */
fn exit_selection_statement(&mut self, _ctx: &Selection_statementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#elseif_clause}.
 * @param ctx the parse tree
 */
fn enter_elseif_clause(&mut self, _ctx: &Elseif_clauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#elseif_clause}.
 * @param ctx the parse tree
 */
fn exit_elseif_clause(&mut self, _ctx: &Elseif_clauseContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#iteration_statement}.
 * @param ctx the parse tree
 */
fn enter_iteration_statement(&mut self, _ctx: &Iteration_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#iteration_statement}.
 * @param ctx the parse tree
 */
fn exit_iteration_statement(&mut self, _ctx: &Iteration_statementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#jump_statement}.
 * @param ctx the parse tree
 */
fn enter_jump_statement(&mut self, _ctx: &Jump_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#jump_statement}.
 * @param ctx the parse tree
 */
fn exit_jump_statement(&mut self, _ctx: &Jump_statementContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#translation_unit}.
 * @param ctx the parse tree
 */
fn enter_translation_unit(&mut self, _ctx: &Translation_unitContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#translation_unit}.
 * @param ctx the parse tree
 */
fn exit_translation_unit(&mut self, _ctx: &Translation_unitContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#func_ident_list}.
 * @param ctx the parse tree
 */
fn enter_func_ident_list(&mut self, _ctx: &Func_ident_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#func_ident_list}.
 * @param ctx the parse tree
 */
fn exit_func_ident_list(&mut self, _ctx: &Func_ident_listContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#func_return_list}.
 * @param ctx the parse tree
 */
fn enter_func_return_list(&mut self, _ctx: &Func_return_listContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#func_return_list}.
 * @param ctx the parse tree
 */
fn exit_func_return_list(&mut self, _ctx: &Func_return_listContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#function_declare_lhs}.
 * @param ctx the parse tree
 */
fn enter_function_declare_lhs(&mut self, _ctx: &Function_declare_lhsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#function_declare_lhs}.
 * @param ctx the parse tree
 */
fn exit_function_declare_lhs(&mut self, _ctx: &Function_declare_lhsContext<'input>) { }

/**
 * Enter a parse tree produced by {@link matlabParser#function_declare}.
 * @param ctx the parse tree
 */
fn enter_function_declare(&mut self, _ctx: &Function_declareContext<'input>) { }
/**
 * Exit a parse tree produced by {@link matlabParser#function_declare}.
 * @param ctx the parse tree
 */
fn exit_function_declare(&mut self, _ctx: &Function_declareContext<'input>) { }

}
