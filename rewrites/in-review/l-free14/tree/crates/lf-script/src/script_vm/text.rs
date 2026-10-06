//! The text keys: state-gated entry resolution with a formatting path.
//!
//! Lifted from the verified `script_vm_resolve_named_key` rewrite. The
//! routine records a flag byte, then probes a state block: when the
//! block's format flag is clear the key resolves directly, otherwise the
//! key formats into a 24-byte buffer, a suffix word appends at the end of
//! the string starting at buffer offset 12, and the buffer resolves
//! instead. A null or empty entry falls back to resolving the raw key.
//! The suffix and cookie words are owned state; the state block, the
//! formatter, the entry table and the cookie check are traits.
//!
//! [`TextDispatch`] is the second text routine, the dual-key dispatch:
//! two keys resolve, a probe-gated setup call runs unless a global pair
//! disagrees, a done-gated sixteen-word draw fires, the done flag rises,
//! and both keys notify. Its globals are fields for the same reason.

// Signatures mirror the 32-bit routines' words one by one, so long
// argument lists are inherent here.
#![allow(clippy::too_many_arguments)]

use lf_core::boundary::Handle32;

/// Length of the formatting buffer.
pub const FORMAT_LEN: usize = 24;
/// Offset the suffix scan starts at: the string's start.
pub const SUFFIX_SCAN_OFF: usize = 12;

/// Tag for the text-entry object.
#[derive(Debug)]
pub struct TextObjTag;

/// The text-entry object: the resolver's `this`, opaque until it lifts.
pub type TextObj = Handle32<TextObjTag>;

/// Tag for a resolved text entry.
#[derive(Debug)]
pub struct EntryTag;

/// A resolved text entry's identity, opaque until entries lift.
pub type EntryId = Handle32<EntryTag>;

/// A resolved text entry: its identity and whether it is a non-empty string.
///
/// The 32-bit routine reads the emptiness from entry memory (a null
/// pointer or a zero first half-word means empty); the lift carries it
/// with the resolve answer because entries are owned by unlifted code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TextEntry {
    /// The entry's opaque identity.
    pub id: EntryId,
    /// Whether the entry is a non-empty string.
    pub nonempty: bool,
}

/// A resolution key: the raw script word or the formatted buffer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextKey {
    /// The raw key word: the direct path and the fallback.
    Raw(u32),
    /// The formatted buffer with the suffix appended.
    Formatted([u8; FORMAT_LEN]),
}

/// Probes the state block: the state callee behind the format flag.
///
/// The 32-bit callee answers a state block whose byte at a fixed offset
/// decides; the lift takes the decided bool.
pub trait StateProbe {
    /// Whether the format flag is set.
    fn format_enabled(&mut self) -> bool;
}

/// Formats a key into the 24-byte buffer: the formatter callee.
///
/// The answer is the buffer's full post-format state; the suffix scan
/// needs a zero byte at or before the fourth word from the end.
pub trait TextFormat {
    /// Formats `key`, answering the buffer's bytes.
    fn format(&mut self, key: u32) -> [u8; FORMAT_LEN];
}

/// Resolves a key to its entry: the entry-table resolver.
pub trait TextResolve {
    /// Resolves `key` on `obj`, answering `None` for a null entry.
    fn resolve(&mut self, obj: TextObj, key: &TextKey) -> Option<TextEntry>;
}

/// Runs the trailing cookie check: the cookie callee.
pub trait CookieCheck {
    /// Checks `cookie`.
    fn check(&mut self, cookie: u32);
}

impl<F: FnMut() -> bool> StateProbe for F {
    fn format_enabled(&mut self) -> bool {
        self()
    }
}

impl<F: FnMut(u32) -> [u8; FORMAT_LEN]> TextFormat for F {
    fn format(&mut self, key: u32) -> [u8; FORMAT_LEN] {
        self(key)
    }
}

impl<F: FnMut(TextObj, &TextKey) -> Option<TextEntry>> TextResolve for F {
    fn resolve(&mut self, obj: TextObj, key: &TextKey) -> Option<TextEntry> {
        self(obj, key)
    }
}

impl<F: FnMut(u32)> CookieCheck for F {
    fn check(&mut self, cookie: u32) {
        self(cookie);
    }
}

/// The text-key state: the suffix and cookie words.
///
/// The 32-bit routine reads both from globals; the lift owns them, and
/// the resolution is a method taking the unlifted collaborators as traits.
#[derive(Debug, Clone, Copy)]
pub struct TextKeys {
    /// The suffix word appended to the formatted buffer.
    suffix: u32,
    /// The cookie word passed to the trailing check.
    cookie: u32,
}

impl TextKeys {
    /// Builds the state over the suffix and cookie words.
    #[must_use]
    pub fn new(suffix: u32, cookie: u32) -> Self {
        Self { suffix, cookie }
    }

    /// The suffix word.
    #[must_use]
    pub fn suffix(&self) -> u32 {
        self.suffix
    }

