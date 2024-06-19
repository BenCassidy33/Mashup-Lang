#[derive(Debug)]
#[allow(non_snake_case, dead_code)]
pub enum Type {
    IntegerLiteral,
    UnsignedIntegerLiteral,
    FloatLiteral,
    StringLiteral,
    StructLiteral,
    EnumLiteral,
}

#[derive(Debug)]
#[allow(non_snake_case, dead_code)]
pub struct Struct {
    Fields: Vec<Field>,
}

#[derive(Debug)]
#[allow(non_snake_case, dead_code)]
pub struct Enum {
    Types: Vec<Type>,
}

#[derive(Debug)]
#[allow(non_snake_case, dead_code)]
pub struct Field {
    Identifier: String,
    Type: Type,
}

#[derive(Debug)]
#[allow(non_snake_case, dead_code)]
pub enum Visiablity {
    #[allow(non_snake_case, dead_code)]
    Public,
    Private,
}

// ----

#[derive(Debug)]
#[allow(non_snake_case, dead_code)]
pub struct Program {
    #[allow(non_snake_case, dead_code)]
    pub Body: Vec<Statement>,
}

#[derive(Debug)]
#[allow(non_snake_case, dead_code)]
pub enum Statement {
    BlockStatement(BlockStatement),
    IfStatement(IfStatement),
    FunctionDecleration(FunctionDecleration),
}

#[derive(Debug)]
#[allow(non_snake_case, dead_code)]
pub struct BlockStatement {}
#[derive(Debug)]
#[allow(non_snake_case, dead_code)]
pub struct IfStatement {}

#[derive(Debug)]
#[allow(non_snake_case, dead_code)]
pub struct FunctionDecleration {
    pub Visiablity: Visiablity,
    pub Identifier: String,
    pub Arguments: Vec<Field>,
    pub BlockStatement: BlockStatement,
}

#[derive(Debug)]
#[allow(non_snake_case, dead_code)]
pub struct StructDecleration {
    pub Identifier: String,
    pub Body: Struct,
}

pub fn test_tree() {
    let _program = Program {
        Body: vec![Statement::FunctionDecleration(FunctionDecleration {
            Visiablity: Visiablity::Public,
            Identifier: "Foo".to_string(),
            Arguments: vec![Field {
                Identifier: "Bar".to_string(),
                Type: Type::StringLiteral,
            }],
            BlockStatement: BlockStatement {},
        })],
    };
}
