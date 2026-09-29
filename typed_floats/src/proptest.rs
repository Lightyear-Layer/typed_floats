use proptest::{
    arbitrary::Arbitrary,
    strategy::{FilterMap, Strategy},
};

use core::ops::RangeInclusive;

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

macro_rules! impl_arbitrary {
    ($type:ident, ($($min:tt)+)..=($($max:tt)+)) => {
        impl_arbitrary!(@gen, $type, (impl_arbitrary!(@to_expression, f32, $($min)+), impl_arbitrary!(@to_expression, f32, $($max)+)), f32);
        impl_arbitrary!(@gen, $type, (impl_arbitrary!(@to_expression, f64, $($min)+), impl_arbitrary!(@to_expression, f64, $($max)+)), f64);
    };
    (@gen, $type:ident, ($min:expr, $max:expr), $float_type:ty) => {
        impl Arbitrary for $type<$float_type> {
            type Parameters = ();

            type Strategy = FilterMap<RangeInclusive<$float_type>, fn($float_type) -> Option<Self>>;

            fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
                (($min)..=($max)).prop_filter_map(
                    concat!("Must be representable by typed_floats::", stringify!($type), "::<", stringify!($float_type), ">"),
                    (into_typed_float as fn($float_type) -> Option<Self>),
                )
            }
        }
    };
    (@to_expression, $float_type:ty, ($($e:tt)+)) => {
        impl_arbitrary!(@to_expression, $float_type, $($e)+)
    };
    (@to_expression, $float_type:ty, $e:tt . $($chain:tt)*) => {
        const { impl_arbitrary!(@to_expression, $float_type, $e) . $($chain)+ }
    };
    (@to_expression, $float_type:ty, $e:literal) => {
        ($e as $float_type)
    };
    (@to_expression, $float_type:ty, $e:ident) => {
        <$float_type>::$e
    };
}

impl_arbitrary!(NonNaN, (NEG_INFINITY)..=(INFINITY));
impl_arbitrary!(NonZeroNonNaN, (NEG_INFINITY)..=(INFINITY));
impl_arbitrary!(NonNaNFinite, (MIN)..=(MAX));
impl_arbitrary!(NonZeroNonNaNFinite, (MIN)..=(MAX));
impl_arbitrary!(Positive, (0.0)..=(INFINITY));
impl_arbitrary!(Negative, (NEG_INFINITY)..=(0.0));
impl_arbitrary!(PositiveFinite, (0.0)..=(MAX));
impl_arbitrary!(NegativeFinite, (MIN)..=(0.0));
impl_arbitrary!(StrictlyPositive, (0.0.next_up())..=(INFINITY));
impl_arbitrary!(StrictlyNegative, (NEG_INFINITY)..=(0.0.next_down()));
impl_arbitrary!(StrictlyPositiveFinite, (0.0.next_up())..=(MAX));
impl_arbitrary!(StrictlyNegativeFinite, (MIN)..=(0.0.next_down()));
