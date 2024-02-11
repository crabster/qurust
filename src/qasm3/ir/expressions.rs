use crate::qasm3::ir::statements::Scope;
use crate::qasm3::ir::types::Type;
use crate::qasm3::ir::AsQasmStr;

#[derive(Debug)]
pub struct Parenthesis {
    expr: Box<Expression>,
}

impl Parenthesis {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<Parenthesis>>(expr: Expression) -> T {
        Self {
            expr: Box::new(expr),
        }
        .into()
    }
}

impl AsQasmStr for Parenthesis {
    fn as_qasm_str(&self) -> String {
        format!("({})", self.expr.as_qasm_str())
    }
}

#[derive(Debug)]
pub struct Index {
    expr: Box<Expression>,
    index: Box<Expression>,
}

impl Index {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<Index>>(expr: Expression, index: Expression) -> T {
        Self {
            expr: Box::new(expr),
            index: Box::new(index),
        }
        .into()
    }
}

impl AsQasmStr for Index {
    fn as_qasm_str(&self) -> String {
        format!("{}[{}]", self.expr.as_qasm_str(), self.index.as_qasm_str())
    }
}

#[derive(Debug)]
pub enum UnaryOperator {
    BitNeg,
    Not,
    Minus,
}

impl AsQasmStr for UnaryOperator {
    fn as_qasm_str(&self) -> String {
        match self {
            UnaryOperator::BitNeg => "~",
            UnaryOperator::Not => "!",
            UnaryOperator::Minus => "-",
        }
        .to_string()
    }
}

#[derive(Debug)]
pub struct UnaryOperation {
    operator: UnaryOperator,
    expr: Box<Expression>,
}

impl UnaryOperation {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<UnaryOperation>>(operator: UnaryOperator, expr: Expression) -> T {
        Self {
            operator,
            expr: Box::new(expr),
        }
        .into()
    }
}

impl AsQasmStr for UnaryOperation {
    fn as_qasm_str(&self) -> String {
        format!("{}{}", self.operator.as_qasm_str(), self.expr.as_qasm_str())
    }
}

#[derive(Debug)]
pub enum BinaryOperator {
    Pow,
    Mul,
    Div,
    Mod,
    Add,
    Sub,
    LShift,
    RShift,
    Lt,
    Leq,
    Gt,
    Geq,
    Eq,
    Neq,
    BitAnd,
    BitXor,
    BitOr,
    And,
    Or,
}

impl AsQasmStr for BinaryOperator {
    fn as_qasm_str(&self) -> String {
        match self {
            BinaryOperator::Pow => "^",
            BinaryOperator::Mul => "*",
            BinaryOperator::Div => "/",
            BinaryOperator::Mod => "%",
            BinaryOperator::Add => "+",
            BinaryOperator::Sub => "-",
            BinaryOperator::LShift => "<<",
            BinaryOperator::RShift => ">>",
            BinaryOperator::Lt => "<",
            BinaryOperator::Leq => "<=",
            BinaryOperator::Gt => ">",
            BinaryOperator::Geq => ">=",
            BinaryOperator::Eq => "==",
            BinaryOperator::Neq => "!=",
            BinaryOperator::BitAnd => "&",
            BinaryOperator::BitXor => "^",
            BinaryOperator::BitOr => "|",
            BinaryOperator::And => "&&",
            BinaryOperator::Or => "||",
        }
        .to_string()
    }
}

#[derive(Debug)]
pub struct BinaryOperation {
    operator: BinaryOperator,
    lhs: Box<Expression>,
    rhs: Box<Expression>,
}

