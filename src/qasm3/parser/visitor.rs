use crate::qasm3::parser::antlr::qasm3VisitorCompat;
use crate::qasm3::parser::antlr::qasm3parser::*;

use antlr_rust::token::Token;
use antlr_rust::tree::ParseTree;
use antlr_rust::tree::ParseTreeVisitorCompat;
use antlr_rust::tree::Tree;

use crate::qasm3::ir;

use std::any::Any;

pub struct VisitorReturn {
    data: Box<dyn Any>,
}

impl Default for VisitorReturn {
    fn default() -> Self {
        VisitorReturn::newt(())
    }
}

impl VisitorReturn {
    fn new(data: Box<dyn Any>) -> Self {
        VisitorReturn { data }
    }

    fn newt<T: 'static>(value: T) -> Self {
        Self::new(Box::new(value))
    }

    fn downcast<T: 'static>(self) -> T {
        *self.data.downcast().unwrap()
    }

    pub fn result(self) -> ir::Program {
        self.downcast()
    }
}

pub struct Visitor(VisitorReturn);

impl Visitor {
    pub fn new() -> Self {
        Visitor(VisitorReturn::default())
    }
}

impl<'input> ParseTreeVisitorCompat<'input> for Visitor {
    type Node = qasm3ContextType;
    type Return = VisitorReturn;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.0
        //&mut self.0
    }

    fn aggregate_results(&self, _: Self::Return, _: Self::Return) -> Self::Return {
        panic!("Should not be reachable")
    }
}

#[allow(non_snake_case)]
#[allow(unused_variables)]
impl<'input> qasm3VisitorCompat<'input> for Visitor {
    fn visit_program(&mut self, ctx: &ProgramContext<'input>) -> Self::Return {
        let version = match ctx.version() {
            Some(version) => Some(self.visit_version(&*version).downcast::<ir::Version>()),
            None => None,
        };

        let statement_or_scope_vec = ctx
            .statementOrScope_all()
            .iter()
            .map(|x| {
                self.visit_statementOrScope(x)
                    .downcast::<ir::StatementOrScope>()
            })
            .collect();
        VisitorReturn::newt(ir::Program::new(version, statement_or_scope_vec))
    }

    fn visit_version(&mut self, ctx: &VersionContext<'input>) -> Self::Return {
        VisitorReturn::newt(ir::Version::new(ctx.VersionSpecifier().unwrap().get_text()))
    }

    fn visit_statement(&mut self, ctx: &StatementContext<'input>) -> Self::Return {
        let mut statement: ir::Statement = self
            .visit(&*ctx.get_child(ctx.get_child_count() - 1).unwrap())
            .downcast::<ir::Statement>();

        let annotations: Vec<_> = ctx
            .annotation_all()
            .iter()
            .map(|x| self.visit_annotation(x).downcast::<ir::Annotation>())
            .collect();
        if !annotations.is_empty() {
            statement = ir::Annotated::newt(annotations, statement);
        }

        VisitorReturn::newt(statement)
    }

