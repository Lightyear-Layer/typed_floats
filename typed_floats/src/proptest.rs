use proptest::{
    arbitrary::Arbitrary,
    strategy::{FilterMap, Strategy},
};

use crate::types::{
    Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN, NonZeroNonNaNFinite, Positive,
    PositiveFinite, StrictlyNegative, StrictlyNegativeFinite, StrictlyPositive,
    StrictlyPositiveFinite,
};

fn into_typed_float<Output, T>(val: T) -> Option<Output>
where
    Output: TryFrom<T>,
{
    Output::try_from(val).ok()
}

macro_rules! impl_arbitrary2 {
    ($type:ident, $($tags:ident)|+) => {
        impl_arbitrary2!($type, f32, $($tags)|+);
        impl_arbitrary2!($type, f64, $($tags)|+);
    };
    ($type:ident, $float_type:ident, $($tags:ident)|+) => {
        impl Arbitrary for $type<$float_type> {
            type Parameters = ();

            type Strategy = FilterMap<proptest::num::$float_type::Any, fn($float_type) -> Option<Self>>;

            fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
                use proptest::num::$float_type::*;

                let base_strategy = $($tags)|+;

                base_strategy.prop_filter_map(
                    concat!("Must be representable by typed_floats::", stringify!($type), "::<", stringify!($float_type), ">"),
                    (into_typed_float as fn($float_type) -> Option<Self>),
                )
            }
        }
    };
}

impl_arbitrary2!(
    NonNaN,
    POSITIVE | NEGATIVE | NORMAL | SUBNORMAL | ZERO | INFINITE
);
impl_arbitrary2!(
    NonZeroNonNaN,
    POSITIVE | NEGATIVE | NORMAL | SUBNORMAL | INFINITE
);
impl_arbitrary2!(NonNaNFinite, POSITIVE | NEGATIVE | NORMAL | SUBNORMAL);
impl_arbitrary2!(
    NonZeroNonNaNFinite,
    POSITIVE | NEGATIVE | NORMAL | SUBNORMAL
);
impl_arbitrary2!(Positive, POSITIVE | NORMAL | SUBNORMAL | ZERO | INFINITE);
impl_arbitrary2!(Negative, NEGATIVE | NORMAL | SUBNORMAL | ZERO | INFINITE);
impl_arbitrary2!(PositiveFinite, POSITIVE | NORMAL | SUBNORMAL | ZERO);
impl_arbitrary2!(NegativeFinite, NEGATIVE | NORMAL | SUBNORMAL | ZERO);
impl_arbitrary2!(StrictlyPositive, POSITIVE | NORMAL | SUBNORMAL | INFINITE);
impl_arbitrary2!(StrictlyNegative, NEGATIVE | NORMAL | SUBNORMAL | INFINITE);
impl_arbitrary2!(StrictlyPositiveFinite, POSITIVE | NORMAL | SUBNORMAL);
impl_arbitrary2!(StrictlyNegativeFinite, NEGATIVE | NORMAL | SUBNORMAL);
