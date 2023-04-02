use super::{NP, NW};
use crate::nat::Nat;
use crate::runtime::Runtime;

impl<T> Runtime<Option<T>> for NP {
    fn runtime() -> Option<T> {
        None
    }
}

impl<T, X: Nat + ?Sized> Runtime<Option<T>> for NW<X>
where
    X: Runtime<T>,
{
    fn runtime() -> Option<T> {
        Some(X::runtime())
    }
}

#[cfg(test)]
mod tests {
    use crate::nat::nums::*;
    use crate::nat::pan::{NP, NW};
    use crate::runtime::rt;

    #[test]
    fn natpan_runtime() {
        assert_eq!(rt::<NP, Option<()>>(), None);
        assert_eq!(rt::<NP, Option<u128>>(), None);
        assert_eq!(rt::<NP, Option<f32>>(), None);

        assert_eq!(rt::<NW<N420>, Option<usize>>(), Some(420));
        assert_eq!(rt::<NW<N69>, Option<u8>>(), Some(69));
    }
}