    fn visit_annotation(&mut self, ctx: &AnnotationContext<'input>) -> Self::Return {
        let id_text = ctx.AnnotationKeyword().unwrap().get_text();
        let mut id_chars = id_text.chars();
        id_chars.next();
        let identifier = ir::Identifier::new(id_chars.collect());
        let content = match ctx.RemainingLineContent() {
            Some(content) => content.get_text(),
            None => "".to_string(),
        };
        VisitorReturn::newt(ir::Annotation::new(identifier, content))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#scope}.
     * @param ctx the parse tree
     */
    fn visit_scope(&mut self, ctx: &ScopeContext<'input>) -> Self::Return {
        VisitorReturn::newt(ir::Scope::new(
            ctx.statementOrScope_all()
                .iter()
                .map(|x| {
                    self.visit_statementOrScope(x)
                        .downcast::<ir::StatementOrScope>()
                })
                .collect(),
        ))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#pragma}.
     * @param ctx the parse tree
     */
    fn visit_pragma(&mut self, ctx: &PragmaContext<'input>) -> Self::Return {
        VisitorReturn::newt(ir::Pragma::new(
            ctx.RemainingLineContent().unwrap().get_text(),
        ))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#statementOrScope}.
     * @param ctx the parse tree
     */
    fn visit_statementOrScope(&mut self, ctx: &StatementOrScopeContext<'input>) -> Self::Return {
        let statement_or_scope: ir::StatementOrScope = if ctx.statement().is_some() {
            self.visit_statement(&*ctx.statement().unwrap())
                .downcast::<ir::Statement>()
                .into()
        } else {
            self.visit_scope(&*ctx.scope().unwrap())
                .downcast::<ir::Scope>()
                .into()
        };

        VisitorReturn::newt(statement_or_scope)
    }

    /**
     * Visit a parse tree produced by {@link qasm3#calibrationGrammarStatement}.
     * @param ctx the parse tree
     */
    fn visit_calibrationGrammarStatement(
        &mut self,
        ctx: &CalibrationGrammarStatementContext<'input>,
    ) -> Self::Return {
        VisitorReturn::newt(ir::CalibrationGrammar::new(
            ctx.StringLiteral().unwrap().get_text(),
        ))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#includeStatement}.
     * @param ctx the parse tree
     */
    fn visit_includeStatement(&mut self, ctx: &IncludeStatementContext<'input>) -> Self::Return {
        VisitorReturn::newt(ir::Include::new(ctx.StringLiteral().unwrap().get_text()))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#breakStatement}.
     * @param ctx the parse tree
     */
    fn visit_breakStatement(&mut self, ctx: &BreakStatementContext<'input>) -> Self::Return {
        VisitorReturn::newt(ir::Statement::Break)
    }

    /**
     * Visit a parse tree produced by {@link qasm3#continueStatement}.
     * @param ctx the parse tree
     */
    fn visit_continueStatement(&mut self, ctx: &ContinueStatementContext<'input>) -> Self::Return {
        VisitorReturn::newt(ir::Statement::Continue)
    }

    /**
     * Visit a parse tree produced by {@link qasm3#endStatement}.
     * @param ctx the parse tree
     */
    fn visit_endStatement(&mut self, ctx: &EndStatementContext<'input>) -> Self::Return {
        VisitorReturn::newt(ir::Statement::End)
    }

    /**
     * Visit a parse tree produced by {@link qasm3#forStatement}.
     * @param ctx the parse tree
     */
    fn visit_forStatement(&mut self, ctx: &ForStatementContext<'input>) -> Self::Return {
        let expr = if ctx.setExpression().is_some() {
            self.visit(&*ctx.setExpression().unwrap())
                .downcast::<ir::Expression>()
        } else if ctx.rangeExpression().is_some() {
            self.visit(&*ctx.rangeExpression().unwrap())
                .downcast::<ir::Expression>()
        } else {
            self.visit(&*ctx.expression().unwrap())
                .downcast::<ir::Expression>()
        };

        let sos = self
            .visit(&*ctx.statementOrScope().unwrap())
            .downcast::<ir::StatementOrScope>();

        VisitorReturn::newt(ir::For::new(
            self.visit(&*ctx.scalarType().unwrap())
                .downcast::<ir::types::Scalar>(),
            ir::Identifier::new(ctx.Identifier().unwrap().get_text()),
            expr,
            match sos {
                ir::StatementOrScope::Scope(scope) => scope,
                ir::StatementOrScope::Statement(_) => ir::Scope::new(vec![sos]),
            },
        ))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#ifStatement}.
     * @param ctx the parse tree
     */
    fn visit_ifStatement(&mut self, ctx: &IfStatementContext<'input>) -> Self::Return {
        VisitorReturn::newt(ir::If::new(
            self.visit(&*ctx.expression().unwrap())
                .downcast::<ir::Expression>(),
            self.visit(&**ctx.if_body.as_ref().unwrap())
                .downcast::<ir::StatementOrScope>(),
            ctx.else_body
                .as_ref()
                .map(|x| self.visit(&**x).downcast::<ir::StatementOrScope>()),
        ))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#returnStatement}.
     * @param ctx the parse tree
     */
    fn visit_returnStatement(&mut self, ctx: &ReturnStatementContext<'input>) -> Self::Return {
        VisitorReturn::newt(ir::Return::new(
            ctx.expression()
                .map(|x| self.visit(&*x).downcast::<ir::Expression>()),
        ))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#whileStatement}.
     * @param ctx the parse tree
     */
    fn visit_whileStatement(&mut self, ctx: &WhileStatementContext<'input>) -> Self::Return {
        VisitorReturn::newt(ir::While::new(
            self.visit(&*ctx.expression().unwrap())
                .downcast::<ir::Expression>(),
            self.visit(&*ctx.statementOrScope().unwrap())
                .downcast::<ir::StatementOrScope>(),
        ))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#switchStatement}.
     * @param ctx the parse tree
     */
    fn visit_switchStatement(&mut self, ctx: &SwitchStatementContext<'input>) -> Self::Return {
        let mut default = None;
        let mut items = vec![];
        for item in ctx.switchCaseItem_all() {
            if item.DEFAULT().is_some() {
                default = Some(self.visit(&*item.scope().unwrap()).downcast::<ir::Scope>());
            } else {
                items.push(self.visit(&*item).downcast::<ir::SwitchItem>());
            }
        }

        VisitorReturn::newt(ir::Switch::new(
            self.visit(&*ctx.expression().unwrap())
                .downcast::<ir::Expression>(),
            items,
            default,
        ))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#switchCaseItem}.
     * @param ctx the parse tree
     */
    fn visit_switchCaseItem(&mut self, ctx: &SwitchCaseItemContext<'input>) -> Self::Return {
        VisitorReturn::newt(ir::SwitchItem::new(
            self.visit(&*ctx.expressionList().unwrap())
                .downcast::<Vec<ir::Expression>>(),
            self.visit(&*ctx.scope().unwrap()).downcast::<ir::Scope>(),
        ))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#barrierStatement}.
     * @param ctx the parse tree
     */
    fn visit_barrierStatement(&mut self, ctx: &BarrierStatementContext<'input>) -> Self::Return {
        VisitorReturn::newt(ir::Barrier::new(
            ctx.gateOperandList()
                .map(|x| self.visit(&*x).downcast::<Vec<ir::Expression>>())
                .unwrap_or(vec![]),
        ))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#boxStatement}.
     * @param ctx the parse tree
     */
    fn visit_boxStatement(&mut self, ctx: &BoxStatementContext<'input>) -> Self::Return {
        VisitorReturn::newt(ir::BoxStatement::new(
            ctx.designator()
                .map(|x| self.visit(&*x).downcast::<ir::Expression>()),
            self.visit(&*ctx.scope().unwrap()).downcast::<ir::Scope>(),
        ))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#delayStatement}.
     * @param ctx the parse tree
     */
    fn visit_delayStatement(&mut self, ctx: &DelayStatementContext<'input>) -> Self::Return {
        VisitorReturn::newt(ir::Delay::new(
            self.visit(&*ctx.designator().unwrap())
                .downcast::<ir::Expression>(),
            ctx.gateOperandList()
                .map(|x| self.visit(&*x).downcast::<Vec<ir::Expression>>())
                .unwrap_or(vec![]),
        ))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#gateCallStatement}.
     * @param ctx the parse tree
     */
    fn visit_gateCallStatement(&mut self, ctx: &GateCallStatementContext<'input>) -> Self::Return {
        let gate_mods = ctx
            .gateModifier_all()
            .iter()
            .map(|x| self.visit_gateModifier(&*x).downcast::<ir::GateMod>())
            .collect();
        let params = ctx
            .expressionList()
            .map(|x| self.visit(&*x).downcast::<Vec<ir::Expression>>())
            .unwrap_or(vec![]);
        let expr = ctx
            .designator()
            .map(|x| self.visit(&*x).downcast::<ir::Expression>());
        let args = ctx
            .gateOperandList()
            .map(|x| self.visit(&*x).downcast::<Vec<ir::Expression>>())
            .unwrap_or(vec![]);

        let id = if ctx.GPHASE().is_some() {
            ir::Identifier::new("gphase".to_string())
        } else {
            ir::Identifier::new(ctx.Identifier().unwrap().get_text())
        };

        VisitorReturn::newt(ir::GateCall::new(gate_mods, id, params, expr, args))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#measureArrowAssignmentStatement}.
     * @param ctx the parse tree
     */
    fn visit_measureArrowAssignmentStatement(
        &mut self,
        ctx: &MeasureArrowAssignmentStatementContext<'input>,
    ) -> Self::Return {
        VisitorReturn::newt(ir::MeasureArrowAssignment::new(
            self.visit(&*ctx.measureExpression().unwrap())
                .downcast::<ir::Expression>(),
            ctx.indexedIdentifier()
                .map(|x| self.visit(&*x).downcast::<ir::Expression>()),
        ))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#resetStatement}.
     * @param ctx the parse tree
     */
    fn visit_resetStatement(&mut self, ctx: &ResetStatementContext<'input>) -> Self::Return {
        VisitorReturn::newt(ir::Reset::new(
            self.visit(&*ctx.gateOperand().unwrap())
                .downcast::<ir::Expression>(),
        ))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#aliasDeclarationStatement}.
     * @param ctx the parse tree
     */
    fn visit_aliasDeclarationStatement(
        &mut self,
        ctx: &AliasDeclarationStatementContext<'input>,
    ) -> Self::Return {
        VisitorReturn::newt(ir::AliasDeclaration::new(
            ir::Identifier::new(ctx.Identifier().unwrap().get_text()),
            self.visit(&*ctx.aliasExpression().unwrap())
                .downcast::<ir::Expression>(),
        ))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#classicalDeclarationStatement}.
     * @param ctx the parse tree
     */
    fn visit_classicalDeclarationStatement(
        &mut self,
        ctx: &ClassicalDeclarationStatementContext<'input>,
    ) -> Self::Return {
        let type_ = match ctx.scalarType() {
            Some(x) => self.visit(&*x).downcast::<ir::Type>(),
            None => self
                .visit(&*ctx.arrayType().unwrap())
                .downcast::<ir::Type>(),
        };
        let id = ir::Identifier::new(ctx.Identifier().unwrap().get_text());
        let expr = ctx
            .declarationExpression()
            .map(|x| self.visit(&*x).downcast::<ir::Expression>());

        VisitorReturn::newt(ir::ClassicalDeclaration::new(type_, id, expr))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#constDeclarationStatement}.
     * @param ctx the parse tree
     */
    fn visit_constDeclarationStatement(
        &mut self,
        ctx: &ConstDeclarationStatementContext<'input>,
    ) -> Self::Return {
        VisitorReturn::newt(ir::ConstDeclaration::new(
            self.visit(&*ctx.scalarType().unwrap())
                .downcast::<ir::Type>(),
            ir::Identifier::new(ctx.Identifier().unwrap().get_text()),
            self.visit(&*ctx.declarationExpression().unwrap())
                .downcast::<ir::Expression>(),
        ))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#ioDeclarationStatement}.
     * @param ctx the parse tree
     */
    fn visit_ioDeclarationStatement(
        &mut self,
        ctx: &IoDeclarationStatementContext<'input>,
    ) -> Self::Return {
        let io_type = match ctx.INPUT() {
            Some(_) => ir::IOType::In,
            None => ir::IOType::Out,
        };
        let type_ = match ctx.scalarType() {
            Some(x) => self.visit(&*x).downcast::<ir::Type>(),
            None => self
                .visit(&*ctx.arrayType().unwrap())
                .downcast::<ir::Type>(),
        };
        let id = ir::Identifier::new(ctx.Identifier().unwrap().get_text());

        VisitorReturn::newt(ir::IODeclaration::new(io_type, type_, id))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#oldStyleDeclarationStatement}.
     * @param ctx the parse tree
     */
    fn visit_oldStyleDeclarationStatement(
        &mut self,
        ctx: &OldStyleDeclarationStatementContext<'input>,
    ) -> Self::Return {
        let reg_type = match ctx.QREG() {
            Some(_) => ir::types::Register::Q,
            None => ir::types::Register::C,
        };
        let id = ir::Identifier::new(ctx.Identifier().unwrap().get_text());
        let expr = ctx
            .designator()
            .map(|x| self.visit(&*x).downcast::<ir::Expression>());
        VisitorReturn::newt(ir::OldStyleDeclaration::new(reg_type.into(), id, expr))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#quantumDeclarationStatement}.
     * @param ctx the parse tree
     */
    fn visit_quantumDeclarationStatement(
        &mut self,
        ctx: &QuantumDeclarationStatementContext<'input>,
    ) -> Self::Return {
        VisitorReturn::newt(ir::QuantumDeclaration::new(
            self.visit(&*ctx.qubitType().unwrap())
                .downcast::<ir::Type>(),
            ir::Identifier::new(ctx.Identifier().unwrap().get_text()),
        ))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#defStatement}.
     * @param ctx the parse tree
     */
    fn visit_defStatement(&mut self, ctx: &DefStatementContext<'input>) -> Self::Return {
        let id = ir::Identifier::new(ctx.Identifier().unwrap().get_text());
        let args = ctx
            .argumentDefinitionList()
            .map(|x| self.visit(&*x).downcast::<Vec<ir::DefArgument>>())
            .unwrap_or(vec![]);
        let ret_type = ctx
            .returnSignature()
            .map(|x| self.visit(&*x).downcast::<ir::Type>());
        let scope = self.visit(&*ctx.scope().unwrap()).downcast::<ir::Scope>();
        VisitorReturn::newt(ir::Def::new(id, args, ret_type, scope))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#externStatement}.
     * @param ctx the parse tree
     */
    fn visit_externStatement(&mut self, ctx: &ExternStatementContext<'input>) -> Self::Return {
        let id = ir::Identifier::new(ctx.Identifier().unwrap().get_text());
        let args = ctx
            .externArgumentList()
            .map(|x| self.visit(&*x).downcast::<Vec<ir::ExternArgument>>())
            .unwrap_or(vec![]);
        let ret_type = ctx
            .returnSignature()
            .map(|x| self.visit(&*x).downcast::<ir::Type>());
        VisitorReturn::newt(ir::Extern::new(id, args, ret_type))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#gateStatement}.
     * @param ctx the parse tree
     */
    fn visit_gateStatement(&mut self, ctx: &GateStatementContext<'input>) -> Self::Return {
        let mut params = vec![];
        if let Some(id_list) = &ctx.params {
            params = self.visit(&**id_list).downcast::<Vec<ir::Identifier>>();
        }
        let args = self
            .visit(&**ctx.qubits.as_ref().unwrap())
            .downcast::<Vec<ir::Identifier>>();

        VisitorReturn::newt(ir::Gate::new(
            ir::Identifier::new(ctx.Identifier().unwrap().get_text()),
            params,
            args,
            self.visit(&*ctx.scope().unwrap()).downcast::<ir::Scope>(),
        ))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#assignmentStatement}.
     * @param ctx the parse tree
     */
    fn visit_assignmentStatement(
        &mut self,
        ctx: &AssignmentStatementContext<'input>,
    ) -> Self::Return {
        let assign_op = ctx.CompoundAssignmentOperator().map(|op| {
            let s = op.get_text();
            ir::BinaryOperator::from_str(&s[0..s.len() - 1])
        });
        let id_expr = self
            .visit(&*ctx.indexedIdentifier().unwrap())
            .downcast::<ir::Expression>();
        let expr = match ctx.expression() {
            Some(expr) => self.visit(&*expr).downcast::<ir::Expression>(),
            None => self
                .visit(&*ctx.measureExpression().unwrap())
                .downcast::<ir::Expression>(),
        };

        VisitorReturn::newt(ir::Assignment::new(id_expr, assign_op, expr))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#expressionStatement}.
     * @param ctx the parse tree
     */
    fn visit_expressionStatement(
        &mut self,
        ctx: &ExpressionStatementContext<'input>,
    ) -> Self::Return {
        VisitorReturn::newt(
            self.visit(&*ctx.expression().unwrap())
                .downcast::<ir::Expression>(),
        )
    }

    /**
     * Visit a parse tree produced by {@link qasm3#calStatement}.
     * @param ctx the parse tree
     */
    fn visit_calStatement(&mut self, ctx: &CalStatementContext<'input>) -> Self::Return {
        VisitorReturn::newt(ir::Cal::new(ctx.CalibrationBlock().map(|x| x.get_text())))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#defcalStatement}.
     * @param ctx the parse tree
     */
    fn visit_defcalStatement(&mut self, ctx: &DefcalStatementContext<'input>) -> Self::Return {
        let target = self
            .visit(&*ctx.defcalTarget().unwrap())
            .downcast::<ir::DefcalTarget>();
        let args = ctx
            .defcalArgumentDefinitionList()
            .map(|x| self.visit(&*x).downcast::<Vec<ir::DefcalArgument>>())
            .unwrap_or(vec![]);
        let operands = self
            .visit(&*ctx.defcalOperandList().unwrap())
            .downcast::<Vec<ir::Expression>>();
        let ret_type = ctx
            .returnSignature()
            .map(|x| self.visit(&*x).downcast::<ir::Type>());
        let cal_block = ctx.CalibrationBlock().map(|x| x.get_text());
        VisitorReturn::newt(ir::Defcal::new(target, args, operands, ret_type, cal_block))
    }

    /**
     * Visit a parse tree produced by the {@code bitwiseXorExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn visit_bitwiseXorExpression(
        &mut self,
        ctx: &BitwiseXorExpressionContext<'input>,
    ) -> Self::Return {
        VisitorReturn::newt(ir::BinaryOperation::new(
            ir::BinaryOperator::BitXor,
            self.visit(&*ctx.expression(0).unwrap())
                .downcast::<ir::Expression>(),
            self.visit(&*ctx.expression(1).unwrap())
                .downcast::<ir::Expression>(),
        ))
    }

    /**
     * Visit a parse tree produced by the {@code additiveExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn visit_additiveExpression(
        &mut self,
        ctx: &AdditiveExpressionContext<'input>,
    ) -> Self::Return {
        VisitorReturn::newt(ir::BinaryOperation::new(
            ir::BinaryOperator::from_str(ctx.op.as_ref().unwrap().get_text()),
            self.visit(&*ctx.expression(0).unwrap())
                .downcast::<ir::Expression>(),
            self.visit(&*ctx.expression(1).unwrap())
                .downcast::<ir::Expression>(),
        ))
    }

    /**
     * Visit a parse tree produced by the {@code durationofExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn visit_durationofExpression(
        &mut self,
        ctx: &DurationofExpressionContext<'input>,
    ) -> Self::Return {
        VisitorReturn::newt(ir::DurationOf::new(
            self.visit(&*ctx.scope().unwrap()).downcast::<ir::Scope>(),
        ))
    }

    /**
     * Visit a parse tree produced by the {@code parenthesisExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn visit_parenthesisExpression(
        &mut self,
        ctx: &ParenthesisExpressionContext<'input>,
    ) -> Self::Return {
        VisitorReturn::newt(
            self.visit(&*ctx.expression().unwrap())
                .downcast::<ir::Expression>(),
        )
    }

    /**
     * Visit a parse tree produced by the {@code comparisonExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn visit_comparisonExpression(
        &mut self,
        ctx: &ComparisonExpressionContext<'input>,
    ) -> Self::Return {
        VisitorReturn::newt(ir::BinaryOperation::new(
            ir::BinaryOperator::from_str(ctx.op.as_ref().unwrap().get_text()),
            self.visit(&*ctx.expression(0).unwrap())
                .downcast::<ir::Expression>(),
            self.visit(&*ctx.expression(1).unwrap())
                .downcast::<ir::Expression>(),
        ))
    }

    /**
     * Visit a parse tree produced by the {@code multiplicativeExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn visit_multiplicativeExpression(
        &mut self,
        ctx: &MultiplicativeExpressionContext<'input>,
    ) -> Self::Return {
        VisitorReturn::newt(ir::BinaryOperation::new(
            ir::BinaryOperator::from_str(ctx.op.as_ref().unwrap().get_text()),
            self.visit(&*ctx.expression(0).unwrap())
                .downcast::<ir::Expression>(),
            self.visit(&*ctx.expression(1).unwrap())
                .downcast::<ir::Expression>(),
        ))
    }

    /**
     * Visit a parse tree produced by the {@code logicalOrExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn visit_logicalOrExpression(
        &mut self,
        ctx: &LogicalOrExpressionContext<'input>,
    ) -> Self::Return {
        VisitorReturn::newt(ir::BinaryOperation::new(
            ir::BinaryOperator::Or,
            self.visit(&*ctx.expression(0).unwrap())
                .downcast::<ir::Expression>(),
            self.visit(&*ctx.expression(1).unwrap())
                .downcast::<ir::Expression>(),
        ))
    }

    /**
     * Visit a parse tree produced by the {@code castExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn visit_castExpression(&mut self, ctx: &CastExpressionContext<'input>) -> Self::Return {
        let cast_type = match ctx.scalarType() {
            Some(x) => self.visit(&*x).downcast::<ir::Type>(),
            None => self
                .visit(&*ctx.arrayType().unwrap())
                .downcast::<ir::Type>(),
        };
        VisitorReturn::newt(ir::Cast::new(
            cast_type,
            self.visit(&*ctx.expression().unwrap())
                .downcast::<ir::Expression>(),
        ))
    }

    /**
     * Visit a parse tree produced by the {@code powerExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn visit_powerExpression(&mut self, ctx: &PowerExpressionContext<'input>) -> Self::Return {
        VisitorReturn::newt(ir::BinaryOperation::new(
            ir::BinaryOperator::Pow,
            self.visit(&*ctx.expression(0).unwrap())
                .downcast::<ir::Expression>(),
            self.visit(&*ctx.expression(1).unwrap())
                .downcast::<ir::Expression>(),
        ))
    }

    /**
     * Visit a parse tree produced by the {@code bitwiseOrExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn visit_bitwiseOrExpression(
        &mut self,
        ctx: &BitwiseOrExpressionContext<'input>,
    ) -> Self::Return {
        VisitorReturn::newt(ir::BinaryOperation::new(
            ir::BinaryOperator::BitOr,
            self.visit(&*ctx.expression(0).unwrap())
                .downcast::<ir::Expression>(),
            self.visit(&*ctx.expression(1).unwrap())
                .downcast::<ir::Expression>(),
        ))
    }

    /**
     * Visit a parse tree produced by the {@code callExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn visit_callExpression(&mut self, ctx: &CallExpressionContext<'input>) -> Self::Return {
        VisitorReturn::newt(ir::Call::new(
            ir::Identifier::new(ctx.Identifier().unwrap().get_text()),
            ctx.expressionList()
                .map(|x| self.visit(&*x).downcast::<Vec<ir::Expression>>())
                .unwrap_or_default(),
        ))
    }

    /**
     * Visit a parse tree produced by the {@code bitshiftExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn visit_bitshiftExpression(
        &mut self,
        ctx: &BitshiftExpressionContext<'input>,
    ) -> Self::Return {
        VisitorReturn::newt(ir::BinaryOperation::new(
            ir::BinaryOperator::from_str(&ctx.BitshiftOperator().unwrap().get_text()),
            self.visit(&*ctx.expression(0).unwrap())
                .downcast::<ir::Expression>(),
            self.visit(&*ctx.expression(1).unwrap())
                .downcast::<ir::Expression>(),
        ))
    }

    /**
     * Visit a parse tree produced by the {@code bitwiseAndExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn visit_bitwiseAndExpression(
        &mut self,
        ctx: &BitwiseAndExpressionContext<'input>,
    ) -> Self::Return {
        VisitorReturn::newt(ir::BinaryOperation::new(
            ir::BinaryOperator::BitAnd,
            self.visit(&*ctx.expression(0).unwrap())
                .downcast::<ir::Expression>(),
            self.visit(&*ctx.expression(1).unwrap())
                .downcast::<ir::Expression>(),
        ))
    }

    /**
     * Visit a parse tree produced by the {@code equalityExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn visit_equalityExpression(
        &mut self,
        ctx: &EqualityExpressionContext<'input>,
    ) -> Self::Return {
        VisitorReturn::newt(ir::BinaryOperation::new(
            ir::BinaryOperator::from_str(&ctx.EqualityOperator().unwrap().get_text()),
            self.visit(&*ctx.expression(0).unwrap())
                .downcast::<ir::Expression>(),
            self.visit(&*ctx.expression(1).unwrap())
                .downcast::<ir::Expression>(),
        ))
    }

    /**
     * Visit a parse tree produced by the {@code logicalAndExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn visit_logicalAndExpression(
        &mut self,
        ctx: &LogicalAndExpressionContext<'input>,
    ) -> Self::Return {
        VisitorReturn::newt(ir::BinaryOperation::new(
            ir::BinaryOperator::And,
            self.visit(&*ctx.expression(0).unwrap())
                .downcast::<ir::Expression>(),
            self.visit(&*ctx.expression(1).unwrap())
                .downcast::<ir::Expression>(),
        ))
    }

    /**
     * Visit a parse tree produced by the {@code indexExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn visit_indexExpression(&mut self, ctx: &IndexExpressionContext<'input>) -> Self::Return {
        VisitorReturn::newt(ir::Index::new(
            self.visit(&*ctx.expression().unwrap())
                .downcast::<ir::Expression>(),
            self.visit(&*ctx.indexOperator().unwrap())
                .downcast::<ir::Expression>(),
        ))
    }

    /**
     * Visit a parse tree produced by the {@code unaryExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'input>) -> Self::Return {
        VisitorReturn::newt(ir::UnaryOperation::new(
            ir::UnaryOperator::from_str(ctx.op.as_ref().unwrap().get_text()),
            self.visit(&*ctx.expression().unwrap())
                .downcast::<ir::Expression>(),
        ))
    }

    /**
     * Visit a parse tree produced by the {@code literalExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn visit_literalExpression(&mut self, ctx: &LiteralExpressionContext<'input>) -> Self::Return {
        VisitorReturn::newt(if ctx.Identifier().is_some() {
            ir::Identifier::newt(ctx.Identifier().unwrap().get_text())
        } else if ctx.BinaryIntegerLiteral().is_some() {
            ir::Literal::BinaryInteger(
                ctx.BinaryIntegerLiteral()
                    .unwrap()
                    .get_text()
                    .parse()
                    .unwrap(),
            )
        } else if ctx.OctalIntegerLiteral().is_some() {
            ir::Literal::OctalInteger(
                ctx.OctalIntegerLiteral()
                    .unwrap()
                    .get_text()
                    .parse()
                    .unwrap(),
            )
        } else if ctx.DecimalIntegerLiteral().is_some() {
            ir::Literal::DecimalInteger(
                ctx.DecimalIntegerLiteral()
                    .unwrap()
                    .get_text()
                    .parse()
                    .unwrap(),
            )
        } else if ctx.HexIntegerLiteral().is_some() {
            ir::Literal::HexInteger(ctx.HexIntegerLiteral().unwrap().get_text().parse().unwrap())
        } else if ctx.FloatLiteral().is_some() {
            ir::Literal::Float(ctx.FloatLiteral().unwrap().get_text().parse().unwrap())
        } else if ctx.ImaginaryLiteral().is_some() {
            ir::Literal::Imaginary(ctx.ImaginaryLiteral().unwrap().get_text().parse().unwrap())
        } else if ctx.BooleanLiteral().is_some() {
            ir::Literal::Boolean(ctx.BooleanLiteral().unwrap().get_text().parse().unwrap())
        } else if ctx.BitstringLiteral().is_some() {
            ir::Literal::BitString(ctx.BitstringLiteral().unwrap().get_text().parse().unwrap())
        } else if ctx.TimingLiteral().is_some() {
            ir::Literal::Timing(1.0, ir::TimingUnit::S)
            // TODO: Parse correctly via regex?
            //ir::Literal::Timing(ctx.TimingLiteral().unwrap().get_text().parse().unwrap())
        } else {
            ir::Literal::HardwareQubit(ctx.HardwareQubit().unwrap().get_text().parse().unwrap())
        })
    }

    /**
     * Visit a parse tree produced by {@link qasm3#aliasExpression}.
     * @param ctx the parse tree
     */
    fn visit_aliasExpression(&mut self, ctx: &AliasExpressionContext<'input>) -> Self::Return {
        VisitorReturn::newt(
            ctx.expression_all()
                .iter()
                .map(|e| self.visit(&**e).downcast::<ir::Expression>())
                .collect::<Vec<_>>(),
        )
    }

    /**
     * Visit a parse tree produced by {@link qasm3#declarationExpression}.
     * @param ctx the parse tree
     */
    fn visit_declarationExpression(
        &mut self,
        ctx: &DeclarationExpressionContext<'input>,
    ) -> Self::Return {
        VisitorReturn::newt(if ctx.arrayLiteral().is_some() {
            self.visit(&*ctx.arrayLiteral().unwrap())
                .downcast::<ir::expressions::Array>()
                .into()
        } else if ctx.expression().is_some() {
            self.visit(&*ctx.expression().unwrap())
                .downcast::<ir::Expression>()
        } else {
            self.visit(&*ctx.measureExpression().unwrap())
                .downcast::<ir::expressions::Measure>()
                .into()
        })
    }

    /**
     * Visit a parse tree produced by {@link qasm3#measureExpression}.
     * @param ctx the parse tree
     */
    fn visit_measureExpression(&mut self, ctx: &MeasureExpressionContext<'input>) -> Self::Return {
        VisitorReturn::newt(ir::expressions::Measure::new(
            self.visit(&*ctx.gateOperand().unwrap())
                .downcast::<ir::Expression>(),
        ))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#rangeExpression}.
     * @param ctx the parse tree
     */
    fn visit_rangeExpression(&mut self, ctx: &RangeExpressionContext<'input>) -> Self::Return {
        let from_expr = ctx
            .from
            .as_ref()
            .map(|e| self.visit(&**e).downcast::<ir::Expression>());
        let to_expr = ctx
            .to
            .as_ref()
            .map(|e| self.visit(&**e).downcast::<ir::Expression>());
        let step_expr = ctx
            .step
            .as_ref()
            .map(|e| self.visit(&**e).downcast::<ir::Expression>());

        VisitorReturn::newt(ir::Range::new(from_expr, to_expr, step_expr))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#setExpression}.
     * @param ctx the parse tree
     */
    fn visit_setExpression(&mut self, ctx: &SetExpressionContext<'input>) -> Self::Return {
        VisitorReturn::newt(ir::expressions::Array::new(
            ctx.expression_all()
                .iter()
                .map(|e| self.visit(&**e).downcast::<ir::Expression>())
                .collect::<Vec<_>>(),
        ))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#arrayLiteral}.
     * @param ctx the parse tree
     */
    fn visit_arrayLiteral(&mut self, ctx: &ArrayLiteralContext<'input>) -> Self::Return {
        VisitorReturn::newt(ir::expressions::Array::new(
            ctx.arrayLiteralElement_all()
                .iter()
                .map(|e| self.visit(&**e).downcast::<ir::Expression>())
                .collect::<Vec<_>>(),
        ))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#arrayLiteralElement}.
     * @param ctx the parse tree
     */
    fn visit_arrayLiteralElement(
        &mut self,
        ctx: &ArrayLiteralElementContext<'input>,
    ) -> Self::Return {
        VisitorReturn::newt(match ctx.expression() {
            Some(e) => self.visit(&*e).downcast::<ir::Expression>(),
            None => self
                .visit(&*ctx.arrayLiteral().unwrap())
                .downcast::<ir::Expression>(),
        })
    }

    /**
     * Visit a parse tree produced by {@link qasm3#indexOperator}.
     * @param ctx the parse tree
     */
    fn visit_indexOperator(&mut self, ctx: &IndexOperatorContext<'input>) -> Self::Return {
        VisitorReturn::newt(match ctx.setExpression() {
            Some(e) => self.visit(&*e).downcast::<ir::Expression>(),
            None => ir::expressions::Array::new(
                ctx.indexOperatorElement_all()
                    .iter()
                    .map(|x| self.visit(&**x).downcast::<ir::Expression>())
                    .collect::<Vec<_>>(),
            )
            .into(),
        })
    }

    /**
     * Visit a parse tree produced by {@link qasm3#indexOperatorElement}.
     * @param ctx the parse tree
     */
    fn visit_indexOperatorElement(
        &mut self,
        ctx: &IndexOperatorElementContext<'input>,
    ) -> Self::Return {
        VisitorReturn::newt(match ctx.expression() {
            Some(e) => self.visit(&*e).downcast::<ir::Expression>(),
            None => self
                .visit(&*ctx.rangeExpression().unwrap())
                .downcast::<ir::Expression>(),
        })
    }

    /**
     * Visit a parse tree produced by {@link qasm3#indexedIdentifier}.
     * @param ctx the parse tree
     */
    fn visit_indexedIdentifier(&mut self, ctx: &IndexedIdentifierContext<'input>) -> Self::Return {
        let id = ir::Identifier::new(ctx.Identifier().unwrap().get_text());
        let expr = ctx.indexOperator_all().iter().fold(id.into(), |acc, e| {
            ir::Index::newt(acc, self.visit(&**e).downcast::<ir::Expression>())
        });
        VisitorReturn::newt(expr)
    }

    /**
     * Visit a parse tree produced by {@link qasm3#returnSignature}.
     * @param ctx the parse tree
     */
    fn visit_returnSignature(&mut self, ctx: &ReturnSignatureContext<'input>) -> Self::Return {
        VisitorReturn::newt(
            self.visit(&*ctx.scalarType().unwrap())
                .downcast::<ir::Type>(),
        )
    }

    /**
     * Visit a parse tree produced by {@link qasm3#gateModifier}.
     * @param ctx the parse tree
     */
    fn visit_gateModifier(&mut self, ctx: &GateModifierContext<'input>) -> Self::Return {
        let expr = ctx
            .expression()
            .map(|x| self.visit(&*x).downcast::<ir::Expression>());

        VisitorReturn::newt(if ctx.INV().is_some() {
            ir::GateMod::Inv
        } else if ctx.POW().is_some() {
            ir::GateMod::Pow(expr.unwrap())
        } else if ctx.CTRL().is_some() {
            ir::GateMod::Ctrl(expr)
        } else {
            ir::GateMod::NegCtrl(expr)
        })
    }

    /**
     * Visit a parse tree produced by {@link qasm3#scalarType}.
     * @param ctx the parse tree
     */
    fn visit_scalarType(&mut self, ctx: &ScalarTypeContext<'input>) -> Self::Return {
        let designator = ctx
            .designator()
            .map(|x| self.visit(&*x).downcast::<ir::Expression>());
        VisitorReturn::newt(if ctx.BIT().is_some() {
            ir::types::Scalar::Bit(designator)
        } else if ctx.INT().is_some() {
            ir::types::Scalar::Int(designator)
        } else if ctx.UINT().is_some() {
            ir::types::Scalar::UInt(designator)
        } else if ctx.FLOAT().is_some() {
            ir::types::Scalar::Float(designator)
        } else if ctx.ANGLE().is_some() {
            ir::types::Scalar::Angle(designator)
        } else if ctx.BOOL().is_some() {
            ir::types::Scalar::Bool
        } else if ctx.DURATION().is_some() {
            ir::types::Scalar::Duration
        } else if ctx.STRETCH().is_some() {
            ir::types::Scalar::Stretch
        } else {
            ir::types::Scalar::Complex(Box::new(
                self.visit(&*ctx.scalarType().unwrap())
                    .downcast::<ir::types::Scalar>(),
            ))
        })
    }

    /**
     * Visit a parse tree produced by {@link qasm3#qubitType}.
     * @param ctx the parse tree
     */
    fn visit_qubitType(&mut self, ctx: &QubitTypeContext<'input>) -> Self::Return {
        VisitorReturn::newt(ir::types::Qubit::new(
            ctx.designator()
                .map(|x| self.visit(&*x).downcast::<ir::Expression>()),
        ))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#arrayType}.
     * @param ctx the parse tree
     */
    fn visit_arrayType(&mut self, ctx: &ArrayTypeContext<'input>) -> Self::Return {
        let scalar_type = self
            .visit(&*ctx.scalarType().unwrap())
            .downcast::<ir::Scalar>();
        let expr_list = self
            .visit(&*ctx.expressionList().unwrap())
            .downcast::<Vec<ir::Expression>>();

        VisitorReturn::newt(ir::types::Array::new(scalar_type, expr_list))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#arrayReferenceType}.
     * @param ctx the parse tree
     */
    fn visit_arrayReferenceType(
        &mut self,
        ctx: &ArrayReferenceTypeContext<'input>,
    ) -> Self::Return {
        let ref_type = if ctx.READONLY().is_some() {
            ir::types::Reference::ReadOnly
        } else {
            ir::types::Reference::Mutable
        };
        let scalar_type = self
            .visit(&*ctx.scalarType().unwrap())
            .downcast::<ir::Scalar>();
        let expr_list = ctx
            .expressionList()
            .map(|x| self.visit(&*x).downcast::<Vec<ir::Expression>>())
            .unwrap_or_default();
        let dim = ctx
            .expression()
            .map(|x| self.visit(&*x).downcast::<ir::Expression>());

        VisitorReturn::newt(ir::types::Array::with_reference(
            ref_type,
            scalar_type,
            expr_list,
            dim,
        ))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#designator}.
     * @param ctx the parse tree
     */
    fn visit_designator(&mut self, ctx: &DesignatorContext<'input>) -> Self::Return {
        VisitorReturn::newt(
            self.visit(&*ctx.expression().unwrap())
                .downcast::<ir::Expression>(),
        )
    }

    /**
     * Visit a parse tree produced by {@link qasm3#defcalTarget}.
     * @param ctx the parse tree
     */
    fn visit_defcalTarget(&mut self, ctx: &DefcalTargetContext<'input>) -> Self::Return {
        VisitorReturn::newt(if ctx.MEASURE().is_some() {
            ir::DefcalTarget::Measure
        } else if ctx.RESET().is_some() {
            ir::DefcalTarget::Reset
        } else if ctx.DELAY().is_some() {
            ir::DefcalTarget::Delay
        } else {
            ir::DefcalTarget::Identifier(ir::Identifier::new(ctx.Identifier().unwrap().get_text()))
        })
    }

    /**
     * Visit a parse tree produced by {@link qasm3#defcalArgumentDefinition}.
     * @param ctx the parse tree
     */
    fn visit_defcalArgumentDefinition(
        &mut self,
        ctx: &DefcalArgumentDefinitionContext<'input>,
    ) -> Self::Return {
        VisitorReturn::newt(if ctx.expression().is_some() {
            ir::DefcalArgument::Expression(
                self.visit(&*ctx.expression().unwrap())
                    .downcast::<ir::Expression>(),
            )
        } else {
            ir::DefcalArgument::DefArgument(
                self.visit(&*ctx.argumentDefinition().unwrap())
                    .downcast::<ir::DefArgument>(),
            )
        })
    }

    /**
     * Visit a parse tree produced by {@link qasm3#defcalOperand}.
     * @param ctx the parse tree
     */
    fn visit_defcalOperand(&mut self, ctx: &DefcalOperandContext<'input>) -> Self::Return {
        VisitorReturn::newt::<ir::Expression>(if ctx.HardwareQubit().is_some() {
            ir::Literal::HardwareQubit(ctx.HardwareQubit().unwrap().get_text().parse().unwrap())
                .into()
        } else {
            ir::Identifier::new(ctx.Identifier().unwrap().get_text()).into()
        })
    }

    /**
     * Visit a parse tree produced by {@link qasm3#gateOperand}.
     * @param ctx the parse tree
     */
    fn visit_gateOperand(&mut self, ctx: &GateOperandContext<'input>) -> Self::Return {
        VisitorReturn::newt(if ctx.indexedIdentifier().is_some() {
            self.visit(&*ctx.indexedIdentifier().unwrap())
                .downcast::<ir::Expression>()
        } else {
            ir::Literal::HardwareQubit(ctx.HardwareQubit().unwrap().get_text().parse().unwrap())
                .into()
        })
    }

    /**
     * Visit a parse tree produced by {@link qasm3#externArgument}.
     * @param ctx the parse tree
     */
    fn visit_externArgument(&mut self, ctx: &ExternArgumentContext<'input>) -> Self::Return {
        let type_: ir::Type = if ctx.scalarType().is_some() {
            self.visit(&*ctx.scalarType().unwrap())
                .downcast::<ir::Scalar>()
                .into()
        } else {
            self.visit(&*ctx.arrayReferenceType().unwrap())
                .downcast::<ir::types::Array>()
                .into()
        };
        VisitorReturn::newt(ir::ExternArgument::new(
            type_,
            ctx.designator()
                .map(|x| self.visit(&*x).downcast::<ir::Expression>()),
        ))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#argumentDefinition}.
     * @param ctx the parse tree
     */
    fn visit_argumentDefinition(
        &mut self,
        ctx: &ArgumentDefinitionContext<'input>,
    ) -> Self::Return {
        let id = ir::Identifier::new(ctx.Identifier().unwrap().get_text());
        let reg_size = ctx
            .designator()
            .map(|x| self.visit(&*x).downcast::<ir::Expression>());
        let type_ = if ctx.scalarType().is_some() {
            self.visit(&*ctx.scalarType().unwrap())
                .downcast::<ir::Scalar>()
                .into()
        } else if ctx.qubitType().is_some() {
            self.visit(&*ctx.qubitType().unwrap())
                .downcast::<ir::types::Qubit>()
                .into()
        } else if ctx.designator().is_some() {
            if ctx.CREG().is_some() {
                ir::types::Register::C
            } else {
                ir::types::Register::Q
            }
            .into()
        } else {
            self.visit(&*ctx.arrayReferenceType().unwrap())
                .downcast::<ir::types::Array>()
                .into()
        };

        VisitorReturn::newt(ir::DefArgument::new(type_, id, reg_size))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#argumentDefinitionList}.
     * @param ctx the parse tree
     */
    fn visit_argumentDefinitionList(
        &mut self,
        ctx: &ArgumentDefinitionListContext<'input>,
    ) -> Self::Return {
        VisitorReturn::newt(
            ctx.argumentDefinition_all()
                .iter()
                .map(|x| self.visit(&**x).downcast::<ir::DefArgument>())
                .collect::<Vec<_>>(),
        )
    }

    /**
     * Visit a parse tree produced by {@link qasm3#defcalArgumentDefinitionList}.
     * @param ctx the parse tree
     */
    fn visit_defcalArgumentDefinitionList(
        &mut self,
        ctx: &DefcalArgumentDefinitionListContext<'input>,
    ) -> Self::Return {
        VisitorReturn::newt(
            ctx.defcalArgumentDefinition_all()
                .iter()
                .map(|x| self.visit(&**x).downcast::<ir::DefcalArgument>())
                .collect::<Vec<_>>(),
        )
    }

    /**
     * Visit a parse tree produced by {@link qasm3#defcalOperandList}.
     * @param ctx the parse tree
     */
    fn visit_defcalOperandList(&mut self, ctx: &DefcalOperandListContext<'input>) -> Self::Return {
        VisitorReturn::newt(
            ctx.defcalOperand_all()
                .iter()
                .map(|x| self.visit(&**x).downcast::<ir::Expression>())
                .collect::<Vec<_>>(),
        )
    }

    /**
     * Visit a parse tree produced by {@link qasm3#expressionList}.
     * @param ctx the parse tree
     */
    fn visit_expressionList(&mut self, ctx: &ExpressionListContext<'input>) -> Self::Return {
        VisitorReturn::newt(
            ctx.expression_all()
                .iter()
                .map(|x| self.visit(&**x).downcast::<ir::Expression>())
                .collect::<Vec<_>>(),
        )
    }

    /**
     * Visit a parse tree produced by {@link qasm3#identifierList}.
     * @param ctx the parse tree
     */
    fn visit_identifierList(&mut self, ctx: &IdentifierListContext<'input>) -> Self::Return {
        VisitorReturn::newt(
            ctx.Identifier_all()
                .iter()
                .map(|x| ir::Identifier::new(x.get_text()))
                .collect::<Vec<_>>(),
        )
    }

    /**
     * Visit a parse tree produced by {@link qasm3#gateOperandList}.
     * @param ctx the parse tree
     */
    fn visit_gateOperandList(&mut self, ctx: &GateOperandListContext<'input>) -> Self::Return {
        VisitorReturn::newt(
            ctx.gateOperand_all()
                .iter()
                .map(|x| self.visit(&**x).downcast::<ir::Expression>())
                .collect::<Vec<_>>(),
        )
    }

    /**
     * Visit a parse tree produced by {@link qasm3#externArgumentList}.
     * @param ctx the parse tree
     */
    fn visit_externArgumentList(
        &mut self,
        ctx: &ExternArgumentListContext<'input>,
    ) -> Self::Return {
        VisitorReturn::newt(
            ctx.externArgument_all()
                .iter()
                .map(|x| self.visit(&**x).downcast::<ir::ExternArgument>())
                .collect::<Vec<_>>(),
        )
    }
}
