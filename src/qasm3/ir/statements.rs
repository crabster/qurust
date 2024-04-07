//! Structures and enums for QASM3 statements and scopes representation.
//!
//! Statements are parts of QASM3 code, which are defined by the `statementOrScope`
//! rule in the antlr4 grammar for QASM3. To learn more about statements, you can refer to
//! the official [QASM3 documentation](https://openqasm.com/versions/3.0/language/index.html)
//! and to view a list of statements, have a look at the
//! [QASM3 grammar files](https://github.com/openqasm/openqasm/tree/main/source/grammar).

use crate::qasm3::ir::expressions::{BinaryOperator, Expression, Identifier};
use crate::qasm3::ir::types::{Scalar, Type};
use crate::qasm3::ir::AsQasmStr;

/// QASM3 alias declaration.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct AliasDeclaration {
    identifier: Identifier,
    exprs: Vec<Expression>,
}

impl AliasDeclaration {
    pub fn new(identifier: Identifier, exprs: Vec<Expression>) -> AliasDeclaration {
        AliasDeclaration { identifier, exprs }
    }

    pub fn newt<T: From<AliasDeclaration>>(identifier: Identifier, exprs: Vec<Expression>) -> T {
        Self::new(identifier, exprs).into()
    }
}

impl AsQasmStr for AliasDeclaration {
    fn as_qasm_str(&self) -> String {
        format!(
            "let {} = {};",
            self.identifier.as_qasm_str(),
            self.exprs
                .iter()
                .map(|expr| expr.as_qasm_str())
                .collect::<Vec<_>>()
                .join(" ++ ")
        )
    }
}

/// QASM3 (compound) assignment statement.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Assignment {
    id_expr: Expression,
    operator: Option<BinaryOperator>,
    expr: Expression,
}

impl Assignment {
    pub fn new(
        id_expr: Expression,
        operator: Option<BinaryOperator>,
        expr: Expression,
    ) -> Assignment {
        Assignment {
            id_expr,
            operator,
            expr,
        }
    }

    pub fn newt<T: From<Assignment>>(
        id_expr: Expression,
        operator: Option<BinaryOperator>,
        expr: Expression,
    ) -> T {
        Self::new(id_expr, operator, expr).into()
    }
}

impl AsQasmStr for Assignment {
    fn as_qasm_str(&self) -> String {
        let assign_op = match &self.operator {
            Some(op) => op.as_qasm_str(),
            None => "".to_string(),
        };

        format!(
            "{} {}= {};",
            self.id_expr.as_qasm_str(),
            assign_op,
            self.expr.as_qasm_str()
        )
    }
}

/// QASM3 barrier statement.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Barrier {
    exprs: Vec<Expression>,
}

impl Barrier {
    pub fn new(exprs: Vec<Expression>) -> Barrier {
        Barrier { exprs }
    }

    pub fn newt<T: From<Barrier>>(exprs: Vec<Expression>) -> T {
        Self::new(exprs).into()
    }
}

impl AsQasmStr for Barrier {
    fn as_qasm_str(&self) -> String {
        if self.exprs.is_empty() {
            "barrier;".to_string()
        } else {
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
}

/// QASM3 box statement.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct BoxStatement {
    expr: Option<Expression>,
    scope: Scope,
}

impl BoxStatement {
    pub fn new(expr: Option<Expression>, scope: Scope) -> BoxStatement {
        BoxStatement { expr, scope }
    }

    pub fn newt<T: From<BoxStatement>>(expr: Option<Expression>, scope: Scope) -> T {
        Self::new(expr, scope).into()
    }
}

impl AsQasmStr for BoxStatement {
    fn as_qasm_str(&self) -> String {
        format!(
            "box{} {}",
            match &self.expr {
                Some(expr) => format!("[{}]", expr.as_qasm_str()),
                None => "".to_string(),
            },
            self.scope.as_qasm_str()
        )
    }
}

/// QASM3 calibration block.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Cal {
    cal_block: Option<String>,
}

impl Cal {
    pub fn new(cal_block: Option<String>) -> Cal {
        Cal { cal_block }
    }

    pub fn newt<T: From<Cal>>(cal_block: Option<String>) -> T {
        Self::new(cal_block).into()
    }
}

impl AsQasmStr for Cal {
    fn as_qasm_str(&self) -> String {
        format!("cal {{{}}}", self.cal_block.clone().unwrap_or_default())
    }
}

/// QASM3 calibration grammar declaration.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CalibrationGrammar {
    grammar: String,
}

impl CalibrationGrammar {
    pub fn new(grammar: String) -> CalibrationGrammar {
        CalibrationGrammar { grammar }
    }

    pub fn newt<T: From<CalibrationGrammar>>(grammar: String) -> T {
        Self::new(grammar).into()
    }
}

impl AsQasmStr for CalibrationGrammar {
    fn as_qasm_str(&self) -> String {
        format!("defcalgrammar {};", self.grammar)
    }
}

/// QASM3 classical declaration statement.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ClassicalDeclaration {
    type_: Type,
    id: Identifier,
    expr: Option<Expression>,
}

impl ClassicalDeclaration {
    pub fn new(type_: Type, id: Identifier, expr: Option<Expression>) -> ClassicalDeclaration {
        ClassicalDeclaration { type_, id, expr }
    }

    pub fn newt<T: From<ClassicalDeclaration>>(
        type_: Type,
        id: Identifier,
        expr: Option<Expression>,
    ) -> T {
        Self::new(type_, id, expr).into()
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

/// QASM3 constant declaration statement.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ConstDeclaration {
    type_: Type,
    id: Identifier,
    expr: Expression,
}

impl ConstDeclaration {
    pub fn new(type_: Type, id: Identifier, expr: Expression) -> ConstDeclaration {
        ConstDeclaration { type_, id, expr }
    }