    /// The cookie word.
    #[must_use]
    pub fn cookie(&self) -> u32 {
        self.cookie
    }

    /// Resolves `key` to its entry, formatting first when enabled.
    ///
    /// Writes `false` through `flag` when present, then resolves
    /// directly when the format flag is clear. Otherwise formats into
    /// the buffer, appends the suffix at the end of the string starting
    /// at offset 12, and resolves the buffer; writes `true` through
    /// `flag` when the entry is a non-empty string, and falls back to
    /// the raw key when it is not. The cookie check runs on every path.
    /// Buffers with no zero byte where the scan can reach panic, as the
    /// rewrite does.
    pub fn resolve_named_key(
        &self,
        obj: TextObj,
        probe: &mut impl StateProbe,
        format: &mut impl TextFormat,
        resolve: &mut impl TextResolve,
        cookie: &mut impl CookieCheck,
        key: u32,
        mut flag: Option<&mut bool>,
    ) -> Option<TextEntry> {
        if let Some(f) = &mut flag {
            **f = false;
        }
        if !probe.format_enabled() {
            let out = resolve.resolve(obj, &TextKey::Raw(key));
            cookie.check(self.cookie);
            return out;
        }
        let mut buf = format.format(key);
        let mut len = SUFFIX_SCAN_OFF;
        while buf[len] != 0 {
            len += 1;
        }
        buf[len..len + 4].copy_from_slice(&self.suffix.to_le_bytes());
        let found = resolve.resolve(obj, &TextKey::Formatted(buf));
        let hit = matches!(found, Some(entry) if entry.nonempty);
        if hit && let Some(f) = &mut flag {
            **f = true;
        }
        let out = if hit {
            found
        } else {
            resolve.resolve(obj, &TextKey::Raw(key))
        };
        cookie.check(self.cookie);
        out
    }
}

/// Flag word the dispatch passes with each resolved key.
pub const LOOKUP_FLAG: u32 = 0;
/// Argument the dispatch passes to the selector reader.
pub const SELECTOR_ARG: u32 = 1;
/// Ninth-from-last word of the setup call.
pub const SETUP_ONE: u32 = 1;
/// Last word of the setup call: all ones.
pub const SETUP_NEG: u32 = 0xFFFF_FFFF;

/// Tag for the dispatch's notify object.
#[derive(Debug)]
pub struct DispatchObjTag;

/// The dispatch's notify object, opaque until it lifts.
pub type DispatchObj = Handle32<DispatchObjTag>;

/// Resolves a dispatch key: the key lookup.
pub trait KeyLookup {
    /// Resolves `key` with `flag`, answering the entry word.
    fn lookup(&mut self, key: u32, flag: u32) -> u32;
}

/// Probes the setup gate: the probe callee behind its flag byte.
///
/// The 32-bit callee answers a block whose flag byte decides; the lift
/// takes whether the byte is nonzero.
pub trait SetupProbe {
    /// Whether the probe's flag byte is nonzero.
    fn probe_nonzero(&mut self) -> bool;
}

/// Runs the fourteen-word setup call: the setup callee.
///
/// Only the two entry words and the two script words vary; the rest are
/// the call's constant shape, passed through so the proof compares them.
pub trait SetupCall {
    /// Runs setup with the two entries and the two script words in place.
    fn setup(
        &mut self,
        r1: u32,
        z0: u32,
        a2: u32,
        z1: u32,
        z2: u32,
        r2: u32,
        z3: u32,
        z4: u32,
        z5: u32,
        z6: u32,
        a3: u32,
        z7: u32,
        one: u32,
        neg: u32,
    );
}

/// Reads the draw selector byte: the selector callee behind its block.
///
/// The 32-bit callee answers a block whose selector byte decides; the
/// lift takes the byte itself.
pub trait Selector {
    /// Reads the selector byte with `arg`.
    fn selector(&mut self, arg: u32) -> u8;
}

/// Runs the sixteen-word draw call: the draw callee.
///
/// The entries, the shared word, the selector and the raw keys vary;
/// the eight all-ones words and the zero are the call's constant shape.
pub trait DrawKeys {
    /// Draws with the entries, the shared word and the keys in place.
    fn draw(
        &mut self,
        r1: u32,
        g1a: u32,
        r2: u32,
        g1b: u32,
        n0: u32,
        n1: u32,
        n2: u32,
        n3: u32,
        n4: u32,
        n5: u32,
        n6: u32,
        n7: u32,
        zero: u32,
        sel: u32,
        key0: u32,
        key1: u32,
    );
}

/// Notifies the first key: the notify pair's first slot.
///
/// The answer is dropped on both sides.
pub trait NotifyA {
    /// Notifies `key` on `obj`, dropping the answer.
    fn notify_a(&mut self, obj: DispatchObj, key: u32);
}

/// Notifies the second key: the notify pair's second slot.
///
/// The answer is the routine's answer.
pub trait NotifyB {
    /// Notifies `key` on `obj`, answering the result.
    fn notify_b(&mut self, obj: DispatchObj, key: u32) -> u32;
}

