use super::{Nat, NS, NZ};

pub type NAdd<Lhs, Rhs> = <Lhs as NatAdd<Rhs>>::Result;

pub trait NatAdd<Rhs: Nat + ?Sized>: Nat {
    type Result: Nat + ?Sized;
}

/// X + 0 => X
impl<X: ?Sized + Nat> NatAdd<NZ> for X {
    type Result = X;
}

/// A + (B +1) => (A + B) +1
impl<A: Nat + ?Sized, B: Nat + ?Sized> NatAdd<NS<B>> for A
where
    A: NatAdd<B>,
{
    type Result = NS<NAdd<A, B>>;
}

#[cfg(test)]
mod tests {
    use super::NAdd;
    use crate::nat::nums::*;
    use crate::runtime::rt;

    macro_rules! test {
        ($a:ident, $b:ident, $t:ident) => {
            assert_eq!(rt::<$a, $t>() + rt::<$b, $t>(), rt::<NAdd<$a, $b>, $t>());
        };
    }

    #[test]
    fn nadd() {
        test!(N0, N0, u8);
        test!(N42, N0, u8);
        test!(N0, N42, u8);
        test!(N20, N30, u8);
        test!(N100, N500, u16);
        test!(N18, N49, isize);
        test!(N18, N79, usize);
        test!(N18, N49, isize);
        test!(N18, N49, isize);
    }

    #[test]
    fn nadd_just_to_be_sure() {
        let it = rt::<NAdd<N80, N123>, i128>();

        assert_eq!(it, 203);
    }
}