    pub fn newt<T: From<ConstDeclaration>>(type_: Type, id: Identifier, expr: Expression) -> T {
        Self::new(type_, id, expr).into()
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

/// QASM3 function argument.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct DefArgument {
    type_: Type,
    id: Identifier,
    reg_size: Option<Expression>,
}

impl DefArgument {
    pub fn new(type_: Type, id: Identifier, reg_size: Option<Expression>) -> DefArgument {
        DefArgument {
            type_,
            id,
            reg_size,
        }
    }

    pub fn newt<T: From<DefArgument>>(
        type_: Type,
        id: Identifier,
        reg_size: Option<Expression>,
    ) -> T {
        Self::new(type_, id, reg_size).into()
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

/// QASM3 function definition.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Def {
    id: Identifier,
    args: Vec<DefArgument>,
    ret_type: Option<Type>,
    scope: Scope,
}

impl Def {
    pub fn new(
        id: Identifier,
        args: Vec<DefArgument>,
        ret_type: Option<Type>,
        scope: Scope,
    ) -> Def {
        Def {
            id,
            args,
            ret_type,
            scope,
        }
    }

    pub fn newt<T: From<Def>>(
        id: Identifier,
        args: Vec<DefArgument>,
        ret_type: Option<Type>,
        scope: Scope,
    ) -> T {
        Self::new(id, args, ret_type, scope).into()
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

/// QASM3 defcal target.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

/// QASM3 defcal argument.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

/// QASM3 defcal statement.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Defcal {
    target: DefcalTarget,
    args: Vec<DefcalArgument>,
    operands: Vec<Expression>,
    ret_type: Option<Type>,
    cal_block: Option<String>,
}

impl Defcal {
    pub fn new(
        target: DefcalTarget,
        args: Vec<DefcalArgument>,
        operands: Vec<Expression>,
        ret_type: Option<Type>,
        cal_block: Option<String>,
    ) -> Defcal {
        Defcal {
            target,
            args,
            operands,
            ret_type,
            cal_block,
        }
    }

    pub fn newt<T: From<Defcal>>(
        target: DefcalTarget,
        args: Vec<DefcalArgument>,
        operands: Vec<Expression>,
        ret_type: Option<Type>,
        cal_block: Option<String>,
    ) -> T {
        Self::new(target, args, operands, ret_type, cal_block).into()
    }
}

impl AsQasmStr for Defcal {
    fn as_qasm_str(&self) -> String {
        let mut args = self
            .args
            .iter()
            .map(|arg| arg.as_qasm_str())
            .collect::<Vec<_>>()
            .join(", ");
        if !args.is_empty() {
            args = format!("({})", args);
        }
        format!(
            "defcal {}{} {}{} {{{}}}",
            self.target.as_qasm_str(),
            args,
            self.operands
                .iter()
                .map(|operand| operand.as_qasm_str())
                .collect::<Vec<_>>()
                .join(", "),
            match &self.ret_type {
                Some(ret_type) => format!(" -> {}", ret_type.as_qasm_str()),
                None => "".to_string(),
            },
            self.cal_block.clone().unwrap_or_default()
        )
    }
}

/// QASM3 delay statement.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Delay {
    duration: Expression,
    operands: Vec<Expression>,
}

impl Delay {
    pub fn new(duration: Expression, operands: Vec<Expression>) -> Delay {
        Delay { duration, operands }
    }

    pub fn newt<T: From<Delay>>(duration: Expression, operands: Vec<Expression>) -> T {
        Self::new(duration, operands).into()
    }
}

impl AsQasmStr for Delay {
    fn as_qasm_str(&self) -> String {
        if self.operands.is_empty() {
            format!("delay [{}];", self.duration.as_qasm_str())
        } else {
            format!(
                "delay [{}] {};",
                self.duration.as_qasm_str(),
                self.operands
                    .iter()
                    .map(|operand| operand.as_qasm_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }
}

/// QASM3 extern argument.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct ExternArgument {
    type_: Type,
    expr: Option<Expression>,
}

impl ExternArgument {
    pub fn new(type_: Type, expr: Option<Expression>) -> ExternArgument {
        ExternArgument { type_, expr }
    }

    pub fn newt<T: From<ExternArgument>>(type_: Type, expr: Option<Expression>) -> T {
        Self::new(type_, expr).into()
    }
}

impl AsQasmStr for ExternArgument {
    fn as_qasm_str(&self) -> String {
        match &self.expr {
            Some(expr) => format!("{}[{}]", self.type_.as_qasm_str(), expr.as_qasm_str()),
            None => self.type_.as_qasm_str(),
        }
    }
}

/// QASM3 extern function declaration.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Extern {
    id: Identifier,
    args: Vec<ExternArgument>,
    ret_type: Option<Type>,
}

impl Extern {
    pub fn new(id: Identifier, args: Vec<ExternArgument>, ret_type: Option<Type>) -> Extern {
        Extern { id, args, ret_type }
    }

