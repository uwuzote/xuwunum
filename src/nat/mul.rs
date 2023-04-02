use super::{NAdd, Nat, NatAdd, NS, NZ};

pub type NMul<Lhs, Rhs> = <Lhs as NatMul<Rhs>>::Result;

pub trait NatMul<Rhs: Nat + ?Sized>: Nat {
    type Result: Nat + ?Sized;
}

/// `X * 0 => 0`
impl<X: Nat + ?Sized> NatMul<NZ> for X {
    type Result = NZ;
}

/// `A * (B +1) => A + (A * B)`
impl<A: Nat + ?Sized, B: Nat + ?Sized> NatMul<NS<B>> for A
where
    A: NatMul<B>,
    A: NatAdd<NMul<A, B>>,
{
    type Result = NAdd<A, NMul<A, B>>;
}

#[cfg(test)]
mod tests {
    use super::NMul;
    use crate::nat::nums::*;
    use crate::runtime::rt;

    macro_rules! case {
        ($a:ident, $b:ident, $t:ident) => {
            assert_eq!(rt::<NMul<$a, $b>, $t>(), rt::<$a, $t>() * rt::<$b, $t>());
        };
    }

    #[test]
    fn nmul() {
        case!(N0, N0, u8);
        case!(N200, N0, i32);
        case!(N0, N200, i32);
        case!(N10, N10, i8);
        case!(N500, N2, isize);
        case!(N2, N500, i128);
        case!(N5, N3, u128);
        case!(N4, N50, u16);
    }

    #[test]
    fn nmul_just_to_be_sure() {
        let it = rt::<NMul<N20, N50>, u32>();

        assert_eq!(it, 1000);
    }
}
