//! What is lifted and proven, per verified routine.
//!
//! One row per `audio_voice_*` routine of the four covered structures
//! (14 proven) plus one row per group routine not reached (64 missing
//! with reasons). A host test pins the counts.

/// Proof state of one routine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Lifted and proven against its verified rewrite.
    Proven,
    /// Not lifted; the note says why and what it needs.
    Missing,
}

/// One registry row: a verified routine and what its lift covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Row {
    /// The routine's original address, as `0x` hex text.
    pub addr: &'static str,
    /// The routine's name in the group list.
    pub name: &'static str,
    /// Its proof state.
    pub state: State,
    /// What the lift covers or, when missing, what it needs.
    pub note: &'static str,
}

/// How many routines are proven.
pub const PROVEN: usize = 14;
/// How many group routines are still missing.
pub const MISSING: usize = 64;

/// Every row: the 14 proven routines, then the 64 missing ones.
pub const ROWS: &[Row] = &[
    // Proven: parameter blocks and the header (params.rs).
    Row { addr: "0x008aafa0", name: "audio_voice_defaults", state: State::Proven,
        note: "ParamBlocks::reset; the this-echo narrows to () and is pinned per case" },
    Row { addr: "0x008ab010", name: "audio_voice_defaults_sub", state: State::Proven,
        note: "ParamBlocks::reset_and_init_sub; helper is SubInit, its address arg (this+0xfc) pinned per case" },
    Row { addr: "0x00985530", name: "audio_voice_reset", state: State::Proven,
        note: "VoiceHead::reset; the this-echo narrows to () and is pinned per case" },
    // Proven: the voice tracker (tracker.rs).
    Row { addr: "0x009e15a0", name: "audio_voice_install", state: State::Proven,
        note: "VoiceTracker::install; pool global becomes an argument, refresh is TrackerWorld" },
    Row { addr: "0x009e1320", name: "audio_voice_release_and_park", state: State::Proven,
        note: "VoiceTracker::release_and_park; the link-address answer narrows to bool, rebuilt per case" },
    Row { addr: "0x009e1540", name: "audio_voice_install_and_bind", state: State::Proven,
        note: "VoiceTracker::install_and_bind; install_pair/bind are TrackerWorld, scripts cover install-like and empty effects" },
    Row { addr: "0x009e1570", name: "audio_voice_handle_resolve", state: State::Proven,
        note: "VoiceTracker::handle_resolve; resolve/attach are TrackerWorld, 1/0 narrows to bool" },
    // Proven: the voice-slot file (slots.rs).
    Row { addr: "0x009aab10", name: "audio_voice_flag_check", state: State::Proven,
        note: "VoiceSlots::flag_check; al answer narrows to bool; out-of-store offsets panic" },
    Row { addr: "0x009aabc0", name: "audio_voice_slot_update", state: State::Proven,
        note: "VoiceSlots::slot_update; notify address narrows to offset, rebuilt per case" },
    Row { addr: "0x009aad00", name: "audio_voice_clear_by_id", state: State::Proven,
        note: "VoiceSlots::clear_by_id; out-of-store offsets panic" },
    Row { addr: "0x009aae00", name: "audio_voice_flag_advance", state: State::Proven,
        note: "VoiceSlots::flag_advance; signed select math mirrored, out-of-store offsets panic" },
    // Proven: the banked voice table (banked.rs).
    Row { addr: "0x008a9eb0", name: "audio_voice_set_flag8", state: State::Proven,
        note: "BankedVoices::set_flag8; table-base answer narrows to (), pinned per case; 0xff panics (original faults)" },
    Row { addr: "0x008aa3a0", name: "audio_voice_store_e0", state: State::Proven,
        note: "BankedVoices::store_slot; value echo narrows to (), pinned per case; 0xff panics (original faults)" },
    Row { addr: "0x008aa3f0", name: "audio_voice_toggle_bit1", state: State::Proven,
        note: "BankedVoices::set_bit1; table_hi|adjust narrows to changed-bool, rebuilt per case; 0xff panics" },
    // Missing: the banked-record routines with callees (same table, other rows).
    Row { addr: "0x0088f7f0", name: "audio_voice_accumulate", state: State::Missing,
        note: "banked record + mixer report; needs BankedVoices extension + mixer trait" },
    Row { addr: "0x00898fd0", name: "audio_voice_update", state: State::Missing,
        note: "banked record dispatcher; 7 callees, needs world traits" },
    Row { addr: "0x0089a130", name: "audio_voice_start", state: State::Missing,
        note: "banked record start; 7 callees, needs world traits" },
    Row { addr: "0x0089a920", name: "audio_voice_commit", state: State::Missing,
        note: "banked record commit; 6 callees, needs world traits" },
    Row { addr: "0x0089ab30", name: "audio_voice_route", state: State::Missing,
        note: "banked record route; 4 callees, needs world traits" },
    Row { addr: "0x0089ac60", name: "audio_voice_probe", state: State::Missing,
        note: "banked record probe; 5 callees, needs world traits" },
    Row { addr: "0x0089d7e0", name: "audio_voice_probe_3slot", state: State::Missing,
        note: "banked 3-sub-slot probe; 2 callees" },
    Row { addr: "0x0089d900", name: "audio_voice_setup_4phase", state: State::Missing,
        note: "banked 4-phase setup; 8 callees" },
    Row { addr: "0x008a12f0", name: "audio_voice_shape", state: State::Missing,
        note: "banked response curves; 3 callees incl. float mixer probe" },
    Row { addr: "0x00997370", name: "audio_voice_slot_update", state: State::Missing,
        note: "banked slot program/bypass; 6 callees" },
    Row { addr: "0x0099dfa0", name: "audio_voice_event_clamped", state: State::Missing,
        note: "banked event fire, clamped; 1 callee" },
    // Missing: the TLS mix/spatialize cluster.
    Row { addr: "0x008a8120", name: "audio_voice_mix_bias15", state: State::Missing,
        note: "TLS voice-table mix row (bias 15); needs TLS-table model; instance of one routine with bias10 (hash differs only in bias)" },
    Row { addr: "0x008a8230", name: "audio_voice_mix_bias10", state: State::Missing,
        note: "TLS voice-table mix row (bias 10); same routine as bias15" },
    Row { addr: "0x008a8340", name: "audio_voice_gain_normalize", state: State::Missing,
        note: "gain normalize over shift table; needs global-table model" },
    Row { addr: "0x008a8700", name: "audio_voice_pool_init", state: State::Missing,
        note: "voice pool init + thread register; 0x1700-byte layout + TLS write" },
    Row { addr: "0x008a8be0", name: "audio_voice_spatialize_a", state: State::Missing,
        note: "TLS spatialize (bias pair); instance of one routine with spatialize_b" },
    Row { addr: "0x008a8f50", name: "audio_voice_spatialize_b", state: State::Missing,
        note: "TLS spatialize (bias pair); same routine as spatialize_a" },
    // Missing: the activation/bitset pair.
    Row { addr: "0x008a9ef0", name: "audio_voice_update", state: State::Missing,
        note: "activation refresh + notify sweep; needs lock + notify traits" },
    Row { addr: "0x008aa440", name: "audio_voice_teardown", state: State::Missing,
        note: "counter/bitset teardown; needs lock + free traits" },
    // Missing: the slot table build/alloc/register trio.
    Row { addr: "0x00981aa0", name: "audio_voice_table_build", state: State::Missing,
        note: "voice table build; 5 callees" },
    Row { addr: "0x00981ff0", name: "audio_voice_slot_alloc", state: State::Missing,
        note: "slot alloc; window/mute/busy gates go to lf-platform" },
    Row { addr: "0x009823d0", name: "audio_voice_register_by_hash", state: State::Missing,
        note: "register by hash; window/mute/busy + hasher gates" },
    // Missing: the 0xac render/update family.
    Row { addr: "0x00acc3f0", name: "audio_voice_update", state: State::Missing,
        note: "level/envelope/phase update; virtual hook + gain globals" },
    Row { addr: "0x00accca0", name: "audio_voice_xfade_update", state: State::Missing,
        note: "crossfade update through the global manager" },
    Row { addr: "0x00ace640", name: "audio_voice_render", state: State::Missing,
        note: "frame render; 5 callees, many float globals" },
    Row { addr: "0x00aceca0", name: "audio_voice_update", state: State::Missing,
        note: "update; 10 callees" },
    Row { addr: "0x00acef70", name: "audio_voice_setup", state: State::Missing,
        note: "setup; 7 callees" },
    Row { addr: "0x00acf190", name: "audio_voice_tick", state: State::Missing,
        note: "tick; 3 callees, float globals" },
    Row { addr: "0x00acf2d0", name: "audio_voice_render", state: State::Missing,
        note: "render through the index chain; 6 callees" },
    Row { addr: "0x00acfee0", name: "audio_voice_start_param", state: State::Missing,
        note: "start with normalized position; gate + registry globals" },
    // Missing: the 0x97 setup/dispatch family.
    Row { addr: "0x00978b60", name: "audio_voice_setup", state: State::Missing,
        note: "slot setup from param block; bank lookup + settle/fallback" },
    Row { addr: "0x009799d0", name: "audio_voice_start_gated", state: State::Missing,
        note: "gated start; audio-gate globals, unchecked return channel" },
    Row { addr: "0x0097b2c0", name: "audio_voice_read_transform", state: State::Missing,
        note: "transform read down a 3-link chain; 3 callees" },
    Row { addr: "0x0097b390", name: "audio_voice_dispatch", state: State::Missing,
        note: "selector dispatch or live snapshot; 1 callee" },
    Row { addr: "0x0097b450", name: "audio_voice_resolve_and_read", state: State::Missing,
        note: "handle resolve + param read; 2 callees" },
    Row { addr: "0x0097b700", name: "audio_voice_init", state: State::Missing,
        note: "init; 4 callees" },
    Row { addr: "0x0097dd20", name: "audio_voice_route_submit", state: State::Missing,
        note: "guarded route submit; descriptor + generation globals" },
    // Missing: the large updates.
    Row { addr: "0x0088ca40", name: "audio_voice_update_volume", state: State::Missing,
        note: "volume update with slew gate; virtual gate + tuning globals" },
    Row { addr: "0x0088e8a0", name: "audio_voice_update", state: State::Missing,
        note: "half-float decode + slew + fx chain; 5 callees" },
    Row { addr: "0x00895730", name: "audio_voice_update", state: State::Missing,
        note: "frame update; 12 callees" },
    Row { addr: "0x008af600", name: "audio_voice_update", state: State::Missing,
        note: "per-frame channel update; 11 callees" },
    Row { addr: "0x00970140", name: "audio_voice_filter_update", state: State::Missing,
        note: "filter row + 32 smoother taps; 13 callees" },
    Row { addr: "0x00996230", name: "audio_voice_update", state: State::Missing,
        note: "spatial mix from emitters; 15 callees" },
    Row { addr: "0x0099eaf0", name: "audio_voice_attach", state: State::Missing,
        note: "buffer attach to mixer slot; 10 callees" },
    Row { addr: "0x009a08a0", name: "audio_voice_update", state: State::Missing,
        note: "per-voice update; 30 callees" },
    // Missing: the gate/setup/dispatch families.
    Row { addr: "0x00b5f790", name: "audio_voice_gate", state: State::Missing,
        note: "update gate on entity flags/stats/level; 5 callees" },
    Row { addr: "0x00b60430", name: "audio_voice_setup", state: State::Missing,
        note: "mix structures + mix chain; 14 callees" },
    Row { addr: "0x00b60870", name: "audio_voice_setup", state: State::Missing,
        note: "setup + mixer dispatch; 10 callees" },
    Row { addr: "0x00b61a20", name: "audio_voice_dispatch", state: State::Missing,
        note: "dispatch by info kind + mix; 9 args" },
    Row { addr: "0x00c8a3e0", name: "audio_voice_gate_retry", state: State::Missing,
        note: "gating check with retry grid; slot hook + listener dot" },
    Row { addr: "0x00c8a590", name: "audio_voice_select_and_spatialize", state: State::Missing,
        note: "slot pick + two spatialize passes; 640-byte rows" },
    Row { addr: "0x00c8ae50", name: "audio_voice_update", state: State::Missing,
        note: "validate + fan out to slots; class switch" },
    Row { addr: "0x00c8b410", name: "audio_voice_build", state: State::Missing,
        note: "build; 7 callees incl. x87 convert idiom" },
    // Missing: the 0xd8 table/scan/spawn/pick family.
    Row { addr: "0x00d8bb70", name: "audio_voice_param_init", state: State::Missing,
        note: "default param program; 5 callees, no args" },
    Row { addr: "0x00d8bc90", name: "audio_voice_scan", state: State::Missing,
        note: "two-limit table scan, early stop past 8 hits; 8 callees" },
    Row { addr: "0x00d8db20", name: "audio_voice_spawn_setup", state: State::Missing,
        note: "spawn from template; factory + 8 callees" },
    Row { addr: "0x00d8e840", name: "audio_voice_release_all", state: State::Missing,
        note: "release 8 handles + mixer buffer; TLS table slot" },
    Row { addr: "0x00d8ffa0", name: "audio_voice_pick", state: State::Missing,
        note: "best-row pick by bounds + distance; callback pre-filter" },
    // Missing: singles.
    Row { addr: "0x00895590", name: "audio_voice_param_block_copy", state: State::Missing,
        note: "call-free 0x6f-byte block copy; easy next lift (lane runtime dialect)" },
    Row { addr: "0x0089ee30", name: "audio_voice_dispatch", state: State::Missing,
        note: "dispatch by resolver verdict; 8 callees" },
    Row { addr: "0x00974f00", name: "audio_voice_active_check", state: State::Missing,
        note: "call-free bound check over 1 global; easy next lift" },
    Row { addr: "0x009a37a0", name: "audio_voice_level_update", state: State::Missing,
        note: "level from tuned globals + x87 truncate; 1 callee" },
    Row { addr: "0x00ad1410", name: "audio_voice_set_rebuild", state: State::Missing,
        note: "bank-set refresh on selection change; probe + chain globals" },
    Row { addr: "0x00ad3a10", name: "audio_voice_preset_apply", state: State::Missing,
        note: "preset apply over loop/word tables; fetch + commit callees" },
];
