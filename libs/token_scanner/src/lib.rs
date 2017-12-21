use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    ID,
    WHILE,
    IF,
    NUM,
    Mas,
    Menos,
    Multiplicacion,
    Division,
    Asignacion,
    PuntoYComa,
    Negacion,
    Mayor,
    MayorIgual,
    Menor,
    MenorIgual,
    Igualdad,
    Diferencia,
    LineComment,
    LeftBlockComment,
    RightBlockComment,
    LeftParentheses,
    RightParentheses,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    DoubleQuote,
    IntType,
    BoolType,
    CharType,
    Undef,
}

use self::TokenType::*;

fn accum_token(accum: &mut VecDeque<(TokenType, String)>, token_type: &TokenType, lexema: &str) {
    accum.push_back((token_type.clone(), lexema.to_string()));
}

fn get_token_type(c: &str) -> TokenType {
    if c.is_empty() {
        Undef
    } else if c.chars().nth(0).unwrap().is_digit(10) {
        NUM
    } else if c == "while" {
        WHILE
    } else if c == "if" {
        IF
    } else if c == "+" {
        Mas
    } else if c == "-" {
        Menos
    } else if c == "*" {
        Multiplicacion
    } else if c == "/" {
        Division
    } else if c == "=" {
        Asignacion
    } else if c == ";" {
        PuntoYComa
    } else if c == "!" {
        Negacion
    } else if c == "<" {
        Menor
    } else if c == "<=" {
        MenorIgual
    } else if c == ">" {
        Mayor
    } else if c == ">=" {
        MayorIgual
    } else if c == "==" {
        Igualdad
    } else if c == "!=" {
        Diferencia
    } else if c == "(" {
        LeftParentheses
    } else if c == ")" {
        RightParentheses
    } else if c == "[" {
        LeftBracket
    } else if c == "]" {
        RightBracket
    } else if c == "{" {
        LeftBrace
    } else if c == "}" {
        RightBrace
    } else if c == "\"" {
        DoubleQuote
    } else if c == "//" {
        LineComment
    } else if c == "/*" {
        LeftBlockComment
    } else if c == "*/" {
        RightBlockComment
    } else if c == "int" {
        IntType
    } else if c == "bool" {
        BoolType
    } else if c == "char" {
        CharType
    } else {
        ID
    }
}

pub fn token_scanner(buffer: &str) -> VecDeque<(TokenType, String)> {
    let mut lexema = String::new();
    let mut token_type = Undef;
    let mut in_comment = false;
    let mut accum: VecDeque<(TokenType, String)> = VecDeque::new();

    for c in buffer.chars() {
        if !c.is_whitespace() {
            let mut do_flush = false;

            match token_type {
                Undef => {}
                NUM => do_flush = !c.is_digit(10),
                ID => do_flush = !c.is_alphanumeric(),
                Mayor | Menor | Asignacion | Negacion => do_flush = c != '=',
                Division => do_flush = c != '/' && c != '*',
                LineComment | LeftBlockComment => in_comment = true,
                WHILE | IF => if c.is_alphanumeric() {
                    token_type = ID;
                } else {
                    do_flush = true;
                },
                _ => {
                    let mut lexema2 = lexema.clone();
                    lexema2.push(c);
                    let new_token_type = get_token_type(&lexema2);

                    do_flush = new_token_type != token_type;
                }
            }

            if do_flush {
                accum_token(&mut accum, &token_type, &lexema);
                lexema.clear();
            }

            if !in_comment {
                lexema.push(c);
                token_type = get_token_type(&lexema);
            } else if token_type == LeftBlockComment {
                if lexema.is_empty() {
                    if c == '*' {
                        lexema.push(c);
                    }
                } else {
                    lexema.push(c);
                    if get_token_type(&lexema) == RightBlockComment {
                        in_comment = false;
                        accum_token(&mut accum, &token_type, &"/*".to_string());
                        token_type = RightBlockComment;
                    } else {
                        lexema.clear();
                    }
                }
            }
        } else {
            in_comment = token_type == LineComment || token_type == LeftBlockComment;

            if in_comment {
                if token_type == LineComment && c as u8 == 10 {
                    in_comment = false;
                    accum_token(&mut accum, &token_type, &lexema);
                    lexema.clear();
                    token_type = Undef;
                }
            } else {
                if !lexema.is_empty() {
                    accum_token(&mut accum, &token_type, &lexema);
                    lexema.clear();
                }

                token_type = Undef;
            }
        }
    }

    accum_token(&mut accum, &token_type, &lexema);
    lexema.clear();

    accum
}
