//! Argument/Input in Scratch block

use std::marker::PhantomData;

use sb_itchy::prelude::{BlockFieldBuilder, BlockInputBuilder, BlockInputValue, FieldKind};

use crate::all_derive;
use crate::scripting::stack::{StackableSide, TypedStackBuilder, UnstackableSide};

/// Marker for [`IntoInput`] that this can be insert into input.
#[derive(Debug, Clone, PartialEq)]
pub struct Reporter<T, S, E>(pub TypedStackBuilder<S, E>, pub PhantomData<T>);
/// Just the classic Scratch repoter
pub type JustReporter<T> = Reporter<T, UnstackableSide, UnstackableSide>;
/// This is a kind of reporter that you can select item from a menu.
pub type MenuReporter = JustReporter<Text>;

impl<T, S, E> Reporter<T, S, E> {
    pub fn new(typed_stack_builder: TypedStackBuilder<S, E>) -> Reporter<T, S, E> {
        Reporter(typed_stack_builder, PhantomData)
    }
}

impl<T, S, E> From<TypedStackBuilder<S, E>> for Reporter<T, S, E> {
    fn from(stb: TypedStackBuilder<S, E>) -> Self {
        Reporter::new(stb)
    }
}

all_derive! {
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]

    pub struct Number;
    pub struct PositiveNumber;
    pub struct PositiveInteger;
    pub struct Integer;
    pub struct Float;
    pub struct Angle;
    pub struct Color;
    pub struct Text;
    pub struct Bool;
    /// Could be text or number
    pub struct Value;

    pub struct Stack;

    /// this is for IntoField when there's no id field
    pub struct NoRef;
    /// this is for IntoField when there's an id field but i've never seen it has id
    pub struct NoRefMaybe;

    pub struct Broadcast;
    pub struct Variable;
    pub struct List;

}

// Input =========================================================================
pub trait IntoInput<T> {
    fn into_input(self) -> BlockInputBuilder;
}

impl<T, S, E> IntoInput<T> for Reporter<T, S, E> {
    fn into_input(self) -> BlockInputBuilder {
        BlockInputBuilder::stack(self.0.into_untyped())
    }
}

macro_rules! into_arg_impl {
    ($($arg:ty => $variant:ident => $from_ty:ty),*) => {
        $(
            impl IntoInput<$arg> for $from_ty {
                fn into_input(self) -> BlockInputBuilder {
                    BlockInputBuilder::value(BlockInputValue::$variant { value: self.to_string().into() })
               }
            }
        )*
    }
}

into_arg_impl! {
    // arg => variant => from
    Number => Number => i64,
    Number => Number => i32,
    Number => Number => i16,
    Number => Number => isize,
    Number => Number => u64,
    Number => Number => u32,
    Number => Number => u16,
    Number => Number => u8,
    Number => Number => usize,
    Number => Number => f64,
    Number => Number => f32,

    PositiveNumber => PositiveNumber => f64,
    PositiveNumber => PositiveNumber => f32,
    PositiveNumber => PositiveNumber => u64,
    PositiveNumber => PositiveNumber => u32,
    PositiveNumber => PositiveNumber => u16,
    PositiveNumber => PositiveNumber => u8,
    PositiveNumber => PositiveNumber => usize,

    PositiveInteger => PositiveInteger => u64,
    PositiveInteger => PositiveInteger => u32,
    PositiveInteger => PositiveInteger => u16,
    PositiveInteger => PositiveInteger => u8,
    PositiveInteger => PositiveInteger => usize,

    Integer => Integer => i64,
    Integer => Integer => i32,
    Integer => Integer => i16,
    Integer => Integer => isize,
    Integer => Integer => u64,
    Integer => Integer => u32,
    Integer => Integer => u16,
    Integer => Integer => u8,
    Integer => Integer => usize,

    Float => Number => f64,
    Float => Number => f32,

    Angle => Angle => i64,
    Angle => Angle => i32,
    Angle => Angle => i16,
    Angle => Angle => isize,
    Angle => Angle => u64,
    Angle => Angle => u32,
    Angle => Angle => u16,
    Angle => Angle => u8,
    Angle => Angle => usize,
    Angle => Angle => f64,
    Angle => Angle => f32,

    Text => String => String,

    Value => Number => i64,
    Value => Number => i32,
    Value => Number => i16,
    Value => Number => isize,
    Value => Number => u64,
    Value => Number => u32,
    Value => Number => u16,
    Value => Number => u8,
    Value => Number => usize,
    Value => Number => f64,
    Value => Number => f32,
    Value => String => String
}

impl IntoInput<Text> for &str {
    fn into_input(self) -> BlockInputBuilder {
        BlockInputBuilder::value(BlockInputValue::String {
            value: self.to_owned().into(),
        })
    }
}

impl IntoInput<Value> for &str {
    fn into_input(self) -> BlockInputBuilder {
        BlockInputBuilder::value(BlockInputValue::String {
            value: self.to_owned().into(),
        })
    }
}

impl<E> IntoInput<Stack> for TypedStackBuilder<StackableSide, E> {
    fn into_input(self) -> BlockInputBuilder {
        BlockInputBuilder::stack(self.into_untyped())
    }
}

// Field ====================================================================
pub trait IntoField<T = NoRefMaybe> {
    fn into_field(self) -> BlockFieldBuilder;
}

impl<S: Into<String>> IntoField for S {
    fn into_field(self) -> BlockFieldBuilder {
        BlockFieldBuilder::new(self.into())
    }
}

impl<S: Into<String>> IntoField<Broadcast> for S {
    fn into_field(self) -> BlockFieldBuilder {
        BlockFieldBuilder::new_with_kind(self.into(), FieldKind::Broadcast)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct GlobalVar<S: Into<String>>(pub S);
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SpriteVar<S: Into<String>>(pub S);
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct GlobalList<S: Into<String>>(pub S);
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SpriteList<S: Into<String>>(pub S);

impl<S: Into<String>> IntoField<Variable> for GlobalVar<S> {
    fn into_field(self) -> BlockFieldBuilder {
        BlockFieldBuilder::new_with_kind(self.0.into(), FieldKind::GlobalVariable)
    }
}

impl<S: Into<String>> IntoField<Variable> for SpriteVar<S> {
    fn into_field(self) -> BlockFieldBuilder {
        BlockFieldBuilder::new_with_kind(self.0.into(), FieldKind::SpriteVariable)
    }
}

impl<S: Into<String>> IntoField<List> for GlobalList<S> {
    fn into_field(self) -> BlockFieldBuilder {
        BlockFieldBuilder::new_with_kind(self.0.into(), FieldKind::GlobalList)
    }
}

impl<S: Into<String>> IntoField<List> for SpriteList<S> {
    fn into_field(self) -> BlockFieldBuilder {
        BlockFieldBuilder::new_with_kind(self.0.into(), FieldKind::SpriteList)
    }
}
