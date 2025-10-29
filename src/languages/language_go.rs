// Go language support - based on tree-sitter-go 0.25.0
// Minimal enum for RCA metrics support

use num_derive::FromPrimitive;

#[derive(Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum Go {
    End = 0,

    // Comments
    LineComment = 1,
    BlockComment = 2,

    // Basic structure
    SourceFile = 10,

    // Packages and imports
    PackageClause = 20,
    ImportDeclaration = 21,
    ImportSpec = 22,

    // Functions and methods
    FunctionDeclaration = 30,
    MethodDeclaration = 31,
    ParameterList = 32,
    Parameter = 33,

    // Types and interfaces
    InterfaceType = 40,
    InterfaceElement = 41,
    StructType = 42,
    FieldDeclarationList = 43,
    FieldDeclaration = 44,

    // Statements
    Block = 50,
    IfStatement = 51,
    ForStatement = 52,
    SwitchStatement = 53,
    SelectStatement = 54,
    DeferStatement = 55,
    GoStatement = 56,

    // Expressions
    Identifier = 60,
    QualifiedName = 61,
    FunctionCall = 62,
    BinaryExpression = 63,
    UnaryExpression = 64,

    // Variable and constant declarations
    VarDeclaration = 70,
    ConstDeclaration = 71,
    VarSpec = 72,
    ConstSpec = 73,

    // Type declarations
    TypeDeclaration = 80,
    TypeSpec = 81,

    // Literals
    IntLiteral = 90,
    FloatLiteral = 91,
    RuneLiteral = 92,
    StringLiteral = 93,
    RawStringLiteral = 94,
    BoolLiteral = 95,

    // Arrays and slices
    ArrayType = 100,
    SliceType = 101,
    MapType = 102,
    PointerType = 103,

    // Error handling
    ErrorType = 110,

    // Assignment
    Assignment = 120,
    ShortVarDeclaration = 121,
}

impl From<u16> for Go {
    fn from(value: u16) -> Self {
        num::FromPrimitive::from_u16(value).unwrap_or(Go::End)
    }
}

impl PartialEq<u16> for Go {
    #[inline(always)]
    fn eq(&self, x: &u16) -> bool {
        *self == Into::<Self>::into(*x)
    }
}

impl PartialEq<Go> for u16 {
    #[inline(always)]
    fn eq(&self, x: &Go) -> bool {
        *x == *self
    }
}
