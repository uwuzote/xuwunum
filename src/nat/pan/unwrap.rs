use super::{NatPan, NP, NW};
use crate::nat::Nat;

pub type NPUnwrap<Expr, Fall> = <Expr as NatPanUnwrap<Fall>>::Result;

pub trait NatPanUnwrap<Fall: Nat + ?Sized>: NatPan {
    type Result: Nat + ?Sized;
}

/// `! !? X => X`
impl<X: Nat + ?Sized> NatPanUnwrap<X> for NP {
    type Result = X;
}

/// `@(A) !? B => A`
impl<A: Nat + ?Sized, B: Nat + ?Sized> NatPanUnwrap<B> for NW<A> {
    type Result = A;
}

#[cfg(test)]
mod tests {
    use super::NPUnwrap;
    use crate::nat::nums::*;
    use crate::nat::pan::{NP, NW};
    use crate::runtime::rt;

    #[test]
    fn npunwrap() {
        assert_eq!(rt::<NPUnwrap<NP, N69>, i128>(), 69i128);

        assert_eq!(rt::<NPUnwrap<NP, N0>, u8>(), 0u8);

        assert_eq!(rt::<NPUnwrap<NP, N420>, usize>(), 420usize);

        assert_eq!(rt::<NPUnwrap<NW<N69>, N420>, u16>(), 69u16);

        assert_eq!(rt::<NPUnwrap<NW<N420>, N69>, i32>(), 420i32);

        assert_eq!(rt::<NPUnwrap<NW<N999>, N0>, i64>(), 999i64);
    }
}
