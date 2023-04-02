use super::{NatPan, NW, NP};
use crate::nat::Nat;

pub type NSwP<Expr, BrP, BrW> = <Expr as NatSwP<BrP, BrW>>::Result;

pub trait NatSwP<BrP: Nat + ?Sized, BrW: Nat + ?Sized>: NatPan {
	type Result: Nat + ?Sized;
}

/// `! !? A : B => A`
impl<A: Nat + ?Sized, B: Nat + ?Sized> NatSwP<A, B> for NP {
	type Result = A;
}

/// `@(X) !? A : B => B`
impl<X: Nat + ?Sized, A: Nat + ?Sized, B: Nat + ?Sized> NatSwP<A, B> for NW<X> {
	type Result = B;
}

#[cfg(test)]
mod tests {
	use super::NSwP;
    use crate::nat::nums::*;
	use crate::nat::pan::{NP, NW};
    use crate::runtime::rt;

    macro_rules! case {
        ($a:ty, $br0:ident, $br1:ident, $t:ident) => {
            assert_eq!(
                rt::<NSwP<$a, $br0, $br1>, $t>(),
                if rt::<$a, Option<$t>>() == None {
                    rt::<$br0, $t>()
                } else {
                    rt::<$br1, $t>()
                }
            );
        };
    }

    #[test]
    fn nswp() {
		case!(NP, N100, N200, usize);
        case!(NW<N0>, N100, N200, i16);
        case!(NW<N1>, N100, N200, i128);
        case!(NW<N9>, N100, N200, u64);
    }

    #[test]
    fn nswp_just_to_be_sure() {
        let it = rt::<NSwP<NP, N69, N420>, u16>();
        assert_eq!(it, 69);

        let it = rt::<NSwP<NW<N10>, N69, N420>, u16>();
        assert_eq!(it, 420);

        let it = rt::<NSwP<NW<N0>, N69, N420>, u16>();
        assert_eq!(it, 420);
    }
}
