//! Note: division by [0](NZ) won't typecheck;

use super::{
    pan::{NPUnwrap, NSwP, NatPanUnwrap, NatSwP},
    NPSub, NZSub, Nat, NatPSub, NatZSub, NS, NZ,
};

/// Quotient of natural division
pub type NDivQuot<Lhs, Rhs> = <Lhs as NatDivQuot<Rhs>>::Result;
/// Remainder of natural division
pub type NDivRem<Lhs, Rhs> = <Lhs as NatDivRem<Rhs>>::Result;

pub trait NatDivQuot<Rhs: Nat + ?Sized>: Nat {
    type Result: Nat + ?Sized;
}

pub trait NatDivRem<Rhs: Nat + ?Sized>: Nat {
    type Result: Nat + ?Sized;
}

/// `0 // (X +1) => 0`
impl<X: Nat + ?Sized> NatDivQuot<NS<X>> for NZ {
    type Result = NZ;
}

/// `0 % (X +1) => 0`
impl<X: Nat + ?Sized> NatDivRem<NS<X>> for NZ {
    type Result = NZ;
}

/// `(A +1) % (B +1) => ((A + 1) -! (B +1)) !? (A +1) : ( (((A +1) -! (B +1)) !? 0) % (B +1)  )` \
/// `(A +1) % (B +1) => (A -! B) !? (A +1) : ( (A - B) % (B +1) )`
impl<A: Nat + ?Sized, B: Nat + ?Sized> NatDivRem<NS<B>> for NS<A>
where
    A: NatPSub<B>,
    NPSub<A, B>: NatPanUnwrap<NZ>,
    NPUnwrap<NPSub<A, B>, NZ>: NatDivRem<NS<B>>,
    NPSub<A, B>: NatSwP<NS<A>, NDivRem<NPUnwrap<NPSub<A, B>, NZ>, NS<B>>>,
{
    type Result = NSwP<NPSub<A, B>, NS<A>, NDivRem<NPUnwrap<NPSub<A, B>, NZ>, NS<B>>>;
}

/// `(A +1) // (B +1) => ((A +1) -! (B +1)) !? 0 : (((((A +1) -! (B +1)) !? 0) // (B +1)) +1)` \
/// `(A +1) // (B +1) => (A - B) !? 0 : (((A - B) // (B+1))+1)`
impl<A: Nat + ?Sized, B: Nat + ?Sized> NatDivQuot<NS<B>> for NS<A>
where
    A: NatPSub<B>,
    A: NatZSub<B>,
    NPSub<A, B>: NatSwP<NZ, NS<NDivQuot<NZSub<A, B>, NS<B>>>>,
    NZSub<A, B>: NatDivQuot<NS<B>>,
{
    type Result = NSwP<NPSub<A, B>, NZ, NS<NDivQuot<NZSub<A, B>, NS<B>>>>;
}

#[cfg(test)]
mod tests {
    use super::{NDivQuot, NDivRem};
    use crate::nat::nums::*;
    use crate::runtime::rt;

    macro_rules! cases {
        ($a:ty, $b:ty, $t:ident) => {
            assert_eq!(rt::<NDivRem<$a, $b>, $t>(), rt::<$a, $t>() % rt::<$b, $t>());

            assert_eq!(
                rt::<NDivQuot<$a, $b>, $t>(),
                rt::<$a, $t>() / rt::<$b, $t>()
            );
        };
    }

    #[test]
    fn ndiv() {
        cases!(N0, N1, u8);
        cases!(N0, N2, u8);
        cases!(N0, N3, u8);
        cases!(N0, N4, u8);
        cases!(N0, N5, u8);
        cases!(N0, N100, u8);

        cases!(N1, N1, u8);
        cases!(N2, N1, u8);
        cases!(N3, N1, u8);
        cases!(N4, N1, u8);
        cases!(N5, N1, u8);
        cases!(N100, N1, u8);

        cases!(N1, N2, u8);
        cases!(N2, N2, u8);
        cases!(N3, N2, u8);
        cases!(N4, N2, u8);
        cases!(N5, N2, u8);
        cases!(N6, N2, u8);

        cases!(N3, N69, u8);
        cases!(N420, N69, u16);
    }
}
