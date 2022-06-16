

use crate::{C1Lexer, C1Token, ParseResult};

pub struct C1Parser<'a> {
    lexer: C1Lexer<'a>,
}


impl<'a> C1Parser<'a> {

    pub fn parse(text: &str) -> ParseResult {
        let mut parser = C1Parser::init_parser(text);

        C1Parser::program( &mut parser)
    }

    fn init_parser(text: &'a str) -> C1Parser {
        let parser = C1Parser {
            lexer: C1Lexer::new(text)
        };
        parser
    }

    fn eat(&mut self) {
        self.lexer.eat();
    }

    fn check_and_eat_token(&mut self, token: Option<C1Token>) -> bool {
        if C1Parser::current_matches(self, token) {
            self.lexer.eat();
            true
        }
        else {
            false
        }
    }

    fn current_matches(&self, token: Option<C1Token>) -> bool {
        if self.lexer.current_token() == token {
            true
        }
        else {
            false
        }
    }

    fn next_matches(&self, token: Option<C1Token>) -> bool {
        if self.lexer.peek_token() == token {
            true
        }
        else {
            false
        }
    }

    //Grammar
    fn program(&mut self) -> Result<(), String> {
        while self.lexer.current_token() != None {
            let result = C1Parser::functiondefinition(self);
            match result {
                Ok(()) => continue,
                Err(string) => return Err(string),
            }
        }
        Ok(())
    }

    fn functiondefinition(&mut self) -> Result<(), String> {
        let mut result = C1Parser::typ(self);
        match result {
            Ok(()) => (),
            Err(string) => return Err(string)
        };

        if !C1Parser::check_and_eat_token(self, Some(C1Token::Identifier)) {
            return Err(format!("Expected Identifier at line {:?}, but was {:?}", //
             self.lexer.current_line_number().unwrap(), self.lexer.current_token().unwrap()))
        }

        if !C1Parser::check_and_eat_token(self, Some(C1Token::LeftParenthesis)) {
            return Err(format!("Expected \"(\" at line {:?}, but was {:?}", //
             self.lexer.current_line_number().unwrap(), self.lexer.current_token().unwrap()))
        }

        if !C1Parser::check_and_eat_token(self, Some(C1Token::RightParenthesis)) {
            return Err(format!("Expected \")\" at line {:?}, but was {:?}", //
             self.lexer.current_line_number().unwrap(), self.lexer.current_token().unwrap()))
        }
        
        if !C1Parser::check_and_eat_token(self, Some(C1Token::LeftBrace)) {
            return Err(format!("Expected \"{{\" at line {:?}, but was {:?}", //
            self.lexer.current_line_number().unwrap(), self.lexer.current_token().unwrap()))

        }

        result = C1Parser::statementlist(self);
        match result {
            Ok(()) => (),
            Err(string) => return Err(string),
        }

        if !C1Parser::check_and_eat_token(self, Some(C1Token::RightBrace)) {
            return Err(format!("Expected \"}}\" at line {:?}, but was {:?}", //
             self.lexer.current_line_number().unwrap(), self.lexer.current_token().unwrap()))
        }

        Ok(())
    }
    
    fn funtioncall(&mut self) -> Result<(), String> {
        if !C1Parser::check_and_eat_token(self, Some(C1Token::Identifier)) {
            return Err(format!("Expected Identifier at line {:?}, but was {:?}", //
             self.lexer.current_line_number().unwrap(), self.lexer.current_token().unwrap()))
        }

        if !C1Parser::check_and_eat_token(self, Some(C1Token::LeftParenthesis)) {
            return Err(format!("Expected \"(\" at line {:?}, but was {:?}", //
             self.lexer.current_line_number().unwrap(), self.lexer.current_token().unwrap()))
        }

        if !C1Parser::check_and_eat_token(self, Some(C1Token::RightParenthesis)) {
            return Err(format!("Expected \")\" at line {:?}, but was {:?}", //
             self.lexer.current_line_number().unwrap(), self.lexer.current_token().unwrap()))
        }

        Ok(())
    }