impl BinaryOperation {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<BinaryOperation>>(
        operator: BinaryOperator,
        lhs: Expression,
        rhs: Expression,
    ) -> T {
        Self {
            operator,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
        .into()
    }
}

impl AsQasmStr for BinaryOperation {
    fn as_qasm_str(&self) -> String {
        format!(
            "{} {} {}",
            self.lhs.as_qasm_str(),
            self.operator.as_qasm_str(),
            self.rhs.as_qasm_str()
        )
    }
}

#[derive(Debug)]
pub struct Cast {
    type_: Type,
    expr: Box<Expression>,
}

impl Cast {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<Cast>>(type_: Type, expr: Expression) -> T {
        Self {
            type_,
            expr: Box::new(expr),
        }
        .into()
    }
}

impl AsQasmStr for Cast {
    fn as_qasm_str(&self) -> String {
        format!("({}){}", self.type_.as_qasm_str(), self.expr.as_qasm_str())
    }
}

#[derive(Debug)]
pub struct DurationOf {
    scope: Box<Scope>,
}

impl DurationOf {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<DurationOf>>(scope: Scope) -> T {
        Self {
            scope: Box::new(scope),
        }
        .into()
    }
}

impl AsQasmStr for DurationOf {
    fn as_qasm_str(&self) -> String {
        format!("durationof({})", self.scope.as_qasm_str())
    }
}

#[derive(Debug)]
pub struct Call {
    identifier: Identifier,
    args: Vec<Expression>,
}

impl Call {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<Call>>(identifier: Identifier, args: Vec<Expression>) -> T {
        Self { identifier, args }.into()
    }
}

impl AsQasmStr for Call {
    fn as_qasm_str(&self) -> String {
        format!(
            "{}({})",
            self.identifier.as_qasm_str(),
            self.args
                .iter()
                .map(|a| a.as_qasm_str())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[derive(Debug)]
pub struct Identifier {
    name: String,
}

impl Identifier {
    pub fn new<T: From<Identifier>>(name: String) -> T {
        Self { name }.into()
    }
}

impl AsQasmStr for Identifier {
    fn as_qasm_str(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug)]
pub enum TimingUnit {
    DT,
    NS,
    US,
    MS,
    S,
}

impl AsQasmStr for TimingUnit {
    fn as_qasm_str(&self) -> String {
        match self {
            TimingUnit::DT => "dt".to_string(),
            TimingUnit::NS => "ns".to_string(),
            TimingUnit::US => "us".to_string(),
            TimingUnit::MS => "ms".to_string(),
            TimingUnit::S => "s".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum Literal {
    Identifier(Identifier),
    BinaryInteger(i64),
    OctalInteger(i64),
    DecimalInteger(i64),
    HexInteger(i64),
    Float(f64),
    Imaginary(f64),
    Boolean(bool),
    BitString(String),
    Timing(f64, TimingUnit),
    HardwareQubit(u32),
}

impl AsQasmStr for Literal {
    fn as_qasm_str(&self) -> String {
        match self {
            Literal::Identifier(id) => id.as_qasm_str(),
            Literal::BinaryInteger(i) => format!("{:b}", i),
            Literal::OctalInteger(i) => format!("{:o}", i),
            Literal::DecimalInteger(i) => i.to_string(),
            Literal::HexInteger(i) => format!("{:x}", i),
            Literal::Float(f) => f.to_string(),
            Literal::Imaginary(f) => format!("{}im", f),
            Literal::Boolean(b) => b.to_string(),
            Literal::BitString(s) => s.clone(),
            Literal::Timing(f, u) => format!("{}{}", f, u.as_qasm_str()),
            Literal::HardwareQubit(u) => format!("${}", u),
        }
    }
}

impl From<Identifier> for Literal {
    fn from(id: Identifier) -> Self {
        Literal::Identifier(id)
    }
}

#[derive(Debug)]
pub struct Alias {
    aliases: Vec<Expression>,
}

impl Alias {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<Alias>>(aliases: Vec<Expression>) -> T {
        Self { aliases }.into()
    }
}

impl AsQasmStr for Alias {
    fn as_qasm_str(&self) -> String {
        self.aliases
            .iter()
            .map(|a| a.as_qasm_str())
            .collect::<Vec<String>>()
            .join(" ++ ")
    }
}

#[derive(Debug)]
pub struct Measure {
    expr: Box<Expression>,
}

impl Measure {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<Measure>>(expr: Expression) -> T {
        Self {
            expr: Box::new(expr),
        }
        .into()
    }
}

impl AsQasmStr for Measure {
    fn as_qasm_str(&self) -> String {
        format!("measure {}", self.expr.as_qasm_str())
    }
}

#[derive(Debug)]
pub struct Range {
    start: Box<Option<Expression>>,
    end: Box<Option<Expression>>,
    step: Box<Option<Expression>>,
}

impl Range {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<Range>>(
        start: Option<Expression>,
        end: Option<Expression>,
        step: Option<Expression>,
    ) -> T {
        Self {
            start: Box::new(start),
            end: Box::new(end),
            step: Box::new(step),
        }
        .into()
    }
}

impl AsQasmStr for Range {
    fn as_qasm_str(&self) -> String {
        format!(
            "{}:{}{}",
            match &*self.start {
                Some(expr) => expr.as_qasm_str(),
                None => "".to_string(),
            },
            match &*self.end {
                Some(expr) => expr.as_qasm_str(),
                None => "".to_string(),
            },
            match &*self.step {
                Some(expr) => expr.as_qasm_str(),
                None => "".to_string(),
            }
        )
    }
}

#[derive(Debug)]
pub struct Array {
    exprs: Vec<Expression>,
}

impl Array {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T: From<Array>>(exprs: Vec<Expression>) -> T {
        Self { exprs }.into()
    }
}

impl AsQasmStr for Array {
    fn as_qasm_str(&self) -> String {
        format!(
            "{{{}}}",
            self.exprs
                .iter()
                .map(|e| e.as_qasm_str())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[derive(Debug)]
pub enum Expression {
    Parenthesis(Parenthesis),
    Index(Index),
    UnaryOp(UnaryOperation),
    BinaryOp(BinaryOperation),
    Cast(Cast),
    DurationOf(DurationOf),
    Call(Call),
    Literal(Literal),
    Alias(Alias),
    Measure(Measure),
    Range(Range),
    Array(Array),
}

impl AsQasmStr for Expression {
    fn as_qasm_str(&self) -> String {
        match self {
            Expression::Parenthesis(par) => par.as_qasm_str(),
            Expression::Index(idx) => idx.as_qasm_str(),
            Expression::UnaryOp(un_op) => un_op.as_qasm_str(),
            Expression::BinaryOp(bin_op) => bin_op.as_qasm_str(),
            Expression::Cast(cast) => cast.as_qasm_str(),
            Expression::DurationOf(dur) => dur.as_qasm_str(),
            Expression::Call(call) => call.as_qasm_str(),
            Expression::Literal(lit) => lit.as_qasm_str(),
            Expression::Alias(alias) => alias.as_qasm_str(),
            Expression::Measure(measure) => measure.as_qasm_str(),
            Expression::Range(range) => range.as_qasm_str(),
            Expression::Array(array) => array.as_qasm_str(),
        }
    }
}

impl From<Parenthesis> for Expression {
    fn from(par: Parenthesis) -> Self {
        Expression::Parenthesis(par)
    }
}

impl From<Index> for Expression {
    fn from(idx: Index) -> Self {
        Expression::Index(idx)
    }
}

impl From<UnaryOperation> for Expression {
    fn from(un_op: UnaryOperation) -> Self {
        Expression::UnaryOp(un_op)
    }
}

impl From<BinaryOperation> for Expression {
    fn from(bin_op: BinaryOperation) -> Self {
        Expression::BinaryOp(bin_op)
    }
}

impl From<Cast> for Expression {
    fn from(cast: Cast) -> Self {
        Expression::Cast(cast)
    }
}

impl From<DurationOf> for Expression {
    fn from(dur: DurationOf) -> Self {
        Expression::DurationOf(dur)
    }
}

impl From<Call> for Expression {
    fn from(call: Call) -> Self {
        Expression::Call(call)
    }
}

impl From<Literal> for Expression {
    fn from(lit: Literal) -> Self {
        Expression::Literal(lit)
    }
}

impl From<Alias> for Expression {
    fn from(alias: Alias) -> Self {
        Expression::Alias(alias)
    }
}

impl From<Measure> for Expression {
    fn from(measure: Measure) -> Self {
        Expression::Measure(measure)
    }
}

impl From<Range> for Expression {
    fn from(range: Range) -> Self {
        Expression::Range(range)
    }
}

impl From<Array> for Expression {
    fn from(array: Array) -> Self {
        Expression::Array(array)
    }
}