    pub fn newt<T: From<Extern>>(
        id: Identifier,
        args: Vec<ExternArgument>,
        ret_type: Option<Type>,
    ) -> T {
        Self::new(id, args, ret_type).into()
    }
}

impl AsQasmStr for Extern {
    fn as_qasm_str(&self) -> String {
        format!(
            "extern {}({}){};",
            self.id.as_qasm_str(),
            self.args
                .iter()
                .map(|arg| arg.as_qasm_str())
                .collect::<Vec<_>>()
                .join(", "),
            match &self.ret_type {
                Some(ret_type) => format!(" -> {}", ret_type.as_qasm_str()),
                None => "".to_string(),
            }
        )
    }
}

/// QASM3 for loop statement.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct For {
    var_type: Scalar,
    var: Identifier,
    range: Expression,
    scope: Scope,
}

impl For {
    pub fn new(var_type: Scalar, var: Identifier, range: Expression, scope: Scope) -> Self {
        For {
            var_type,
            var,
            range,
            scope,
        }
    }
    pub fn newt<T: From<For>>(
        var_type: Scalar,
        var: Identifier,
        range: Expression,
        scope: Scope,
    ) -> T {
        Self::new(var_type, var, range, scope).into()
    }
}

impl AsQasmStr for For {
    fn as_qasm_str(&self) -> String {
        format!(
            "for {} {} in {} {}",
            self.var_type.as_qasm_str(),
            self.var.as_qasm_str(),
            match &self.range {
                Expression::Range(range) => format!("[{}]", range.as_qasm_str()),
                _ => self.range.as_qasm_str(),
            },
            self.scope.as_qasm_str()
        )
    }
}

/// QASM3 gate modifier.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum GateMod {
    Inv,
    Pow(Expression),
    Ctrl(Option<Expression>),
    NegCtrl(Option<Expression>),
}

impl AsQasmStr for GateMod {
    fn as_qasm_str(&self) -> String {
        let opt_expr_str = |expr: &Option<Expression>| match expr {
            Some(expr) => format!("({})", expr.as_qasm_str()),
            None => "".to_string(),
        };

        match self {
            GateMod::Inv => "inv @".to_string(),
            GateMod::Pow(expr) => format!("pow({}) @", expr.as_qasm_str()),
            GateMod::Ctrl(expr) => format!("ctrl{} @", opt_expr_str(expr)),
            GateMod::NegCtrl(expr) => format!("negctrl{} @", opt_expr_str(expr)),
        }
    }
}

/// QASM3 gate call statement.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct GateCall {
    mods: Vec<GateMod>,
    id: Identifier,
    params: Vec<Expression>,
    expr: Option<Expression>,
    args: Vec<Expression>,
}

impl GateCall {
    pub fn new(
        mods: Vec<GateMod>,
        id: Identifier,
        params: Vec<Expression>,
        expr: Option<Expression>,
        args: Vec<Expression>,
    ) -> GateCall {
        GateCall {
            mods,
            id,
            params,
            expr,
            args,
        }
    }

