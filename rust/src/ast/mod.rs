

#[derive(Debug)]
pub enum Node {
    Program {
        statements: Vec<Statement>,
    }
}

#[derive(Debug)]
pub enum Statement {
    LetStatement,
    ConstStatement,
    ReturnStatement,
}

#[derive(Debug)]
pub enum Expression {
    Identifier {
        name: String,
    },
    BooleanExpression,
    IntegerLiteralExpression(i32),
}