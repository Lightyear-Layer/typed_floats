use defmt::{write, Format, Formatter};

use crate::types::{
    FromStrError, InvalidNumber, Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN,
    NonZeroNonNaNFinite, Positive, PositiveFinite, StrictlyNegative, StrictlyNegativeFinite,
    StrictlyPositive, StrictlyPositiveFinite,
};

macro_rules! impl_format {
    ($type:ident) => {
        impl Format for $type<f64> {
            fn format(&self, fmt: Formatter<'_>) {
                write!(fmt, "{=f64}", self.get());
            }
        }

        impl Format for $type<f32> {
            fn format(&self, fmt: Formatter<'_>) {
                write!(fmt, "{=f32}", self.get());
            }
        }
    };
}

impl_format!(NonNaN);
impl_format!(NonZeroNonNaN);
impl_format!(NonNaNFinite);
impl_format!(NonZeroNonNaNFinite);
impl_format!(Positive);
impl_format!(Negative);
impl_format!(PositiveFinite);
impl_format!(NegativeFinite);
impl_format!(StrictlyPositive);
impl_format!(StrictlyNegative);
impl_format!(StrictlyPositiveFinite);
impl_format!(StrictlyNegativeFinite);

impl Format for InvalidNumber {
    fn format(&self, fmt: Formatter<'_>) {
        match self {
            Self::NaN => write!(fmt, "{=str}", "Number is NaN"),
            Self::Zero => write!(fmt, "{=str}", "Number is zero"),
            Self::Negative => write!(fmt, "{=str}", "Number is negative"),
            Self::Positive => write!(fmt, "{=str}", "Number is positive"),
            Self::Infinite => write!(fmt, "{=str}", "Number is infinite"),
        }
    }
}

impl Format for FromStrError {
    fn format(&self, fmt: Formatter<'_>) {
        match self {
            Self::ParseFloatError(e) => write!(fmt, "{:?}", defmt::Display2Format(e)),
            Self::InvalidNumber(invalid_number) => invalid_number.format(fmt),
        }
    }
}
