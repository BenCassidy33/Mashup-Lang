use crate::parser::variable;

#[derive(PartialEq, Debug)]
pub struct WhileStatement {}

#[derive(PartialEq, Debug)]
pub enum BinaryCondition {
    LessThan,
    GreaterThan,
    Equal,
    And,
}

#[derive(PartialEq, Debug)]
pub enum ConditionType<'a> {
    Static(variable::VariableTypeLiteral),
    Variable(&'a variable::Variable),
}

#[derive(PartialEq, Debug)]
pub struct IfStatement<'a> {
    pub condition: BinaryCondition,
    pub left: ConditionType<'a>,
    pub right: ConditionType<'a>,
}

#[derive(PartialEq, Debug)]
pub enum Statement<'a> {
    If(IfStatement<'a>),
    While(WhileStatement),
}

#[derive(Debug)]
pub enum StatementGenerationError {
    VariableNotDefined,
    InvalidTypeComparison,
}
