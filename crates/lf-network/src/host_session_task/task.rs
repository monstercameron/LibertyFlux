//! The hosting task itself: gamer list, countdown, state and probe.
//!
//! Lifted from the verified rewrites of `rage::snHostSessionTask`. The
//! 32-bit object is a header (started latch, countdown, session link, state
//! and probe words), a gamer-list block (config and split words, two counts,
//! a 259-word array, the open-extra and trailer-size words with the trailer
//! record after them); here those are plain fields, with the session link
//! carried as an opaque cookie and reached through [`HostSessionWorld`].

use lf_core::Handle32;

use super::{EntryTag, SessionTag};

/// Words in the gamer array the setup copies.
pub const GAMER_WORDS: usize = 259;

/// Signed threshold used by the verified count adjustment.
pub const GAMER_COUNT_THRESHOLD: i32 = 32;

/// File tag word of the first completion descriptor, relocated against the
/// image base before it is submitted.
pub const DESCRIPTOR_TAG_FIRST: u32 = 0xFE_3504;

/// File tag word of the second completion descriptor, relocated against the
/// image base before it is submitted.
pub const DESCRIPTOR_TAG_SECOND: u32 = 0xFE_341C;

/// File base of the key table handed to the third key-block step, relocated
/// against the image base.
pub const KEY_TABLE_BASE: u32 = 0x019F_3230;

/// Session flag bit the completion side path releases and clears.
const SESSION_AUX_PENDING: u8 = 0x01;

/// Session flag bit the completion main path clears when the list-split
/// word is zero.
const SESSION_UP: u8 = 0x02;

/// Session flag bit the completion main path sets between the two
/// descriptors.
const SESSION_KEYED: u8 = 0x20;

/// Setup flag bit that sends the state machine to key resolution.
const SETUP_KEYED: u8 = 0x02;

/// Entry flag bits the completion main path sets on a fresh entry.
const ENTRY_LIVE_MASK: u8 = 0x03;

/// Session states that count as live (joining or joined).
const SESSION_LIVE_FIRST: i32 = 2;
/// Last live session state; see [`SESSION_LIVE_FIRST`].
const SESSION_LIVE_LAST: i32 = 3;

/// Globals the hosting task reads, filled when state is loaded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostSessionGlobals {
    /// Base the descriptor tags and the key table relocate against.
    pub image_base: u32,
    /// The cookie word the completion paths check.
    pub cookie: u32,
    /// Low byte of the setup flag word; bit 1 selects key resolution.
    pub setup_flag: u8,
    /// The four setup key words handed to key resolution.
    pub setup_key: [u32; 4],
}

/// The gamer-list words the setup stores.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GamerListParams {
    /// Stored as the config word; selects the configured paths later.
    pub list: u32,
    /// Stored as the split word; zero merges the two counts into one.
    pub split: u32,
    /// First gamer count.
    pub first: u32,
    /// Second gamer count.
    pub second: u32,
    /// Stored as the open-extra word for the hosted open.
    pub extra: u32,
    /// Handed to the trailer append.
    pub trailer_arg: u32,
    /// Stored as the trailer-size word; zero skips the trailer append.
    pub trailer_size: u32,
}

