//! Building a stack

use std::marker::PhantomData;

use sb_itchy::{
    block::{BlockBuilder, BlockNormalBuilder, BlockVarListBuilder},
    stack::StackBuilder,
};

/// State/Marker for [`TypedStackBuilder`] that this side can be stacked.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct StackableSide;
/// State/Marker for [`TypedStackBuilder`] that this side cannot be stacked.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct UnstackableSide;

/// This stack/block can be be stack on top of another stack/block but otherwise cannot be done.
pub type HatBlock = TypedStackBuilder<UnstackableSide, StackableSide>;
/// This stack/block can be be stack below another stack/block but otherwise cannot be done.
pub type CapBlock = TypedStackBuilder<StackableSide, UnstackableSide>;
/// This stack/block can be be stack top/below another stack/block
pub type StackBlock = TypedStackBuilder<StackableSide, StackableSide>;

#[derive(Debug, Clone, PartialEq)]
pub struct TypedStackBuilder<S, E> {
    stack_builder: StackBuilder,
    start: PhantomData<S>,
    end: PhantomData<E>,
}

impl<S, E> TypedStackBuilder<S, E> {
    /// Start a stack
    ///
    /// Though, you probably want to use blocks in [`crate::blocks`] to create predefined block.
    pub fn start(block_builder: BlockNormalBuilder) -> TypedStackBuilder<S, E> {
        TypedStackBuilder {
            stack_builder: StackBuilder::start(block_builder),
            start: PhantomData,
            end: PhantomData,
        }
    }

    /// Start a varaible or list reporter block
    pub fn start_varlist(
        block_builder: BlockVarListBuilder,
    ) -> TypedStackBuilder<UnstackableSide, UnstackableSide> {
        TypedStackBuilder {
            stack_builder: StackBuilder::start_varlist(block_builder),
            start: PhantomData,
            end: PhantomData,
        }
    }

    pub fn into_untyped(self) -> StackBuilder {
        self.stack_builder
    }

    pub fn start_with_capacity(
        capacity: usize,
        block_builder: BlockBuilder,
    ) -> TypedStackBuilder<S, E> {
        TypedStackBuilder {
            stack_builder: StackBuilder::start_with_capacity(capacity, block_builder),
            start: PhantomData,
            end: PhantomData,
        }
    }

    pub fn assume_typed(stack_builder: StackBuilder) -> TypedStackBuilder<S, E> {
        TypedStackBuilder {
            stack_builder,
            start: PhantomData,
            end: PhantomData,
        }
    }
}

impl<S> TypedStackBuilder<S, StackableSide> {
    pub fn next<NE>(
        self,
        next_stack: TypedStackBuilder<StackableSide, NE>,
    ) -> TypedStackBuilder<S, NE> {
        let stack = self.into_untyped();
        let next_stack = next_stack.into_untyped();
        TypedStackBuilder {
            stack_builder: stack.next(next_stack),
            start: PhantomData,
            end: PhantomData,
        }
    }
}
