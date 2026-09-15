use proptest::{
    arbitrary::{any_with, Arbitrary},
    strategy::{NewTree, Strategy, ValueTree},
    test_runner::{Reason, TestRunner},
};

use crate::types::{
    InvalidNumber, Negative, NegativeFinite, NonNaN, NonNaNFinite, NonZeroNonNaN,
    NonZeroNonNaNFinite, Positive, PositiveFinite, StrictlyNegative, StrictlyNegativeFinite,
    StrictlyPositive, StrictlyPositiveFinite,
};

#[derive(Clone, Copy, Debug)]
pub struct TypedFloatStrategy<T, InnerStrategy> {
    _marker: core::marker::PhantomData<T>,
    inner: InnerStrategy,
}

#[derive(Clone, Copy, Debug)]
pub struct TypedFloatValueTree<InnerValueTree, TypedFloatType> {
    inner: InnerValueTree,
    value: TypedFloatType,
}

impl<T, InnerStrategy> TypedFloatStrategy<T, InnerStrategy> {
    const fn new(inner: InnerStrategy) -> Self {
        Self {
            _marker: core::marker::PhantomData,
            inner,
        }
    }
}

macro_rules! impl_arbitrary {
    ($type:ident, $float_type:ty) => {
        impl<InnerValueTree: ValueTree<Value = $float_type>>
            TypedFloatValueTree<InnerValueTree, $type<$float_type>>
        {
            fn new(inner: InnerValueTree) -> Result<Self, Reason> {
                $type::<$float_type>::new(inner.current())
                    .map(|value| Self { inner, value })
                    .map_err(|e| match e {
                        InvalidNumber::NaN => "no NaN".into(),
                        InvalidNumber::Zero => "no zero".into(),
                        InvalidNumber::Negative => "no negative numbers".into(),
                        InvalidNumber::Positive => "no positive numbers".into(),
                        InvalidNumber::Infinite => "no infinite".into(),
                    })
            }
        }

        impl<InnerValueTree: ValueTree<Value = $float_type>> ValueTree
            for TypedFloatValueTree<InnerValueTree, $type<$float_type>>
        {
            type Value = $type<$float_type>;

            fn current(&self) -> Self::Value {
                self.value
            }

            fn simplify(&mut self) -> bool {
                self.inner.simplify()
            }

            fn complicate(&mut self) -> bool {
                self.inner.complicate()
            }
        }

        impl<InnerStrategy: Strategy<Value = $float_type>> Strategy
            for TypedFloatStrategy<$type<$float_type>, InnerStrategy>
        {
            type Tree = TypedFloatValueTree<InnerStrategy::Tree, $type<$float_type>>;
            type Value = $type<$float_type>;

            fn new_tree(&self, runner: &mut TestRunner) -> NewTree<Self> {
                self.inner
                    .new_tree(runner)
                    .and_then(TypedFloatValueTree::<InnerStrategy::Tree, $type<$float_type>>::new)
            }
        }

        impl Arbitrary for $type<$float_type> {
            type Parameters = <$float_type as Arbitrary>::Parameters;
            type Strategy = TypedFloatStrategy<Self, <$float_type as Arbitrary>::Strategy>;

            fn arbitrary_with(args: Self::Parameters) -> Self::Strategy {
                TypedFloatStrategy::new(any_with::<$float_type>(args))
            }
        }
    };
    ($type:ident) => {
        impl_arbitrary!($type, f32);
        impl_arbitrary!($type, f64);
    };
}

impl_arbitrary!(NonNaN);
impl_arbitrary!(NonZeroNonNaN);
impl_arbitrary!(NonNaNFinite);
impl_arbitrary!(NonZeroNonNaNFinite);
impl_arbitrary!(Positive);
impl_arbitrary!(Negative);
impl_arbitrary!(PositiveFinite);
impl_arbitrary!(NegativeFinite);
impl_arbitrary!(StrictlyPositive);
impl_arbitrary!(StrictlyNegative);
impl_arbitrary!(StrictlyPositiveFinite);
impl_arbitrary!(StrictlyNegativeFinite);
