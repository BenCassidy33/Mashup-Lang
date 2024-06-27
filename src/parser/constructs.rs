use crate::parser::variable::*;

pub enum BinaryCondition {
    LessThan,
    GreaterThan,
    Equal,
    And,
}

pub struct IfStatement {
    condition: BinaryCondition,
    left: Variable,
    right: Variable,
}
