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

use std::fmt;
use std::str::FromStr;

use matching::Match;
use binding::Bind;

mod parser;

// TODO: implement equality check
/// The `Expression` type.
#[unstable(feature = "ers1")]
pub enum Expression {
    /// Contains a boxed slice of `Expressions`
    List(Vec<Expression>),
    /// Represents a string expression
    Atom(String),
    /// An unnamed pattern matching a single expression
    Blank,
    /// An unnamed pattern matching one or more expressions
    BlankSeq,
    /// An unnamed pattern matching zero or more expressions
    BlankNullSeq,
    /// A named pattern matching a single expression
    Pattern(String),
    /// A named pattern matching one or more expressions
    PatternSeq(String),
    /// A named pattern matching zero or more expressions
    PatternNullSeq(String),
}

#[unstable(feature = "ers1")]
impl Expression {
    /// Matches the `Expression` with a pattern and if successful binds returns
    /// the bound template.
    ///
    /// # Example
    /// ```
    /// use ers::Expression;
    ///
    /// let expr = "(x z)".parse::<Expression>().unwrap();
    /// let pattern = "(x a_)".parse::<Expression>().unwrap();
    /// let template = "(y a)".parse::<Expression>().unwrap();
    ///
    /// expr.replace(&pattern, template).unwrap(); // => (y z)
    /// ```
    #[unstable(feature = "ers1")]
    pub fn replace(&self, pattern: &Expression, template: Expression) -> Option<Expression> {
        self.match_pattern(pattern).map(move |bs| template.bind(&bs))
    }

    /// Replaces all expressions and subexpressions with the provided pattern
    /// and replaces it by the bound template
    ///
    /// # Example
    /// ```
    /// use ers::Expression;
    ///
    /// let expr = "((x r) (x s))".parse::<Expression>().unwrap();
    /// let pattern = "(x a_)".parse::<Expression>().unwrap();
    /// let template = "(y a)".parse::<Expression>().unwrap();
    ///
    /// expr.replace_all(&pattern, template); // => ((y r) (y s))
    /// ```
    #[unstable(feature = "ers1")]
    pub fn replace_all(&self, pattern: &Expression, template: Expression) -> Expression {
        let (e, _) = self.replace_rec(pattern, template);
        e
    }

    /// Replaces all expressions and subexpression repeatedly until the
    /// expression does not change anymore.
    /// The hardcoded limit is 1000 repetitions. The function panics if the
    /// limit is reached.
    ///
    /// # Example
    /// ```
    /// use ers::Expression;
    ///
    /// let expr = "(x (x (x z)))".parse::<Expression>().unwrap();
    /// let pattern = "(x a_)".parse::<Expression>().unwrap();
    /// let template = "(y a)".parse::<Expression>().unwrap();
    ///
    /// expr.replace_repeated(&pattern, template); // => (y (y (y z)))
    /// ```
    #[unstable(feature = "experimental")]
    pub fn replace_repeated(&self, pattern: &Expression, template: Expression) -> Expression {
        // TODO: set limit as global constant
        let mut limit = 1000;
        let mut expr = self.clone();
        while limit >= 0 {
            limit -= 1;
            let (new_expr, replaced) = expr.replace_rec(pattern, template.clone());
            if !replaced {
                return new_expr;
            }
            expr = new_expr;
        }
        // TODO: panic might not be the right thing to do
        panic!("replacement limit reached!");
    }

    fn replace_rec(&self, pattern: &Expression, template: Expression) -> (Expression, bool) {
        match self.match_pattern(pattern) {
            None => {
                match self {
                    &Expression::List(ref es) => {
                        let mut v: Vec<Expression> = Vec::new();
                        let mut replaced = false;
                        for e in es {
                            let (new_e, r) = e.replace_rec(pattern, template.clone());
                            if r {
                                replaced = true;
                            }
                            v.push(new_e);
                        }
                        (Expression::List(v), replaced)
                    }
                    _ => (self.clone(), false) // not replaced
                }
            }
            Some(bs) => (template.bind(&bs), true) // replaced
        }
    }
}

