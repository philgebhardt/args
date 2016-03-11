use std::fmt;
use std::fmt::{Display,Formatter};

use super::ArgsError;

/// A trait designed to provide validation for command line argument parsing.
pub trait Validation {
    /// The `type` (e.g. i32, String, etc.) to which the validation is applied.
    type T;

    // Instance methods
    /// Returns an `ArgsError` describing the invalid state for the provided `value`.
    fn error(&self, value: &Self::T) -> ArgsError;
    /// Returns a `bool` indicating if the `Validation` passes for the provided `value`.
    fn is_valid(&self, value: &Self::T) -> bool;

    // Defaulted instance methods
    /// Returns a `bool` indicating if the `Validation` fails for the provided `value`.
    fn is_invalid(&self, value: &Self::T) -> bool { !self.is_valid(value) }
}

/// The relationship to use when validating an `OrderValidation`.
pub enum Order {
    /// Represents a strictly greater than relationship.
    GreaterThan,
    /// Represents a greater than relationship that allows equality.
    GreaterThanOrEqual,
    /// Represents a strictly less than relationship.
    LessThan,
    /// Represents a less than relationship that allows equality.
    LessThanOrEqual
}

impl Order {
    /// Compares the provided `value` to the provided `bound`
    pub fn compare<T: PartialOrd>(&self, bound: &T, value: &T) -> bool {
        match *self {
            Order::GreaterThan => { value > bound },
            Order::GreaterThanOrEqual => { value >= bound },
            Order::LessThan => { value < bound },
            Order::LessThanOrEqual => { value <= bound }
        }
    }
}

impl Display for Order {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let display = match *self {
            Order::GreaterThan => { "greater than" },
            Order::GreaterThanOrEqual => { "greater than or equal to" },
            Order::LessThan => { "less than" },
            Order::LessThanOrEqual => { "less than or equal to" }
        };
        write!(f, "{}", display)
    }
}

/// An implementation of the `Validation` trait which tests whether or not
/// a value adheres to the provided `order` and `bound`. It should be noted
/// that the type of `bound`, `O`, must implement `Clone`, `Display` and `PartialOrd`.
///
/// # Example
///
/// ```{.rust}
/// use args::{Order,OrderValidation};
///
/// let validation = OrderValidation::new(Order::GreaterThan, 0u32);
/// validation.is_valid(1u32) // true
/// validation.is_valid(0u32) // false
///
/// if validation.is_invalid(0u32) {
///     // do things
///     error!("{}", validation.error(0u32));
/// }
/// ```
pub struct OrderValidation<O: Clone + Display + PartialOrd> {
    bound: O,
    order: Order
}

impl<O: Clone + Display + PartialOrd> OrderValidation<O> {
    /// Creates a new `OrderValidation` with the provided `order` and `bound`.
    pub fn new(order: Order, bound: O) -> OrderValidation<O> {
        OrderValidation { bound: bound.clone(), order: order }
    }
}

impl<O: Clone + Display + PartialOrd> Validation for OrderValidation<O> {
    type T = O;

    fn error(&self, value: &O) -> ArgsError {
        ArgsError::new("order invalid", &format!("{} is not {} {}", value, self.order, self.bound))
    }

    fn is_valid(&self, value: &O) -> bool {
        self.order.compare(&self.bound, value)
    }
}

