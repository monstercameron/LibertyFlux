//! The collaborators of the replay progress bar, as one trait.
//!
//! Every call the lifted methods make that is not itself lifted is one
//! method here: the object's base routines, the tokens and finalizers of
//! the teardown, the owned collaborators it releases, the update's
//! helpers, and the reads of the inner object. The production game
//! implements the trait over its own subsystems; tests implement it with
//! a recorder. Arguments are in the lifted form: handles instead of
//! object addresses, words instead of out-pointers (a helper that fills
//! words through out-pointers returns them), and the calling convention
//! is not visible here.

use lf_core::Handle32;

/// Tag for the handle to the progress bar object itself.
pub struct BarTag;
/// Tag for the handle to the inner object the bar drives.
pub struct InnerTag;
/// Tag for the handle to an owned collaborator the teardown releases.
pub struct OwnedTag;

/// Which part of the bar a call receives: the object itself, or the
/// sub-object that begins at its offset `0x1c`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Part {
    /// The progress bar object.
    Whole,
    /// The sub-object at offset `0x1c` of the progress bar object.
    Sub1c,
}

/// The calls the progress bar's lifted methods make on their collaborators.
///
/// Names describe the role each call plays in the method that makes it.
/// Where a call's result is consumed, the result is returned. Where the
/// original passes a constant that carries no meaning of its own, the
/// lifted call still passes it, so the value is part of the call's record.
pub trait ReplayBarWorld {
    /// The base teardown, run on the object itself first in the teardown.
    fn base_teardown(&mut self, bar: Handle32<BarTag>);
    /// Opens the teardown's token. The token's argument is a fixed cell of
    /// the game's data, which the production world supplies itself.
    fn open_token(&mut self) -> u32;
    /// Attaches the opened token to the teardown.
    fn attach_token(&mut self, token: u32);
    /// The destructor of an owned collaborator (its release).
    fn destruct(&mut self, owned: Handle32<OwnedTag>);
    /// Frees the storage of an owned collaborator after its destructor.
    fn free_owned(&mut self, owned: Handle32<OwnedTag>);
    /// Closes the token.
    fn close_token(&mut self, token: u32);
    /// First finalizer run at the end of the teardown.
    fn finalize_a(&mut self);
    /// Second finalizer run at the end of the teardown.
    fn finalize_b(&mut self);
    /// Frees the storage of the progress bar object itself.
    fn release_storage(&mut self, bar: Handle32<BarTag>);
    /// Sets up the inner object for an update, with one argument word.
    fn setup_inner(&mut self, inner: Handle32<InnerTag>, arg: u32);
    /// Produces three words for the update. The first word is produced but
    /// never read back by the update.
    fn produce(&mut self, bar: Handle32<BarTag>) -> [u32; 3];
    /// Produces the update's token and two words.
    /// Returns the token, then the two words in order.
    fn fill_pair(&mut self) -> (u32, u32, u32);
    /// Consumes the token for the bar. The two words of `fill_pair` are passed by
    /// reference: the call receives their addresses and may read them.
    fn consume(&mut self, bar: Handle32<BarTag>, token: u32, first: &mut u32, second: &mut u32);
    /// The measurement the update adds to the total.
    fn measure(&mut self) -> u32;
    /// Uses one word of the second phase.
    fn use_a(&mut self, a: u32);
    /// Uses a word of the second phase on the bar and returns a word.
    fn use_b(&mut self, bar: Handle32<BarTag>, a: u32, second: u32) -> u32;
    /// Uses the word returned by `use_b`.
    fn use_c(&mut self, b: u32);
    /// Uses a word of the second phase together with `use_b`'s word.
    fn use_d(&mut self, c: u32, b: u32);
    /// Uses the word returned by the second `use_b`.
    fn use_e(&mut self, d: u32);
    /// Combines the bar part with the two words of `fill_pair`.
    fn combine(&mut self, part: Part, first: u32, second: u32);
    /// Steps the bar.
    fn step(&mut self, bar: Handle32<BarTag>);
    /// The comparison value the update checks against the produced word.
    fn compare(&mut self) -> u32;
    /// Raises the signal with the given code.
    fn signal(&mut self, code: u32);
    /// Pushes the word produced by `produce` onto the bar.
    fn push_s4(&mut self, bar: Handle32<BarTag>, word: u32);
    /// Whether the inner object's flag byte is set (read, not called).
    fn inner_flag_set(&mut self, inner: Handle32<InnerTag>) -> bool;
    /// First gate step, run when the inner flag is clear.
    fn gate_a(&mut self, bar: Handle32<BarTag>);
    /// Second gate step, run when the inner flag is clear.
    fn gate_b(&mut self, bar: Handle32<BarTag>);
    /// Third gate step, run whenever the gate global is set.
    fn gate_c(&mut self, bar: Handle32<BarTag>);
    /// The finish step of the update's base path.
    fn finish(&mut self, bar: Handle32<BarTag>);
    /// The base update: its word is the update's result on that path.
    fn base_update(&mut self, bar: Handle32<BarTag>) -> u32;
}