    pub fn newt<T: From<GateCall>>(
        mods: Vec<GateMod>,
        id: Identifier,
        params: Vec<Expression>,
        expr: Option<Expression>,
        args: Vec<Expression>,
    ) -> T {
        Self::new(mods, id, params, expr, args).into()
    }
}

impl AsQasmStr for GateCall {
    fn as_qasm_str(&self) -> String {
        let mods_str = if self.mods.is_empty() {
            "".to_string()
        } else {
            format!(
                "{} ",
                self.mods
                    .iter()
                    .map(|mod_| mod_.as_qasm_str())
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        };
        let params_str = if self.params.is_empty() {
            "".to_string()
        } else {
            format!(
                "({})",
                self.params
                    .iter()
                    .map(|param| param.as_qasm_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };
        let args_str = if self.args.is_empty() {
            "".to_string()
        } else {
            format!(
                " {}",
                self.args
                    .iter()
                    .map(|arg| arg.as_qasm_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };
        format!(
            "{}{}{}{}{};",
            mods_str,
            self.id.as_qasm_str(),
            params_str,
            match &self.expr {
                Some(expr) => format!("[{}]", expr.as_qasm_str()),
                None => "".to_string(),
            },
            args_str
        )
    }
}

/// QASM3 gate definition.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Gate {
    id: Identifier,
    params: Vec<Identifier>,
    args: Vec<Identifier>,
    scope: Scope,
}

impl Gate {
    pub fn new(
        id: Identifier,
        params: Vec<Identifier>,
        args: Vec<Identifier>,
        scope: Scope,
    ) -> Self {
        Gate {
            id,
            params,
            args,
            scope,
        }
    }

    pub fn newt<T: From<Gate>>(
        id: Identifier,
        params: Vec<Identifier>,
        args: Vec<Identifier>,
        scope: Scope,
    ) -> T {
        Self::new(id, params, args, scope).into()
    }
}

impl AsQasmStr for Gate {
    fn as_qasm_str(&self) -> String {
        format!(
            "gate {}{} {} {}",
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

/// QASM3 if statement.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct If {
    condition: Box<Expression>,
    body: Box<StatementOrScope>,
    else_body: Box<Option<StatementOrScope>>,
}

impl If {
    pub fn new(
        condition: Expression,
        body: StatementOrScope,
        else_body: Option<StatementOrScope>,
    ) -> Self {
        If {
            condition: Box::new(condition),
            body: Box::new(body),
            else_body: Box::new(else_body),
        }
    }

    pub fn newt<T: From<If>>(
        condition: Expression,
        body: StatementOrScope,
        else_body: Option<StatementOrScope>,
    ) -> T {
        Self::new(condition, body, else_body).into()
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

/// QASM3 include statement.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Include {
    path: String,
}

impl Include {
    pub fn new(path: String) -> Self {
        Include { path }
    }

    pub fn newt<T: From<Include>>(path: String) -> T {
        Self::new(path).into()
    }
}

impl AsQasmStr for Include {
    fn as_qasm_str(&self) -> String {
        format!("include \"{}\";", self.path)
    }
}

/// QASM3 let statement.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

/// QASM3 IO declaration.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct IODeclaration {
    io_type: IOType,
    type_: Type,
    id: Identifier,
}

impl IODeclaration {
    pub fn new(io_type: IOType, type_: Type, id: Identifier) -> Self {
        IODeclaration { io_type, type_, id }
    }

    pub fn newt<T: From<IODeclaration>>(io_type: IOType, type_: Type, id: Identifier) -> T {
        Self::new(io_type, type_, id).into()
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

/// QASM3 measure arrow statement.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MeasureArrowAssignment {
    measure_expr: Expression,
    expr: Option<Expression>,
}

impl MeasureArrowAssignment {
    pub fn new(measure_expr: Expression, expr: Option<Expression>) -> Self {
        MeasureArrowAssignment { measure_expr, expr }
    }

    pub fn newt<T: From<MeasureArrowAssignment>>(
        measure_expr: Expression,
        expr: Option<Expression>,
    ) -> T {
        Self::new(measure_expr, expr).into()
    }
}

impl AsQasmStr for MeasureArrowAssignment {
    fn as_qasm_str(&self) -> String {
        format!(
            "{}{};",
            self.measure_expr.as_qasm_str(),
            match &self.expr {
                Some(expr) => format!(" -> {}", expr.as_qasm_str()),
                None => "".to_string(),
            }
        )
    }
}

/// QASM3 old style declaration statement.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct OldStyleDeclaration {
    type_: Type,
    id: Identifier,
    expr: Option<Expression>,
}

impl OldStyleDeclaration {
    pub fn new(type_: Type, id: Identifier, expr: Option<Expression>) -> Self {
        OldStyleDeclaration { type_, id, expr }
    }

    pub fn newt<T: From<OldStyleDeclaration>>(
        type_: Type,
        id: Identifier,
        expr: Option<Expression>,
    ) -> T {
        Self::new(type_, id, expr).into()
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

/// QASM3 pragma statement.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Pragma {
    content: String,
}

impl Pragma {
    pub fn new(content: String) -> Pragma {
        Pragma { content }
    }

    pub fn newt<T: From<Pragma>>(content: String) -> T {
        Pragma::new(content).into()
    }
}

impl AsQasmStr for Pragma {
    fn as_qasm_str(&self) -> String {
        format!("pragma {}", self.content)
    }
}

/// QASM3 quantum declaration statement.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct QuantumDeclaration {
    type_: Type,
    id: Identifier,
}

impl QuantumDeclaration {
    pub fn new(type_: Type, id: Identifier) -> Self {
        QuantumDeclaration { type_, id }
    }

    pub fn newt<T: From<QuantumDeclaration>>(type_: Type, id: Identifier) -> T {
        Self::new(type_, id).into()
    }
}

impl AsQasmStr for QuantumDeclaration {
    fn as_qasm_str(&self) -> String {
        format!("{} {};", self.type_.as_qasm_str(), self.id.as_qasm_str())
    }
}

/// QASM3 reset statement.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Reset {
    expr: Expression,
}

impl Reset {
    pub fn new(expr: Expression) -> Reset {
        Reset { expr }
    }

    pub fn newt<T: From<Reset>>(expr: Expression) -> T {
        Reset::new(expr).into()
    }
}

impl AsQasmStr for Reset {
    fn as_qasm_str(&self) -> String {
        format!("reset {};", self.expr.as_qasm_str())
    }
}

/// QASM3 return statement.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Return {
    expr: Option<Expression>,
}

impl Return {
    pub fn new(expr: Option<Expression>) -> Return {
        Return { expr }
    }

    pub fn newt<T: From<Return>>(expr: Option<Expression>) -> T {
        Self::new(expr).into()
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

/// QASM3 while loop statement.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct While {
    condition: Expression,
    body: Box<StatementOrScope>,
}

impl While {
    pub fn new(condition: Expression, body: StatementOrScope) -> While {
        While {
            condition,
            body: Box::new(body),
        }
    }

    pub fn newt<T: From<While>>(condition: Expression, body: StatementOrScope) -> T {
        Self::new(condition, body).into()
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

/// QASM3 switch item.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct SwitchItem {
    exprs: Vec<Expression>,
    body: Scope,
}

impl SwitchItem {
    pub fn new(exprs: Vec<Expression>, body: Scope) -> SwitchItem {
        SwitchItem { exprs, body }
    }

    pub fn newt<T: From<SwitchItem>>(exprs: Vec<Expression>, body: Scope) -> T {
        Self::new(exprs, body).into()
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

/// QASM3 switch statement.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Switch {
    expr: Expression,
    items: Vec<SwitchItem>,
    default: Option<Scope>,
    indent: usize,
}

impl Switch {
    pub fn new(expr: Expression, items: Vec<SwitchItem>, default: Option<Scope>) -> Switch {
        Switch {
            expr,
            items,
            default,
            indent: 4,
        }
    }

    pub fn newt<T: From<Switch>>(
        expr: Expression,
        items: Vec<SwitchItem>,
        default: Option<Scope>,
    ) -> T {
        Self::new(expr, items, default).into()
    }

    pub fn with_indent(
        expr: Expression,
        items: Vec<SwitchItem>,
        default: Option<Scope>,
        indent: usize,
    ) -> Switch {
        Switch {
            expr,
            items,
            default,
            indent,
        }
    }
}

impl AsQasmStr for Switch {
    fn as_qasm_str(&self) -> String {
        let mut body = "".to_string();
        if !self.items.is_empty() || self.default.is_some() {
            body = format!(
                "\n{}\n{}",
                self.items
                    .iter()
                    .map(|item| item.as_qasm_str())
                    .collect::<Vec<String>>()
                    .join("\n"),
                match &self.default {
                    Some(scope) => format!("default: {}\n", scope.as_qasm_str()),
                    None => "".to_string(),
                }
            );
        }

        format!(
            "switch ({}) {{{}}}",
            self.expr.as_qasm_str(),
            indent::indent_all_by(self.indent, body),
        )
    }
}

/// QASM3 annotation.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
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
        let content = if self.content.is_empty() {
            "".to_string()
        } else {
            format!(" {}", self.content)
        };
        format!("@{}{}", self.id.as_qasm_str(), content)
    }
}

/// QASM3 annotated statement.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Annotated {
    annotations: Vec<Annotation>,
    stmt: Box<Statement>,
}

impl Annotated {
    pub fn new(annotations: Vec<Annotation>, stmt: Statement) -> Annotated {
        Annotated {
            annotations,
            stmt: Box::new(stmt),
        }
    }

    pub fn newt<T: From<Annotated>>(annotations: Vec<Annotation>, stmt: Statement) -> T {
        Self::new(annotations, stmt).into()
    }
}

impl AsQasmStr for Annotated {
    fn as_qasm_str(&self) -> String {
        if self.annotations.is_empty() {
            self.stmt.as_qasm_str()
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

/// QASM3 statement enum representing all possible statements.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Statement {
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
    Pragma(Pragma),
    QuantumDeclaration(QuantumDeclaration),
    Reset(Reset),
    Return(Return),
    While(While),
    Switch(Switch),
    Annotated(Annotated),
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
            Statement::MeasureArrowAssignment(assignment) => assignment.as_qasm_str(),
            Statement::OldStyleDeclaration(decl) => decl.as_qasm_str(),
            Statement::Pragma(pragma) => pragma.as_qasm_str(),
            Statement::QuantumDeclaration(decl) => decl.as_qasm_str(),
            Statement::Reset(reset) => reset.as_qasm_str(),
            Statement::Return(return_stmt) => return_stmt.as_qasm_str(),
            Statement::While(while_stmt) => while_stmt.as_qasm_str(),
            Statement::Switch(switch) => switch.as_qasm_str(),
        }
    }
}

impl From<AliasDeclaration> for Statement {
    fn from(decl: AliasDeclaration) -> Self {
        Statement::AliasDeclaration(decl)
    }
}

impl From<Assignment> for Statement {
    fn from(assignment: Assignment) -> Self {
        Statement::Assignment(assignment)
    }
}

impl From<Barrier> for Statement {
    fn from(barrier: Barrier) -> Self {
        Statement::Barrier(barrier)
    }
}

impl From<BoxStatement> for Statement {
    fn from(box_statement: BoxStatement) -> Self {
        Statement::Box(box_statement)
    }
}

impl From<Cal> for Statement {
    fn from(cal: Cal) -> Self {
        Statement::Cal(cal)
    }
}

impl From<CalibrationGrammar> for Statement {
    fn from(grammar: CalibrationGrammar) -> Self {
        Statement::CalibrationGrammar(grammar)
    }
}

impl From<ClassicalDeclaration> for Statement {
    fn from(decl: ClassicalDeclaration) -> Self {
        Statement::ClassicalDeclaration(decl)
    }
}

impl From<ConstDeclaration> for Statement {
    fn from(decl: ConstDeclaration) -> Self {
        Statement::ConstDeclaration(decl)
    }
}

impl From<Def> for Statement {
    fn from(def: Def) -> Self {
        Statement::Def(def)
    }
}

impl From<Defcal> for Statement {
    fn from(defcal: Defcal) -> Self {
        Statement::Defcal(defcal)
    }
}

impl From<Delay> for Statement {
    fn from(delay: Delay) -> Self {
        Statement::Delay(delay)
    }
}

impl From<Extern> for Statement {
    fn from(extern_stmt: Extern) -> Self {
        Statement::Extern(extern_stmt)
    }
}

impl From<For> for Statement {
    fn from(for_stmt: For) -> Self {
        Statement::For(for_stmt)
    }
}

impl From<GateCall> for Statement {
    fn from(gate_call: GateCall) -> Self {
        Statement::GateCall(gate_call)
    }
}

impl From<Gate> for Statement {
    fn from(gate: Gate) -> Self {
        Statement::Gate(gate)
    }
}

impl From<If> for Statement {
    fn from(if_stmt: If) -> Self {
        Statement::If(if_stmt)
    }
}

impl From<Include> for Statement {
    fn from(include: Include) -> Self {
        Statement::Include(include)
    }
}

impl From<IODeclaration> for Statement {
    fn from(decl: IODeclaration) -> Self {
        Statement::IODeclaration(decl)
    }
}

impl From<MeasureArrowAssignment> for Statement {
    fn from(assignment: MeasureArrowAssignment) -> Self {
        Statement::MeasureArrowAssignment(assignment)
    }
}

impl From<OldStyleDeclaration> for Statement {
    fn from(decl: OldStyleDeclaration) -> Self {
        Statement::OldStyleDeclaration(decl)
    }
}

impl From<Pragma> for Statement {
    fn from(pragma: Pragma) -> Self {
        Statement::Pragma(pragma)
    }
}

impl From<QuantumDeclaration> for Statement {
    fn from(decl: QuantumDeclaration) -> Self {
        Statement::QuantumDeclaration(decl)
    }
}

impl From<Reset> for Statement {
    fn from(reset: Reset) -> Self {
        Statement::Reset(reset)
    }
}

impl From<Return> for Statement {
    fn from(return_stmt: Return) -> Self {
        Statement::Return(return_stmt)
    }
}

impl From<While> for Statement {
    fn from(while_stmt: While) -> Self {
        Statement::While(while_stmt)
    }
}

impl From<Switch> for Statement {
    fn from(switch: Switch) -> Self {
        Statement::Switch(switch)
    }
}

impl From<Annotated> for Statement {
    fn from(annotated: Annotated) -> Self {
        Statement::Annotated(annotated)
    }
}

/// QASM3 scope.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Scope {
    body: Vec<StatementOrScope>,
    indent: usize,
}

impl Scope {
    pub fn new(body: Vec<StatementOrScope>) -> Scope {
        Scope { body, indent: 4 }
    }

    pub fn newt<T: From<Scope>>(body: Vec<StatementOrScope>) -> T {
        Self::new(body).into()
    }

    pub fn with_indent(body: Vec<StatementOrScope>, indent: usize) -> Scope {
        Scope { body, indent }
    }
}

impl AsQasmStr for Scope {
    fn as_qasm_str(&self) -> String {
        if self.body.is_empty() {
            "{}".to_string()
        } else {
            format!(
                "{{\n{}\n}}",
                indent::indent_all_by(
                    self.indent,
                    self.body
                        .iter()
                        .map(|stmt_or_scope| stmt_or_scope.as_qasm_str())
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            )
        }
    }
}

/// Enum representing either a QASM3 statement or a QASM3 scope.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    use crate::qasm3::ir::expressions::*;
    use crate::qasm3::ir::types;

    #[test]
    fn test_alias_declaration() {
        assert_eq!(
            AliasDeclaration::new(
                Identifier::newt("a".to_string()),
                vec![Identifier::newt("b".to_string())]
            )
            .as_qasm_str(),
            "let a = b;"
        );
    }

    #[test]
    fn test_assignment() {
        assert_eq!(
            Assignment::new(
                Identifier::newt("a".to_string()),
                None,
                Identifier::newt("b".to_string())
            )
            .as_qasm_str(),
            "a = b;"
        );
        assert_eq!(
            Assignment::new(
                Identifier::newt("a".to_string()),
                Some(BinaryOperator::Add),
                Identifier::newt("b".to_string())
            )
            .as_qasm_str(),
            "a += b;"
        );
    }

    #[test]
    fn test_barrier() {
        assert_eq!(
            Barrier::new(vec![Identifier::newt("q".to_string())]).as_qasm_str(),
            "barrier q;"
        );
    }

    #[test]
    fn test_box_statement() {
        assert_eq!(
            BoxStatement::new(None, Scope::new(vec![])).as_qasm_str(),
            "box {}"
        );
        assert_eq!(
            BoxStatement::new(
                Some(Literal::Timing(1.0, TimingUnit::NS).into()),
                Scope::new(vec![])
            )
            .as_qasm_str(),
            "box[1ns] {}"
        );
    }

    #[test]
    fn test_cal() {
        assert_eq!(Cal::new(None).as_qasm_str(), "cal {}");
    }

    #[test]
    fn test_calibration_grammar() {
        assert_eq!(
            CalibrationGrammar::new("\"openpulse\"".to_string()).as_qasm_str(),
            "defcalgrammar \"openpulse\";"
        );
    }

    #[test]
    fn test_classical_declaration() {
        assert_eq!(
            ClassicalDeclaration::new(
                Scalar::Bit(None).into(),
                Identifier::newt("a".to_string()),
                None
            )
            .as_qasm_str(),
            "bit a;"
        );
        assert_eq!(
            ClassicalDeclaration::new(
                types::Array::newt(Scalar::Bit(None), vec![Literal::DecimalInteger(1).into()]),
                Identifier::newt("a".to_string()),
                None,
            )
            .as_qasm_str(),
            "array[bit, 1] a;"
        );
        assert_eq!(
            ClassicalDeclaration::new(
                Scalar::Bit(None).into(),
                Identifier::newt("a".to_string()),
                Some(Literal::DecimalInteger(0).into())
            )
            .as_qasm_str(),
            "bit a = 0;"
        );
    }

    #[test]
    fn test_const_declaration() {
        assert_eq!(
            ConstDeclaration::new(
                Scalar::Bit(None).into(),
                Identifier::newt("a".to_string()),
                Literal::DecimalInteger(0).into()
            )
            .as_qasm_str(),
            "const bit a = 0;"
        );
    }

    #[test]
    fn test_def_argument() {
        assert_eq!(
            DefArgument::new(
                Scalar::Bit(None).into(),
                Identifier::newt("a".to_string()),
                None
            )
            .as_qasm_str(),
            "bit a"
        );
        assert_eq!(
            DefArgument::new(
                types::Register::C.into(),
                Identifier::newt("c".to_string()),
                Some(Literal::DecimalInteger(1).into())
            )
            .as_qasm_str(),
            "creg c[1]"
        );
    }

    #[test]
    fn test_def() {
        assert_eq!(
            Def::new(
                Identifier::newt("foo".to_string()),
                vec![DefArgument::newt(
                    Scalar::Bit(None).into(),
                    Identifier::newt("a".to_string()),
                    None
                )],
                None,
                Scope::new(vec![])
            )
            .as_qasm_str(),
            "def foo(bit a) {}"
        );

        assert_eq!(
            Def::new(
                Identifier::newt("foo".to_string()),
                vec![DefArgument::newt(
                    Scalar::Bit(None).into(),
                    Identifier::newt("a".to_string()),
                    None
                )],
                Some(Scalar::Bit(None).into()),
                Scope::new(vec![])
            )
            .as_qasm_str(),
            "def foo(bit a) -> bit {}"
        );
    }

    #[test]
    fn test_defcal_target() {
        assert_eq!(DefcalTarget::Measure.as_qasm_str(), "measure");
        assert_eq!(DefcalTarget::Reset.as_qasm_str(), "reset");
        assert_eq!(DefcalTarget::Delay.as_qasm_str(), "delay");
        assert_eq!(
            DefcalTarget::Identifier(Identifier::newt("foo".to_string())).as_qasm_str(),
            "foo"
        );
    }

    #[test]
    fn test_defcal() {
        assert_eq!(
            Defcal::new(
                DefcalTarget::Measure,
                vec![],
                vec![Literal::HardwareQubit(1).into()],
                None,
                None
            )
            .as_qasm_str(),
            "defcal measure $1 {}"
        );
        assert_eq!(
            Defcal::new(
                DefcalTarget::Measure,
                vec![DefcalArgument::DefArgument(DefArgument::newt(
                    Scalar::Bit(None).into(),
                    Identifier::newt("a".to_string()),
                    None
                ))],
                vec![Literal::HardwareQubit(1).into()],
                None,
                None
            )
            .as_qasm_str(),
            "defcal measure(bit a) $1 {}"
        );
        assert_eq!(
            Defcal::new(
                DefcalTarget::Measure,
                vec![DefcalArgument::DefArgument(DefArgument::newt(
                    Scalar::Bit(None).into(),
                    Identifier::newt("a".to_string()),
                    None
                ))],
                vec![Literal::HardwareQubit(1).into()],
                Some(Scalar::Bit(None).into()),
                None
            )
            .as_qasm_str(),
            "defcal measure(bit a) $1 -> bit {}"
        );
        assert_eq!(
            Defcal::new(
                DefcalTarget::Measure,
                vec![DefcalArgument::DefArgument(DefArgument::newt(
                    Scalar::Bit(None).into(),
                    Identifier::newt("a".to_string()),
                    None
                ))],
                vec![Literal::HardwareQubit(1).into()],
                Some(Scalar::Bit(None).into()),
                Some("...".to_string())
            )
            .as_qasm_str(),
            "defcal measure(bit a) $1 -> bit {...}"
        );
    }

    #[test]
    fn test_delay() {
        assert_eq!(
            Delay::new(Literal::Timing(1.0, TimingUnit::NS).into(), vec![]).as_qasm_str(),
            "delay [1ns];"
        );
        assert_eq!(
            Delay::new(
                Literal::Timing(1.0, TimingUnit::NS).into(),
                vec![Literal::HardwareQubit(1).into()]
            )
            .as_qasm_str(),
            "delay [1ns] $1;"
        );
    }

    #[test]
    fn test_extern_argument() {
        assert_eq!(
            ExternArgument::new(Scalar::Bit(None).into(), None).as_qasm_str(),
            "bit"
        );
        assert_eq!(
            ExternArgument::new(
                types::Array::with_reference(
                    types::Reference::Mutable,
                    Scalar::Bit(None),
                    vec![Literal::DecimalInteger(1).into()],
                    None
                )
                .into(),
                None
            )
            .as_qasm_str(),
            "mutable array[bit, 1]"
        );
        assert_eq!(
            ExternArgument::new(
                types::Register::C.into(),
                Some(Literal::DecimalInteger(1).into())
            )
            .as_qasm_str(),
            "creg[1]"
        );
    }

    #[test]
    fn test_extern() {
        assert_eq!(
            Extern::new(Identifier::newt("foo".to_string()), vec![], None).as_qasm_str(),
            "extern foo();"
        );
        assert_eq!(
            Extern::new(
                Identifier::newt("foo".to_string()),
                vec![ExternArgument::newt(Scalar::Bit(None).into(), None)],
                None
            )
            .as_qasm_str(),
            "extern foo(bit);"
        );
        assert_eq!(
            Extern::new(
                Identifier::newt("foo".to_string()),
                vec![ExternArgument::newt(Scalar::Bit(None).into(), None)],
                Some(Scalar::Bit(None).into())
            )
            .as_qasm_str(),
            "extern foo(bit) -> bit;"
        );
    }

    #[test]
    fn test_for() {
        assert_eq!(
            For::new(
                Scalar::Bit(None),
                Identifier::newt("a".to_string()),
                Range::newt(
                    Some(Literal::DecimalInteger(0).into()),
                    Some(Literal::DecimalInteger(1).into()),
                    None
                ),
                Scope::new(vec![])
            )
            .as_qasm_str(),
            "for bit a in [0:1] {}"
        );
    }

    #[test]
    fn test_gate_mod() {
        assert_eq!(GateMod::Inv.as_qasm_str(), "inv @");
        assert_eq!(
            GateMod::Pow(Literal::DecimalInteger(2).into()).as_qasm_str(),
            "pow(2) @"
        );
        assert_eq!(
            GateMod::Ctrl(Some(Literal::DecimalInteger(1).into())).as_qasm_str(),
            "ctrl(1) @"
        );
        assert_eq!(
            GateMod::NegCtrl(Some(Literal::DecimalInteger(1).into())).as_qasm_str(),
            "negctrl(1) @"
        );
        assert_eq!(GateMod::NegCtrl(None).as_qasm_str(), "negctrl @");
    }

    #[test]
    fn test_gate_call() {
        assert_eq!(
            GateCall::new(
                vec![],
                Identifier::newt("gphase".to_string()),
                vec![],
                None,
                vec![Identifier::newt("pi".to_string())]
            )
            .as_qasm_str(),
            "gphase pi;"
        );
        assert_eq!(
            GateCall::new(
                vec![],
                Identifier::newt("foo".to_string()),
                vec![],
                None,
                vec![Identifier::newt("q".to_string())]
            )
            .as_qasm_str(),
            "foo q;"
        );
        assert_eq!(
            GateCall::new(
                vec![GateMod::Inv],
                Identifier::newt("foo".to_string()),
                vec![],
                None,
                vec![Identifier::newt("q".to_string())],
            )
            .as_qasm_str(),
            "inv @ foo q;"
        );
        assert_eq!(
            GateCall::new(
                vec![],
                Identifier::newt("foo".to_string()),
                vec![Literal::DecimalInteger(1).into()],
                None,
                vec![Identifier::newt("q".to_string())],
            )
            .as_qasm_str(),
            "foo(1) q;"
        );
    }

    #[test]
    fn test_gate() {
        assert_eq!(
            Gate::new(
                Identifier::newt("foo".to_string()),
                vec![],
                vec![Identifier::newt("q".to_string())],
                Scope::new(vec![])
            )
            .as_qasm_str(),
            "gate foo q {}"
        );
        assert_eq!(
            Gate::new(
                Identifier::newt("foo".to_string()),
                vec![Identifier::newt("a".to_string())],
                vec![Identifier::newt("q".to_string())],
                Scope::new(vec![])
            )
            .as_qasm_str(),
            "gate foo(a) q {}"
        );
    }

    #[test]
    fn test_if() {
        assert_eq!(
            If::new(
                Identifier::newt("a".to_string()),
                StatementOrScope::Statement(Statement::Break),
                None
            )
            .as_qasm_str(),
            "if (a) break;"
        );
        assert_eq!(
            If::new(
                Identifier::newt("a".to_string()),
                StatementOrScope::Statement(Statement::Break),
                Some(StatementOrScope::Statement(Statement::Continue))
            )
            .as_qasm_str(),
            "if (a) break; else continue;"
        );
    }

    #[test]
    fn test_include() {
        assert_eq!(
            Include::new("stdgates.qasm".to_string()).as_qasm_str(),
            "include \"stdgates.qasm\";"
        );
    }

    #[test]
    fn test_io_declaration() {
        assert_eq!(
            IODeclaration::new(
                IOType::In,
                Scalar::Bit(None).into(),
                Identifier::newt("a".to_string())
            )
            .as_qasm_str(),
            "input bit a;"
        );
        assert_eq!(
            IODeclaration::new(
                IOType::Out,
                Scalar::Bit(None).into(),
                Identifier::newt("a".to_string())
            )
            .as_qasm_str(),
            "output bit a;"
        );
        assert_eq!(
            IODeclaration::new(
                IOType::In,
                types::Array::newt(Scalar::Bit(None), vec![Literal::DecimalInteger(1).into()]),
                Identifier::newt("a".to_string())
            )
            .as_qasm_str(),
            "input array[bit, 1] a;"
        );
    }

    #[test]
    fn test_measure_arrow_assignment() {
        assert_eq!(
            MeasureArrowAssignment::new(Measure::newt(Identifier::newt("a".to_string())), None)
                .as_qasm_str(),
            "measure a;"
        );
        assert_eq!(
            MeasureArrowAssignment::new(
                Measure::newt(Identifier::newt("a".to_string())),
                Some(Identifier::newt("b".to_string()))
            )
            .as_qasm_str(),
            "measure a -> b;"
        );
    }

    #[test]
    fn test_old_style_declaration() {
        assert_eq!(
            OldStyleDeclaration::new(
                types::Register::C.into(),
                Identifier::newt("c".to_string()),
                None
            )
            .as_qasm_str(),
            "creg c;"
        );
        assert_eq!(
            OldStyleDeclaration::new(
                types::Register::C.into(),
                Identifier::newt("c".to_string()),
                Some(Literal::DecimalInteger(1).into())
            )
            .as_qasm_str(),
            "creg c[1];"
        );
    }

    #[test]
    fn test_pragma() {
        assert_eq!(Pragma::new("foo".to_string()).as_qasm_str(), "pragma foo");
    }

    #[test]
    fn test_quantum_declaration() {
        assert_eq!(
            QuantumDeclaration::new(types::Qubit::newt(None), Identifier::newt("q".to_string()))
                .as_qasm_str(),
            "qubit q;"
        );
    }

    #[test]
    fn test_reset() {
        assert_eq!(
            Reset::new(Identifier::newt("q".to_string())).as_qasm_str(),
            "reset q;"
        );
    }

    #[test]
    fn test_return() {
        assert_eq!(Return::new(None).as_qasm_str(), "return;");
        assert_eq!(
            Return::new(Some(Identifier::newt("a".to_string()))).as_qasm_str(),
            "return a;"
        );
    }

    #[test]
    fn test_while() {
        assert_eq!(
            While::new(
                Identifier::newt("a".to_string()),
                StatementOrScope::Statement(Statement::Break)
            )
            .as_qasm_str(),
            "while (a) break;"
        );
    }

    #[test]
    fn test_switch_item() {
        assert_eq!(
            SwitchItem::new(vec![Identifier::newt("a".to_string())], Scope::new(vec![]))
                .as_qasm_str(),
            "case a {}"
        );
    }

    #[test]
    fn test_switch() {
        assert_eq!(
            Switch::new(
                Identifier::newt("a".to_string()),
                vec![SwitchItem::newt(
                    vec![Identifier::newt("b".to_string())],
                    Scope::new(vec![])
                )],
                None
            )
            .as_qasm_str(),
            indoc! {r#"
                switch (a) {
                    case b {}
                }"#}
        );
        assert_eq!(
            Switch::new(
                Identifier::newt("a".to_string()),
                vec![SwitchItem::newt(
                    vec![Identifier::newt("b".to_string())],
                    Scope::new(vec![])
                )],
                Some(Scope::new(vec![]))
            )
            .as_qasm_str(),
            indoc! {r#"
                switch (a) {
                    case b {}
                    default: {}
                }"#}
        );
    }

    #[test]
    fn test_annotation() {
        assert_eq!(
            Annotation::new(Identifier::newt("foo".to_string()), "bar".to_string()).as_qasm_str(),
            "@foo bar"
        );
    }

    #[test]
    fn test_annotated() {
        assert_eq!(
            Annotated::new(
                vec![Annotation::new(
                    Identifier::newt("noswap".to_string()),
                    "".to_string()
                )],
                BoxStatement::newt(None, Scope::new(vec![]))
            )
            .as_qasm_str(),
            "@noswap\nbox {}"
        );
    }

    #[test]
    fn test_scope() {
        assert_eq!(Scope::new(vec![]).as_qasm_str(), "{}");
        assert_eq!(
            Scope::new(vec![StatementOrScope::Statement(Statement::Break)]).as_qasm_str(),
            indoc! {r#"{
                break;
            }"#}
        );
    }
}
