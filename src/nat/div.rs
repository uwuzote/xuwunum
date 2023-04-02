//! Note: division by [0](NZ) won't typecheck;

use super::{
    Nat, NZ, NS,
    pan::{
        NSwP,
        NPUnwrap
    },
    NPSub, NatPSub
};

/// Quotient of natural division
pub type NDivQ<Lhs, Rhs> = <Lhs as NatDivQ<Rhs>>::Result;
/// Remainder of natural division
pub type NDivR<Lhs, Rhs> = <Lhs as NatDivR<Rhs>>::Result;

pub trait NatDivQ<Rhs: Nat + ?Sized>: Nat {
	type Result: Nat + ?Sized;
}

pub trait NatDivR<Rhs: Nat + ?Sized>: Nat {
	type Result: Nat + ?Sized;
}

/// `0 // (X +1) => 0`
impl<X: Nat + ?Sized> NatDivQ<NS<X>> for NZ {
	type Result = NZ;
}

/// `0 % (X +1) => 0`
impl<X: Nat + ?Sized> NatDivR<NS<X>> for NZ {
	type Result = NZ;
}

/// `(A +1) % (B +1) => ((A + 1) -! (B +1)) !? (A +1) : ( (((A +1) -! (B +1)) ?! 0) % (B +1)  )`
/// `(A +1) % (B +1) => (A -! B) !? (A +1) : ( (A - B) % (B +1) )`
impl<A: Nat + ?Sized, B: Nat + ?Sized> NatDivR<NS<B>> for NS<A>
where
    A: NatPSub<B>,
{
    type Result = NSwP<NPSub<A, B>, NS<A>, NDivR<NPUnwrap<NPSub<A, B>, NZ>, NS<B>>>;
}

