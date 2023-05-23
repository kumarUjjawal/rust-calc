// Construct AST, where each node represents a token. The AST is a recursive tree structure of
// token nodes
// Methods:
// new(): To create a new instance of the parser. This will create a tokenizer instance passing in
// the arithmetic expression and then stores the first token in its current_token field.
// parse(): To generate the AST from the tokens, which is the main output of the parser
//


Result<Self, ParseError> {
    let mut lexer = Tokenizer::new(expr);
    let cur_token = match lexer.next() {
        Some(token) => token,
        None => return Err(ParseError::InvalidOperator("Invalid character".into())),
    }; 
    Ok(Parser {
        tokenizer: lexer,
        current_token: cur_token,
    })
}

pub fn parse(&mut self) -> Result<Node, ParseError> {
    let ast = self.generate_ast(OperPrec::DefaultZero);
    match ast {
        Ok(ast) => Ok(ast),
        Err(e) => Err(e),
    }
}

fn get_next_token(&mut self) -> Result<(), ParseError> {
    let next_token = match self.tokenizer.next() {
        Some(token) => token,
        None => return Err(ParseError::InvalidOperator("Invalid character".into())),
    }; 
    self.current_token = next_token;
    Ok(())
}

// check if matching pair of parentheses are in the expression

fn check_paren(&mut self, expected: Token) -> Result<(), 
   ParseError> {
       if expected == self.current_token {
           self.get_next_token()?;
           Ok(())
       } else {
           Err(ParseError::InvalidOperator(format!(
                       "Expected {:?}, got {:?}",
                       expected, self.current_token
                       )))
       }
   }

// checks for: 
// whether the token is a number of the form Num(i)
// whether the token has a sign
// pairs of parentheses

fn parse_number(&mut self) -> Result<Node, ParseError> {
    let token = self.current_token.clone();
    match token {
        Token::Subtract => {
            self.get_next_token()?;
            let expr = self.generate_ast(OperPrec::Negative)?;
            Ok(Node::Negative(Box::new(expr)))
        }
        Token::Num(i) => {
            self.get_next_token()?;
            Ok(Node::Number(i))
        }
        Token::LeftParen => {
            self.get_next_token()?;
            let expr = self.generate_ast(OperPrec::DefaultZero)?;
            self.check_paren(Token::RightParen)?;
            if self.current_token == Token::LeftParen {
                let right = self.generate_ast(OperPrec::MulDiv)?;

                return Ok(Node::Multiply(Box::new(expr),
                    Box::new(right)));
            }
            Ok(expr)
        }
        _ => Err(ParseError::UnableToParse("Unable to parse".to_string())),
    }
}
