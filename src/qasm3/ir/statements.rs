use crate::qasm3::ir::expressions::{BinaryOperator, Expression, Identifier};
use crate::qasm3::ir::types::{Scalar, Type};
use crate::qasm3::ir::AsQasmStr;

#[derive(Debug)]
pub struct AliasDeclaration {
    identifier: Identifier,
    expr: Expression,
}

impl AliasDeclaration {
    pub fn new<T: From<AliasDeclaration>>(identifier: Identifier, expr: Expression) -> T {
        AliasDeclaration { identifier, expr }.into()
    }
}

impl AsQasmStr for AliasDeclaration {
    fn as_qasm_str(&self) -> String {
        format!(
            "let {} = {};",
            self.identifier.as_qasm_str(),
            self.expr.as_qasm_str()
        )
    }
}

#[derive(Debug)]
pub struct Assignment {
    identifier: Identifier,
    operator: Option<BinaryOperator>,
    expr: Expression,
}

impl Assignment {
    pub fn new<T: From<Assignment>>(identifier: Identifier, expr: Expression) -> T {
        Assignment {
            identifier,
            operator: None,
            expr,
        }.into()
    }

    pub fn with_op<T: From<Assignment>>(
        identifier: Identifier,
        operator: BinaryOperator,
        expr: Expression,
    ) -> T {
        Assignment {
            identifier,
            operator: Some(operator),
            expr,
        }.into()
    }
}

impl AsQasmStr for Assignment {
    fn as_qasm_str(&self) -> String {
        let assign_op = match &self.operator {
            Some(op) => op.as_qasm_str(),
            None => "=".to_string(),
        };

        format!(
            "{} {} {}",
            self.identifier.as_qasm_str(),
            assign_op,
            self.expr.as_qasm_str()
        )
    }
}

#[derive(Debug)]
pub struct Barrier {
    exprs: Vec<Expression>,
}

impl Barrier {
    pub fn new<T: From<Barrier>>(exprs: Vec<Expression>) -> T {
        Barrier { exprs }.into()
    }
}

