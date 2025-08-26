mod type_list;

pub trait ResourceType {}

pub struct TopResource;
impl ResourceType for TopResource {}

pub struct SubResource<P: Endpoint> {
    _parent: std::marker::PhantomData<P>,
}

impl<P: Endpoint> ResourceType for SubResource<P> {}

pub trait Endpoint {
    const URL_PATH_SEGMENT: &'static str;
    type Parent: ParentEndpoint;
}

pub trait ParentEndpoint {}
pub struct Root;
impl ParentEndpoint for Root {}
impl<E: Resource> ParentEndpoint for E {}

pub trait Resource: Endpoint {
    type Id: ToString;
}
use path_builder::{MakeChain, PathBuilder};
pub trait EndpointExt: Endpoint + MakeChain {
    fn path<'a>() -> PathBuilder<'a, Self, Self::Chain>
    where
        Self: Sized,
    {
        PathBuilder::new()
    }
}
impl<E> EndpointExt for E where E: Endpoint + MakeChain {}

pub trait Action: Endpoint {}

mod path_builder {
    use std::borrow::Cow;
    use std::marker::PhantomData;

    use super::{Action, Endpoint, Resource, Root};
    use crate::type_list::{Cons, Empty, Init, Last, Nil, NonEmpty, TypeList};

    type NextId<L> = <Last<L> as Resource>::Id;

    pub trait MakeChain {
        type Chain: TypeList;
    }

    impl MakeChain for Root {
        type Chain = Nil;
    }

    impl<E> MakeChain for E
    where
        E: Endpoint,
        E::Parent: MakeChain,
    {
        type Chain = Cons<E, <E::Parent as MakeChain>::Chain>;
    }

    pub struct PathBuilder<'a, E: Endpoint, S: State> {
        segments: Vec<Cow<'a, str>>,
        _endpoint: PhantomData<E>,
        _state: PhantomData<S>,
    }
    trait State {}
    impl State for () {}
    impl<T: TypeList> State for T {}

    impl<'a, E> PathBuilder<'a, E, ()>
    where
        E: Endpoint + MakeChain,
    {
        pub fn new() -> PathBuilder<'a, E, E::Chain> {
            PathBuilder {
                segments: Vec::new(),
                _endpoint: PhantomData,
                _state: PhantomData,
            }
        }
    }

    impl<'a, E: Endpoint, S> PathBuilder<'a, E, S>
    where
        S: NonEmpty,
        Last<S>: Resource,
        Init<S>: NonEmpty,
    {
        #[inline]
        pub fn bind(mut self, id: NextId<S>) -> PathBuilder<'a, E, Init<S>> {
            self.segments
                .push(Cow::Borrowed(Last::<S>::URL_PATH_SEGMENT));
            self.segments.push(Cow::Owned(id.to_string()));
            PathBuilder {
                segments: self.segments,
                _endpoint: PhantomData,
                _state: PhantomData,
            }
        }
    }

    impl<'a, E: Endpoint, S> PathBuilder<'a, E, S>
    where
        S: NonEmpty,
        Last<S>: Resource,
        Init<S>: Empty,
    {
        #[inline]
        pub fn single(mut self, id: NextId<S>) -> PathBuilder<'a, E, Nil> {
            self.segments
                .push(Cow::Borrowed(Last::<S>::URL_PATH_SEGMENT));
            self.segments.push(Cow::Owned(id.to_string()));
            PathBuilder {
                segments: self.segments,
                _endpoint: PhantomData,
                _state: PhantomData,
            }
        }

        #[inline]
        pub fn list(mut self) -> PathBuilder<'a, E, Nil> {
            self.segments
                .push(Cow::Borrowed(Last::<S>::URL_PATH_SEGMENT));
            PathBuilder {
                segments: self.segments,
                _endpoint: PhantomData,
                _state: PhantomData,
            }
        }
    }

    impl<'a, E: Endpoint, S> PathBuilder<'a, E, S>
    where
        S: NonEmpty,
        Last<S>: Action,
        Init<S>: Empty,
    {
        #[inline]
        pub fn action(mut self) -> PathBuilder<'a, E, Nil> {
            self.segments
                .push(Cow::Borrowed(Last::<S>::URL_PATH_SEGMENT));
            PathBuilder {
                segments: self.segments,
                _endpoint: PhantomData,
                _state: PhantomData,
            }
        }
    }

    impl<'a, E: Endpoint> PathBuilder<'a, E, Nil> {
        #[inline]
        pub fn build(&self) -> String {
            self.segments.join("/")
        }
    }
}
