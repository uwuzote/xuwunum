use super::{Nat, NS, NZ};
use crate::runtime::Runtime;

macro_rules! impl_runtime {
	($t:ident) => {
		impl Runtime<$t> for NZ {
			fn runtime() -> $t { 0 }
		}

		impl<P: ?Sized + Nat + Runtime<$t>> Runtime<$t> for NS<P> {
			fn runtime() -> $t { P::runtime() + 1 }
		}
	};

	($t1:ident $($t2:ident)+) => {
		impl_runtime!($t1);
		impl_runtime!($($t2)+);
	}
}

impl_runtime!(u8 i8 u16 i16 u32 i32 u64 i64 usize isize u128 i128);

#[cfg(test)]
mod test {
    use crate::nat::nums::*;

    macro_rules! gentest {
        ($t:ident, $ty:ident, $e:expr) => {
            assert_eq!(crate::runtime::rt::<$t, $ty>(), $e);
        };
    }

    #[test]
    fn nat_runtime() {
        gentest!(N0, u8, 0);
        gentest!(N1, u8, 1);
        gentest!(N2, u8, 2);
        gentest!(N8, u8, 8);
        gentest!(N69, i128, 69);
        gentest!(N100, u8, 100);
        gentest!(N420, i128, 420);
        gentest!(N1000, u16, 1000);
    }

    #[test]
    #[should_panic]
    fn nat_runtime_overflow() {
        let _ = crate::runtime::rt::<N300, u8>();
    }
}
