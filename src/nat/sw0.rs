use super::{Nat, NS, NZ};

pub type NSw0<Expr, Br0, Br1> = <Expr as NatSw0<Br0, Br1>>::Result;

pub trait NatSw0<Br0: Nat + ?Sized, Br1: Nat + ?Sized>: Nat {
    type Result: Nat + ?Sized;
}

/// `0 z? Br0 : Br1 => Br0`
impl<Br0: Nat + ?Sized, Br1: Nat + ?Sized> NatSw0<Br0, Br1> for NZ {
    type Result = Br0;
}

/// `(X +1) z? Br0 : Br1 => Br1`
impl<Br0: Nat + ?Sized, Br1: Nat + ?Sized, X: Nat + ?Sized> NatSw0<Br0, Br1> for NS<X> {
    type Result = Br1;
}

#[cfg(test)]
mod tests {
    use super::NSw0;
    use crate::nat::nums::*;
    use crate::runtime::rt;

    macro_rules! case {
        ($a:ident, $br0:ident, $br1:ident, $t:ident) => {
            assert_eq!(
                rt::<NSw0<$a, $br0, $br1>, $t>(),
                if rt::<$a, $t>() == 0 {
                    rt::<$br0, $t>()
                } else {
                    rt::<$br1, $t>()
                }
            );
        };
    }

    #[test]
    fn nsw0() {
        case!(N0, N100, N200, isize);
        case!(N1, N100, N200, i128);
        case!(N0, N0, N1000, u32);
        case!(N123, N34, N874, u64);
    }

    #[test]
    fn nsw0_just_to_be_sure() {
        let it = rt::<NSw0<N0, N69, N420>, u16>();
        assert_eq!(it, 69);

        let it = rt::<NSw0<N197, N69, N420>, u16>();
        assert_eq!(it, 420);
    }
}
