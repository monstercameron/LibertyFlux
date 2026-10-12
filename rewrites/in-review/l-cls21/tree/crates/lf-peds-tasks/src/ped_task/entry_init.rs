//! The ped task entry initialiser: sets the entry's kind slots and defaults,
//! resolves its source into a handle through two collaborators, and maps the
//! handle into the entry's index slot when asked to.
//!
//! The owning type is [`TaskEntry`]: the words the initialiser writes, by
//! their offsets in the 32-bit layout. Two of them, the handle and the mapped
//! index, keep their earlier values when the source does not resolve or the
//! conversion is not asked for, so the initialiser takes the entry by
//! mutable reference rather than building a fresh one.
//!
//! The collaborators travel as [`EntryCallees`]: the setup call on the entry,
//! the resolver (which answers a token for a registry and a key), the fetch
//! (which answers the word the token's record holds), and the mapping call.
//! The fetch's scratch descriptor is the implementation's frame, and the
//! mapping call's first argument is a constant zero that the implementation
//! supplies.
//!
//! The handle-to-index conversion is a truncation toward zero that gives the
//! most negative integer for a value outside the signed 32-bit range or not a
//! number. That is the processor's convert-with-truncation result, and the
//! verified rewrite performs it the same way.

/// The sentinel the four slots after the kind slots start with.
pub const SENTINEL: u32 = 0xFFFF_FFFF;

/// Arguments the initialiser takes from its caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EntryArgs {
    /// The first argument: its low byte is the entry's first kind slot.
    pub kind: u32,
    /// The second argument: its low byte is the entry's second kind slot.
    pub kind_flag: u32,
    /// The third argument: the source pointer, kept in the entry when the
    /// source is present.
    pub source: u32,
    /// The key the source holds at offset 4, read by the caller when the
    /// source pointer is present.
    pub source_key: Option<u32>,
    /// The fourth argument: when its low byte is nonzero the handle is
    /// mapped into the index slot.
    pub convert: u32,
}

/// The words of the entry the initialiser writes. Offsets are those of the
/// 32-bit layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaskEntry {
    /// Byte at offset 0xbc0: the first kind slot.
    pub kind: u8,
    /// Word at offset 0xbc4: the second kind slot.
    pub kind_flag: u32,
    /// Word at offset 0x960: the source pointer.
    pub source: u32,
    /// Word at offset 0x99c: cleared.
    pub cleared_99c: u32,
    /// Word at offset 0x994: the handle. Kept when the source does not resolve.
    pub handle: u32,
    /// Word at offset 0x990: the mapped index. Kept unless the conversion runs.
    pub mapped: u32,
    /// Word at offset 0xaa0: cleared.
    pub cleared_aa0: u32,
    /// Words at offsets 0xaa4, 0xaa8, 0xaac and 0xab0: the sentinels.
    pub sentinels: [u32; 4],
    /// Halfword at offset 0xbca: set to one.
    pub enabled: u16,
    /// Word at offset 0xbb8: cleared.
    pub cleared_bb8: u32,
}

/// The collaborators of the initialiser.
pub trait EntryCallees {
    /// The setup call on the entry (its handle is the argument).
    fn setup(&mut self, entry: u32);
    /// Resolves a key in a registry into a token; zero means none.
    fn resolve(&mut self, registry: u32, key: u32) -> u32;
    /// Fetches through a token; answers the word the token's record holds.
    fn fetch(&mut self, token: u32) -> u32;
    /// Maps an index value through the mapping call; the implementation
    /// supplies the constant first argument.
    fn map(&mut self, index: u32) -> u32;
}

/// Truncation toward zero to a signed 32-bit integer, with the processor's
/// indefinite value (the most negative integer) for a not-a-number or a value
/// outside the range.
#[must_use]
pub fn truncate_indefinite(value: f32) -> i32 {
    if value >= -2_147_483_648.0 && value < 2_147_483_648.0 {
        value as i32
    } else {
        i32::MIN
    }
}

impl TaskEntry {
    /// Initialises the entry at `entry` from its arguments.
    ///
    /// The setup call runs first. The kind slots, the cleared words, the
    /// sentinels and the enable halfword are then written. A present source
    /// is resolved through the registry; when the token is nonzero the
    /// fetched word becomes the handle. When the conversion is asked for, the
    /// handle, widened to double precision and narrowed to single precision,
    /// is scaled by the image's factor and truncated into the mapped index.
    /// The last cleared word is written after that.
    pub fn initialise(
        &mut self,
        entry: u32,
        args: EntryArgs,
        registry: u32,
        scale: f32,
        calls: &mut impl EntryCallees,
    ) {
        calls.setup(entry);
        self.kind = (args.kind & 0xFF) as u8;
        self.kind_flag = args.kind_flag & 0xFF;
        self.source = args.source;
        self.cleared_99c = 0;
        self.enabled = 1;
        self.cleared_aa0 = 0;
        self.sentinels = [SENTINEL; 4];
        if let Some(key) = args.source_key {
            let token = calls.resolve(registry, key);
            if token != 0 {
                self.handle = calls.fetch(token);
            }
        }
        if (args.convert & 0xFF) as u8 != 0 {
            let widened = f64::from(self.handle) as f32;
            let scaled = core::hint::black_box(widened) * core::hint::black_box(scale);
            let index = truncate_indefinite(scaled);
            self.mapped = calls.map(index as u32);
        }
        self.cleared_bb8 = 0;
    }
}
