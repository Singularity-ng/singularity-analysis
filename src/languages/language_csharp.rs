// C# language support - based on tree-sitter-c-sharp 0.23.1
// Minimal enum for RCA metrics support

use num_derive::FromPrimitive;

#[derive(Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum Csharp {
    End = 0,

    // Comments
    SingleLineComment = 1,
    MultiLineComment = 2,

    // Basic structure
    CompilationUnit = 10,

    // Using statements and namespaces
    UsingStatement = 20,
    NamespaceDeclaration = 21,

    // Classes and structs
    ClassDeclaration = 30,
    StructDeclaration = 31,
    InterfaceDeclaration = 32,
    EnumDeclaration = 33,

    // Methods and properties
    MethodDeclaration = 40,
    PropertyDeclaration = 41,
    ConstructorDeclaration = 42,
    IndexerDeclaration = 43,

    // Fields
    FieldDeclaration = 50,
    EventDeclaration = 51,

    // Statements
    Block = 60,
    IfStatement = 61,
    SwitchStatement = 62,
    ForStatement = 63,
    ForeachStatement = 64,
    WhileStatement = 65,
    DoStatement = 66,
    TryStatement = 67,

    // Expressions
    Identifier = 70,
    QualifiedName = 71,
    InvocationExpression = 72,
    MemberAccessExpression = 73,
    BinaryExpression = 74,
    UnaryExpression = 75,

    // Variable declarations
    LocalVariableDeclaration = 80,
    VariableDeclarator = 81,

    // Type declarations and annotations
    TypeDeclaration = 90,
    GenericName = 91,
    ArrayType = 92,
    PointerType = 93,
    NullableType = 94,

    // Literals
    StringLiteral = 100,
    InterpolatedStringExpression = 101,
    NumericLiteral = 102,
    BooleanLiteral = 103,
    CharacterLiteral = 104,

    // Attributes
    AttributeList = 110,
    Attribute = 111,

    // Modifiers (access, async, etc.)
    Modifier = 120,

    // Lambda and anonymous functions
    LambdaExpression = 130,
    AnonymousMethodExpression = 131,

    // LINQ
    QueryExpression = 140,

    // Exception handling
    CatchClause = 150,
    FinallyClause = 151,

    // Assignment
    Assignment = 160,
}

impl From<u16> for Csharp {
    fn from(value: u16) -> Self {
        num::FromPrimitive::from_u16(value).unwrap_or(Csharp::End)
    }
}

impl PartialEq<u16> for Csharp {
    #[inline(always)]
    fn eq(&self, x: &u16) -> bool {
        *self == Into::<Self>::into(*x)
    }
}

impl PartialEq<Csharp> for u16 {
    #[inline(always)]
    fn eq(&self, x: &Csharp) -> bool {
        *x == *self
    }
}