    fn statementlist(&mut self) -> Result<(), String> {
        while   C1Parser::current_matches(self, Some(C1Token::LeftBrace)) || //
                C1Parser::current_matches(self, Some(C1Token::KwIf)) || //
                C1Parser::current_matches(self, Some(C1Token::KwReturn)) || //
                C1Parser::current_matches(self, Some(C1Token::KwPrintf)) || //
                C1Parser::current_matches(self, Some(C1Token::Identifier)) {
        
                    let result = C1Parser::block(self);
                    match result {
                        Ok(()) => (),
                        Err(string) => return Err(string)
                    };
        }

        Ok(())
    }

    fn block(&mut self) -> Result<(), String> {
        if C1Parser::current_matches(self, Some(C1Token::LeftBrace)) {
            C1Parser::eat(self);
            let result = C1Parser::statementlist(self);
            match result {
                Ok(()) => (),
                Err(string) => return Err(string)
            };
            if !C1Parser::check_and_eat_token(self, Some(C1Token::RightBrace)) {
                return Err(format!("Expected \"}}\" at line {:?}, but was {:?}", //
                 self.lexer.current_line_number().unwrap(), self.lexer.current_token().unwrap()))
            }
        }
        else {
            let result = C1Parser::statement(self);
            match result {
                Ok(()) => (),
                Err(string) => return Err(string)
            };
        }
        Ok(())
    }

    fn statement(&mut self) -> Result<(), String> {
        if C1Parser::current_matches(self, Some(C1Token::KwIf)){
            let result = C1Parser::ifstatement(self);
            match result {
                Ok(()) => (),
                Err(string) => return Err(string)
            };
        }
        if C1Parser::current_matches(self, Some(C1Token::KwReturn)){
            let result = C1Parser::returnstatement(self);
            match result {
                Ok(()) => (),
                Err(string) => return Err(string)
            };
            if !C1Parser::check_and_eat_token(self, Some(C1Token::Semicolon)) {
                return Err(format!("Expected \";\" at line {:?}, but was {:?}", //
                 self.lexer.current_line_number().unwrap(), self.lexer.current_token().unwrap()))
            }
        }
        
        if C1Parser::current_matches(self, Some(C1Token::KwPrintf)){
            let result = C1Parser::printf(self);
            match result {
                Ok(()) => (),
                Err(string) => return Err(string)
            };
            if !C1Parser::check_and_eat_token(self, Some(C1Token::Semicolon)) {
                return Err(format!("Expected \";\" at line {:?}, but was {:?}", //
                 self.lexer.current_line_number().unwrap(), self.lexer.current_token().unwrap()))
            }
        }

        if C1Parser::current_matches(&self, Some(C1Token::Identifier)) {
            if C1Parser::next_matches(self, Some(C1Token::LeftParenthesis)) {
                let result = C1Parser::funtioncall(self);
                 match result {
                    Ok(()) => (),
                    Err(string) => return Err(string)
                };
                if !C1Parser::check_and_eat_token(self, Some(C1Token::Semicolon)) {
                    return Err(format!("Expected \";\" at line {:?}, but was {:?}", //
                     self.lexer.current_line_number().unwrap(), self.lexer.current_token().unwrap()))
                }
            }
            if C1Parser::next_matches(self, Some(C1Token::Assign)) {
                let result = C1Parser::statassignment(self);
                 match result {
                    Ok(()) => (),
                    Err(string) => return Err(string)
                };
                if !C1Parser::check_and_eat_token(self, Some(C1Token::Semicolon)) {
                    return Err(format!("Expected \";\" at line {:?}, but was {:?}", //
                     self.lexer.current_line_number().unwrap(), self.lexer.current_token().unwrap()))
                }
            }
        }
        
        Ok(())
    }

