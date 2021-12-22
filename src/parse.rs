enum Token {
    Lambda(String),
    Lbracket,
    Rbracket,
    Variable(String),
}

enum ParseError {
    UnexpectedCharacter((usize, char, char)),
    UnexpectedVariable((usize, String)),
}
