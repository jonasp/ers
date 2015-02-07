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

#![feature(ers1)]

extern crate ers;

use ers::Expression;

#[cfg(not(test))]
fn main() {
    let expr = "((x r) (x s))".parse::<Expression>().unwrap();
    let pattern = "(x a_)".parse::<Expression>().unwrap();
    let template = "(y a)".parse::<Expression>().unwrap();

    println!("{:?}", expr.replace_all(&pattern, template)); // => ((y z))

    //let t = "a".parse::<Expression>().unwrap();
    //let p = "x".parse::<Expression>().unwrap();

    //let e = "(x x ((x) x) x)".parse::<Expression>().unwrap();
    //for sub in e.subexpressions() {
        //print!("{:?} -> ", sub);
        //match sub.match_pattern(&p) {
            //Some(bs) => {
                //println!("{:?}", t.clone().bind(&bs));
            //}
            //None => {
                //println!("{:?}", sub);
            //}
        //}
    //}
}
