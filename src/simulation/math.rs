use crate::simulation::{Float, OVec2};
use auto_ops::*;
use std::ops;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use vector2math::{FloatingVector2, Pair, Vector2};

impl_op!(+ |a:OVec2, b: OVec2| -> OVec2 { Vector2::add(a,b) });
impl_op!(-|a: OVec2, b: OVec2| -> OVec2 { Vector2::sub(a, b) });
impl_op!(*|a: OVec2, b: OVec2| -> OVec2 { Vector2::mul2(a, b) });
impl_op!(/ |a:OVec2, b: OVec2| -> OVec2 { Vector2::div2(a,b) });

impl_op!(+ |a:OVec2, b: Float| -> OVec2 { Vector2::add(a,OVec2::from_items(b,b)) });
impl_op!(-|a: OVec2, b: Float| -> OVec2 { Vector2::sub(a, OVec2::from_items(b, b)) });
impl_op!(*|a: OVec2, b: Float| -> OVec2 { Vector2::mul2(a, OVec2::from_items(b, b)) });
impl_op!(/ |a:OVec2, b: Float| -> OVec2 { Vector2::div2(a,OVec2::from_items(b,b)) });

impl_op!(+= |a: &mut OVec2, b: OVec2| { Vector2::add_assign(a,b) });
impl_op!(-= |a: &mut OVec2, b: OVec2| { Vector2::sub_assign(a, b) });
impl_op!(*= |a: &mut OVec2, b: OVec2| { Vector2::mul2_assign(a, b) });
impl_op!(/= |a: &mut OVec2, b: OVec2| { Vector2::div2_assign(a,b) });

impl_op!(+= |a: &mut OVec2, b: Float| { Vector2::add_assign(a,OVec2::from_items(b,b)) });
impl_op!(-= |a: &mut OVec2, b: Float| { Vector2::sub_assign(a, OVec2::from_items(b,b)) });
impl_op!(*= |a: &mut OVec2, b: Float| { Vector2::mul2_assign(a, OVec2::from_items(b,b)) });
impl_op!(/= |a: &mut OVec2, b: Float| { Vector2::div2_assign(a,OVec2::from_items(b,b)) });

impl OVec2 {
    pub fn length(&self) -> f64 {
        self.dist(OVec2::from_items(0.0, 0.0))
    }

    pub fn zero() -> Self {
        OVec2::from_items(0.0, 0.0)
    }
}
