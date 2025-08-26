use std::marker::PhantomData;

use sealed::Sealed;
mod sealed {
    pub trait Sealed {}
}

/// Type-level Linked List
pub trait TypeList: Sealed {
    /// associated const equality is incomplete
    /// see issue [#92827](https://github.com/rust-lang/rust/issues/92827)
    /// for more information
    type IsEmpty: Bool;
}

impl TypeList for Nil {
    type IsEmpty = True;
}
impl<H, T: TypeList> TypeList for Cons<H, T> {
    type IsEmpty = False;
}

impl Sealed for Nil {}
impl<H, T: TypeList> Sealed for Cons<H, T> {}

pub struct Nil;
pub struct Cons<H, T: TypeList>(PhantomData<(H, T)>);
pub type First<L> = <L as NonEmpty>::First;
pub type Rest<L> = <L as NonEmpty>::Rest;
pub type Last<L> = <L as NonEmpty>::Last;
pub type Init<L> = <L as NonEmpty>::Init;
pub type IfEmpty<L, Then, Else> = <<L as TypeList>::IsEmpty as Bool>::If<Then, Else>;

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

impl<H> NonEmpty for Cons<H, Nil> {
    type First = H;
    type Rest = Nil;
    type Last = H;
    type Init = Nil;
}

impl<H, T: NonEmpty> NonEmpty for Cons<H, T> {
    type First = H;
    type Rest = T;
    type Last = <T as NonEmpty>::Last;
    type Init = Cons<H, <T as NonEmpty>::Init>;
}

trait Bool: Sealed {
    type If<Then, Else>;
}
pub struct True;
pub struct False;

impl Bool for True {
    type If<Then, Else> = Then;
}
impl Bool for False {
    type If<Then, Else> = Else;
}
impl Sealed for True {}
impl Sealed for False {}

#[cfg(test)]
mod tests {
    use typenum::assert_type_eq;

    use super::{Cons, First, IfEmpty, Init, Last, Nil, Rest};

    #[test]
    fn test() {
        type OneList = Cons<u8, Cons<u16, Cons<u32, Cons<u64, Nil>>>>;
        type EmptyList = Nil;

        assert_type_eq!(First<OneList>, u8);
        assert_type_eq!(Rest<OneList>, Cons<u16, Cons<u32, Cons<u64, Nil>>>);

        assert_type_eq!(Last<OneList>, u64);
        assert_type_eq!(Init<OneList>, Cons<u8, Cons<u16, Cons<u32, Nil>>>);

        assert_type_eq!(IfEmpty<OneList, u8, u16>, u16);
        assert_type_eq!(IfEmpty<EmptyList, u8, u16>, u8);
    }
}
