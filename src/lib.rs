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

//! # Exprssion Rewriting System
//!
//! The Expression Rewriting System provides the tools
//! to create and transform expressions based on patterns
//! and rewriting rules.
//!
//!
//! ## Expressions
//!
//! An [`Expression`](enum.Expression.html) is the fundamental
//! object wich will be created and transformed. The string representation
//! of an [`Expression`](enum.Expression.html) is an S-Expression.
//!
//! We have several types of expressions where most of them are needed for
//! matching. But we have two fundamental basic blocks one is an `Atom`
//! and the other one is a `List`.  An `Atom` represents a string entry while
//! a `List` contains a list of expressions. Denoted as S-Expressions we have
//! for example `(x (y z))` which is a list of two expressions. The first is
//! an `Atom` with the value `x`. The second is another `List` which itself
//! contains two `Atom` expressions representing `y` and `z`.
//!
//! ```
//! use ers::Expression;
//!
//! let x = Expression::Atom("x".to_string());
//! let y = Expression::Atom("y".to_string());
//! let z = Expression::Atom("z".to_string());
//!
//! let ls = Expression::List(vec![y, z]);
//! format!("{:?}", ls); // => "(y z)"
//!
//! let ers = Expression::List(vec![x, ls]);
//! format!("{:?}", ers); // => "(x (y z))"
//! ```
//!
//! ### Parsing
//!
//! We can also build an `Expression` by parsing an S-Expression string.
//!
//! ```
//! use ers::Expression;
//!
//! let ls = "(x (y z))".parse::<Expression>();
//!
//! assert_eq!(format!("{:?}", ls.unwrap()), "(x (y z))");
//! ```
//!
//!
//! ## Matching
//!
//! The other types of expressions are `Blank` and `Pattern` together with
//! their derivatives `BlankSeq`, `BlankNullSeq` as well as `PatternSeq`
//! and `PatternNullSeq`. These expressions are used when we want to match
//! another expression and create bindings which can be used to rewrite
//! expressions. The difference between a `Pattern` and `Blank` is
//! that `Blank` is unnamed while `Pattern` does carry a name. The S-Expression
//! representation is `_` for a `Blank` and `x_` for a `Pattern` with the name
//! `x`. `BlankSeq` and `PatternSeq` are sequence patterns and are denoted
//! `__` as well as `x__`. Finally `BlankNullSeq` and `PatternNullSeq` use
//! three underscores `___` and `x___`.
//!
//! ```
//! use ers::{Expression, Match};
//!
//! let expr = "(x (y z))".parse::<Expression>().unwrap();
//! let pattern = "(_ a_)".parse::<Expression>().unwrap();
//!
//! // create bindings mapping `a` to `(y z)`
//! let bindings = expr.match_pattern(&pattern);
//! ```
//!
//! ## Binding
//!
//! Once we successfully matched an expression and generated bindings we can use
//! these to bind them to a template. A template is an expression where the
//! variable contained in the bindings are an `Atom` expression within the
//! template.
//!
//! ```
//! use ers::{Expression, Match, Bind};
//!
//! let expr = "(x (y z))".parse::<Expression>().unwrap();
//! let pattern = "(_ a_)".parse::<Expression>().unwrap();
//!
//! // create bindings mapping `a` to `(y z)`
//! let bindings =  expr.match_pattern(&pattern).unwrap();
//!
//! let template = "(a b)".parse::<Expression>().unwrap();
//!
//! // replace the atom `a` in the template with the expression `(y z)`
//! // according to the bindings
//! let replaced = template.bind(&bindings);
//!
//! assert_eq!(format!("{:?}", replaced), "((y z) b)");
//! ```

#![feature(staged_api)]
#![staged_api]
#![unstable(feature = "ers1")]

#![feature(collections)]

#![deny(missing_docs)]

#![crate_type = "rlib"]
#![crate_type = "dylib"]

pub use expression::Expression;
pub use matching::Match;
pub use binding::Binding;
pub use binding::Bind;

mod expression;
mod matching;
mod binding;
