//! The token follow-up methods of the layout frame.
//!
//! A frame holds a token. A follow-up resolves the token to a target object,
//! asks the frame for its own step's answer, and hands that answer to one of
//! the target's follow-up steps. Several sibling methods run this sequence and
//! then do something else: enable the frame, reset it, or answer the target's
//! result.
//!
//! The methods here call out, so they take a [`FollowUpHost`]: the caller
//! supplies the resolver, the frame's own steps, the target's steps and the
//! shared flags and object. The lift fixes the order of those calls and what
//! each one receives; the host decides what each call does. Each sibling
//! method is listed, with what its proof leaves out, in [`super::registry`].

use super::frame::{UILayoutFrame, field};

/// An opaque handle to the target object a token resolves to. The lift never
/// looks inside it; it only passes the handle to the target's steps.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Target(pub u32);

/// Which of the target's follow-up steps a method calls. Each sibling method
/// names the step it uses; the host maps each name to that step.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TargetStep {
    /// The step used by the two sibling methods that enable the frame or
    /// reset it after the follow-up.
    Sibling107And108,
    /// The step used by the sibling method that answers the target's result.
    Sibling111,
    /// The step used by the sibling method that clears its pending flag.
    Sibling112,
    /// The step used by the sibling method that clears the other pending flag.
    Sibling113,
    /// The step used by the sibling method that hands off to the shared routine.
    Sibling114,
}

/// A byte of shared state the sibling methods read and clear. The siblings
/// read different bytes of the shared block; the host maps each name to its
/// byte.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SharedFlag {
    /// The pending flag the sibling method that clears its pending flag reads.
    PendingFor112,
    /// The pending flag the sibling method that clears the other one reads.
    PendingFor113,
    /// The handoff flag the sibling method that hands off reads.
    Handoff,
}

/// The calls that the token follow-up methods make outside the frame's bytes.
pub trait FollowUpHost {
    /// Resolves a non-zero token to the target object it names.
    fn resolve_token(&mut self, token: u32) -> Target;

    /// The frame's own first step. The frame is its receiver; the answer is
    /// the argument handed to the target.
    fn frame_step(&mut self, frame: &mut UILayoutFrame) -> u32;

    /// One of the target's follow-up steps, with the frame's step answer.
    fn target_step(&mut self, target: Target, step: TargetStep, arg: u32) -> u32;

    /// The frame's first enable step, with one argument.
    fn frame_first_step(&mut self, frame: &mut UILayoutFrame, arg: u32) -> u32;

    /// The frame's second step: an enable step after the first, or a reset
    /// when its argument is zero.
    fn frame_second_step(&mut self, frame: &mut UILayoutFrame, arg: u32) -> u32;

    /// The frame's own token query: its virtual answer for the frame's token.
    fn frame_token(&mut self, frame: &mut UILayoutFrame) -> u32;

    /// The target's own follow-up query, with no argument. Answers its result.
    fn target_follow_up(&mut self, target: Target) -> u32;

    /// Reads a byte of shared state.
    fn read_flag(&mut self, flag: SharedFlag) -> u8;

    /// Writes a byte of shared state.
    fn write_flag(&mut self, flag: SharedFlag, value: u8);

    /// The shared notifier, called with one argument. Answers its result.
    fn notify(&mut self, arg: u32) -> u32;

    /// The shared object word: the sink the stamped call goes to, and the
    /// object the handoff routine receives.
    fn shared_object(&mut self) -> u32;

    /// The sink object's call with a stamp. Answers its result.
    fn call_sink(&mut self, sink: u32, stamp: u32) -> u32;

    /// The shared handoff routine, called with the shared object. Answers its result.
    fn handoff(&mut self, object: u32) -> u32;
}

impl UILayoutFrame {
    /// The shared sequence: resolve the token, ask the frame's step for its
    /// argument, and run the target's step with it. Answers the target's step.
    fn run_token_follow_up<H: FollowUpHost>(
        &mut self,
        host: &mut H,
        token: u32,
        step: TargetStep,
    ) -> u32 {
        let target = host.resolve_token(token);
        let arg = host.frame_step(self);
        host.target_step(target, step, arg)
    }

    /// Asks the frame for its token, resolves that token to its target, asks
    /// the target for its follow-up answer, and answers that answer plus one.
    pub fn resolve_token_and_follow_up_plus_one<H: FollowUpHost>(&mut self, host: &mut H) -> u32 {
        let token = host.frame_token(self);
        let target = host.resolve_token(token);
        host.target_follow_up(target).wrapping_add(1)
    }

