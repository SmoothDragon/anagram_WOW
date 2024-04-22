/* NOTEs
 * FromStr - Many to one and not reversible
 * TryFrom<&str> - generally one-to-one and reversible, but may fail
 * From - intended for perfect conversions, no failure
 * Into - Conversion is always possible
 * TryInto - Conversion may fail
 *
 * Always prefer From to Into since Into is derivabel from From.
 * */

pub mod char_set;
pub mod char_prime;
pub mod char_multiset;
pub mod query;

pub use crate::char_set::CharSet;
pub use crate::char_multiset::CharMultiSet;
pub use crate::query::Query;