impl<F: FnMut(u32, u32) -> u32> KeyLookup for F {
    fn lookup(&mut self, key: u32, flag: u32) -> u32 {
        self(key, flag)
    }
}

impl<F: FnMut() -> bool> SetupProbe for F {
    fn probe_nonzero(&mut self) -> bool {
        self()
    }
}

impl<F: FnMut(u32) -> u8> Selector for F {
    fn selector(&mut self, arg: u32) -> u8 {
        self(arg)
    }
}

impl<F: FnMut(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32)> SetupCall
    for F
{
    fn setup(
        &mut self,
        r1: u32,
        z0: u32,
        a2: u32,
        z1: u32,
        z2: u32,
        r2: u32,
        z3: u32,
        z4: u32,
        z5: u32,
        z6: u32,
        a3: u32,
        z7: u32,
        one: u32,
        neg: u32,
    ) {
        self(r1, z0, a2, z1, z2, r2, z3, z4, z5, z6, a3, z7, one, neg);
    }
}

impl<F: FnMut(u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32)>
    DrawKeys for F
{
    fn draw(
        &mut self,
        r1: u32,
        g1a: u32,
        r2: u32,
        g1b: u32,
        n0: u32,
        n1: u32,
        n2: u32,
        n3: u32,
        n4: u32,
        n5: u32,
        n6: u32,
        n7: u32,
        zero: u32,
        sel: u32,
        key0: u32,
        key1: u32,
    ) {
        self(
            r1, g1a, r2, g1b, n0, n1, n2, n3, n4, n5, n6, n7, zero, sel, key0, key1,
        );
    }
}

impl<F: FnMut(DispatchObj, u32)> NotifyA for F {
    fn notify_a(&mut self, obj: DispatchObj, key: u32) {
        self(obj, key);
    }
}

impl<F: FnMut(DispatchObj, u32) -> u32> NotifyB for F {
    fn notify_b(&mut self, obj: DispatchObj, key: u32) -> u32 {
        self(obj, key)
    }
}

/// The dual-key dispatch state: the shared word, the gate pair and the
/// done flag.
///
/// The 32-bit routine reads all four from globals and raises the done
/// flag on every path; the lift owns them and the dispatch is a method.
#[derive(Debug, Clone, Copy)]
pub struct TextDispatch {
    /// The word shared by both draw entries.
    g1: u32,
    /// The gate pair: a set, disagreeing pair skips the setup call.
    g2: [u32; 2],
    /// Whether the done flag is raised.
    done: bool,
}

impl TextDispatch {
    /// Builds the state over the shared word, the gate pair and the flag.
    #[must_use]
    pub fn new(g1: u32, g2: [u32; 2], done: bool) -> Self {
        Self { g1, g2, done }
    }

    /// The shared word.
    #[must_use]
    pub fn g1(&self) -> u32 {
        self.g1
    }

    /// The gate pair.
    #[must_use]
    pub fn g2(&self) -> [u32; 2] {
        self.g2
    }

    /// Whether the done flag is raised.
    #[must_use]
    pub fn done(&self) -> bool {
        self.done
    }

    /// Dispatches two keys through setup, draw and notify.
    ///
    /// Both keys resolve first. The setup call runs unless the probe
    /// byte is clear while the gate pair is set and disagrees. The draw
    /// fires when setup ran and the done flag is raised, with the
    /// selector byte widened to a word. The done flag rises on every
    /// path, then both keys notify; the second answer is returned.
    pub fn dispatch(
        &mut self,
        obj: DispatchObj,
        lookup: &mut impl KeyLookup,
        probe: &mut impl SetupProbe,
        setup: &mut impl SetupCall,
        sel: &mut impl Selector,
        draw: &mut impl DrawKeys,
        notify_a: &mut impl NotifyA,
        notify_b: &mut impl NotifyB,
        key0: u32,
        key1: u32,
        a2: u32,
        a3: u32,
    ) -> u32 {
        let r1 = lookup.lookup(key0, LOOKUP_FLAG);
        let r2 = lookup.lookup(key1, LOOKUP_FLAG);
        let mut setup_done = true;
        if !probe.probe_nonzero() && self.g2[0] != 0 && self.g2[0] != self.g2[1] {
            setup_done = false;
        }
        if setup_done {
            setup.setup(r1, 0, a2, 0, 0, r2, 0, 0, 0, 0, a3, 0, SETUP_ONE, SETUP_NEG);
        }
        if setup_done && self.done {
            let selector = sel.selector(SELECTOR_ARG);
            draw.draw(
                r1,
                self.g1,
                r2,
                self.g1,
                SETUP_NEG,
                SETUP_NEG,
                SETUP_NEG,
                SETUP_NEG,
                SETUP_NEG,
                SETUP_NEG,
                SETUP_NEG,
                SETUP_NEG,
                0,
                u32::from(selector),
                key0,
                key1,
            );
        }
        self.done = true;
        notify_a.notify_a(obj, key0);
        notify_b.notify_b(obj, key1)
    }
}
