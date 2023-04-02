use super::{Nat, NS, NZ};

/// Zeroing subtarction
pub type NZSub<Lhs, Rhs> = <Lhs as NatZSub<Rhs>>::Result;

/// See [`NZSub`]
pub trait NatZSub<Rhs: Nat + ?Sized>: Nat {
    type Result: Nat + ?Sized;
}

/// `0 -0 (X +1) => 0`
impl<X: Nat + ?Sized> NatZSub<NS<X>> for NZ {
    type Result = NZ;
}

/// `X -0 0 => X`
impl<X: Nat + ?Sized> NatZSub<NZ> for X {
    type Result = X;
}

/// `(A +1) -0 (B +1) => A -0 B`
impl<A: Nat + ?Sized, B: Nat + ?Sized> NatZSub<NS<B>> for NS<A>
where
    A: NatZSub<B>,
{
    type Result = NZSub<A, B>;
}

#[cfg(test)]
mod tests {
    use super::NZSub;
    use crate::nat::nums::*;
    use crate::runtime::rt;

    macro_rules! case {
        ($a:ident, $b:ident, $t:ident) => {
            assert_eq!(rt::<NZSub<$a, $b>, $t>(), rt::<$a, $t>() - rt::<$b, $t>(),);
        };
        ($a:ident, $b:ident, $t:ident, $r:expr) => {
            assert_eq!(rt::<NZSub<$a, $b>, $t>(), $r);
        };
    }

    #[test]
    fn nzsub_normal() {
        case!(N80, N20, u8);
        case!(N600, N1, u16);
        case!(N300, N15, isize);
        case!(N420, N69, i64);
    }

    #[test]
    fn nzsub_zeroing() {
        case!(N20, N80, u8, 0);
        case!(N1, N600, u16, 0);
        case!(N15, N300, isize, 0);
        case!(N69, N420, i64, 0);
    }

    #[test]
    fn nzsub_just_to_be_sure() {
        let it = rt::<NZSub<N690, N420>, i32>();
        assert_eq!(it, 270i32);

        let it = rt::<NZSub<N420, N690>, i8>();
        assert_eq!(it, 0);
    }
}
