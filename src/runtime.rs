pub trait Runtime<R> {
    fn runtime() -> R;
}

#[must_use]
pub fn rt<From: ?Sized, To>() -> To
where
    From: Runtime<To>,
{
    From::runtime()
}
