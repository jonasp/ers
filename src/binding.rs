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

use std::collections::HashMap;

use expression::Expression;

// TODO: make Binding not clonable
#[derive(Debug, Clone)]
/// The `Binding` type.
#[unstable(feature = "ers1")]
pub enum Binding<'a> {
    /// Binding for a single expression
    Expression(&'a Expression),
    /// Binding for a set of zero or more expressions
    Sequence(&'a[Expression]),
}

/// The `Bind` interface allows us to bind variables according to the bindings.
#[unstable(feature = "ers1")]
pub trait Bind {
    /// Bind the variables according to the provided bindings. Returns itself
    /// if no variables were bound.
    fn bind(self, bs: &HashMap<String, Binding>) -> Self;
}

impl Bind for Expression {
    /// Returns an `Expression` with elements replaced according to the bindings
    /// # Example
    ///
    /// ```
    /// use ers::{Expression, Match, Bind};
    ///
    /// let expr = "(x (y z))".parse::<Expression>().unwrap();
    /// let pattern = "(_ a_)".parse::<Expression>().unwrap();
    ///
    /// let bindings =  expr.match_pattern(&pattern).unwrap();
    ///
    /// let template = "(a)".parse::<Expression>().unwrap();
    ///
    /// template.bind(&bindings); // => ((y z))
    /// ```
    fn bind(self, bs: &HashMap<String, Binding>) -> Expression {
        match self {
             Expression::Atom(s) => {
                 match bs.get(&s) {
                     Some(&Binding::Sequence(seq)) => {
                         // This should only happen in the root. If this
                         // shows up deeper in the list we did something
                         // wrong when iterating through a list.

                         let mut v = vec![
                            Expression::Atom("Sequence".to_string()),
                         ];
                         for s in seq {
                             v.push(s.clone());
                         }

                         Expression::List(v.into_boxed_slice())

                     }
                     Some(&Binding::Expression(e)) => {
                         e.clone()
                     }
                     None => {
                         Expression::Atom(s)
                     }
                 }
             }
             Expression::List(es) => {
                 Expression::List(es.bind(bs))
             }
             _ => self
         }
    }
}

impl Bind for Box<[Expression]> {
    fn bind(self, bs: &HashMap<String, Binding>) -> Box<[Expression]> {
        let mut v: Vec<Expression> = Vec::new();

        // TODO: implement IntoIter on boxed slice
        //       but into_vec() is basically free
        for e in self.into_vec() {
            // check if a binding could be a sequence as
            // we need to insert it at this point.
            // TODO: This match returns true/false and executes a
            // push if true. Can we do this in the match only?
            if match e {
                Expression::Atom(ref s) => {
                    match bs.get(s) {
                        Some(&Binding::Sequence(seq)) => {
                            for s in seq {
                                v.push(s.clone())
                            }
                            false
                        }
                        _ => true
                    }
                }
                _ => true
            } {
                v.push(e.bind(bs));
            }
        }

        v.into_boxed_slice()
    }
}
