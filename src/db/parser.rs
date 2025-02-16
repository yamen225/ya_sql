use nom::{bytes::complete::tag, character::complete::{alphanumeric1, multispace0}, combinator::map, Err, IResult};

use super::query::Identifier;


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Literal(String),
    Operator(String),
    Whitespace,
    Comma,
    LeftParenthesis,
    RightParenthesis,
    Eof,
}


pub enum ASTNode{
    SelectStatement{
        projection: Vec<Identifier>,
        table: Identifier,
    },
    Identifier(String)
}

pub struct Parser{
    tokens: Vec<Token>,
    current: usize
}

impl Parser{
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0
        }
    }

    pub fn parse_v2(&mut self) -> ASTNode {
        self.parse_select_statement()
    }

    pub fn parse_select_statement(&mut self) -> ASTNode {
        assert_eq!(self.next_token(), Token::Keyword("SELECT".to_string()));
        let mut projection = Vec::new();
        loop {
            match self.next_token() {
                Token::Identifier(ident) => projection.push(Identifier(ident)),
                Token::Comma => continue,
                _ => break,
            }
        }

        assert_eq!(self.next_token(), Token::Keyword("FROM".to_string()));
        let table = match self.next_token() {
            Token::Identifier(ident) => Identifier(ident),
            _ => panic!("Expected identifier for table name.")
        };

        ASTNode::SelectStatement { 
            projection, 
            table
        }
    }

    fn next_token(&mut self) -> Token{
        let token = self.tokens.get(self.current).cloned().unwrap_or(Token::Eof);
        self.current += 1;
        token
    }
    
    fn identifier(input: &str) -> IResult<&str, Identifier> {
        map(alphanumeric1, |s: &str| Identifier(s.to_string()))(input)
    }

    fn simple_select_statemnt(input: &str) -> IResult<&str, ASTNode> {
        let (input, _) = tag("SELECT")(input)?; //SELECT
        let (input, _) = multispace0(input)?; // Whitespace
        let (input, _) = tag("*")(input)?; // *
        let (input, _) = multispace0(input)?; // Whitespace
        let (input, _) = tag("FROM")(input)?; // FROM
        let (input, _) = multispace0(input)?; // Whitespace
        let (input, table) = Parser::identifier(input)?; // Table name

        Ok((
            input,
            ASTNode::SelectStatement { projection: vec![Identifier('*'.to_string())], table: table }
        ))
    }

    pub fn parse(input: &str) -> Result<ASTNode, String> {
        match Parser::simple_select_statemnt(input) {
            Ok((remaining, ast)) => {
                if remaining.is_empty() {
                    Ok(ast)
                } else {
                    Err(format!("Unexpected input: {}", remaining))
                }
            }
            Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
                Err(format!("Parse Error: {:?}", e))
            }
            Err(nom::Err::Incomplete(_)) => Err("Incomplete input".to_string()),
        }
    }
}