/// What the hosting task needs from the engine around it: the session
/// object, its own completion entry, the open/key/poll helpers, the
/// descriptor and key-block sinks, and the allocator for host entries.
///
/// Every method is one callee, session-field access or virtual-slot role
/// from the verified rewrites, with addresses narrowed to opaque cookies,
/// constant zero words dropped, pointer arguments narrowed to the values
/// they carry in and out, and unused answers unmodelled (see the registry
/// for the per-method narrowings).
pub trait HostSessionWorld {
    /// Reads the session state word; 2 and 3 count as live.
    fn session_state(&mut self, session: Option<Handle32<SessionTag>>) -> u32;
    /// The task's own completion entry (its table slot) with the two
    /// report words.
    fn own_complete(&mut self, path: u32, note: u32);
    /// Appends the trailer record through the trailer sink, answering its
    /// tag. The sink fills `trailer` with the record words.
    fn append_trailer(&mut self, trailer: &mut Vec<u32>, arg: u32, len: u32) -> u32;
    /// Prepares the session's primary setup block; answers nothing used.
    fn prepare_primary(&mut self, session: Option<Handle32<SessionTag>>);
    /// Prepares the session object; answers nothing used.
    fn prepare_session(&mut self, session: Option<Handle32<SessionTag>>);
    /// Opens the plain session over one count and the gamer array;
    /// answers nonzero low byte on success. May update the probe word.
    fn open_plain(
        &mut self,
        session: Option<Handle32<SessionTag>>,
        net: u32,
        first: u32,
        gamers: &[u32; GAMER_WORDS],
        probe: &mut u32,
    ) -> u32;
    /// Opens the hosted session over both counts, the gamer array and the
    /// open-extra word; answers nonzero low byte on success. May update
    /// the probe word.
    #[allow(clippy::too_many_arguments)]
    fn open_hosted(
        &mut self,
        session: Option<Handle32<SessionTag>>,
        net: u32,
        split: u32,
        first: u32,
        second: u32,
        gamers: &[u32; GAMER_WORDS],
        extra: u32,
        probe: &mut u32,
    ) -> u32;
    /// Promotes the hosted session; answers nothing used.
    fn promote_hosted(&mut self);
    /// Resolves the host key over the setup key; answers nonzero low byte
    /// to continue toward reporting. May update the empty and probe words.
    fn resolve_host_key(
        &mut self,
        session: Option<Handle32<SessionTag>>,
        key: &[u32; 4],
        empty: &mut u32,
        probe: &mut u32,
    ) -> u32;
    /// Polls the hosted session; answers nonzero low byte to advance. May
    /// update the probe word.
    fn poll_host(&mut self, session: Option<Handle32<SessionTag>>, probe: &mut u32) -> u32;
    /// The loop's progress check; a nonzero low byte repeats the pass.
    fn progress_check(&mut self) -> u32;
    /// Mirrors the session's word pair into its derived slot.
    fn mirror_session_block(&mut self, session: Option<Handle32<SessionTag>>);
    /// Reads the session flag byte.
    fn session_flag(&mut self, session: Option<Handle32<SessionTag>>) -> u8;
    /// Writes the session flag byte.
    fn set_session_flag(&mut self, session: Option<Handle32<SessionTag>>, value: u8);
    /// Brings the session up; answers nothing used.
    fn session_up(&mut self, session: Option<Handle32<SessionTag>>);
    /// Notifies the host sink with the select and note words; answers
    /// nothing used.
    fn notify_host(&mut self, path: u32, note: u32);
    /// Submits one completion descriptor tagged with the relocated tag.
    fn submit_descriptor(&mut self, session: Option<Handle32<SessionTag>>, tag: u32);
    /// First key-block step; answers nothing used.
    fn derive_block_a(&mut self);
    /// Second key-block step; answers nothing used.
    fn derive_block_b(&mut self);
    /// Third key-block step over the relocated key table; answers nothing
    /// used.
    fn derive_block_c(&mut self, table: u32);
    /// Fourth key-block step over the session; answers nothing used.
    fn derive_block_d(&mut self, session: Option<Handle32<SessionTag>>);
    /// Finishes the key block over the empty word, the trailer record and
    /// its size word; answers nothing used.
    fn finish_key_block(
        &mut self,
        session: Option<Handle32<SessionTag>>,
        empty: u32,
        trailer: &[u32],
        trailer_len: u32,
    );
    /// Allocates a host-session entry over the session.
    fn alloc_session_entry(
        &mut self,
        session: Option<Handle32<SessionTag>>,
    ) -> Option<Handle32<EntryTag>>;
    /// Reads a host entry's flag byte.
    fn entry_flags(&mut self, entry: Option<Handle32<EntryTag>>) -> u8;
    /// Writes a host entry's flag byte.
    fn set_entry_flags(&mut self, entry: Option<Handle32<EntryTag>>, value: u8);
    /// Checks the cookie word; answers nothing used.
    fn verify_cookie(&mut self, cookie: u32);
    /// Closes the session auxiliary block; answers nothing used.
    fn close_session_aux(&mut self, session: Option<Handle32<SessionTag>>);
    /// Releases the session timer; answers nothing used.
    fn release_session_timer(&mut self, session: Option<Handle32<SessionTag>>);
}

/// A session-hosting task, owning its gamer list and its progress words.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostSessionTask {
    /// The started latch.
    pub started: u32,
    /// Ticks left before the state machine runs freely.
    pub countdown: u32,
    /// The hosted session; the completion paths drop it.
    pub session: Option<Handle32<SessionTag>>,
    /// The state-machine state.
    pub state: u32,
    /// The probe word the helpers update.
    pub probe: u32,
    /// The config word: nonzero selects the configured paths.
    pub config: u32,
    /// Whether the first count is non-positive, as a word.
    pub first_empty: u32,
    /// The list-split word: zero merges the counts, nonzero opens hosted.
    pub list_split: u32,
    /// First gamer count.
    pub first_count: u32,
    /// Second gamer count.
    pub second_count: u32,
    /// The gamer array.
    pub gamers: [u32; GAMER_WORDS],
    /// The open-extra word for the hosted open.
    pub open_extra: u32,
    /// The trailer-size word; zero skips the trailer append.
    pub trailer_size: u32,
    /// The trailer record, filled by the world.
    pub trailer: Vec<u32>,
}

