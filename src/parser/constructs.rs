use crate::parser::variable;

#[derive(PartialEq, Debug, Default)]
pub struct WhileStatement {}

#[derive(PartialEq, Debug, Default)]
pub enum BinaryConditionType {
    LessThan,
    GreaterThan,
    Equal,
    And,
    #[default]
    None,
}

#[derive(PartialEq, Debug)]
pub enum ConditionType<'a> {
    Static(variable::VariableTypeLiteral),
    Variable(&'a variable::Variable),
}

impl Default for ConditionType<'_> {
    fn default() -> Self {
        return ConditionType::Static(variable::VariableTypeLiteral::Unit);
    }
}

#[derive(Debug)]
pub enum StatementGenerationError {
    VariableNotDefined,
    InvalidTypeComparison,
}

#[derive(PartialEq, Debug, Default)]
pub struct StatementCondition<'a> {
    left_type_id: ConditionType<'a>,
    right_type_id: ConditionType<'a>,
    binary_condition_type: BinaryConditionType,
    statement_type: StatementType,
}

#[derive(PartialEq, Debug, Default)]
pub enum StatementType {
    If,
    While,
    For,
    #[default]
    None,
}
