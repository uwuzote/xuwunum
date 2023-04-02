use core::marker::PhantomData;

pub struct NZ;
pub struct NS<P: ?Sized + Nat>(pub PhantomData<P>);

pub trait Nat {}

impl Nat for NZ {}
impl<P: ?Sized + Nat> Nat for NS<P> {}