impl HostSessionTask {
    /// Stores the list words, applies the original's signed count adjustment,
    /// copies the gamer array, and appends the trailer record unless its size
    /// is zero. Answers the trailer tag, or the zero size, with its low byte
    /// forced to 1.
    ///
    /// The adjustment uses wrapping signed arithmetic exactly as the rewrite
    /// does. In particular, some positive overflow cases increase the stored
    /// counts; this follows the verified rewrite even though its comment calls
    /// the operation a clamp.
    pub fn setup_gamer_data<W: HostSessionWorld>(
        &mut self,
        world: &mut W,
        params: &GamerListParams,
        src: &[u32; GAMER_WORDS],
    ) -> u32 {
        self.config = params.list;
        self.list_split = params.split;
        if params.split == 0 {
            self.second_count = 0;
            self.first_count = params.first.wrapping_add(params.second);
        } else {
            self.first_count = params.first;
            self.second_count = params.second;
        }
        let mut first = self.first_count as i32;
        let mut second = self.second_count as i32;
        if first.wrapping_add(second) > GAMER_COUNT_THRESHOLD {
            let over = GAMER_COUNT_THRESHOLD
                .wrapping_sub(first)
                .wrapping_sub(second);
            first = first.wrapping_sub(over);
            let clamped = if first > 0 { first } else { 0 };
            self.first_count = clamped as u32;
            if clamped.wrapping_add(second) > GAMER_COUNT_THRESHOLD {
                second = second.wrapping_sub(over);
                let clamped2 = if second > 0 { second } else { 0 };
                self.second_count = clamped2 as u32;
            }
        }
        self.first_empty = u32::from((self.first_count as i32) <= 0);
        self.gamers.copy_from_slice(src);
        self.open_extra = params.extra;
        self.trailer_size = params.trailer_size;
        let tag = if params.trailer_size != 0 {
            world.append_trailer(&mut self.trailer, params.trailer_arg, params.trailer_size)
        } else {
            params.trailer_size
        };
        (tag & 0xFFFF_FF00) | 1
    }

    /// Latches the started flag, then reports completion with (0, 0)
    /// through the task's own entry while the session state is 2 or 3 and
    /// otherwise parks the state at idle.
    ///
    /// # Panics
    ///
    /// When the session link is missing (the original reads through it
    /// unconditionally).
    pub fn start<W: HostSessionWorld>(&mut self, world: &mut W) {
        if self.started == 0 {
            self.started = 1;
        }
        let session = self
            .session
            .expect("host session task needs its session link to start");
        let state = world.session_state(Some(session));
        if state == 2 || state == 3 {
            world.own_complete(0, 0);
        } else {
            self.state = 0;
        }
    }

