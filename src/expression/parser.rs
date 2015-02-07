// Copyright (C) 2015  Jonas Pollok <jonas.p@gmail.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use expression::Expression;

pub struct Parser<T> {
    iter: T,
    ch: Option<char>
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ErrorCode {
    InvalidPattern,
    UnbalancedParens,
    EmptyInput,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ParserError {
    // TODO: add line/col
    /// msg
    SyntaxError(ErrorCode),
    /// should not happen, if you see this there is some bug
    InternalError
}

impl<T: Iterator<Item=char>> Parser<T> {
    pub fn new(it: T) -> Parser<T> {
        let mut p = Parser {
            iter: it,
            ch: None,
        };

        // go to the first char
        p.bump();

        p
    }

    // root ::= expression
    pub fn parse(&mut self) -> Result<Expression, ParserError> {
        self.skip_whitespace();
        self.parse_expression()
    }

    // expression ::= '(' expression* ')'
    //            ::| blank
    //            ::| blank_seq
    //            ::| blank_null_seq
    //            ::| pattern
    //            ::| pattern_seq
    //            ::| pattern_null_seq
    //            ::| atom
    //
    // filter first if list or not
    // EOF is invalid as it should not be called in that case
    fn parse_expression(&mut self) -> Result<Expression, ParserError> {
        match self.ch {
            Some('(') => self.parse_list(),
            Some(')') => Err(ParserError::SyntaxError(ErrorCode::UnbalancedParens)),
            // EOF
            None => Err(ParserError::SyntaxError(ErrorCode::EmptyInput)),
            _ => self.parse_atomic(),
        }
    }

    // parse expressions until list is properly treminated by ')'
    fn parse_list(&mut self) -> Result<Expression, ParserError> {
        // consume '('
        self.bump();

        let mut v: Vec<Expression> = Vec::new();
        loop {
            self.skip_whitespace();
            match self.ch {
                Some(')') => {
                    // consume ')'
                    self.bump();

                    return Ok(Expression::List(v.into_boxed_slice()))
                }
                // EOF
                None => { return Err(ParserError::SyntaxError(ErrorCode::UnbalancedParens)); }
                _ => {
                    let exp = try!{ self.parse_expression() };
                    v.push(exp);
                }
            }
        }
    }

    // parse until terminated by '(', ')', whitespace or EOF
    // A '_' termination indicates a pattern/blank type
    fn parse_atomic(&mut self) -> Result<Expression, ParserError> {
        let mut s: String = String::new();

        loop {
            if self.ch_is_terminator() {
                break
            }

            if self.ch == Some('_') {
                // consume '_'
                self.bump();

                if s.len() == 0 {
                    return self.parse_blank();
                } else {
                    return self.parse_pattern(s);
                }
            }

            // safe as we checked for None in self.ch_is_terminator
            s.push(self.ch.unwrap());

            self.bump();
        }

        if s.len() == 0 {
            Err(ParserError::InternalError)
        } else {
            Ok(Expression::Atom(s))
        }
    }

    fn parse_blank(&mut self) -> Result<Expression, ParserError> {
        if self.ch == Some('_') {
            // consume '_'
            self.bump();
            return self.parse_blank_seq();
        }

        if !self.ch_is_terminator() {
            // invalid termination
            return Err(ParserError::SyntaxError(ErrorCode::InvalidPattern));
        }

        Ok(Expression::Blank)
    }

    fn parse_blank_seq(&mut self) -> Result<Expression, ParserError> {
        if self.ch == Some('_') {
            // consume '_'
            self.bump();
            return self.parse_blank_null_seq();
        }

        if !self.ch_is_terminator() {
            // invalid termination
            return Err(ParserError::SyntaxError(ErrorCode::InvalidPattern));
        }

        Ok(Expression::BlankSeq)
    }

    fn parse_blank_null_seq(&mut self) -> Result<Expression, ParserError> {
        if !self.ch_is_terminator() {
            // invalid termination
            return Err(ParserError::SyntaxError(ErrorCode::InvalidPattern));
        }

        Ok(Expression::BlankNullSeq)
    }

    fn parse_pattern(&mut self, s: String) -> Result<Expression, ParserError> {
        if self.ch == Some('_') {
            // consume '_'
            self.bump();
            return self.parse_pattern_seq(s);
        }

        if !self.ch_is_terminator() {
            // invalid termination
            return Err(ParserError::SyntaxError(ErrorCode::InvalidPattern));
        }

        Ok(Expression::Pattern(s))
    }

    fn parse_pattern_seq(&mut self, s: String) -> Result<Expression, ParserError> {
        if self.ch == Some('_') {
            // consume '_'
            self.bump();
            return self.parse_pattern_null_seq(s);
        }

        if !self.ch_is_terminator() {
            // invalid termination
            return Err(ParserError::SyntaxError(ErrorCode::InvalidPattern));
        }

        Ok(Expression::PatternSeq(s))
    }

    fn parse_pattern_null_seq(&mut self, s: String) -> Result<Expression, ParserError> {
        if !self.ch_is_terminator() {
            // invalid termination
            return Err(ParserError::SyntaxError(ErrorCode::InvalidPattern));
        }

        Ok(Expression::PatternNullSeq(s))
    }

    fn ch_is_terminator(&self) -> bool {
        self.ch_is_whitespace()
        || self.ch == Some('(')
        || self.ch == Some(')')
        || self.ch == None
    }

    fn ch_is_whitespace(&self) -> bool {
        match self.ch {
            None => false,
            Some(c) => c.is_whitespace()
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch_is_whitespace() {
            self.ch = self.iter.next();
        }
    }

    fn bump(&mut self) {
        self.ch = self.iter.next();
    }
}