    fn ifstatement(&mut self) -> Result<(), String> {
        if !C1Parser::check_and_eat_token(self, Some(C1Token::KwIf)) {
            return Err(format!("Expected \"if\" at line {:?}, but was {:?}", //
             self.lexer.current_line_number().unwrap(), self.lexer.current_token().unwrap()))
        }

        if !C1Parser::check_and_eat_token(self, Some(C1Token::LeftParenthesis)) {
            return Err(format!("Expected \"(\" at line {:?}, but was {:?}", //
             self.lexer.current_line_number().unwrap(), self.lexer.current_token().unwrap()))
        }

        let mut result = C1Parser::assignment(self);
        match result {
            Ok(()) => (),
            Err(string) => return Err(string)
        };

        if !C1Parser::check_and_eat_token(self, Some(C1Token::RightParenthesis)) {
            return Err(format!("Expected \")\" at line {:?}, but was {:?}", //
             self.lexer.current_line_number().unwrap(), self.lexer.current_token().unwrap()))
        }
        
        result = C1Parser::block(self);
        match result {
            Ok(()) => (),
            Err(string) => return Err(string)
        };

        Ok(())
    }

    fn returnstatement(&mut self) -> Result<(), String> {
        if !C1Parser::check_and_eat_token(self, Some(C1Token::KwReturn)) {
            return Err(format!("Expected \"return\" at line {:?}, but was {:?}", //
             self.lexer.current_line_number().unwrap(), self.lexer.current_token().unwrap()))
        }

        if  C1Parser::current_matches(self, Some(C1Token::Identifier)) || // 
            C1Parser::current_matches(self, Some(C1Token::Minus)) || //
            C1Parser::current_matches(self, Some(C1Token::ConstInt)) || //
            C1Parser::current_matches(self, Some(C1Token::ConstFloat)) || //
            C1Parser::current_matches(self, Some(C1Token::ConstBoolean)) || //
            C1Parser::current_matches(self, Some(C1Token::LeftParenthesis)) {
            let result = C1Parser::assignment(self);
            match result {
                Ok(()) => (),
                Err(string) => return Err(string)
            };
        }
        
        Ok(())
    }

    fn printf(&mut self) -> Result<(), String> {
        if !C1Parser::check_and_eat_token(self, Some(C1Token::KwPrintf)) {
            return Err(format!("Expected \"return\" at line {:?}, but was {:?}", //
             self.lexer.current_line_number().unwrap(), self.lexer.current_token().unwrap()))
        }

        if !C1Parser::check_and_eat_token(self, Some(C1Token::LeftParenthesis)) {
            return Err(format!("Expected \"(\" at line {:?}, but was {:?}", //
             self.lexer.current_line_number().unwrap(), self.lexer.current_token().unwrap()))
        }

        let result = C1Parser::assignment(self);
        match result {
            Ok(()) => (),
            Err(string) => return Err(string)
        };

        if !C1Parser::check_and_eat_token(self, Some(C1Token::RightParenthesis)) {
            return Err(format!("Expected \")\" at line {:?}, but was {:?}", //
             self.lexer.current_line_number().unwrap(), self.lexer.current_token().unwrap()))
        }

        Ok(())
    }

    fn typ(&mut self) -> Result<(), String> {
        if C1Parser::current_matches(self, Some(C1Token::KwBoolean)) {
            C1Parser::eat(self);
            return Ok(())
        }

        if C1Parser::current_matches(self, Some(C1Token::KwFloat)) {
            C1Parser::eat(self);
            return Ok(())
        }

        if C1Parser::current_matches(self, Some(C1Token::KwInt)) {
            C1Parser::eat(self);
            return Ok(())
        }

        if C1Parser::current_matches(self, Some(C1Token::KwVoid)) {
            C1Parser::eat(self);
            return Ok(())
        }

        Err(format!("Expected return value definition at line {:?}, but was missing, current token: {:?}", //
            self.lexer.current_line_number().unwrap(), self.lexer.current_token().unwrap()))
    }

    fn statassignment(&mut self) -> Result<(), String> {
        if !C1Parser::check_and_eat_token(self, Some(C1Token::Identifier)) {
            return Err(format!("Expected identifier at line {:?}, but was {:?}", //
                self.lexer.current_line_number(), self.lexer.current_token()))
        }

        if !C1Parser::check_and_eat_token(self, Some(C1Token::Assign)) {
            return Err(format!("Expected \"=\" at line {:?}, but was {:?}", //
                self.lexer.current_line_number().unwrap(), self.lexer.current_token().unwrap()))
        }

        let result = C1Parser::assignment(self);
        match result {
            Ok(()) => (),
            Err(string) => return Err(string)
        };

        Ok(())
    }

