//! marker traits

/// marks a type as being usable as a scalar in linear algebra
pub trait Scalar {}

impl Scalar for f64 {}
impl Scalar for f32 {}
impl Scalar for i128 {}
impl Scalar for u128 {}
impl Scalar for i64 {}
impl Scalar for u64 {}
impl Scalar for i32 {}
impl Scalar for u32 {}
impl Scalar for i16 {}
impl Scalar for u16 {}
impl Scalar for i8 {}
impl Scalar for u8 {}
impl Scalar for isize {}
impl Scalar for usize {}