impl Clone for Expression {
    fn clone(&self) -> Self {
        match self {
            &Expression::List(ref es) => {
                let mut v: Vec<Expression> = Vec::new();
                for e in es {
                    v.push(e.clone());
                }
                Expression::List(v)
            }
            &Expression::Atom(ref s) => {
                Expression::Atom(s.clone())
            }
            &Expression::Blank => {
                Expression::Blank
            }
            &Expression::BlankSeq => {
                Expression::BlankSeq
            }
            &Expression::BlankNullSeq => {
                Expression::BlankNullSeq
            }
            &Expression::Pattern(ref s) => {
                Expression::Pattern(s.clone())
            }
            &Expression::PatternSeq(ref s) => {
                Expression::PatternSeq(s.clone())
            }
            &Expression::PatternNullSeq(ref s) => {
                Expression::PatternNullSeq(s.clone())
            }
        }
    }
}

impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Expression::Atom(ref s) => { write!(f, "{}", s) }
            Expression::List(ref es) => {
                let mut s = String::new();
                for i in (0..es.len()) {
                    let f = format!("{:?}", es[i]);
                    s = s + &f[..];
                    if i != es.len() - 1 {
                        s.push(' ');
                    }
                }
                write!(f, "({})", s)
            }
            Expression::Blank => { write!(f, "_") }
            Expression::BlankSeq => { write!(f, "__") }
            Expression::BlankNullSeq => { write!(f, "___") }
            Expression::Pattern(ref s) => { write!(f, "{}_", s)}
            Expression::PatternSeq(ref s) => { write!(f, "{}__", s)}
            Expression::PatternNullSeq(ref s) => { write!(f, "{}___", s)}
        }
    }
}

impl FromStr for Expression {
    type Err = parser::ParserError;
    fn from_str(s: &str) -> Result<Expression, parser::ParserError> {
        let mut parser = parser::Parser::new(s.chars());

        parser.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::Expression;

    #[test]
    fn debug() {
        let a = Expression::Atom("a".to_string());
        let b = Expression::Atom("b".to_string());
        let c = Expression::Atom("c".to_string());
        let d = Expression::Atom("d".to_string());

        let ls = Expression::List(vec![c, d]);

        let expr = Expression::List(vec![a, b, ls]);

        assert_eq!(format!("{:?}", expr), "(a b (c d))");
    }

    #[test]
    fn parse() {
        let expr = "(a b (c d))".parse::<Expression>();
        assert_eq!(format!("{:?}", expr.unwrap()), "(a b (c d))");
    }

    #[test]
    fn replace() {
        let expr = "(x z)".parse::<Expression>().unwrap();
        let pattern = "(x a_)".parse::<Expression>().unwrap();
        let template = "(y a)".parse::<Expression>().unwrap();

        let res = expr.replace(&pattern, template).unwrap();

        assert_eq!(format!("{:?}", res), "(y z)");
    }

    #[test]
    fn replace_all() {
        let expr = "((x r) (x s))".parse::<Expression>().unwrap();
        let pattern = "(x a_)".parse::<Expression>().unwrap();
        let template = "(y a)".parse::<Expression>().unwrap();

        let res = expr.replace_all(&pattern, template);

        assert_eq!(format!("{:?}", res), "((y r) (y s))");
    }

    #[test]
    fn replace_repeated() {
        let expr = "(x (x (x z)))".parse::<Expression>().unwrap();
        let pattern = "(x a_)".parse::<Expression>().unwrap();
        let template = "(y a)".parse::<Expression>().unwrap();

        let res = expr.replace_repeated(&pattern, template);

        assert_eq!(format!("{:?}", res), "(y (y (y z)))");
    }
}