    fn assignment(&mut self) -> Result<(), String> {
        if C1Parser::current_matches(self, Some(C1Token::Identifier)) && C1Parser::next_matches(self, Some(C1Token::Assign)) {
            C1Parser::eat(self);
            C1Parser::eat(self);
                let result = C1Parser::assignment(self);
                match result {
                    Ok(()) => (),
                    Err(string) => return Err(string)
                };
        }
        else {
            let result = C1Parser::expr(self);
            match result {
                Ok(()) => (),
                Err(string) => return Err(string)
            };
        }
        Ok(())
    }

    fn expr(&mut self) -> Result<(), String> {
        let mut result = C1Parser::simexpr(self);
        match result {
            Ok(()) => (),
            Err(string) => return Err(string)
        };

        if  C1Parser::current_matches(self, Some(C1Token::Equal)) || //
            C1Parser::current_matches(self, Some(C1Token::NotEqual)) || //
            C1Parser::current_matches(self, Some(C1Token::GreaterEqual)) || //
            C1Parser::current_matches(self, Some(C1Token::LessEqual)) || //
            C1Parser::current_matches(self, Some(C1Token::Greater)) || //
            C1Parser::current_matches(self, Some(C1Token::Less)) {
                C1Parser::eat(self);
                result = C1Parser::simexpr(self);
                match result {
                    Ok(()) => (),
                    Err(string) => return Err(string)
                };
        }

        Ok(())
    }

    fn simexpr(&mut self) -> Result<(), String> {
        if C1Parser::current_matches(self, Some(C1Token::Minus)) {
            C1Parser::eat(self);
        }

        let mut result = C1Parser::term(self);
        match result {
            Ok(()) => (),
            Err(string) => return Err(string)
        };

        while   C1Parser::current_matches(self, Some(C1Token::Plus)) || //
                C1Parser::current_matches(self, Some(C1Token::Minus)) || //
                C1Parser::current_matches(self, Some(C1Token::Or)) {
                    C1Parser::eat(self);
                    result = C1Parser::term(self);
                    match result {
                        Ok(()) => (),
                        Err(string) => return Err(string)
                    };        
            }

        Ok(())
    }

    fn term(&mut self) -> Result<(), String> {
        let mut result = C1Parser::factor(self);
        match result {
            Ok(()) => (),
            Err(string) => return Err(string)
        };

        while   C1Parser::current_matches(self, Some(C1Token::Asterisk)) || //
                C1Parser::current_matches(self, Some(C1Token::Slash)) || //
                C1Parser::current_matches(self, Some(C1Token::And)) {
                    C1Parser::eat(self);
                    result = C1Parser::factor(self);
                    match result {
                        Ok(()) => (),
                        Err(string) => return Err(string)
                    };        
        }

        Ok(())
    }

    fn factor(&mut self) -> Result<(), String> {
        if C1Parser::current_matches(self, Some(C1Token::LeftParenthesis)) {
            C1Parser::eat(self);
            let result = C1Parser::assignment(self);
            match result {
                Ok(()) => (),
                Err(string) => return Err(string)
            };

            if !C1Parser::check_and_eat_token(self, Some(C1Token::RightParenthesis)) {
                return Err(format!("Expected \")\" at line {:?}, but was {:?}", //
                 self.lexer.current_line_number().unwrap(), self.lexer.current_token().unwrap()))
            }  
        }

        if  C1Parser::current_matches(self, Some(C1Token::ConstInt)) || //
            C1Parser::current_matches(self, Some(C1Token::ConstFloat)) || //
            C1Parser::current_matches(self, Some(C1Token::ConstBoolean)) {
                C1Parser::eat(self);
                return Ok(())
        }
        
        
        if C1Parser::current_matches(self, Some(C1Token::Identifier)) {
            if C1Parser::next_matches(self, Some(C1Token::LeftParenthesis)) {
                let result = C1Parser::funtioncall(self);
                match result {
                    Ok(()) => (),
                    Err(string) => return Err(string)
                };
            }
            else{
                C1Parser::eat(self);
            }
        }

        Ok(())
    }
}