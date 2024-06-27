#[derive(PartialEq, Debug)]
pub struct StructField {
    name: String,
    type_id: VariableType,
}

#[derive(PartialEq, Debug)]
pub struct CustomStruct {
    name: String,
    fields: Vec<StructField>,
}

#[derive(PartialEq, Debug)]
pub struct EnumField {
    name: String,
    value: Option<VariableType>,
}

#[derive(PartialEq, Debug)]
pub struct CustomEnum {
    name: String,
    fields: Vec<EnumField>,
}

pub enum CustomVariableType {
    CustomStruct(CustomStruct),
    CustomEnum(CustomEnum),
}

#[derive(PartialEq, Default, Debug)]
pub enum VariableType {
    CustomStruct(CustomStruct),
    CustomEnum(CustomEnum),
    Int(isize),
    Float(f32),
    Usize(usize),
    String(String),
    #[default]
    Unit,
    Unassigned,
    Vector(Vector),
}

/**
MutableDynamic: Both vector size and the elments within can be changed
MutableStatic: Vector Size cannot be changed but elements within can be changed
ImmutableDynamic: Vector size can be changed but elements within cannot
ImmutableStatic: Both vector size and the elements within cannot be changed
*/
#[derive(PartialEq, Debug)]
pub enum VectorMutibility {
    MutableDynamic,
    MutableStatic, // size
    ImmutableStatic,
    ImmutableDynamic, // elements can be pushed and removed but not changed
}

#[derive(PartialEq, Debug)]
pub enum InitalVectorSize {
    Set(usize),
    Auto,
}

#[derive(PartialEq, Debug)]
pub struct Vector {
    pub type_id: Box<VariableType>,
    pub mutability: VectorMutibility,
    pub inital_size: InitalVectorSize,
}

#[derive(Default, Debug, PartialEq)]
pub struct Variable {
    pub name: String,
    pub type_id: VariableType,
    pub mutable: bool,
}

pub enum VariableAssignmentError {
    BadType,
    AssignmentFailure,
}

pub enum VariableValueRequestError {
    UninitalizedVariable,
    NoInnerValue,
    //InvalidMemoryAddress,
}

#[derive(Debug)]
pub enum VariableGenerationError {
    NoLetStatement,
    InvalidType,
    InvalidAssignment,
    MismatchedTypeAndAssignment,
    InvalidSyntax,
}

pub enum TokenTypeGenerationType {
    SingleType,
    Tuple,
    Vector,
}

impl Variable {
    pub fn get_inner_value(&self) -> Result<&VariableType, VariableValueRequestError> {
        match &self.type_id {
            VariableType::Unassigned => {
                return Err(VariableValueRequestError::UninitalizedVariable)
            }
            VariableType::Unit => {
                return Err(VariableValueRequestError::NoInnerValue);
            }

            v => return Ok(&v),
        }
    }
}
