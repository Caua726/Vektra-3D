use crate::lexer::{Token};
use crate::grammar::*;

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0 }
    }

    pub fn parse(&mut self) -> Result<Program, String> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            match self.peek() {
                Some(Token::Import) => {
                    statements.push(self.parse_import()?);
                }
                Some(Token::Object) => {
                    statements.push(self.parse_object_definition()?);
                }
                Some(Token::Keyframe) => {
                    statements.push(self.parse_keyframe_definition()?);
                }
                Some(Token::Interpolate) => {
                    statements.push(self.parse_interpolate_definition()?);
                }
                Some(Token::Newline) => {
                    self.advance(); // Skip newlines
                }
                Some(Token::Eof) => break,
                Some(tok) => return Err(format!("Unexpected token at top level: {:?}", tok)),
                None => break,
            }
        }
        Ok(statements)
    }

    fn parse_import(&mut self) -> Result<Statement, String> {
        self.consume(&Token::Import, "Expected 'import'")?;
        
        let path = self.consume_identifier("Expected path after 'import'")?;
        
        // Handle .obj extension
        let path = if self.peek() == Some(&Token::Dot) {
            self.advance(); // consume '.'
            let extension = self.consume_identifier("Expected extension after '.'")?;
            format!("{}.{}", path, extension)
        } else {
            path
        };

        self.consume(&Token::As, "Expected 'as' after import path")?;
        let alias = self.consume_identifier("Expected alias after 'as'")?;
        Ok(Statement::Import(ImportStatement { path, alias }))
    }

    fn parse_object_definition(&mut self) -> Result<Statement, String> {
        self.consume(&Token::Object, "Expected 'object'")?;
        self.consume(&Token::LBrace, "Expected '{' after 'object'")?;
        
        let mut properties = Vec::new();
        let mut obj_name_from_prop = None;
        while self.peek() != Some(&Token::RBrace) && !self.is_at_end() {
            if self.peek() == Some(&Token::Newline) {
                self.advance();
                continue;
            }
            let prop = self.parse_property_assignment()?;
            if prop.name == "obj" {
                 if let PropertyValue::Identifier(name) = &prop.value {
                    obj_name_from_prop = Some(name.clone());
                 }
            }
            properties.push(prop);
            if self.peek() == Some(&Token::Comma) {
                self.advance();
            }
        }
        
        self.consume(&Token::RBrace, "Expected '}' after object definition")?;

        let name = obj_name_from_prop.ok_or("Object definition must have an 'obj' property with an identifier")?;

        Ok(Statement::Object(ObjectDefinition { name, properties }))
    }

    fn parse_keyframe_definition(&mut self) -> Result<Statement, String> {
        self.consume(&Token::Keyframe, "Expected 'keyframe'")?;
        self.consume(&Token::LParen, "Expected '(' after 'keyframe'")?;
        let time = self.consume_number("Expected number for keyframe time")?;
        self.consume(&Token::RParen, "Expected ')' after keyframe time")?;
        self.consume(&Token::LBrace, "Expected '{' after keyframe declaration")?;
        
        while self.peek() == Some(&Token::Newline) {
            self.advance();
        }

        self.consume(&Token::Object, "Expected 'object' inside keyframe")?;
        self.consume(&Token::Colon, "Expected ':' after 'object'")?;
        let object_name = self.consume_identifier("Expected object name for keyframe")?;
        
        self.consume(&Token::LBrace, "Expected '{' for keyframe properties")?;

        let mut properties = Vec::new();
        while self.peek() != Some(&Token::RBrace) && !self.is_at_end() {
            if self.peek() == Some(&Token::Newline) {
                self.advance();
                continue;
            }
            properties.push(self.parse_property_assignment()?);
            if self.peek() == Some(&Token::Comma) {
                self.advance();
            }
        }

        self.consume(&Token::RBrace, "Expected '}' after keyframe properties")?;
        
        while self.peek() == Some(&Token::Newline) {
            self.advance();
        }
        
        self.consume(&Token::RBrace, "Expected '}' after keyframe block")?;
        
        Ok(Statement::Keyframe(KeyframeDefinition { time, object_name, properties }))
    }
    
    fn parse_interpolate_definition(&mut self) -> Result<Statement, String> {
        self.consume(&Token::Interpolate, "Expected 'interpolate'")?;
        self.consume(&Token::LParen, "Expected '(' after 'interpolate'")?;
        let from = self.consume_number("Expected 'from' keyframe time")?;
        self.consume(&Token::Arrow, "Expected '->' between keyframe times")?;
        let to = self.consume_number("Expected 'to' keyframe time")?;
        self.consume(&Token::RParen, "Expected ')' after interpolation range")?;
        self.consume(&Token::LBrace, "Expected '{' after interpolate declaration")?;

        let mut properties = Vec::new();
        while self.peek() != Some(&Token::RBrace) && !self.is_at_end() {
             if self.peek() == Some(&Token::Newline) {
                self.advance();
                continue;
            }
            properties.push(self.parse_property_assignment()?);
            if self.peek() == Some(&Token::Comma) {
                self.advance();
            }
        }

        self.consume(&Token::RBrace, "Expected '}' after interpolate properties")?;
        
        Ok(Statement::Interpolate(InterpolateDefinition { from, to, properties }))
    }


    fn parse_property_assignment(&mut self) -> Result<PropertyAssignment, String> {
        let name = match self.peek() {
            Some(Token::Identifier(_)) => self.consume_identifier("Expected property name")?,
            Some(Token::Type) => {
                self.advance();
                "type".to_string()
            }
            _ => return Err("Expected property name".to_string()),
        };
        self.consume(&Token::Colon, "Expected ':' after property name")?;
        
        let value = match self.peek() {
            Some(Token::LBracket) => {
                let vector = self.parse_vector()?;
                PropertyValue::Vector(vector)
            }
            Some(Token::Identifier(_)) => {
                let identifier = self.consume_identifier("Expected identifier for property value")?;
                PropertyValue::Identifier(identifier)
            }
            Some(Token::Type) => {
                self.advance();
                PropertyValue::Identifier("type".to_string())
            }
            _ => return Err("Unexpected token for property value".to_string()),
        };

        Ok(PropertyAssignment { name, value })
    }

    fn parse_vector(&mut self) -> Result<Vector3, String> {
        self.consume(&Token::LBracket, "Expected '[' to start a vector")?;
        let x = self.consume_number("Expected a number for vector's x component")?;
        self.consume(&Token::Comma, "Expected ',' after vector's x component")?;
        let y = self.consume_number("Expected a number for vector's y component")?;
        self.consume(&Token::Comma, "Expected ',' after vector's y component")?;
        let z = self.consume_number("Expected a number for vector's z component")?;
        self.consume(&Token::RBracket, "Expected ']' to end a vector")?;
        Ok(Vector3 { x, y, z })
    }

    // Helper functions
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.position += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek() == Some(&Token::Eof)
    }

    fn previous(&self) -> Option<&Token> {
        self.tokens.get(self.position - 1)
    }

    fn consume(&mut self, expected: &Token, message: &str) -> Result<(), String> {
        // We cannot compare tokens that have values inside, so we check the variant
        match (self.peek(), expected) {
            (Some(Token::Identifier(_)), Token::Identifier(_)) => {
                 self.advance();
                 Ok(())
            },
            (Some(Token::Number(_)), Token::Number(_)) => {
                self.advance();
                Ok(())
            }
            _ => {
                if self.peek() == Some(expected) {
                    self.advance();
                    Ok(())
                } else {
                    Err(format!("{} - found {:?}", message, self.peek()))
                }
            }
        }
    }

    fn consume_identifier(&mut self, message: &str) -> Result<String, String> {
        if let Some(Token::Identifier(name)) = self.peek().cloned() {
            self.advance();
            Ok(name)
        } else {
            Err(message.to_string())
        }
    }

    fn consume_number(&mut self, message: &str) -> Result<f64, String> {
        if let Some(Token::Number(val)) = self.peek().cloned() {
            self.advance();
            Ok(val)
        } else {
            Err(message.to_string())
        }
    }
}