    /// Drops a positive countdown by `delta` (wrapping; an exact zero
    /// becomes all-ones), then runs the state machine until the probe reads
    /// 1 or the progress check answers a zero low byte.
    ///
    /// State 0 prepares and opens (plain or hosted by the split word),
    /// moving to 1 unless the open answers a zero low byte; 1 promotes to 2
    /// on probe 3 and exits quietly on probe 1; 2 jumps to 4 when configured
    /// or the setup flag's key bit is clear, else resolves the host key and
    /// lands on 4 or 3; 3 reports (1, 0) on probe 3, exits quietly on probe
    /// 1, else moves to 4; 4 polls and moves to 5 unless the poll answers a
    /// zero low byte; 5 reports (0, 0) unless the probe is 1; states above 5
    /// do nothing. Reports go through the task's own entry.
    pub fn advance<W: HostSessionWorld>(
        &mut self,
        world: &mut W,
        globals: &HostSessionGlobals,
        delta: u32,
    ) {
        if (self.countdown as i32) > 0 {
            let next = self.countdown.wrapping_sub(delta);
            self.countdown = if next == 0 { 0xFFFF_FFFF } else { next };
        }
        let session = self.session;
        loop {
            match self.state {
                0 => {
                    world.prepare_primary(session);
                    world.prepare_session(session);
                    let answer = if self.list_split == 0 {
                        world.open_plain(
                            session,
                            self.config,
                            self.first_count,
                            &self.gamers,
                            &mut self.probe,
                        )
                    } else {
                        world.open_hosted(
                            session,
                            self.config,
                            self.list_split,
                            self.first_count,
                            self.second_count,
                            &self.gamers,
                            self.open_extra,
                            &mut self.probe,
                        )
                    };
                    if answer & 0xFF == 0 {
                        world.own_complete(0, 0);
                    } else {
                        self.state = 1;
                    }
                }
                1 => {
                    let probe = self.probe;
                    if probe == 3 {
                        world.promote_hosted();
                        self.state = 2;
                    } else if probe != 1 {
                        world.own_complete(0, 0);
                    }
                }
                2 => {
                    if self.config != 0 {
                        self.state = 4;
                    } else if globals.setup_flag & SETUP_KEYED == 0 {
                        self.state = 4;
                    } else {
                        let answer = world.resolve_host_key(
                            session,
                            &globals.setup_key,
                            &mut self.first_empty,
                            &mut self.probe,
                        );
                        self.state = if answer & 0xFF == 0 { 4 } else { 3 };
                    }
                }
                3 => {
                    let probe = self.probe;
                    if probe == 3 {
                        world.own_complete(1, 0);
                    } else if probe != 1 {
                        self.state = 4;
                    }
                }
                4 => {
                    let answer = world.poll_host(session, &mut self.probe);
                    if answer & 0xFF == 0 {
                        world.own_complete(0, 0);
                    } else {
                        self.state = 5;
                    }
                }
                5 => {
                    if self.probe != 1 {
                        world.own_complete(0, 0);
                    }
                }
                _ => {}
            }
            if self.probe == 1 {
                break;
            }
            if world.progress_check() & 0xFF == 0 {
                break;
            }
        }
    }

    /// Runs the completion entry: with a live session (state 2 or 3) and
    /// select 1 the main path mirrors the session block, brings the session
    /// up (or clears its up bit), notifies, drops the link, submits both
    /// descriptors, derives the key block, allocates and marks a host entry,
    /// and checks the cookie; with a live session and any other select the
    /// side path polls and releases before falling into the tail, which
    /// notifies, drops the link and checks the cookie.
    ///
    /// # Panics
    ///
    /// When the session link is missing (the original reads through it
    /// unconditionally).
    pub fn complete<W: HostSessionWorld>(
        &mut self,
        world: &mut W,
        globals: &HostSessionGlobals,
        path: u32,
        note: u32,
    ) {
        let session = self
            .session
            .expect("host session task needs its session link to complete");
        let mode = world.session_state(Some(session));
        let live = (SESSION_LIVE_FIRST..=SESSION_LIVE_LAST).contains(&(mode as i32));
        if live && path == 1 {
            world.mirror_session_block(Some(session));
            if self.list_split != 0 {
                world.session_up(Some(session));
            } else {
                let flag = world.session_flag(Some(session));
                world.set_session_flag(Some(session), flag & !SESSION_UP);
            }
            world.notify_host(1, note);
            self.session = None;
            world.submit_descriptor(
                Some(session),
                globals.image_base.wrapping_add(DESCRIPTOR_TAG_FIRST),
            );
            let flag = world.session_flag(Some(session));
            world.set_session_flag(Some(session), flag | SESSION_KEYED);
            world.submit_descriptor(
                Some(session),
                globals.image_base.wrapping_add(DESCRIPTOR_TAG_SECOND),
            );
            world.derive_block_a();
            world.derive_block_b();
            if self.config == 0 {
                world.derive_block_c(globals.image_base.wrapping_add(KEY_TABLE_BASE));
            }
            world.derive_block_d(Some(session));
            let trailer: &[u32] = if self.trailer_size != 0 {
                &self.trailer
            } else {
                &[]
            };
            world.finish_key_block(Some(session), self.first_empty, trailer, self.trailer_size);
            let entry = world.alloc_session_entry(Some(session));
            let flags = world.entry_flags(entry);
            world.set_entry_flags(entry, flags | ENTRY_LIVE_MASK);
            world.verify_cookie(globals.cookie);
            return;
        }
        if live {
            if self.probe == 1 {
                world.poll_host(Some(session), &mut self.probe);
            }
            if world.session_flag(Some(session)) & SESSION_AUX_PENDING != 0 {
                world.close_session_aux(Some(session));
                world.release_session_timer(Some(session));
                let flag = world.session_flag(Some(session));
                world.set_session_flag(Some(session), flag & !SESSION_AUX_PENDING);
            }
        }
        world.notify_host(path, note);
        self.session = None;
        world.verify_cookie(globals.cookie);
    }
}
