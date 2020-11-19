pub enum CowMut<'a, T>
where
    T: ?Sized + 'a,
{
    Owned(Box<T>),
    Borrowed(&'a mut T),
}

impl<'a, T> From<Box<T>> for CowMut<'a, T> {
    fn from(boxed: Box<T>) -> CowMut<'a, T> {
        CowMut::Owned(boxed)
    }
}

impl<'a, T> From<&'a mut T> for CowMut<'a, T> {
    fn from(borrowed: &'a mut T) -> CowMut<'a, T> {
        CowMut::Borrowed(borrowed)
    }
}

impl<'a, T> CowMut<'a, T>
where
    T: ?Sized,
{
    pub fn to_mut(&mut self) -> &mut T {
        match self {
            CowMut::Owned(owned) => owned,
            CowMut::Borrowed(borrowed) => borrowed,
        }
    }
}
