use crate::nat::Nat;
use core::marker::PhantomData;

pub struct NP;
pub struct NW<N: Nat + ?Sized>(pub PhantomData<N>);

pub trait NatPan {}

impl NatPan for NP {}
impl<X: Nat + ?Sized> NatPan for NW<X> {}
