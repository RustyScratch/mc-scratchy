//! Building a stack

use std::marker::PhantomData;

use sb_itchy::{
    block::{BlockBuilder, BlockNormalBuilder},
    stack::StackBuilder as ItchyStackBuilder,
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
    stack_builder: ItchyStackBuilder,
    /// Start/Top of the stack marker if it's stackable or not
    start: PhantomData<S>,
    /// End/Bottom of the stack marker if it's stackable or not
    end: PhantomData<E>,
}

mod test {
    use crate as sb_scratchy;
    fn test() {
        use sb_scratchy::blocks::*;
        let mut stack = when_flag_clicked();
        stack = stack.next(move_steps(10)).next(wait(10.0));
        stack = stack.next(say(
            "Connection Terminated. I'm sorry to interrupt you Elizabeth,",
        ));
        // Do something with stack
    }
}

impl<S, E> TypedStackBuilder<S, E> {
    /// Start building stack
    ///
    /// Though, you probably want to use predefined blocks in [`crate::blocks`].
    ///
    /// # Examples
    ///
    /// See [`crate::scripting::arg`] for more detail about block input.
    /// ```
    /// use sb_itchy::block::BlockNormalBuilder;
    /// use sb_scratchy::scripting::{
    ///     arg::{IntoInput, Number},
    ///     stack::{StackBlock, TypedStackBuilder},
    /// };
    ///
    /// // Defining new block with no argument
    /// fn some_block() -> StackBlock {
    ///     TypedStackBuilder::start(BlockNormalBuilder::new("anOpCodeOfYourBlock"))
    /// }
    ///
    /// // Defining new block with some argument
    /// fn some_block_with_arg(sec: impl IntoInput<Number>) -> StackBlock {
    ///     let mut b = BlockNormalBuilder::new("anotherOpCodeOfYourBlock");
    ///     b.add_input("SEC", sec.into_input());
    ///     TypedStackBuilder::start(b)
    /// }
    ///
    /// // Or
    /// fn some_block_with_arg_2<Sec>(sec: Sec) -> StackBlock
    /// where
    ///     Sec: IntoInput<Number>,
    /// {
    ///     let mut b = BlockNormalBuilder::new("anotherOpCodeOfYourBlock");
    ///     b.add_input("SEC", sec.into_input());
    ///     TypedStackBuilder::start(b)
    /// }
    /// ```
    pub fn start(block_builder: BlockNormalBuilder) -> TypedStackBuilder<S, E> {
        TypedStackBuilder {
            stack_builder: ItchyStackBuilder::start(block_builder),
            start: PhantomData,
            end: PhantomData,
        }
    }

    pub fn into_untyped(self) -> ItchyStackBuilder {
        self.stack_builder
    }

    pub fn start_with_capacity(
        capacity: usize,
        block_builder: BlockBuilder,
    ) -> TypedStackBuilder<S, E> {
        TypedStackBuilder {
            stack_builder: ItchyStackBuilder::start_with_capacity(capacity, block_builder),
            start: PhantomData,
            end: PhantomData,
        }
    }

    /// # Safety
    /// I mark this unsafe first because I'm unsure what kind of dark power incorrectly typed boys are packing.
    pub unsafe fn assume_typed(stack_builder: ItchyStackBuilder) -> TypedStackBuilder<S, E> {
        TypedStackBuilder {
            stack_builder,
            start: PhantomData,
            end: PhantomData,
        }
    }
}

impl<S> TypedStackBuilder<S, StackableSide> {
    /// Adding block to end of the stack
    ///
    /// # Examples
    ///
    /// ```
    /// ```
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
