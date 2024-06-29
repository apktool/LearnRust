use chapter19::{ident_one, ident_three, ident_two};
use chapter19::{define_matcher, define_rule};
use chapter19::macros2::{OptRelNodeTyp, RuleMatcher};

fn main() {
    ident_one!(a, "Hello World");
    ident_two!(a, "Hello Rust"; b, "Hello Python");
    ident_three!(a, "foo");
    define_rule!((OptRelNodeTyp::Projection, (OptRelNodeTyp::Filter, child, [cond]), [exprs]));
}
