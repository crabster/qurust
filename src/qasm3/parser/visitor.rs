use crate::qasm3::parser::antlr::qasm3VisitorCompat;
use crate::qasm3::parser::antlr::qasm3parser::*;

use antlr_rust::token::Token;
use antlr_rust::tree::ParseTree;
use antlr_rust::tree::ParseTreeVisitorCompat;
use antlr_rust::tree::Tree;

use crate::qasm3::ir;

use regex::Regex;
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
    }

    fn aggregate_results(&self, _: Self::Return, _: Self::Return) -> Self::Return {
        panic!("Should not be reachable")
    }
}

fn parse_hardware_qubit(qubit: &str) -> ir::Expression {
    let re = Regex::new(r"\$(\d+)").unwrap();
    let caps = re.captures(qubit).unwrap();
    ir::Literal::HardwareQubit(caps.get(1).unwrap().as_str().parse().unwrap()).into()
}

#[allow(non_snake_case)]
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
        let mut statement = if ctx.expressionStatement().is_some() {
            ir::Statement::Expression(
                self.visit(&*ctx.expressionStatement().unwrap())
                    .downcast::<ir::Expression>(),
            )
        } else {
            self.visit(&*ctx.get_child(ctx.get_child_count() - 1).unwrap())
                .downcast::<ir::Statement>()
        };

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
        VisitorReturn::newt::<ir::Statement>(ir::Pragma::newt(
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
        VisitorReturn::newt::<ir::Statement>(ir::CalibrationGrammar::newt(
            ctx.StringLiteral().unwrap().get_text(),
        ))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#includeStatement}.
     * @param ctx the parse tree
     */
    fn visit_includeStatement(&mut self, ctx: &IncludeStatementContext<'input>) -> Self::Return {
        let str_lit = ctx.StringLiteral().unwrap().get_text();
        let str_lit = str_lit[1..str_lit.len() - 1].to_string();
        VisitorReturn::newt::<ir::Statement>(ir::Include::new(str_lit).into())
    }

    /**
     * Visit a parse tree produced by {@link qasm3#breakStatement}.
     * @param ctx the parse tree
     */
    fn visit_breakStatement(&mut self, _: &BreakStatementContext<'input>) -> Self::Return {
        VisitorReturn::newt(ir::Statement::Break)
    }

    /**
     * Visit a parse tree produced by {@link qasm3#continueStatement}.
     * @param ctx the parse tree
     */
    fn visit_continueStatement(&mut self, _: &ContinueStatementContext<'input>) -> Self::Return {
        VisitorReturn::newt(ir::Statement::Continue)
    }

    /**
     * Visit a parse tree produced by {@link qasm3#endStatement}.
     * @param ctx the parse tree
     */
    fn visit_endStatement(&mut self, _: &EndStatementContext<'input>) -> Self::Return {
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

        VisitorReturn::newt::<ir::Statement>(
            ir::For::new(
                self.visit(&*ctx.scalarType().unwrap())
                    .downcast::<ir::types::Scalar>(),
                ir::Identifier::new(ctx.Identifier().unwrap().get_text()),
                expr,
                match sos {
                    ir::StatementOrScope::Scope(scope) => scope,
                    ir::StatementOrScope::Statement(_) => ir::Scope::new(vec![sos]),
                },
            )
            .into(),
        )
    }

    /**
     * Visit a parse tree produced by {@link qasm3#ifStatement}.
     * @param ctx the parse tree
     */
    fn visit_ifStatement(&mut self, ctx: &IfStatementContext<'input>) -> Self::Return {
        VisitorReturn::newt::<ir::Statement>(ir::If::newt(
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
        VisitorReturn::newt::<ir::Statement>(ir::Return::newt(
            ctx.expression()
                .map(|x| self.visit(&*x).downcast::<ir::Expression>()),
        ))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#whileStatement}.
     * @param ctx the parse tree
     */
    fn visit_whileStatement(&mut self, ctx: &WhileStatementContext<'input>) -> Self::Return {
        VisitorReturn::newt::<ir::Statement>(ir::While::newt(
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
        let expr = self
            .visit(&*ctx.expression().unwrap())
            .downcast::<ir::Expression>();
        let mut default = None;
        let mut items = vec![];
        for item in ctx.switchCaseItem_all() {
            if item.DEFAULT().is_some() {
                default = Some(self.visit(&*item.scope().unwrap()).downcast::<ir::Scope>());
            } else {
                items.push(self.visit(&*item).downcast::<ir::SwitchItem>());
            }
        }

        VisitorReturn::newt::<ir::Statement>(ir::Switch::newt(expr, items, default))
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
        VisitorReturn::newt::<ir::Statement>(ir::Barrier::newt(
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
        VisitorReturn::newt::<ir::Statement>(ir::BoxStatement::newt(
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
        VisitorReturn::newt::<ir::Statement>(ir::Delay::newt(
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

        VisitorReturn::newt::<ir::Statement>(ir::GateCall::newt(gate_mods, id, params, expr, args))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#measureArrowAssignmentStatement}.
     * @param ctx the parse tree
     */
    fn visit_measureArrowAssignmentStatement(
        &mut self,
        ctx: &MeasureArrowAssignmentStatementContext<'input>,
    ) -> Self::Return {
        VisitorReturn::newt::<ir::Statement>(ir::MeasureArrowAssignment::newt(
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
        VisitorReturn::newt::<ir::Statement>(ir::Reset::newt(
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
        let exprs = self
            .visit(&*ctx.aliasExpression().unwrap())
            .downcast::<Vec<ir::Expression>>();
        VisitorReturn::newt::<ir::Statement>(ir::AliasDeclaration::newt(
            ir::Identifier::new(ctx.Identifier().unwrap().get_text()),
            exprs,
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
            Some(x) => self.visit(&*x).downcast::<ir::Scalar>().into(),
            None => self
                .visit(&*ctx.arrayType().unwrap())
                .downcast::<ir::Type>(),
        };
        let id = ir::Identifier::new(ctx.Identifier().unwrap().get_text());
        let expr = ctx
            .declarationExpression()
            .map(|x| self.visit(&*x).downcast::<ir::Expression>());

        VisitorReturn::newt::<ir::Statement>(ir::ClassicalDeclaration::newt(type_, id, expr))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#constDeclarationStatement}.
     * @param ctx the parse tree
     */
    fn visit_constDeclarationStatement(
        &mut self,
        ctx: &ConstDeclarationStatementContext<'input>,
    ) -> Self::Return {
        VisitorReturn::newt::<ir::Statement>(ir::ConstDeclaration::newt(
            self.visit(&*ctx.scalarType().unwrap())
                .downcast::<ir::Scalar>()
                .into(),
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
            Some(x) => self.visit(&*x).downcast::<ir::Scalar>().into(),
            None => self
                .visit(&*ctx.arrayType().unwrap())
                .downcast::<ir::Type>(),
        };
        let id = ir::Identifier::new(ctx.Identifier().unwrap().get_text());

        VisitorReturn::newt::<ir::Statement>(ir::IODeclaration::newt(io_type, type_, id))
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
        VisitorReturn::newt::<ir::Statement>(ir::OldStyleDeclaration::newt(
            reg_type.into(),
            id,
            expr,
        ))
    }

    /**
     * Visit a parse tree produced by {@link qasm3#quantumDeclarationStatement}.
     * @param ctx the parse tree
     */
    fn visit_quantumDeclarationStatement(
        &mut self,
        ctx: &QuantumDeclarationStatementContext<'input>,
    ) -> Self::Return {
        VisitorReturn::newt::<ir::Statement>(ir::QuantumDeclaration::newt(
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
        VisitorReturn::newt::<ir::Statement>(ir::Def::newt(id, args, ret_type, scope))
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
        VisitorReturn::newt::<ir::Statement>(ir::Extern::newt(id, args, ret_type))
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

        VisitorReturn::newt::<ir::Statement>(ir::Gate::newt(
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

        VisitorReturn::newt::<ir::Statement>(ir::Assignment::newt(id_expr, assign_op, expr))
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
        VisitorReturn::newt::<ir::Statement>(ir::Cal::newt(
            ctx.CalibrationBlock().map(|x| x.get_text()),
        ))
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
        VisitorReturn::newt::<ir::Statement>(ir::Defcal::newt(
            target, args, operands, ret_type, cal_block,
        ))
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
        VisitorReturn::newt::<ir::Expression>(ir::BinaryOperation::newt(
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
        VisitorReturn::newt::<ir::Expression>(ir::BinaryOperation::newt(
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
        VisitorReturn::newt::<ir::Expression>(ir::DurationOf::newt(
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
        VisitorReturn::newt::<ir::Expression>(ir::Parenthesis::newt(
            self.visit(&*ctx.expression().unwrap())
                .downcast::<ir::Expression>(),
        ))
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
        VisitorReturn::newt::<ir::Expression>(ir::BinaryOperation::newt(
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
        VisitorReturn::newt::<ir::Expression>(ir::BinaryOperation::newt(
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
        VisitorReturn::newt::<ir::Expression>(ir::BinaryOperation::newt(
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
            Some(x) => self.visit(&*x).downcast::<ir::Scalar>().into(),
            None => self
                .visit(&*ctx.arrayType().unwrap())
                .downcast::<ir::Type>(),
        };
        VisitorReturn::newt::<ir::Expression>(ir::Cast::newt(
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
        VisitorReturn::newt::<ir::Expression>(ir::BinaryOperation::newt(
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
        VisitorReturn::newt::<ir::Expression>(ir::BinaryOperation::newt(
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
        VisitorReturn::newt::<ir::Expression>(ir::Call::newt(
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
        VisitorReturn::newt::<ir::Expression>(ir::BinaryOperation::newt(
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
        VisitorReturn::newt::<ir::Expression>(ir::BinaryOperation::newt(
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
        VisitorReturn::newt::<ir::Expression>(ir::BinaryOperation::newt(
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
        VisitorReturn::newt::<ir::Expression>(ir::BinaryOperation::newt(
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
        VisitorReturn::newt::<ir::Expression>(ir::Index::newt(
            self.visit(&*ctx.expression().unwrap())
                .downcast::<ir::Expression>(),
            self.visit(&*ctx.indexOperator().unwrap())
                .downcast::<Vec<ir::Expression>>(),
        ))
    }

    /**
     * Visit a parse tree produced by the {@code unaryExpression}
     * labeled alternative in {@link qasm3#expression}.
     * @param ctx the parse tree
     */
    fn visit_unaryExpression(&mut self, ctx: &UnaryExpressionContext<'input>) -> Self::Return {
        VisitorReturn::newt::<ir::Expression>(ir::UnaryOperation::newt(
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
        VisitorReturn::newt::<ir::Expression>(if ctx.Identifier().is_some() {
            ir::Identifier::new(ctx.Identifier().unwrap().get_text()).into()
        } else if ctx.BinaryIntegerLiteral().is_some() {
            ir::Literal::BinaryInteger(
                i64::from_str_radix(
                    ctx.BinaryIntegerLiteral()
                        .unwrap()
                        .get_text()
                        .trim_start_matches("0b"),
                    2,
                )
                .unwrap(),
            )
            .into()
        } else if ctx.OctalIntegerLiteral().is_some() {
            ir::Literal::OctalInteger(
                i64::from_str_radix(
                    ctx.OctalIntegerLiteral()
                        .unwrap()
                        .get_text()
                        .trim_start_matches("0o"),
                    8,
                )
                .unwrap(),
            )
            .into()
        } else if ctx.DecimalIntegerLiteral().is_some() {
            ir::Literal::DecimalInteger(
                ctx.DecimalIntegerLiteral()
                    .unwrap()
                    .get_text()
                    .parse()
                    .unwrap(),
            )
            .into()
        } else if ctx.HexIntegerLiteral().is_some() {
            ir::Literal::HexInteger(
                i64::from_str_radix(
                    ctx.HexIntegerLiteral()
                        .unwrap()
                        .get_text()
                        .trim_start_matches("0x"),
                    16,
                )
                .unwrap(),
            )
            .into()
        } else if ctx.FloatLiteral().is_some() {
            ir::Literal::Float(ctx.FloatLiteral().unwrap().get_text().parse().unwrap()).into()
        } else if ctx.ImaginaryLiteral().is_some() {
            let re = Regex::new(r"(\d+(\.\d+)?)\s*im").unwrap();
            let imaginary_str = ctx.ImaginaryLiteral().unwrap().get_text();
            let split = re.captures(&imaginary_str).unwrap();
            ir::Literal::Imaginary(split[1].parse().unwrap()).into()
        } else if ctx.BooleanLiteral().is_some() {
            ir::Literal::Boolean(ctx.BooleanLiteral().unwrap().get_text().parse().unwrap()).into()
        } else if ctx.BitstringLiteral().is_some() {
            ir::Literal::BitString(ctx.BitstringLiteral().unwrap().get_text().parse().unwrap())
                .into()
        } else if ctx.TimingLiteral().is_some() {
            let re = Regex::new(r"(\d+(\.\d+)?)\s*(s|ms|us|ns|dt)").unwrap();
            let timing_str = ctx.TimingLiteral().unwrap().get_text();
            let split = re.captures(&timing_str).unwrap();
            let num = split[1].parse().unwrap();
            let unit = ir::TimingUnit::from_str(&split[3]);
            ir::Literal::Timing(num, unit).into()
        } else {
            parse_hardware_qubit(ctx.HardwareQubit().unwrap().get_text().as_str()).into()
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
                .downcast::<ir::Expression>()
        })
    }

    /**
     * Visit a parse tree produced by {@link qasm3#measureExpression}.
     * @param ctx the parse tree
     */
    fn visit_measureExpression(&mut self, ctx: &MeasureExpressionContext<'input>) -> Self::Return {
        VisitorReturn::newt::<ir::Expression>(ir::Measure::newt(
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

        VisitorReturn::newt::<ir::Expression>(ir::Range::new(from_expr, to_expr, step_expr).into())
    }

    /**
     * Visit a parse tree produced by {@link qasm3#setExpression}.
     * @param ctx the parse tree
     */
    fn visit_setExpression(&mut self, ctx: &SetExpressionContext<'input>) -> Self::Return {
        VisitorReturn::newt::<ir::Expression>(
            ir::expressions::Array::new(
                ctx.expression_all()
                    .iter()
                    .map(|e| self.visit(&**e).downcast::<ir::Expression>())
                    .collect::<Vec<_>>(),
            )
            .into(),
        )
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
                .downcast::<ir::expressions::Array>()
                .into(),
        })
    }

    /**
     * Visit a parse tree produced by {@link qasm3#indexOperator}.
     * @param ctx the parse tree
     */
    fn visit_indexOperator(&mut self, ctx: &IndexOperatorContext<'input>) -> Self::Return {
        VisitorReturn::newt(match ctx.setExpression() {
            Some(e) => vec![self.visit(&*e).downcast::<ir::Expression>()],
            None => ctx
                .indexOperatorElement_all()
                .iter()
                .map(|x| self.visit(&**x).downcast::<ir::Expression>())
                .collect::<Vec<_>>(),
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
        let expr: ir::Expression = ctx.indexOperator_all().iter().fold(id.into(), |acc, e| {
            ir::Index::newt(acc, self.visit(&**e).downcast::<Vec<ir::Expression>>())
        });
        VisitorReturn::newt(expr)
    }

    /**
     * Visit a parse tree produced by {@link qasm3#returnSignature}.
     * @param ctx the parse tree
     */
    fn visit_returnSignature(&mut self, ctx: &ReturnSignatureContext<'input>) -> Self::Return {
        VisitorReturn::newt::<ir::Type>(
            self.visit(&*ctx.scalarType().unwrap())
                .downcast::<ir::Scalar>()
                .into(),
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
            ir::types::Scalar::Complex(
                ctx.scalarType()
                    .map(|x| Box::new(self.visit(&*x).downcast::<ir::Scalar>())),
            )
        })
    }

    /**
     * Visit a parse tree produced by {@link qasm3#qubitType}.
     * @param ctx the parse tree
     */
    fn visit_qubitType(&mut self, ctx: &QubitTypeContext<'input>) -> Self::Return {
        VisitorReturn::newt::<ir::Type>(ir::types::Qubit::newt(
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

        VisitorReturn::newt::<ir::Type>(ir::types::Array::newt(scalar_type, expr_list))
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

        VisitorReturn::newt::<ir::Type>(
            ir::types::Array::with_reference(ref_type, scalar_type, expr_list, dim).into(),
        )
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
            parse_hardware_qubit(ctx.HardwareQubit().unwrap().get_text().as_str())
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
            parse_hardware_qubit(ctx.HardwareQubit().unwrap().get_text().as_str()).into()
        })
    }

    /**
     * Visit a parse tree produced by {@link qasm3#externArgument}.
     * @param ctx the parse tree
     */
    fn visit_externArgument(&mut self, ctx: &ExternArgumentContext<'input>) -> Self::Return {
        let type_ = if ctx.scalarType().is_some() {
            self.visit(&*ctx.scalarType().unwrap())
                .downcast::<ir::Scalar>()
                .into()
        } else if ctx.arrayReferenceType().is_some() {
            self.visit(&*ctx.arrayReferenceType().unwrap())
                .downcast::<ir::Type>()
        } else {
            ir::types::Register::C.into()
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
                .downcast::<ir::Type>()
                .into()
        } else if ctx.CREG().is_some() {
            ir::types::Register::C.into()
        } else if ctx.QREG().is_some() {
            ir::types::Register::Q.into()
        } else {
            self.visit(&*ctx.arrayReferenceType().unwrap())
                .downcast::<ir::Type>()
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
        // TODO: There should be no commas in Identifier_all
        VisitorReturn::newt(
            ctx.Identifier_all()
                .iter()
                .filter(|id| id.get_text() != ",")
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qasm3::ir;
    use crate::qasm3::ir::AsQasmStr;
    use crate::qasm3::parser::antlr;
    use antlr_rust::common_token_stream::CommonTokenStream;
    use antlr_rust::input_stream::InputStream;
    use antlr_rust::tree::ParseTreeVisitorCompat;
    use indoc::indoc;

    fn check_program(program_str: &str) {
        let input = InputStream::new(program_str);
        let lexer = antlr::qasm3lexer::qasm3Lexer::new(input);
        let token_source = CommonTokenStream::new(lexer);
        let mut parser = antlr::qasm3parser::qasm3::new(token_source);
        let root = parser.program().unwrap();

        let mut visitor = Visitor::new();
        let program = visitor.visit(&*root).downcast::<ir::Program>();

        assert_eq!(program.as_qasm_str(), program_str);
    }

    #[test]
    fn visit_program() {
        check_program("\n");
    }

    #[test]
    fn visit_version() {
        check_program(indoc! {"
            OPENQASM 3.0;
        "});
    }

    #[test]
    fn visit_statement() {
        // Note: This function is tested by other test cases
    }

    #[test]
    fn visit_annotation() {
        check_program(indoc! {r#"
            @crosstalk
            @noise profile "gate_noise.qnf"
            defcal noisy_gate $0, $1 { ... }
        "#});
    }

    #[test]
    fn visit_scope() {
        check_program(indoc! {r#"
            {
                a;
            }
        "#});
    }

    #[test]
    fn visit_pragma() {
        check_program(indoc! {r#"
            pragma user alice account 12345678
        "#});
    }

    #[test]
    fn visit_statement_or_scope() {
        check_program(indoc! {r#"
            a;
        "#});
        check_program(indoc! {r#"
            {}
        "#});
    }

    #[test]
    fn visit_calibration_grammar_statement() {
        check_program(indoc! {r#"
            defcalgrammar "openpulse";
        "#});
    }

    #[test]
    fn visit_include_statement() {
        check_program(indoc! {r#"
            include "stdgates.qasm";
        "#});
    }

    #[test]
    fn visit_break_statement() {
        check_program(indoc! {r#"
            break;
        "#});
    }

    #[test]
    fn visit_continue_statement() {
        check_program(indoc! {r#"
            continue;
        "#});
    }

    #[test]
    fn visit_end_statement() {
        check_program(indoc! {r#"
            end;
        "#});
    }

    #[test]
    fn visit_for_statement() {
        check_program(indoc! {r#"
            for int[16] i in {1, 2, 3} {}
        "#});
        check_program(indoc! {r#"
            for int[16] i in [0:3] {}
        "#});
        check_program(indoc! {r#"
            for int[16] i in l {}
        "#});
    }

    #[test]
    fn visit_if_statement() {
        check_program(indoc! {r#"
            if (a) {}
        "#});
        check_program(indoc! {r#"
            if (a) {} else {}
        "#});
    }

    #[test]
    fn visit_return_statement() {
        check_program(indoc! {r#"
            return;
        "#});
        check_program(indoc! {r#"
            return 1;
        "#});
    }

    #[test]
    fn visit_while_statement() {
        check_program(indoc! {r#"
            while (a) {}
        "#});
    }

    #[test]
    fn visit_switch_statement() {
        check_program(indoc! {r#"
            switch (a) {}
        "#});
    }

    #[test]
    fn visit_switch_case_item() {
        check_program(indoc! {r#"
            switch (a) {
                case 1 {
                    break;
                }
            }
        "#});
        check_program(indoc! {r#"
            switch (a) {
                case 1 {
                    break;
                }
                default: {
                    end;
                }
            }
        "#});
    }

    #[test]
    fn visit_barrier_statement() {
        check_program(indoc! {r#"
            barrier;
        "#});
        check_program(indoc! {r#"
            barrier q;
        "#});
    }

    #[test]
    fn visit_box_statement() {
        check_program(indoc! {r#"
            box {}
        "#});
        check_program(indoc! {r#"
            box[5ns] {}
        "#});
    }

    #[test]
    fn visit_delay_statement() {
        check_program(indoc! {r#"
            delay [5ns];
        "#});
        check_program(indoc! {r#"
            delay [5ns] $1;
        "#});
    }

    #[test]
    fn visit_gate_call_statement() {
        check_program(indoc! {r#"
            gphase pi;
        "#});
        check_program(indoc! {r#"
            inv @ gphase pi;
        "#});
        check_program(indoc! {r#"
            gphase(pi) pi;
        "#});
        check_program(indoc! {r#"
            gphase[2] pi;
        "#});
        check_program(indoc! {r#"
            gphase;
        "#});
        check_program(indoc! {r#"
            h q;
        "#});
        check_program(indoc! {r#"
            inv @ h q;
        "#});
        check_program(indoc! {r#"
            h(pi) q;
        "#});
        check_program(indoc! {r#"
            h[2] q;
        "#});
    }

    #[test]
    fn visit_measure_arrow_assignment_statement() {
        check_program(indoc! {r#"
            measure a;
        "#});
        check_program(indoc! {r#"
            measure a -> b;
        "#});
    }

    #[test]
    fn visit_reset_statement() {
        check_program(indoc! {r#"
            reset q;
        "#});
    }

    #[test]
    fn visit_alias_declaration_statement() {
        check_program(indoc! {r#"
            let q1 = q2;
        "#});
    }

    #[test]
    fn visit_classical_declaration_statement() {
        check_program(indoc! {r#"
            bit[16] a;
        "#});
        check_program(indoc! {r#"
            array[bit, 16] a;
        "#});
        check_program(indoc! {r#"
            bit[16] a = 1;
        "#});
    }

    #[test]
    fn visit_const_declaration_statement() {
        check_program(indoc! {r#"
            const int[16] a = 1;
        "#});
    }

    #[test]
    fn visit_io_declaration_statement() {
        check_program(indoc! {r#"
            input bit[16] a;
        "#});
        check_program(indoc! {r#"
            output array[bit, 16] a;
        "#});
    }

    #[test]
    fn visit_old_style_declaration_statement() {
        check_program(indoc! {r#"
            creg q;
        "#});
        check_program(indoc! {r#"
            qreg q[5];
        "#});
    }

    #[test]
    fn visit_quantum_declaration_statement() {
        check_program(indoc! {r#"
            qubit q;
        "#});
    }

    #[test]
    fn visit_def_statement() {
        check_program(indoc! {r#"
            def a() {}
        "#});
        check_program(indoc! {r#"
            def a(int[16] b) {}
        "#});
        check_program(indoc! {r#"
            def a(int[16] b) -> int[16] {}
        "#});
    }

    #[test]
    fn visit_extern_statement() {
        check_program(indoc! {r#"
            extern a();
        "#});
        check_program(indoc! {r#"
            extern a(int[16]);
        "#});
        check_program(indoc! {r#"
            extern a(int[16]) -> int[16];
        "#});
    }

    #[test]
    fn visit_gate_statement() {
        check_program(indoc! {r#"
            gate a q {}
        "#});
        check_program(indoc! {r#"
            gate a(b) q {}
        "#});
    }

    #[test]
    fn visit_assignment_statement() {
        check_program(indoc! {r#"
            a = measure b;
        "#});
        check_program(indoc! {r#"
            a += 1;
        "#});
    }

    #[test]
    fn visit_expression_statement() {
        check_program(indoc! {r#"
            a;
        "#});
    }

    #[test]
    fn visit_cal_statement() {
        check_program(indoc! {r#"
            cal {}
        "#});
        check_program(indoc! {r#"
            cal {...}
        "#});
    }

    #[test]
    fn visit_defcal_statement() {
        check_program(indoc! {r#"
            defcal a $1 {}
        "#});
        check_program(indoc! {r#"
            defcal a(int[16] b) $1 {}
        "#});
        check_program(indoc! {r#"
            defcal a(int[16] b) $1 -> int[16] {}
        "#});
        check_program(indoc! {r#"
            defcal a(int[16] b) $1 -> int[16] {...}
        "#});
    }

    #[test]
    fn visit_bitwise_xor_expression() {
        check_program(indoc! {r#"
            a ^ b;
        "#});
    }

    #[test]
    fn visit_additive_expression() {
        check_program(indoc! {r#"
            a + b;
        "#});
        check_program(indoc! {r#"
            a - b;
        "#});
    }

    #[test]
    fn visit_durationof_expression() {
        check_program(indoc! {r#"
            durationof({});
        "#});
    }

    #[test]
    fn visit_parenthesis_expression() {
        check_program(indoc! {r#"
            (a);
        "#});
    }

    #[test]
    fn visit_comparison_expression() {
        check_program(indoc! {r#"
            a == b;
        "#});
        check_program(indoc! {r#"
            a != b;
        "#});
        check_program(indoc! {r#"
            a < b;
        "#});
        check_program(indoc! {r#"
            a <= b;
        "#});
        check_program(indoc! {r#"
            a > b;
        "#});
        check_program(indoc! {r#"
            a >= b;
        "#});
    }

    #[test]
    fn visit_multiplicative_expression() {
        check_program(indoc! {r#"
            a * b;
        "#});
        check_program(indoc! {r#"
            a / b;
        "#});
        check_program(indoc! {r#"
            a % b;
        "#});
    }

    #[test]
    fn visit_logical_or_expression() {
        check_program(indoc! {r#"
            a || b;
        "#});
    }

    #[test]
    fn visit_cast_expression() {
        check_program(indoc! {r#"
            int[16](a);
        "#});
        check_program(indoc! {r#"
            array[int, 16](a);
        "#});
    }

    #[test]
    fn visit_power_expression() {
        check_program(indoc! {r#"
            a ** b;
        "#});
    }

    #[test]
    fn visit_bitwise_or_expression() {
        check_program(indoc! {r#"
            a | b;
        "#});
    }

    #[test]
    fn visit_call_expression() {
        check_program(indoc! {r#"
            a();
        "#});
        check_program(indoc! {r#"
            a(b);
        "#});
    }

    #[test]
    fn visit_bitshift_expression() {
        check_program(indoc! {r#"
            a << b;
        "#});
        check_program(indoc! {r#"
            a >> b;
        "#});
    }

    #[test]
    fn visit_bitwise_and_expression() {
        check_program(indoc! {r#"
            a & b;
        "#});
    }

    #[test]
    fn visit_equality_expression() {
        check_program(indoc! {r#"
            a == b;
        "#});
        check_program(indoc! {r#"
            a != b;
        "#});
    }

    #[test]
    fn visit_logical_and_expression() {
        check_program(indoc! {r#"
            a && b;
        "#});
    }

    #[test]
    fn visit_index_expression() {
        check_program(indoc! {r#"
            a[b];
        "#});
    }

    #[test]
    fn visit_unary_expression() {
        check_program(indoc! {r#"
            -a;
        "#});
        check_program(indoc! {r#"
            !a;
        "#});
        check_program(indoc! {r#"
            ~a;
        "#});
    }

    #[test]
    fn visit_literal_expression() {
        check_program(indoc! {r#"
            a;
        "#});
        check_program(indoc! {r#"
            0b1;
        "#});
        check_program(indoc! {r#"
            0o1;
        "#});
        check_program(indoc! {r#"
            1;
        "#});
        check_program(indoc! {r#"
            0x1;
        "#});
        check_program(indoc! {r#"
            1.1;
        "#});
        check_program(indoc! {r#"
            1im;
        "#});
        check_program(indoc! {r#"
            true;
        "#});
        check_program(indoc! {r#"
            "101";
        "#});
        check_program(indoc! {r#"
            1ns;
        "#});
        check_program(indoc! {r#"
            $1;
        "#});
    }

    #[test]
    fn visit_alias_expression() {
        check_program(indoc! {r#"
            let a = b;
        "#});
        check_program(indoc! {r#"
            let a = b ++ c;
        "#});
        check_program(indoc! {r#"
            let a = b ++ c ++ d;
        "#});
    }

    #[test]
    fn visit_declaration_expression() {
        check_program(indoc! {r#"
            array[int[16], 1] a = {1};
        "#});
        check_program(indoc! {r#"
            int[16] a = 1;
        "#});
        check_program(indoc! {r#"
            bit b = measure q;
        "#});
    }

    #[test]
    fn visit_measure_expression() {
        check_program(indoc! {r#"
            measure q;
        "#});
    }

    #[test]
    fn visit_range_expression() {
        check_program(indoc! {r#"
            a[:];
        "#});
        check_program(indoc! {r#"
            a[0:3];
        "#});
        check_program(indoc! {r#"
            a[0:];
        "#});
        check_program(indoc! {r#"
            a[:3];
        "#});
        check_program(indoc! {r#"
            a[0:3:1];
        "#});
        check_program(indoc! {r#"
            a[:3:1];
        "#});
        check_program(indoc! {r#"
            a[0::1];
        "#});
        check_program(indoc! {r#"
            a[::1];
        "#});
    }

    #[test]
    fn visit_set_expression() {
        check_program(indoc! {r#"
            a[{1, 2, 3}];
        "#});
    }

    #[test]
    fn visit_array_literal() {
        check_program(indoc! {r#"
            array[int[16], 2] a = {1, 2};
        "#});
    }

    #[test]
    fn visit_array_literal_element() {
        check_program(indoc! {r#"
            array[int[16], 2] a = {1, 2};
        "#});
        check_program(indoc! {r#"
            array[int[16], 2, 2] a = {{1, 2}, {3, 4}};
        "#});
    }

    #[test]
    fn visit_index_operator() {
        check_program(indoc! {r#"
            a[{1, 2, 3}];
        "#});
        check_program(indoc! {r#"
            a[1, 2:4, 5];
        "#});
    }

    #[test]
    fn visit_index_operator_element() {
        check_program(indoc! {r#"
            a[{1, 2}];
        "#});
        check_program(indoc! {r#"
            a[1:2];
        "#});
        check_program(indoc! {r#"
            a[1:2, 3:4];
        "#});
    }

    #[test]
    fn visit_indexed_identifier() {
        check_program(indoc! {r#"
            measure q[1];
        "#});
    }

    #[test]
    fn visit_return_signature() {
        check_program(indoc! {r#"
            def a(int[16] b) -> int[16] {}
        "#});
    }

    #[test]
    fn visit_gate_modifier() {
        check_program(indoc! {r#"
            inv @ h q;
        "#});
        check_program(indoc! {r#"
            pow(2) @ h q;
        "#});
        check_program(indoc! {r#"
            ctrl @ h q;
        "#});
        check_program(indoc! {r#"
            negctrl(1) @ h q;
        "#});
    }

    #[test]
    fn visit_scalar_type() {
        check_program(indoc! {r#"
            bit a;
        "#});
        check_program(indoc! {r#"
            bit[16] a;
        "#});
        check_program(indoc! {r#"
            int a;
        "#});
        check_program(indoc! {r#"
            int[16] a;
        "#});
        check_program(indoc! {r#"
            uint a;
        "#});
        check_program(indoc! {r#"
            uint[16] a;
        "#});
        check_program(indoc! {r#"
            float a;
        "#});
        check_program(indoc! {r#"
            float[16] a;
        "#});
        check_program(indoc! {r#"
            angle a;
        "#});
        check_program(indoc! {r#"
            angle[16] a;
        "#});
        check_program(indoc! {r#"
            bool a;
        "#});
        check_program(indoc! {r#"
            duration a;
        "#});
        check_program(indoc! {r#"
            stretch a;
        "#});
        check_program(indoc! {r#"
            complex[int[16]] a;
        "#});
        check_program(indoc! {r#"
            complex a;
        "#});
    }

    #[test]
    fn visit_qubit_type() {
        check_program(indoc! {r#"
            qubit q;
        "#});
        check_program(indoc! {r#"
            qubit[16] q;
        "#});
    }

    #[test]
    fn visit_array_type() {
        check_program(indoc! {r#"
            array[bit, 16, 16] a;
        "#});
    }

    #[test]
    fn visit_array_reference_type() {
        check_program(indoc! {r#"
            extern foo(readonly array[bit, 16]);
        "#});
        check_program(indoc! {r#"
            extern foo(mutable array[bit, 16]);
        "#});
        check_program(indoc! {r#"
            extern foo(readonly array[bit, 16, 16]);
        "#});
        check_program(indoc! {r#"
            extern foo(readonly array[bit, #dim = 16]);
        "#});
    }

    #[test]
    fn visit_designator() {
        check_program(indoc! {r#"
            qubit[16] q;
        "#});
    }

    #[test]
    fn visit_defcal_target() {
        check_program(indoc! {r#"
            defcal measure $1 {}
        "#});
        check_program(indoc! {r#"
            defcal reset $1 {}
        "#});
        check_program(indoc! {r#"
            defcal delay $1 {}
        "#});
        check_program(indoc! {r#"
            defcal a $1 {}
        "#});
    }

    #[test]
    fn visit_defcal_argument_definition() {
        check_program(indoc! {r#"
            defcal a(1) $1 {}
        "#});
        check_program(indoc! {r#"
            defcal a(int[16] b) $1 {}
        "#});
    }

    #[test]
    fn visit_defcal_operand() {
        check_program(indoc! {r#"
            defcal a $1 {}
        "#});
        check_program(indoc! {r#"
            defcal a b {}
        "#});
    }

    #[test]
    fn visit_gate_operand() {
        check_program(indoc! {r#"
            reset q[1];
        "#});
        check_program(indoc! {r#"
            reset $1;
        "#});
    }

    #[test]
    fn visit_extern_argument() {
        check_program(indoc! {r#"
            extern foo(int[16]);
        "#});
        check_program(indoc! {r#"
            extern foo(readonly array[bit, 16]);
        "#});
        check_program(indoc! {r#"
            extern foo(creg);
        "#});
        check_program(indoc! {r#"
            extern foo(creg[2]);
        "#});
    }

    #[test]
    fn visit_argument_definition() {
        check_program(indoc! {r#"
            def a(int[16] b) {}
        "#});
        check_program(indoc! {r#"
            def a(qubit[2] q) {}
        "#});
        check_program(indoc! {r#"
            def a(creg b) {}
        "#});
        check_program(indoc! {r#"
            def a(qreg q[2]) {}
        "#});
        check_program(indoc! {r#"
            def a(readonly array[bit, 16] b) {}
        "#});
    }

    #[test]
    fn visit_argument_definition_list() {
        check_program(indoc! {r#"
            def a(int[16] b) {}
        "#});
        check_program(indoc! {r#"
            def a(int[16] b, int[16] c) {}
        "#});
    }

    #[test]
    fn visit_defcal_argument_definition_list() {
        check_program(indoc! {r#"
            defcal a(1) $1 {}
        "#});
        check_program(indoc! {r#"
            defcal a(1, 2) $1 {}
        "#});
    }

    #[test]
    fn visit_defcal_operand_list() {
        check_program(indoc! {r#"
            defcal a $1 {}
        "#});
        check_program(indoc! {r#"
            defcal a $1, $2 {}
        "#});
    }

    #[test]
    fn visit_expression_list() {
        check_program(indoc! {r#"
            foo(a);
        "#});
        check_program(indoc! {r#"
            foo(a, b);
        "#});
    }

    #[test]
    fn visit_identifier_list() {
        check_program(indoc! {r#"
            gate g(a) q {}
        "#});
        check_program(indoc! {r#"
            gate g(a, b) q {}
        "#});
    }

    #[test]
    fn visit_gate_operand_list() {
        check_program(indoc! {r#"
            barrier $1;
        "#});
        check_program(indoc! {r#"
            barrier $1, $2;
        "#});
    }

    #[test]
    fn visit_extern_argument_list() {
        check_program(indoc! {r#"
            extern foo(int[16]);
        "#});
        check_program(indoc! {r#"
            extern foo(int[16], int[16]);
        "#});
    }
}
