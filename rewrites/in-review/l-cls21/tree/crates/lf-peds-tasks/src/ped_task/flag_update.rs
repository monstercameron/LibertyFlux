//! The owner's subject-flag update: a random draw decides one flag bit of
//! the owner's subject, then the owner either applies its task, found
//! through the component's acceptance and lookup calls, or falls back.
//!
//! The owner holds its flag byte, its mode byte and a pointer to the subject
//! object; the subject holds the flag word that the update writes. The
//! owner's component is a sub-object at a fixed offset from the owner, passed
//! to the collaborators as an opaque handle, as are the owner and the task.
//!
//! The collaborators travel as [`FlagCallees`]. The three scratch words the
//! original passes by pointer to one of them are opaque tokens here: the
//! collaborator is given the owner and the token, and the implementation
//! supplies the tokens, as the original's frame does. The original's final
//! read through the task pointer has no effect on the result and is not
//! performed by the lift.

/// The task-flag bit that suppresses the subject update.
pub const SKIP_SUBJECT_FLAG: u8 = 0x08;
/// The subject flag bit the random draw sets or clears.
pub const RANDOM_FLAG: u32 = 0x0200_0000;
/// Draws at or above this value clear the bit; below it, the bit is set.
pub const DRAW_THRESHOLD: i32 = 0x3FFF;
/// Offset of the component sub-object from the owner.
pub const COMPONENT_OFFSET: u32 = 0x2B0;

/// The subject object, as far as the update reads and writes it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubjectFlag {
    /// The subject's handle, passed to the acceptance call.
    pub handle: u32,
    /// The flag word, at the subject's offset 0x210, which the update writes.
    pub flag: u32,
}

/// The owner words the update reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FlagOwner {
    /// The owner's handle, passed to the collaborators.
    pub handle: u32,
    /// The task-flag byte at offset 0x268.
    pub task_flags: u8,
    /// The subject, or none when the owner's subject pointer is null.
    pub subject: Option<SubjectFlag>,
    /// The mode byte at offset 0x219. Zero enables the task.
    pub mode: u8,
}

/// The collaborators of the update. The constant arguments the original
/// fixes are supplied by the implementation.
pub trait FlagCallees {
    /// The random draw.
    fn draw(&mut self) -> u32;
    /// Whether the component accepts the subject (its low byte is tested).
    fn accepts(&mut self, component: u32, subject: u32) -> u32;
    /// The task the component finds for the owner, given the enable word.
    fn find_task(&mut self, component: u32, owner: u32, enable: u32) -> u32;
    /// The owner's scratch-filling call; the scratch words are the
    /// implementation's frame.
    fn fill_scratch(&mut self, owner: u32) -> u32;
    /// Applies the found task; the implementation's frame supplies the
    /// scratch word and the two zero arguments.
    fn apply_task(&mut self, task: u32) -> u32;
    /// The fallback call on the owner.
    fn fallback(&mut self, owner: u32) -> u32;
}

impl FlagOwner {
    /// Runs the update. The subject flag is changed only when the owner's
    /// task flags do not carry the skip bit and the subject is present; the
    /// draw then sets the random bit when it is below the threshold and
    /// clears it otherwise.
    ///
    /// The component is then asked whether it accepts the subject. An
    /// accepting component finds the task, which the owner fills and applies;
    /// otherwise the owner falls back.
    pub fn update(&mut self, calls: &mut impl FlagCallees) {
        if self.task_flags & SKIP_SUBJECT_FLAG == 0 {
            if let Some(subject) = self.subject.as_mut() {
                let draw = calls.draw();
                let desired = if (draw as i32) < DRAW_THRESHOLD {
                    RANDOM_FLAG
                } else {
                    0
                };
                let flag = subject.flag;
                subject.flag = flag ^ ((flag ^ desired) & RANDOM_FLAG);
            }
        }
        let component = self.handle.wrapping_add(COMPONENT_OFFSET);
        let subject_handle = self.subject.map_or(0, |subject| subject.handle);
        let accepted = calls.accepts(component, subject_handle);
        if accepted & 0xFF != 0 {
            let enable = u32::from(self.mode == 0);
            let task = calls.find_task(component, self.handle, enable);
            calls.fill_scratch(self.handle);
            calls.apply_task(task);
        } else {
            calls.fallback(self.handle);
        }
    }
}