impl AsQasmStr for Barrier {
    fn as_qasm_str(&self) -> String {
        format!(
            "barrier {};",
            self.exprs
                .iter()
                .map(|e| e.as_qasm_str())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[derive(Debug)]
pub struct BoxStatement {
    expr: Option<Expression>,
    scope: Scope,
}

impl BoxStatement {
    pub fn new<T: From<BoxStatement>>(expr: Option<Expression>, scope: Scope) -> T {
        BoxStatement { expr, scope }.into()
    }
}

impl AsQasmStr for BoxStatement {
    fn as_qasm_str(&self) -> String {
        format!(
            "box{} {}",
            match &self.expr {
                Some(expr) => expr.as_qasm_str(),
                None => "".to_string(),
            },
            self.scope.as_qasm_str()
        )
    }
}

#[derive(Debug)]
pub struct Cal {
    cal_block: String,
}

impl Cal {
    pub fn new<T: From<Cal>>(cal_block: String) -> T {
        Cal { cal_block }.into()
    }
}

impl AsQasmStr for Cal {
    fn as_qasm_str(&self) -> String {
        format!("cal {{{}}}", self.cal_block)
    }
}

#[derive(Debug)]
pub struct CalibrationGrammar {
    grammar: String,
}

impl CalibrationGrammar {
    pub fn new<T: From<CalibrationGrammar>>(grammar: String) -> T {
        CalibrationGrammar { grammar }.into()
    }
}

impl AsQasmStr for CalibrationGrammar {
    fn as_qasm_str(&self) -> String {
        format!("defcalgrammar \"{}\";", self.grammar)
    }
}

#[derive(Debug)]
pub struct ClassicalDeclaration {
    type_: Type,
    id: Identifier,
    expr: Option<Expression>,
}

impl ClassicalDeclaration {
    pub fn new<T: From<ClassicalDeclaration>>(
        type_: Type,
        id: Identifier,
        expr: Option<Expression>,
    ) -> T {
        ClassicalDeclaration { type_, id, expr }.into()
    }
}

impl AsQasmStr for ClassicalDeclaration {
    fn as_qasm_str(&self) -> String {
        format!(
            "{} {}{};",
            self.type_.as_qasm_str(),
            self.id.as_qasm_str(),
            match &self.expr {
                Some(expr) => format!(" = {}", expr.as_qasm_str()),
                None => "".to_string(),
            }
        )
    }
}

#[derive(Debug)]
pub struct ConstDeclaration {
    type_: Type,
    id: Identifier,
    expr: Expression,
}

impl ConstDeclaration {
    pub fn new<T: From<ConstDeclaration>>(type_: Type, id: Identifier, expr: Expression) -> T {
        ConstDeclaration { type_, id, expr }.into()
    }
}

impl AsQasmStr for ConstDeclaration {
    fn as_qasm_str(&self) -> String {
        format!(
            "const {} {} = {};",
            self.type_.as_qasm_str(),
            self.id.as_qasm_str(),
            self.expr.as_qasm_str()
        )
    }
}

#[derive(Debug)]
pub struct DefArgument {
    type_: Type,
    id: Identifier,
    reg_size: Option<Expression>,
}

impl DefArgument {
    pub fn new<T: From<DefArgument>>(type_: Type, id: Identifier) -> T {
        DefArgument {
            type_,
            id,
            reg_size: None,
        }
        .into()
    }

    pub fn with_reg_size<T: From<DefArgument>>(
        type_: Type,
        id: Identifier,
        reg_size: Expression,
    ) -> T {
        DefArgument {
            type_,
            id,
            reg_size: Some(reg_size),
        }
        .into()
    }
}

impl AsQasmStr for DefArgument {
    fn as_qasm_str(&self) -> String {
        format!(
            "{} {}{}",
            self.type_.as_qasm_str(),
            self.id.as_qasm_str(),
            match &self.reg_size {
                Some(expr) => format!("[{}]", expr.as_qasm_str()),
                None => "".to_string(),
            }
        )
    }
}

#[derive(Debug)]
pub struct Def {
    id: Identifier,
    args: Vec<DefArgument>,
    ret_type: Option<Type>,
    scope: Scope,
}

impl Def {
    pub fn new<T: From<Def>>(
        id: Identifier,
        args: Vec<DefArgument>,
        ret_type: Option<Type>,
        scope: Scope,
    ) -> T {
        Def {
            id,
            args,
            ret_type,
            scope,
        }.into()
    }
}

impl AsQasmStr for Def {
    fn as_qasm_str(&self) -> String {
        format!(
            "def {}({}){} {}",
            self.id.as_qasm_str(),
            self.args
                .iter()
                .map(|arg| arg.as_qasm_str())
                .collect::<Vec<_>>()
                .join(", "),
            match &self.ret_type {
                Some(ret_type) => format!(" -> {}", ret_type.as_qasm_str()),
                None => "".to_string(),
            },
            self.scope.as_qasm_str()
        )
    }
}

#[derive(Debug)]
pub enum DefcalTarget {
    Measure,
    Reset,
    Delay,
    Identifier(Identifier),
}

impl AsQasmStr for DefcalTarget {
    fn as_qasm_str(&self) -> String {
        match self {
            DefcalTarget::Measure => "measure".to_string(),
            DefcalTarget::Reset => "reset".to_string(),
            DefcalTarget::Delay => "delay".to_string(),
            DefcalTarget::Identifier(id) => id.as_qasm_str(),
        }
    }
}

#[derive(Debug)]
pub enum DefcalArgument {
    DefArgument(DefArgument),
    Expression(Expression),
}

impl AsQasmStr for DefcalArgument {
    fn as_qasm_str(&self) -> String {
        match self {
            DefcalArgument::DefArgument(arg) => arg.as_qasm_str(),
            DefcalArgument::Expression(expr) => expr.as_qasm_str(),
        }
    }
}

impl From<DefArgument> for DefcalArgument {
    fn from(arg: DefArgument) -> Self {
        DefcalArgument::DefArgument(arg)
    }
}

impl From<Expression> for DefcalArgument {
    fn from(expr: Expression) -> Self {
        DefcalArgument::Expression(expr)
    }
}

#[derive(Debug)]
pub struct Defcal {
    target: DefcalTarget,
    args: Vec<DefcalArgument>,
    operands: Vec<Expression>,
    ret_type: Option<Type>,
    cal_block: String,
}

impl Defcal {
    pub fn new<T: From<Defcal>>(
        target: DefcalTarget,
        args: Vec<DefcalArgument>,
        operands: Vec<Expression>,
        ret_type: Option<Type>,
        cal_block: String,
    ) -> T {
        Defcal {
            target,
            args,
            operands,
            ret_type,
            cal_block,
        }.into()
    }
}

impl AsQasmStr for Defcal {
    fn as_qasm_str(&self) -> String {
        format!(
            "defcal {}({}){}{} {{{}}}",
            self.target.as_qasm_str(),
            self.args
                .iter()
                .map(|arg| arg.as_qasm_str())
                .collect::<Vec<_>>()
                .join(", "),
            self.operands
                .iter()
                .map(|operand| operand.as_qasm_str())
                .collect::<Vec<_>>()
                .join(", "),
            match &self.ret_type {
                Some(ret_type) => format!(" -> {}", ret_type.as_qasm_str()),
                None => "".to_string(),
            },
            self.cal_block
        )
    }
}

#[derive(Debug)]
pub struct Delay {
    duration: Expression,
    operands: Vec<Expression>,
}

impl Delay {
    pub fn new<T: From<Delay>>(duration: Expression, operands: Vec<Expression>) -> T {
        Delay { duration, operands }.into()
    }
}

impl AsQasmStr for Delay {
    fn as_qasm_str(&self) -> String {
        format!(
            "delay[{}] {};",
            self.duration.as_qasm_str(),
            self.operands
                .iter()
                .map(|operand| operand.as_qasm_str())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[derive(Debug)]
pub struct Extern {
    id: Identifier,
    args: Vec<(Type, Option<Expression>)>,
    ret_type: Option<Type>,
}

impl Extern {
    pub fn new<T: From<Extern>>(
        id: Identifier,
        args: Vec<(Type, Option<Expression>)>,
        ret_type: Option<Type>,
    ) -> T {
        Extern { id, args, ret_type }.into()
    }
}

impl AsQasmStr for Extern {
    fn as_qasm_str(&self) -> String {
        format!(
            "extern {}({}){};",
            self.id.as_qasm_str(),
            self.args
                .iter()
                .map(|(type_, expr)| {
                    format!(
                        "{}{}",
                        type_.as_qasm_str(),
                        match expr {
                            Some(expr) => format!(" {}", expr.as_qasm_str()),
                            None => "".to_string(),
                        }
                    )
                })
                .collect::<Vec<_>>()
                .join(", "),
            match &self.ret_type {
                Some(ret_type) => format!(" -> {}", ret_type.as_qasm_str()),
                None => "".to_string(),
            }
        )
    }
}

#[derive(Debug)]
pub struct For {
    var_type: Scalar,
    var: Identifier,
    range: Expression,
    scope: Scope,
}

impl For {
    pub fn new<T: From<For>>(
        var_type: Scalar,
        var: Identifier,
        range: Expression,
        scope: Scope,
    ) -> T {
        For {
            var_type,
            var,
            range,
            scope,
        }.into()
    }
}

impl AsQasmStr for For {
    fn as_qasm_str(&self) -> String {
        format!(
            "for {} {} in {} {}",
            self.var_type.as_qasm_str(),
            self.var.as_qasm_str(),
            self.range.as_qasm_str(),
            self.scope.as_qasm_str()
        )
    }
}

#[derive(Debug)]
pub enum GateMod {
    Inv,
    Pow(Expression),
    Ctrl(Expression),
    NegCtrl(Expression),
}

impl AsQasmStr for GateMod {
    fn as_qasm_str(&self) -> String {
        match self {
            GateMod::Inv => "inv @".to_string(),
            GateMod::Pow(expr) => format!("pow({}) @", expr.as_qasm_str()),
            GateMod::Ctrl(expr) => format!("ctrl({}) @", expr.as_qasm_str()),
            GateMod::NegCtrl(expr) => format!("negctrl({}) @", expr.as_qasm_str()),
        }
    }
}

#[derive(Debug)]
pub struct GateCall {
    mods: Vec<GateMod>,
    id: Identifier,
    params: Vec<Expression>,
    expr: Option<Expression>,
    args: Vec<Expression>,
}

impl GateCall {
    pub fn new<T: From<GateCall>>(
        mods: Vec<GateMod>,
        id: Identifier,
        params: Vec<Expression>,
        expr: Option<Expression>,
        args: Vec<Expression>,
    ) -> T {
        GateCall {
            mods,
            id,
            params,
            expr,
            args,
        }.into()
    }
}

impl AsQasmStr for GateCall {
    fn as_qasm_str(&self) -> String {
        format!(
            "{}{}({}){} {};",
            self.mods
                .iter()
                .map(|mod_| mod_.as_qasm_str())
                .collect::<Vec<_>>()
                .join(" "),
            self.id.as_qasm_str(),
            self.params
                .iter()
                .map(|param| param.as_qasm_str())
                .collect::<Vec<_>>()
                .join(", "),
            match &self.expr {
                Some(expr) => format!("[{}]", expr.as_qasm_str()),
                None => "".to_string(),
            },
            self.args
                .iter()
                .map(|arg| arg.as_qasm_str())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[derive(Debug)]
pub struct Gate {
    id: Identifier,
    params: Vec<Identifier>,
    args: Vec<Identifier>,
    scope: Scope,
}

impl Gate {
    pub fn new<T: From<Gate>>(
        id: Identifier,
        params: Vec<Identifier>,
        args: Vec<Identifier>,
        scope: Scope,
    ) -> T {
        Gate {
            id,
            params,
            args,
            scope,
        }.into()
    }
}

impl AsQasmStr for Gate {
    fn as_qasm_str(&self) -> String {
        format!(
            "gate {}{}{} {}",
            self.id.as_qasm_str(),
            match self.params.len() {
                0 => "".to_string(),
                _ => format!(
                    "({})",
                    self.params
                        .iter()
                        .map(|param| param.as_qasm_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            },
            self.args
                .iter()
                .map(|arg| arg.as_qasm_str())
                .collect::<Vec<_>>()
                .join(", "),
            self.scope.as_qasm_str()
        )
    }
}

#[derive(Debug)]
pub struct If {
    condition: Box<Expression>,
    body: Box<StatementOrScope>,
    else_body: Box<Option<StatementOrScope>>,
}

impl If {
    pub fn new<T: From<If>>(condition: Expression, body: StatementOrScope) -> T {
        If {
            condition: Box::new(condition),
            body: Box::new(body),
            else_body: Box::new(None),
        }.into()
    }

    pub fn with_else<T: From<If>>(
        condition: Expression,
        body: StatementOrScope,
        else_body: StatementOrScope,
    ) -> T {
        If {
            condition: Box::new(condition),
            body: Box::new(body),
            else_body: Box::new(Some(else_body)),
        }.into()
    }
}

impl AsQasmStr for If {
    fn as_qasm_str(&self) -> String {
        format!(
            "if ({}) {}{}",
            self.condition.as_qasm_str(),
            self.body.as_qasm_str(),
            match &*self.else_body {
                Some(else_body) => format!(" else {}", else_body.as_qasm_str()),
                None => "".to_string(),
            }
        )
    }
}

#[derive(Debug)]
pub struct Include {
    path: String,
}

impl Include {
    pub fn new<T: From<Include>>(path: String) -> T {
        Include { path }.into()
    }
}

impl AsQasmStr for Include {
    fn as_qasm_str(&self) -> String {
        format!("include \"{}\";", self.path)
    }
}

#[derive(Debug)]
pub enum IOType {
    In,
    Out,
}

impl AsQasmStr for IOType {
    fn as_qasm_str(&self) -> String {
        match self {
            IOType::In => "input".to_string(),
            IOType::Out => "output".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct IODeclaration {
    io_type: IOType,
    type_: Type,
    id: Identifier,
}

impl IODeclaration {
    pub fn new<T: From<IODeclaration>>(io_type: IOType, type_: Type, id: Identifier) -> T {
        IODeclaration { io_type, type_, id }.into()
    }
}

impl AsQasmStr for IODeclaration {
    fn as_qasm_str(&self) -> String {
        format!(
            "{} {} {};",
            self.io_type.as_qasm_str(),
            self.type_.as_qasm_str(),
            self.id.as_qasm_str()
        )
    }
}

#[derive(Debug)]
pub struct MeasureArrowAssignment {
    measure_expr: Expression,
    expr: Option<Expression>,
}

impl MeasureArrowAssignment {
    pub fn new<T: From<MeasureArrowAssignment>>(
        measure_expr: Expression,
        expr: Option<Expression>,
    ) -> T {
        MeasureArrowAssignment { measure_expr, expr }.into()
    }
}

impl AsQasmStr for MeasureArrowAssignment {
    fn as_qasm_str(&self) -> String {
        format!(
            "{}{}",
            self.measure_expr.as_qasm_str(),
            match &self.expr {
                Some(expr) => format!(" -> {}", expr.as_qasm_str()),
                None => "".to_string(),
            }
        )
    }
}

#[derive(Debug)]
pub struct OldStyleDeclaration {
    type_: Type,
    id: Identifier,
    expr: Option<Expression>,
}

impl OldStyleDeclaration {
    pub fn new<T: From<OldStyleDeclaration>>(
        type_: Type,
        id: Identifier,
        expr: Option<Expression>,
    ) -> T {
        OldStyleDeclaration { type_, id, expr }.into()
    }
}

impl AsQasmStr for OldStyleDeclaration {
    fn as_qasm_str(&self) -> String {
        format!(
            "{} {}{};",
            self.type_.as_qasm_str(),
            self.id.as_qasm_str(),
            match &self.expr {
                Some(expr) => format!("[{}]", expr.as_qasm_str()),
                None => "".to_string(),
            }
        )
    }
}

#[derive(Debug)]
pub struct Pragma {
    content: String,
}

impl Pragma {
    pub fn new<T: From<Pragma>>(content: String) -> T {
        Pragma { content }.into()
    }
}

impl AsQasmStr for Pragma {
    fn as_qasm_str(&self) -> String {
        format!("pragma {}", self.content)
    }
}

#[derive(Debug)]
pub struct QuantumDeclaration {
    type_: Type,
    id: Identifier,
}

impl QuantumDeclaration {
    pub fn new<T: From<QuantumDeclaration>>(type_: Type, id: Identifier) -> T {
        QuantumDeclaration { type_, id }.into()
    }
}

impl AsQasmStr for QuantumDeclaration {
    fn as_qasm_str(&self) -> String {
        format!("{} {};", self.type_.as_qasm_str(), self.id.as_qasm_str())
    }
}

#[derive(Debug)]
pub struct Reset {
    id: Identifier,
}

impl Reset {
    pub fn new<T: From<Reset>>(id: Identifier) -> T {
        Reset { id }.into()
    }
}

impl AsQasmStr for Reset {
    fn as_qasm_str(&self) -> String {
        format!("reset {};", self.id.as_qasm_str())
    }
}

#[derive(Debug)]
pub struct Return {
    expr: Option<Expression>,
}

impl Return {
    pub fn new<T: From<Return>>(expr: Option<Expression>) -> T {
        Return { expr }.into()
    }
}

impl AsQasmStr for Return {
    fn as_qasm_str(&self) -> String {
        match &self.expr {
            Some(expr) => format!("return {};", expr.as_qasm_str()),
            None => "return;".to_string(),
        }
    }
}

#[derive(Debug)]
pub struct While {
    condition: Expression,
    body: Box<StatementOrScope>,
}

impl While {
    pub fn new<T: From<While>>(condition: Expression, body: StatementOrScope) -> T {
        While { condition, body: Box::new(body) }.into()
    }
}

impl AsQasmStr for While {
    fn as_qasm_str(&self) -> String {
        format!(
            "while ({}) {}",
            self.condition.as_qasm_str(),
            self.body.as_qasm_str()
        )
    }
}

#[derive(Debug)]
pub struct SwitchItem {
    exprs: Vec<Expression>,
    body: Scope,
}

impl SwitchItem {
    pub fn new<T: From<SwitchItem>>(exprs: Vec<Expression>, body: Scope) -> T {
        SwitchItem { exprs, body }.into()
    }
}

impl AsQasmStr for SwitchItem {
    fn as_qasm_str(&self) -> String {
        format!(
            "case {} {}",
            self.exprs
                .iter()
                .map(|expr| expr.as_qasm_str())
                .collect::<Vec<String>>()
                .join(", "),
            self.body.as_qasm_str()
        )
    }
}

#[derive(Debug)]
pub struct Switch {
    expr: Expression,
    items: Vec<SwitchItem>,
    default: Option<Scope>,
}

impl Switch {
    pub fn new<T: From<Switch>>(
        expr: Expression,
        items: Vec<SwitchItem>,
        default: Option<Scope>,
    ) -> T {
        Switch {
            expr,
            items,
            default,
        }.into()
    }
}

impl AsQasmStr for Switch {
    fn as_qasm_str(&self) -> String {
        format!(
            "switch ({}) {{\n{}\n{}}}",
            self.expr.as_qasm_str(),
            self.items
                .iter()
                .map(|item| item.as_qasm_str())
                .collect::<Vec<String>>()
                .join("\n"),
            match &self.default {
                Some(scope) => format!("default: {}\n", scope.as_qasm_str()),
                None => "".to_string(),
            }
        )
    }
}

#[derive(Debug)]
pub struct Annotation {
    id: Identifier,
    content: String,
}

impl Annotation {
    pub fn new(id: Identifier, content: String) -> Self {
        Annotation { id, content }
    }
}

impl AsQasmStr for Annotation {
    fn as_qasm_str(&self) -> String {
        format!("@{} {}", self.id.as_qasm_str(), self.content)
    }
}

#[derive(Debug)]
pub struct Annotated {
    annotations: Vec<Annotation>, 
    stmt: Box<Statement>,
}

impl Annotated {
    pub fn new<T: From<Annotated>>(annotations: Vec<Annotation>, stmt: Box<Statement>) -> T {
        Annotated { annotations, stmt }.into()
    }
}

impl AsQasmStr for Annotated {
    fn as_qasm_str(&self) -> String {
        if self.annotations.is_empty() {
            return self.stmt.as_qasm_str();
        } else {
            format!(
                "{}\n{}",
                self.annotations
                    .iter()
                    .map(|annotation| annotation.as_qasm_str())
                    .collect::<Vec<String>>()
                    .join("\n"),
                self.stmt.as_qasm_str()
            )
        }
    }
}

#[derive(Debug)]
pub enum Statement {
    Annotated(Annotated),
    AliasDeclaration(AliasDeclaration),
    Assignment(Assignment),
    Barrier(Barrier),
    Box(BoxStatement),
    Break,
    Cal(Cal),
    CalibrationGrammar(CalibrationGrammar),
    ClassicalDeclaration(ClassicalDeclaration),
    ConstDeclaration(ConstDeclaration),
    Continue,
    Def(Def),
    Defcal(Defcal),
    Delay(Delay),
    End,
    Expression(Expression),
    Extern(Extern),
    For(For),
    GateCall(GateCall),
    Gate(Gate),
    If(If),
    Include(Include),
    IODeclaration(IODeclaration),
    MeasureArrowAssignment(MeasureArrowAssignment),
    OldStyleDeclaration(OldStyleDeclaration),
    Pragma(String),
    QuantumDeclaration(QuantumDeclaration),
    Reset(Reset),
    Return(Return),
    While(While),
    Switch(Switch),
}

impl AsQasmStr for Statement {
    fn as_qasm_str(&self) -> String {
        match self {
            Statement::Annotated(annotated) => annotated.as_qasm_str(),
            Statement::AliasDeclaration(alias_declaration) => alias_declaration.as_qasm_str(),
            Statement::Assignment(assignment) => assignment.as_qasm_str(),
            Statement::Barrier(barrier) => barrier.as_qasm_str(),
            Statement::Box(box_statement) => box_statement.as_qasm_str(),
            Statement::Break => "break;".to_string(),
            Statement::Cal(cal) => cal.as_qasm_str(),
            Statement::CalibrationGrammar(grammar) => grammar.as_qasm_str(),
            Statement::ClassicalDeclaration(decl) => decl.as_qasm_str(),
            Statement::ConstDeclaration(decl) => decl.as_qasm_str(),
            Statement::Continue => "continue;".to_string(),
            Statement::Def(def) => def.as_qasm_str(),
            Statement::Defcal(defcal) => defcal.as_qasm_str(),
            Statement::Delay(delay) => delay.as_qasm_str(),
            Statement::End => "end;".to_string(),
            Statement::Expression(expr) => format!("{};", expr.as_qasm_str()),
            Statement::Extern(extern_stmt) => extern_stmt.as_qasm_str(),
            Statement::For(for_stmt) => for_stmt.as_qasm_str(),
            Statement::GateCall(gate_call) => gate_call.as_qasm_str(),
            Statement::Gate(gate) => gate.as_qasm_str(),
            Statement::If(if_stmt) => if_stmt.as_qasm_str(),
            Statement::Include(include) => include.as_qasm_str(),
            Statement::IODeclaration(decl) => decl.as_qasm_str(),
            Statement::MeasureArrowAssignment(assignment) => {
                assignment.as_qasm_str()
            }
            Statement::OldStyleDeclaration(decl) => decl.as_qasm_str(),
            Statement::Pragma(pragma) => format!("#pragma {}", pragma),
            Statement::QuantumDeclaration(decl) => decl.as_qasm_str(),
            Statement::Reset(reset) => reset.as_qasm_str(),
            Statement::Return(return_stmt) => return_stmt.as_qasm_str(),
            Statement::While(while_stmt) => while_stmt.as_qasm_str(),
            Statement::Switch(switch) => switch.as_qasm_str(),
        }
    }
}

#[derive(Debug)]
pub struct Scope {
    body: Vec<StatementOrScope>,
}

impl Scope {
    pub fn new<T: From<Scope>>(body: Vec<StatementOrScope>) -> T {
        Scope { body }.into()
    }
}

impl AsQasmStr for Scope {
    fn as_qasm_str(&self) -> String {
        format!(
            "{{\n{}\n}}",
            self.body
                .iter()
                .map(|stmt_or_scope| stmt_or_scope.as_qasm_str())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

#[derive(Debug)]
pub enum StatementOrScope {
    Statement(Statement),
    Scope(Scope),
}

impl AsQasmStr for StatementOrScope {
    fn as_qasm_str(&self) -> String {
        match self {
            StatementOrScope::Statement(stmt) => stmt.as_qasm_str(),
            StatementOrScope::Scope(scope) => scope.as_qasm_str(),
        }
    }
}

impl From<Scope> for StatementOrScope {
    fn from(scope: Scope) -> Self {
        StatementOrScope::Scope(scope)
    }
}

impl<T: Into<Statement>> From<T> for StatementOrScope {
    fn from(stmt: T) -> Self {
        StatementOrScope::Statement(stmt.into())
    }
}
