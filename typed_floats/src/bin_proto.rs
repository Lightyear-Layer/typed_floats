use bin_proto::{BitDecode, BitEncode, BitRead, BitWrite, Endianness, Error};

use crate::types::{
    InvalidNumber, Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN,
    NonZeroNonNaNFinite, Positive, PositiveFinite, StrictlyNegative, StrictlyNegativeFinite,
    StrictlyPositive, StrictlyPositiveFinite,
};

macro_rules! impl_bin_proto {
    ($type:ident) => {
        impl_bin_proto!($type, f32);
        impl_bin_proto!($type, f64);
    };
    ($type:ident, $float_type:ty) => {
        impl<Ctx, Tag> BitEncode<Ctx, Tag> for $type<$float_type>
        where
            $float_type: BitEncode<Ctx, Tag>,
        {
            fn encode<W, E>(&self, write: &mut W, ctx: &mut Ctx, tag: Tag) -> Result<(), Error>
            where
                W: BitWrite,
                E: Endianness,
            {
                self.get().encode::<W, E>(write, ctx, tag)
            }
        }

        impl<Ctx, Tag> BitDecode<Ctx, Tag> for $type<$float_type>
        where
            $float_type: BitDecode<Ctx, Tag>,
        {
            fn decode<R, E>(read: &mut R, ctx: &mut Ctx, tag: Tag) -> Result<Self, Error>
            where
                R: BitRead,
                E: Endianness,
            {
                <$float_type>::decode::<R, E>(read, ctx, tag).and_then(|value| {
                    Self::new(value).map_err(|e| match e {
                        InvalidNumber::NaN => Error::Other("invalid NaN"),
                        InvalidNumber::Zero => Error::Other("invalid zero value"),
                        InvalidNumber::Negative => Error::Other("invalid negative value"),
                        InvalidNumber::Positive => Error::Other("invalid positive value"),
                        InvalidNumber::Infinite => Error::Other("invalid infinite value"),
                    })
                })
            }
        }
    };
}

impl_bin_proto!(NonNaN);
impl_bin_proto!(NonZeroNonNaN);
impl_bin_proto!(NonNaNFinite);
impl_bin_proto!(NonZeroNonNaNFinite);
impl_bin_proto!(Positive);
impl_bin_proto!(Negative);
impl_bin_proto!(PositiveFinite);
impl_bin_proto!(NegativeFinite);
impl_bin_proto!(StrictlyPositive);
impl_bin_proto!(StrictlyNegative);
impl_bin_proto!(StrictlyPositiveFinite);
impl_bin_proto!(StrictlyNegativeFinite);
