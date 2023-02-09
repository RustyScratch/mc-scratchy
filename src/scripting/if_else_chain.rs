//! Create flat if-else chain
//!
//! # Explaination
//!
//! In Scratch you'd sometimes normally create multiple if-else a little like this:
//! ```txt
//! if <cond> {
//!     
//! } else {
//!     if <cond> else {
//!         
//!     } else {
//!         
//!     }
//! }
//! ```
//!
//! This builder can reduce it to something a little like this:
//! ```txt
//! if <cond> {
//!     
//! } else if <cond> {
//!     
//! } else {
//!     
//! }
//! ```
//!
//! # Usage
//!
//! Without macro:
//! ```
//! # use sb_scratchy::blocks::{move_steps, turn_left, say, equals};
//! # use sb_scratchy::if_else_chain::if_;
//! // This uses the shortcut
//! if_(equals(1, 0), Some(
//!     move_steps(10)
//! ))
//! .else_if(equals(1, 1), Some(
//!     turn_left(20)
//! ))
//! .else_(Some(
//!     say("wassup")
//! ));
//! ```

use std::marker::PhantomData;

use crate::scripting::{arg::*, stack::*};
use sb_itchy::blocks::{if_ as if_block, if_else as if_else_block};
use sb_itchy::prelude::BlockInputBuilder;

/// State/Marker for [`IfElseChainBuilder`] that it's building.
pub struct Building;
/// State/Marker for [`IfElseChainBuilder`] that the process has ended.
pub struct End;

/// Shortcut to [`IfElseChainBuilder::if_`].
pub fn if_(
    cond: impl IntoInput<Bool>,
    then: Option<impl IntoInput<Stack>>,
) -> IfElseChainBuilder<Building> {
    IfElseChainBuilder::<Building>::if_(cond, then)
}

#[derive(Debug, Clone, PartialEq)]
/// Builder to create flat if else chain.
///
/// See top module documentation for usage.
pub struct IfElseChainBuilder<S> {
    if_: (BlockInputBuilder, Option<BlockInputBuilder>),
    else_ifs: Vec<(BlockInputBuilder, Option<BlockInputBuilder>)>,
    else_: Option<Option<BlockInputBuilder>>,
    marker: PhantomData<S>,
}

impl IfElseChainBuilder<Building> {
    /// Create else if
    pub fn else_if(
        mut self,
        cond: impl IntoInput<Bool>,
        then: Option<impl IntoInput<Stack>>,
    ) -> IfElseChainBuilder<Building> {
        self.else_ifs
            .push((cond.into_input(), then.map(IntoInput::<Stack>::into_input)));
        self
    }

    /// Create else. After this you cannot add anymore statement
    pub fn else_(mut self, else_: Option<impl IntoInput<Stack>>) -> IfElseChainBuilder<End> {
        self.else_ = Some(else_.map(IntoInput::<Stack>::into_input));
        let IfElseChainBuilder {
            if_,
            else_ifs,
            else_,
            marker: _,
        } = self;
        IfElseChainBuilder {
            if_,
            else_ifs,
            else_,
            marker: PhantomData,
        }
    }
}

impl<S> IfElseChainBuilder<S> {
    /// Create if
    pub fn if_(
        cond: impl IntoInput<Bool>,
        then: Option<impl IntoInput<Stack>>,
    ) -> IfElseChainBuilder<Building> {
        IfElseChainBuilder {
            if_: (cond.into_input(), then.map(|s| s.into_input())),
            else_ifs: vec![],
            else_: None,
            marker: PhantomData,
        }
    }

    /// End chain
    pub fn end(self) -> StackBlock {
        let IfElseChainBuilder {
            if_,
            else_ifs,
            else_,
            marker: _,
        } = self;
        // not very readable - fix later
        let b = match (else_ifs.len(), else_) {
            (0, None) => if_block(if_.0, if_.1),
            (0, Some(else_)) => if_else_block(if_.0, if_.1, else_),
            (_, None) => {
                // building from inside out
                let mut else_ifs_rev_iter = else_ifs.into_iter().rev();
                let last_else_if = {
                    let (a, substack) = else_ifs_rev_iter.next().unwrap();
                    if_block(a, substack)
                };
                match else_ifs_rev_iter.next() {
                    Some((parent_a, parent_subtack)) => {
                        let mut prev_parent = if_else_block(
                            parent_a,
                            parent_subtack,
                            Some(BlockInputBuilder::stack(last_else_if)),
                        );
                        for (parent_a, parent_substack) in else_ifs_rev_iter {
                            prev_parent = if_else_block(
                                parent_a,
                                parent_substack,
                                Some(BlockInputBuilder::stack(prev_parent)),
                            );
                        }
                        if_else_block(if_.0, if_.1, Some(BlockInputBuilder::stack(prev_parent)))
                    }
                    None => {
                        if_else_block(if_.0, if_.1, Some(BlockInputBuilder::stack(last_else_if)))
                    }
                }
            }
            (_, Some(else_)) => {
                // building from inside out
                let mut else_ifs_rev_iter = else_ifs.into_iter().rev();
                let last_else = else_;
                match else_ifs_rev_iter.next() {
                    Some((parent_a, parent_subtack)) => {
                        let mut prev_parent = if_else_block(parent_a, parent_subtack, last_else);
                        for (parent_a, parent_substack) in else_ifs_rev_iter {
                            prev_parent = if_else_block(
                                parent_a,
                                parent_substack,
                                Some(BlockInputBuilder::stack(prev_parent)),
                            );
                        }
                        if_else_block(if_.0, if_.1, Some(BlockInputBuilder::stack(prev_parent)))
                    }
                    None => if_else_block(if_.0, if_.1, last_else),
                }
            }
        };
        TypedStackBuilder::assume_typed(b)
    }
}
