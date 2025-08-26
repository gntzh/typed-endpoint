use std::marker::PhantomData;

use sealed::Sealed;
mod sealed {
    pub trait Sealed {}
}

/// Type-level Linked List
pub trait TypeList: Sealed {
    const IS_EMPTY: bool;
}
impl TypeList for Nil {
    const IS_EMPTY: bool = true;
}
impl<H, T: TypeList> TypeList for Cons<H, T> {
    const IS_EMPTY: bool = false;
}

impl Sealed for Nil {}
impl<H, T: TypeList> Sealed for Cons<H, T> {}

pub struct Nil;
pub struct Cons<H, T: TypeList>(PhantomData<(H, T)>);

pub type Last<L> = <L as NonEmpty>::Last;
pub type Init<L> = <L as NonEmpty>::Init;

/// Empty Constraint Trait
pub trait Empty: TypeList {}
impl Empty for Nil {}

/// NonEmpty Constraint Trait
pub trait NonEmpty: TypeList {
    type First;
    type Rest: TypeList;
    type Last;
    type Init: TypeList;
}
impl<H, T: TypeList> NonEmpty for Cons<H, T>
where
    Self: SplitLast,
{
    type First = H;
    type Rest = T;
    type Last = <Self as SplitLast>::Last;
    type Init = <Self as SplitLast>::Init;
}

pub trait SplitLast: TypeList {
    type Last;
    type Init: TypeList;
}

impl<H> SplitLast for Cons<H, Nil> {
    type Last = H;
    type Init = Nil;
}

impl<H, T> SplitLast for Cons<H, T>
where
    T: SplitLast + TypeList,
{
    type Last = T::Last;
    type Init = Cons<H, T::Init>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use typenum::assert_type_eq;

    #[test]
    fn test() {
        type OneList = Cons<u8, Cons<u16, Cons<u32, Nil>>>;
        assert_type_eq!(Last<OneList>, u32);
        assert_type_eq!(Init<OneList>, Cons<u8, Cons<u16, Nil>>);
    }
}
