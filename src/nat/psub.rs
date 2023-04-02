use super::pan::{NatPan, NP, NW};
use super::{Nat, NS, NZ};

/// Panicing natural sub
pub type NPSub<Lhs, Rhs> = <Lhs as NatPSub<Rhs>>::Result;

pub trait NatPSub<Rhs: Nat + ?Sized>: Nat {
    type Result: NatPan + ?Sized;
}

/// `0 -! 0 => @(0)`
impl NatPSub<NZ> for NZ {
    type Result = NW<NZ>;
}

/// `0 -! (X +1) => !`
impl<X: Nat + ?Sized> NatPSub<NS<X>> for NZ {
    type Result = NP;
}

/// `(X +1) -! 0 => @(X +1)`
impl<X: Nat + ?Sized> NatPSub<NZ> for NS<X> {
    type Result = NW<NS<X>>;
}

/// `(A +1) -! (B +1) => A -! B`
impl<A: Nat + ?Sized, B: Nat + ?Sized> NatPSub<NS<B>> for NS<A>
where
    A: NatPSub<B>,
{
    type Result = NPSub<A, B>;
}

#[cfg(test)]
mod tests {
    use super::NPSub;
    use crate::nat::nums::*;
    use crate::runtime::rt;

    macro_rules! case {
        ($a:ident, $b:ident, $t:ident) => {
            assert_eq!(
                rt::<NPSub<$a, $b>, Option<$t>>(),
                Some(rt::<$a, $t>() - rt::<$b, $t>())
            );
        };
        ($a:ident, $b:ident, $t:ident, $r:expr) => {
            assert_eq!(rt::<NPSub<$a, $b>, Option<$t>>(), $r);
        };
    }

    #[test]
    fn npsub_normal() {
        case!(N80, N20, u8);
        case!(N600, N1, u16);
        case!(N300, N15, isize);
        case!(N420, N69, i64);
    }

    #[test]
    fn npsub_panic() {
        case!(N20, N80, u8, None);
        case!(N1, N600, u16, None);
        case!(N15, N300, isize, None);
        case!(N69, N420, i64, None);
    }

    #[test]
    fn npsub_just_to_be_sure() {
        let it = rt::<NPSub<N690, N420>, Option<i32>>();
        assert_eq!(it, Some(270i32));

        let it = rt::<NPSub<N420, N690>, Option<i8>>();
        assert_eq!(it, None);
    }
}
