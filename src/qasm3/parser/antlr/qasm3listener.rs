#![allow(nonstandard_style)]
// Generated from src/qasm3/parser/antlr/qasm3.g4 by ANTLR 4.8
use super::qasm3parser::*;
use antlr_rust::tree::ParseTreeListener;

pub trait qasm3Listener<'input>: ParseTreeListener<'input, qasm3ContextType> {
    /**
     * Enter a parse tree produced by {@link qasm3#program}.
     * @param ctx the parse tree
     */
    fn enter_program(&mut self, _ctx: &ProgramContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#program}.
     * @param ctx the parse tree
     */
    fn exit_program(&mut self, _ctx: &ProgramContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#version}.
     * @param ctx the parse tree
     */
    fn enter_version(&mut self, _ctx: &VersionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#version}.
     * @param ctx the parse tree
     */
    fn exit_version(&mut self, _ctx: &VersionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#statement}.
     * @param ctx the parse tree
     */
    fn enter_statement(&mut self, _ctx: &StatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#statement}.
     * @param ctx the parse tree
     */
    fn exit_statement(&mut self, _ctx: &StatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#annotation}.
     * @param ctx the parse tree
     */
    fn enter_annotation(&mut self, _ctx: &AnnotationContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#annotation}.
     * @param ctx the parse tree
     */
    fn exit_annotation(&mut self, _ctx: &AnnotationContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#scope}.
     * @param ctx the parse tree
     */
    fn enter_scope(&mut self, _ctx: &ScopeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#scope}.
     * @param ctx the parse tree
     */
    fn exit_scope(&mut self, _ctx: &ScopeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#pragma}.
     * @param ctx the parse tree
     */
    fn enter_pragma(&mut self, _ctx: &PragmaContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#pragma}.
     * @param ctx the parse tree
     */
    fn exit_pragma(&mut self, _ctx: &PragmaContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#statementOrScope}.
     * @param ctx the parse tree
     */
    fn enter_statementOrScope(&mut self, _ctx: &StatementOrScopeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#statementOrScope}.
     * @param ctx the parse tree
     */
    fn exit_statementOrScope(&mut self, _ctx: &StatementOrScopeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#calibrationGrammarStatement}.
     * @param ctx the parse tree
     */
    fn enter_calibrationGrammarStatement(
        &mut self,
        _ctx: &CalibrationGrammarStatementContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link qasm3#calibrationGrammarStatement}.
     * @param ctx the parse tree
     */
    fn exit_calibrationGrammarStatement(
        &mut self,
        _ctx: &CalibrationGrammarStatementContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link qasm3#includeStatement}.
     * @param ctx the parse tree
     */
    fn enter_includeStatement(&mut self, _ctx: &IncludeStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#includeStatement}.
     * @param ctx the parse tree
     */
    fn exit_includeStatement(&mut self, _ctx: &IncludeStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#breakStatement}.
     * @param ctx the parse tree
     */
    fn enter_breakStatement(&mut self, _ctx: &BreakStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#breakStatement}.
     * @param ctx the parse tree
     */
    fn exit_breakStatement(&mut self, _ctx: &BreakStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#continueStatement}.
     * @param ctx the parse tree
     */
    fn enter_continueStatement(&mut self, _ctx: &ContinueStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#continueStatement}.
     * @param ctx the parse tree
     */
    fn exit_continueStatement(&mut self, _ctx: &ContinueStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#endStatement}.
     * @param ctx the parse tree
     */
    fn enter_endStatement(&mut self, _ctx: &EndStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#endStatement}.
     * @param ctx the parse tree
     */
    fn exit_endStatement(&mut self, _ctx: &EndStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#forStatement}.
     * @param ctx the parse tree
     */
    fn enter_forStatement(&mut self, _ctx: &ForStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#forStatement}.
     * @param ctx the parse tree
     */
    fn exit_forStatement(&mut self, _ctx: &ForStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#ifStatement}.
     * @param ctx the parse tree
     */
    fn enter_ifStatement(&mut self, _ctx: &IfStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#ifStatement}.
     * @param ctx the parse tree
     */
    fn exit_ifStatement(&mut self, _ctx: &IfStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#returnStatement}.
     * @param ctx the parse tree
     */
    fn enter_returnStatement(&mut self, _ctx: &ReturnStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#returnStatement}.
     * @param ctx the parse tree
     */
    fn exit_returnStatement(&mut self, _ctx: &ReturnStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#whileStatement}.
     * @param ctx the parse tree
     */
    fn enter_whileStatement(&mut self, _ctx: &WhileStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#whileStatement}.
     * @param ctx the parse tree
     */
    fn exit_whileStatement(&mut self, _ctx: &WhileStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#switchStatement}.
     * @param ctx the parse tree
     */
    fn enter_switchStatement(&mut self, _ctx: &SwitchStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#switchStatement}.
     * @param ctx the parse tree
     */
    fn exit_switchStatement(&mut self, _ctx: &SwitchStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#switchCaseItem}.
     * @param ctx the parse tree
     */
    fn enter_switchCaseItem(&mut self, _ctx: &SwitchCaseItemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#switchCaseItem}.
     * @param ctx the parse tree
     */
    fn exit_switchCaseItem(&mut self, _ctx: &SwitchCaseItemContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#barrierStatement}.
     * @param ctx the parse tree
     */
    fn enter_barrierStatement(&mut self, _ctx: &BarrierStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#barrierStatement}.
     * @param ctx the parse tree
     */
    fn exit_barrierStatement(&mut self, _ctx: &BarrierStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#boxStatement}.
     * @param ctx the parse tree
     */
    fn enter_boxStatement(&mut self, _ctx: &BoxStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#boxStatement}.
     * @param ctx the parse tree
     */
    fn exit_boxStatement(&mut self, _ctx: &BoxStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#delayStatement}.
     * @param ctx the parse tree
     */
    fn enter_delayStatement(&mut self, _ctx: &DelayStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#delayStatement}.
     * @param ctx the parse tree
     */
    fn exit_delayStatement(&mut self, _ctx: &DelayStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#gateCallStatement}.
     * @param ctx the parse tree
     */
    fn enter_gateCallStatement(&mut self, _ctx: &GateCallStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#gateCallStatement}.
     * @param ctx the parse tree
     */
    fn exit_gateCallStatement(&mut self, _ctx: &GateCallStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#measureArrowAssignmentStatement}.
     * @param ctx the parse tree
     */
    fn enter_measureArrowAssignmentStatement(
        &mut self,
        _ctx: &MeasureArrowAssignmentStatementContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link qasm3#measureArrowAssignmentStatement}.
     * @param ctx the parse tree
     */
    fn exit_measureArrowAssignmentStatement(
        &mut self,
        _ctx: &MeasureArrowAssignmentStatementContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link qasm3#resetStatement}.
     * @param ctx the parse tree
     */
    fn enter_resetStatement(&mut self, _ctx: &ResetStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#resetStatement}.
     * @param ctx the parse tree
     */
    fn exit_resetStatement(&mut self, _ctx: &ResetStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#aliasDeclarationStatement}.
     * @param ctx the parse tree
     */
    fn enter_aliasDeclarationStatement(&mut self, _ctx: &AliasDeclarationStatementContext<'input>) {
    }
    /**
     * Exit a parse tree produced by {@link qasm3#aliasDeclarationStatement}.
     * @param ctx the parse tree
     */
    fn exit_aliasDeclarationStatement(&mut self, _ctx: &AliasDeclarationStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#classicalDeclarationStatement}.
     * @param ctx the parse tree
     */
    fn enter_classicalDeclarationStatement(
        &mut self,
        _ctx: &ClassicalDeclarationStatementContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link qasm3#classicalDeclarationStatement}.
     * @param ctx the parse tree
     */
    fn exit_classicalDeclarationStatement(
        &mut self,
        _ctx: &ClassicalDeclarationStatementContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link qasm3#constDeclarationStatement}.
     * @param ctx the parse tree
     */
    fn enter_constDeclarationStatement(&mut self, _ctx: &ConstDeclarationStatementContext<'input>) {
    }
    /**
     * Exit a parse tree produced by {@link qasm3#constDeclarationStatement}.
     * @param ctx the parse tree
     */
    fn exit_constDeclarationStatement(&mut self, _ctx: &ConstDeclarationStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#ioDeclarationStatement}.
     * @param ctx the parse tree
     */
    fn enter_ioDeclarationStatement(&mut self, _ctx: &IoDeclarationStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#ioDeclarationStatement}.
     * @param ctx the parse tree
     */
    fn exit_ioDeclarationStatement(&mut self, _ctx: &IoDeclarationStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#oldStyleDeclarationStatement}.
     * @param ctx the parse tree
     */
    fn enter_oldStyleDeclarationStatement(
        &mut self,
        _ctx: &OldStyleDeclarationStatementContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link qasm3#oldStyleDeclarationStatement}.
     * @param ctx the parse tree
     */
    fn exit_oldStyleDeclarationStatement(
        &mut self,
        _ctx: &OldStyleDeclarationStatementContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link qasm3#quantumDeclarationStatement}.
     * @param ctx the parse tree
     */
    fn enter_quantumDeclarationStatement(
        &mut self,
        _ctx: &QuantumDeclarationStatementContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link qasm3#quantumDeclarationStatement}.
     * @param ctx the parse tree
     */
    fn exit_quantumDeclarationStatement(
        &mut self,
        _ctx: &QuantumDeclarationStatementContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link qasm3#defStatement}.
     * @param ctx the parse tree
     */
    fn enter_defStatement(&mut self, _ctx: &DefStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#defStatement}.
     * @param ctx the parse tree
     */
    fn exit_defStatement(&mut self, _ctx: &DefStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#externStatement}.
     * @param ctx the parse tree
     */
    fn enter_externStatement(&mut self, _ctx: &ExternStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#externStatement}.
     * @param ctx the parse tree
     */
    fn exit_externStatement(&mut self, _ctx: &ExternStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#gateStatement}.
     * @param ctx the parse tree
     */
    fn enter_gateStatement(&mut self, _ctx: &GateStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#gateStatement}.
     * @param ctx the parse tree
     */
    fn exit_gateStatement(&mut self, _ctx: &GateStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#assignmentStatement}.
     * @param ctx the parse tree
     */
    fn enter_assignmentStatement(&mut self, _ctx: &AssignmentStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#assignmentStatement}.
     * @param ctx the parse tree
     */
    fn exit_assignmentStatement(&mut self, _ctx: &AssignmentStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#expressionStatement}.
     * @param ctx the parse tree
     */
    fn enter_expressionStatement(&mut self, _ctx: &ExpressionStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#expressionStatement}.
     * @param ctx the parse tree
     */
    fn exit_expressionStatement(&mut self, _ctx: &ExpressionStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#calStatement}.
     * @param ctx the parse tree
     */
    fn enter_calStatement(&mut self, _ctx: &CalStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#calStatement}.
     * @param ctx the parse tree
     */
    fn exit_calStatement(&mut self, _ctx: &CalStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#defcalStatement}.
     * @param ctx the parse tree
     */
    fn enter_defcalStatement(&mut self, _ctx: &DefcalStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#defcalStatement}.
     * @param ctx the parse tree
     */
    fn exit_defcalStatement(&mut self, _ctx: &DefcalStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code bitwiseXorExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn enter_bitwiseXorExpression(&mut self, _ctx: &BitwiseXorExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code bitwiseXorExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn exit_bitwiseXorExpression(&mut self, _ctx: &BitwiseXorExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code additiveExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn enter_additiveExpression(&mut self, _ctx: &AdditiveExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code additiveExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn exit_additiveExpression(&mut self, _ctx: &AdditiveExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code durationofExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn enter_durationofExpression(&mut self, _ctx: &DurationofExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code durationofExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn exit_durationofExpression(&mut self, _ctx: &DurationofExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code parenthesisExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn enter_parenthesisExpression(&mut self, _ctx: &ParenthesisExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code parenthesisExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn exit_parenthesisExpression(&mut self, _ctx: &ParenthesisExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code comparisonExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn enter_comparisonExpression(&mut self, _ctx: &ComparisonExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code comparisonExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn exit_comparisonExpression(&mut self, _ctx: &ComparisonExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code multiplicativeExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn enter_multiplicativeExpression(&mut self, _ctx: &MultiplicativeExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code multiplicativeExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn exit_multiplicativeExpression(&mut self, _ctx: &MultiplicativeExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code logicalOrExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn enter_logicalOrExpression(&mut self, _ctx: &LogicalOrExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code logicalOrExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn exit_logicalOrExpression(&mut self, _ctx: &LogicalOrExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code castExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn enter_castExpression(&mut self, _ctx: &CastExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code castExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn exit_castExpression(&mut self, _ctx: &CastExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code powerExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn enter_powerExpression(&mut self, _ctx: &PowerExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code powerExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn exit_powerExpression(&mut self, _ctx: &PowerExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code bitwiseOrExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn enter_bitwiseOrExpression(&mut self, _ctx: &BitwiseOrExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code bitwiseOrExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn exit_bitwiseOrExpression(&mut self, _ctx: &BitwiseOrExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code callExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn enter_callExpression(&mut self, _ctx: &CallExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code callExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn exit_callExpression(&mut self, _ctx: &CallExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code bitshiftExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn enter_bitshiftExpression(&mut self, _ctx: &BitshiftExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code bitshiftExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn exit_bitshiftExpression(&mut self, _ctx: &BitshiftExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code bitwiseAndExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn enter_bitwiseAndExpression(&mut self, _ctx: &BitwiseAndExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code bitwiseAndExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn exit_bitwiseAndExpression(&mut self, _ctx: &BitwiseAndExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code equalityExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn enter_equalityExpression(&mut self, _ctx: &EqualityExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code equalityExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn exit_equalityExpression(&mut self, _ctx: &EqualityExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code logicalAndExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn enter_logicalAndExpression(&mut self, _ctx: &LogicalAndExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code logicalAndExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn exit_logicalAndExpression(&mut self, _ctx: &LogicalAndExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code indexExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn enter_indexExpression(&mut self, _ctx: &IndexExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code indexExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn exit_indexExpression(&mut self, _ctx: &IndexExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code unaryExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn enter_unaryExpression(&mut self, _ctx: &UnaryExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code unaryExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn exit_unaryExpression(&mut self, _ctx: &UnaryExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code literalExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn enter_literalExpression(&mut self, _ctx: &LiteralExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code literalExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn exit_literalExpression(&mut self, _ctx: &LiteralExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#aliasExpression}.
     * @param ctx the parse tree
     */
    fn enter_aliasExpression(&mut self, _ctx: &AliasExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#aliasExpression}.
     * @param ctx the parse tree
     */
    fn exit_aliasExpression(&mut self, _ctx: &AliasExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#declarationExpression}.
     * @param ctx the parse tree
     */
    fn enter_declarationExpression(&mut self, _ctx: &DeclarationExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#declarationExpression}.
     * @param ctx the parse tree
     */
    fn exit_declarationExpression(&mut self, _ctx: &DeclarationExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#measureExpression}.
     * @param ctx the parse tree
     */
    fn enter_measureExpression(&mut self, _ctx: &MeasureExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#measureExpression}.
     * @param ctx the parse tree
     */
    fn exit_measureExpression(&mut self, _ctx: &MeasureExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#rangeExpression}.
     * @param ctx the parse tree
     */
    fn enter_rangeExpression(&mut self, _ctx: &RangeExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#rangeExpression}.
     * @param ctx the parse tree
     */
    fn exit_rangeExpression(&mut self, _ctx: &RangeExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#setExpression}.
     * @param ctx the parse tree
     */
    fn enter_setExpression(&mut self, _ctx: &SetExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#setExpression}.
     * @param ctx the parse tree
     */
    fn exit_setExpression(&mut self, _ctx: &SetExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#arrayLiteral}.
     * @param ctx the parse tree
     */
    fn enter_arrayLiteral(&mut self, _ctx: &ArrayLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#arrayLiteral}.
     * @param ctx the parse tree
     */
    fn exit_arrayLiteral(&mut self, _ctx: &ArrayLiteralContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#arrayLiteralElement}.
     * @param ctx the parse tree
     */
    fn enter_arrayLiteralElement(&mut self, _ctx: &ArrayLiteralElementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#arrayLiteralElement}.
     * @param ctx the parse tree
     */
    fn exit_arrayLiteralElement(&mut self, _ctx: &ArrayLiteralElementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#indexOperator}.
     * @param ctx the parse tree
     */
    fn enter_indexOperator(&mut self, _ctx: &IndexOperatorContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#indexOperator}.
     * @param ctx the parse tree
     */
    fn exit_indexOperator(&mut self, _ctx: &IndexOperatorContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#indexOperatorElement}.
     * @param ctx the parse tree
     */
    fn enter_indexOperatorElement(&mut self, _ctx: &IndexOperatorElementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#indexOperatorElement}.
     * @param ctx the parse tree
     */
    fn exit_indexOperatorElement(&mut self, _ctx: &IndexOperatorElementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#indexedIdentifier}.
     * @param ctx the parse tree
     */
    fn enter_indexedIdentifier(&mut self, _ctx: &IndexedIdentifierContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#indexedIdentifier}.
     * @param ctx the parse tree
     */
    fn exit_indexedIdentifier(&mut self, _ctx: &IndexedIdentifierContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#returnSignature}.
     * @param ctx the parse tree
     */
    fn enter_returnSignature(&mut self, _ctx: &ReturnSignatureContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#returnSignature}.
     * @param ctx the parse tree
     */
    fn exit_returnSignature(&mut self, _ctx: &ReturnSignatureContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#gateModifier}.
     * @param ctx the parse tree
     */
    fn enter_gateModifier(&mut self, _ctx: &GateModifierContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#gateModifier}.
     * @param ctx the parse tree
     */
    fn exit_gateModifier(&mut self, _ctx: &GateModifierContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#scalarType}.
     * @param ctx the parse tree
     */
    fn enter_scalarType(&mut self, _ctx: &ScalarTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#scalarType}.
     * @param ctx the parse tree
     */
    fn exit_scalarType(&mut self, _ctx: &ScalarTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#qubitType}.
     * @param ctx the parse tree
     */
    fn enter_qubitType(&mut self, _ctx: &QubitTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#qubitType}.
     * @param ctx the parse tree
     */
    fn exit_qubitType(&mut self, _ctx: &QubitTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#arrayType}.
     * @param ctx the parse tree
     */
    fn enter_arrayType(&mut self, _ctx: &ArrayTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#arrayType}.
     * @param ctx the parse tree
     */
    fn exit_arrayType(&mut self, _ctx: &ArrayTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#arrayReferenceType}.
     * @param ctx the parse tree
     */
    fn enter_arrayReferenceType(&mut self, _ctx: &ArrayReferenceTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#arrayReferenceType}.
     * @param ctx the parse tree
     */
    fn exit_arrayReferenceType(&mut self, _ctx: &ArrayReferenceTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#designator}.
     * @param ctx the parse tree
     */
    fn enter_designator(&mut self, _ctx: &DesignatorContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#designator}.
     * @param ctx the parse tree
     */
    fn exit_designator(&mut self, _ctx: &DesignatorContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#defcalTarget}.
     * @param ctx the parse tree
     */
    fn enter_defcalTarget(&mut self, _ctx: &DefcalTargetContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#defcalTarget}.
     * @param ctx the parse tree
     */
    fn exit_defcalTarget(&mut self, _ctx: &DefcalTargetContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#defcalArgumentDefinition}.
     * @param ctx the parse tree
     */
    fn enter_defcalArgumentDefinition(&mut self, _ctx: &DefcalArgumentDefinitionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#defcalArgumentDefinition}.
     * @param ctx the parse tree
     */
    fn exit_defcalArgumentDefinition(&mut self, _ctx: &DefcalArgumentDefinitionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#defcalOperand}.
     * @param ctx the parse tree
     */
    fn enter_defcalOperand(&mut self, _ctx: &DefcalOperandContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#defcalOperand}.
     * @param ctx the parse tree
     */
    fn exit_defcalOperand(&mut self, _ctx: &DefcalOperandContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#gateOperand}.
     * @param ctx the parse tree
     */
    fn enter_gateOperand(&mut self, _ctx: &GateOperandContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#gateOperand}.
     * @param ctx the parse tree
     */
    fn exit_gateOperand(&mut self, _ctx: &GateOperandContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#externArgument}.
     * @param ctx the parse tree
     */
    fn enter_externArgument(&mut self, _ctx: &ExternArgumentContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#externArgument}.
     * @param ctx the parse tree
     */
    fn exit_externArgument(&mut self, _ctx: &ExternArgumentContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#argumentDefinition}.
     * @param ctx the parse tree
     */
    fn enter_argumentDefinition(&mut self, _ctx: &ArgumentDefinitionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#argumentDefinition}.
     * @param ctx the parse tree
     */
    fn exit_argumentDefinition(&mut self, _ctx: &ArgumentDefinitionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#argumentDefinitionList}.
     * @param ctx the parse tree
     */
    fn enter_argumentDefinitionList(&mut self, _ctx: &ArgumentDefinitionListContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#argumentDefinitionList}.
     * @param ctx the parse tree
     */
    fn exit_argumentDefinitionList(&mut self, _ctx: &ArgumentDefinitionListContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#defcalArgumentDefinitionList}.
     * @param ctx the parse tree
     */
    fn enter_defcalArgumentDefinitionList(
        &mut self,
        _ctx: &DefcalArgumentDefinitionListContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link qasm3#defcalArgumentDefinitionList}.
     * @param ctx the parse tree
     */
    fn exit_defcalArgumentDefinitionList(
        &mut self,
        _ctx: &DefcalArgumentDefinitionListContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link qasm3#defcalOperandList}.
     * @param ctx the parse tree
     */
    fn enter_defcalOperandList(&mut self, _ctx: &DefcalOperandListContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#defcalOperandList}.
     * @param ctx the parse tree
     */
    fn exit_defcalOperandList(&mut self, _ctx: &DefcalOperandListContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#expressionList}.
     * @param ctx the parse tree
     */
    fn enter_expressionList(&mut self, _ctx: &ExpressionListContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#expressionList}.
     * @param ctx the parse tree
     */
    fn exit_expressionList(&mut self, _ctx: &ExpressionListContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#identifierList}.
     * @param ctx the parse tree
     */
    fn enter_identifierList(&mut self, _ctx: &IdentifierListContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#identifierList}.
     * @param ctx the parse tree
     */
    fn exit_identifierList(&mut self, _ctx: &IdentifierListContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#gateOperandList}.
     * @param ctx the parse tree
     */
    fn enter_gateOperandList(&mut self, _ctx: &GateOperandListContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#gateOperandList}.
     * @param ctx the parse tree
     */
    fn exit_gateOperandList(&mut self, _ctx: &GateOperandListContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link qasm3#externArgumentList}.
     * @param ctx the parse tree
     */
    fn enter_externArgumentList(&mut self, _ctx: &ExternArgumentListContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link qasm3#externArgumentList}.
     * @param ctx the parse tree
     */
    fn exit_externArgumentList(&mut self, _ctx: &ExternArgumentListContext<'input>) {}
}

antlr_rust::coerce_from! { 'input : qasm3Listener<'input> }