    /// Runs the token follow-up when the token at 0xD4 is non-zero, then
    /// enables the frame: the first step with one, then the second step with
    /// one. Answers the second step's answer.
    pub fn token_follow_up_then_two_enables<H: FollowUpHost>(&mut self, host: &mut H) -> u32 {
        let token = self.word_at(field::WORD_D4);
        if token != 0 {
            self.run_token_follow_up(host, token, TargetStep::Sibling107And108);
        }
        host.frame_first_step(self, 1);
        host.frame_second_step(self, 1)
    }

    /// Runs the token follow-up when the token at 0xD4 is non-zero, then
    /// resets the frame: the second step with a zero argument. Answers the
    /// second step's answer.
    pub fn token_follow_up_then_reset<H: FollowUpHost>(&mut self, host: &mut H) -> u32 {
        let token = self.word_at(field::WORD_D4);
        if token != 0 {
            self.run_token_follow_up(host, token, TargetStep::Sibling107And108);
        }
        host.frame_second_step(self, 0)
    }

    /// Runs the token follow-up when the token at 0xD8 is non-zero and answers
    /// the target's step. A zero token does nothing and answers zero.
    pub fn token_follow_up_when_nonzero<H: FollowUpHost>(&mut self, host: &mut H) -> u32 {
        let token = self.word_at(field::WORD_D8);
        if token == 0 {
            return 0;
        }
        self.run_token_follow_up(host, token, TargetStep::Sibling111)
    }

    /// Runs the token follow-up when the token at 0xE0 is non-zero, then
    /// clears the pending flag for this sibling. When the flag is clear at the
    /// point after the follow-up, the notifier runs first. Answers the last
    /// handler that ran, or zero when there was no token.
    pub fn token_follow_up_then_clear_pending<H: FollowUpHost>(&mut self, host: &mut H) -> u32 {
        let token = self.word_at(field::WORD_E0);
        let result = if token == 0 {
            0
        } else {
            let target = host.resolve_token(token);
            let arg = host.frame_step(self);
            let stepped = host.target_step(target, TargetStep::Sibling113, arg);
            if host.read_flag(SharedFlag::PendingFor113) == 0 {
                host.notify(1)
            } else {
                stepped
            }
        };
        host.write_flag(SharedFlag::PendingFor113, 0);
        result
    }

    /// Runs three gated follow-ups and then clears the pending flag for this
    /// sibling: the token at 0xDC runs the follow-up; a marker byte at 0xEB
    /// (when the flag is clear) takes a fresh stamp from the frame's step and
    /// calls the sink with it; the token at 0xE0 re-notifies. Each section
    /// notifies unless the flag is set. Answers the last handler that ran, or
    /// zero when no section fired.
    pub fn three_gated_follow_ups_then_clear_pending<H: FollowUpHost>(
        &mut self,
        host: &mut H,
    ) -> u32 {
        let mut result = 0;

        let token = self.word_at(field::WORD_DC);
        if token != 0 {
            let target = host.resolve_token(token);
            let arg = host.frame_step(self);
            result = host.target_step(target, TargetStep::Sibling112, arg);
            if host.read_flag(SharedFlag::PendingFor112) == 0 {
                result = host.notify(1);
            }
        }

        let marker = self.byte_at(field::FLAG_EB);
        if marker != 0 && host.read_flag(SharedFlag::PendingFor112) == 0 {
            let stamp = host.frame_step(self);
            let sink = host.shared_object();
            result = host.call_sink(sink, stamp);
            if host.read_flag(SharedFlag::PendingFor112) == 0 {
                result = host.notify(1);
            }
        }

        let tail = self.word_at(field::WORD_E0);
        if tail != 0 && host.read_flag(SharedFlag::PendingFor112) == 0 {
            result = host.notify(1);
        }

        host.write_flag(SharedFlag::PendingFor112, 0);
        result
    }

    /// Runs the token follow-up when the token at 0xE4 is non-zero, then hands
    /// off when the handoff flag is clear: the shared routine receives the
    /// shared object and its answer is returned. When the flag is set it is
    /// cleared and the follow-up's answer is returned. A zero token answers
    /// zero.
    pub fn token_follow_up_then_handoff<H: FollowUpHost>(&mut self, host: &mut H) -> u32 {
        let token = self.word_at(field::WORD_E4);
        if token == 0 {
            return 0;
        }
        let target = host.resolve_token(token);
        let arg = host.frame_step(self);
        let stepped = host.target_step(target, TargetStep::Sibling114, arg);
        if host.read_flag(SharedFlag::Handoff) == 0 {
            let object = host.shared_object();
            host.handoff(object)
        } else {
            host.write_flag(SharedFlag::Handoff, 0);
            stepped
        }
    }
}
