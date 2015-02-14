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

use binding::Binding;
use expression::Expression;

// TODO: rewrite Match trait to allow implementing match_epression and
// match_seq with this trait
/// The `Match` interface is not thought out yet and will be documented later
#[unstable(feature = "ers1")]
pub trait Match {
    // TODO: fix after rewriting
    /// The pattern matching function returns the bindings.
    fn match_pattern<'a>(&'a self, p: &Expression) -> Option<HashMap<String, Binding<'a>>>;
}

// TODO: rewrite match_expression and match_seq as impl of the Match trait
impl Match for Expression {
    /// Returns `Some(HashMap<String, Binding>)` if the expression matches
    /// the pattern and `None` otherwise
    ///
    /// # Example
    ///
    /// ```
    /// use ers::{Expression, Match};
    ///
    /// let expr = "(x (y z))".parse::<Expression>().unwrap();
    /// let pattern = "(_ a_)".parse::<Expression>().unwrap();
    ///
    /// expr.match_pattern(&pattern); // => Some(HashMap {"a": Expression((y z))})
    /// ```
    fn match_pattern<'a>(&'a self, p: &Expression) -> Option<HashMap<String, Binding<'a>>> {
        let mut bs: HashMap<String, Binding> = HashMap::new();
        if match_expression(self, p, &mut bs) {
            Some(bs)
        } else {
            None
        }
    }
}

fn match_expression<'a>(e: &'a Expression, p: &Expression, bs: &mut HashMap<String, Binding<'a>>) -> bool {
    match (e, p) {
        (_, &Expression::Blank) => { true }
        (_, &Expression::BlankSeq) => { true }
        (_, &Expression::BlankNullSeq) => { true }
        (exp, &Expression::Pattern(ref s)) => {
            bs.insert(s.clone(), Binding::Expression(exp));
            true
        }
        (&Expression::Atom(ref i), &Expression::Atom(ref j)) => {
            i == j
        }
        (&Expression::List(ref es), &Expression::List(ref ps)) => {
            match_seq(es, ps, bs)
        }
        _ => { false } // catch all - should not happen
    }
}

fn match_seq<'a>(es: &'a [Expression], ps: &[Expression], bs: &mut HashMap<String, Binding<'a>>) -> bool {
    if ps.len() == 0 {
        return es.len() == 0;
    }

    match ps[0] {
        Expression::BlankSeq => {
            for i in (1..es.len() + 1) {
                if match_seq(&es[i..], &ps[1..], bs) {
                    return true;
                }
            }
            false
        }
        Expression::BlankNullSeq => {
            if es.len() == 0 {
                return true;
            }

            for i in (0..es.len() + 1) {
                if match_seq(&es[i..], &ps[1..], bs) {
                    return true;
                }
            }
            false
        }
        Expression::PatternSeq(ref s) => {
            for i in (1..es.len() + 1) {
                let mut h: HashMap<String, Binding<'a>> = HashMap::new();
                h.insert(s.clone(), Binding::Sequence(&es[0..i]));
                if match_seq(&es[i..], &ps[1..], &mut h) {
                    for (key, val) in h.iter() {
                        bs.insert(key.clone(), val.clone());
                    }
                    return true;
                }
            }
            false
        }
        Expression::PatternNullSeq(ref s) => {
            if es.len() == 0 {
                bs.insert(s.clone(), Binding::Sequence(es));
                return true;
            }

            for i in (0..es.len() + 1) {
                let mut h: HashMap<String, Binding<'a>> = HashMap::new();
                h.insert(s.clone(), Binding::Sequence(&es[0..i]));
                if match_seq(&es[i..], &ps[1..], &mut h) {
                    for (key, val) in h.iter() {
                        bs.insert(key.clone(), val.clone());
                    }
                    return true;
                }
            }
            false
        }
        _ => {
            if es.len() == 0 {
                return false;
            }

            match_expression(&es[0], &ps[0], bs) && match_seq(&es[1..], &ps[1..], bs)
        }
    }
}
