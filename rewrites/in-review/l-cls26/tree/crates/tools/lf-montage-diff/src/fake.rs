//! The recorder both sides of a differential case write their calls into.
//!
//! The lifted dispatcher runs against [`FakeWorld`], which records each
//! collaborator call as text with its arguments named by [`sym`]. The
//! verified rewrite runs against the runtime's stubs, whose recorded
//! numbered and virtual calls are turned into the same text by
//! [`normalise`]. Both sides answer from the same scripted answers, so a
//! difference in order, arguments or results shows up as a difference in
//! the text. Reads through collaborator objects (element words,
//! sub-object links) are answered from the script on both sides but only
//! the lift names them; the rewrite reads them from case-owned memory, so
//! they appear in neither log.

use std::collections::{HashMap, VecDeque};

use lf_core::Handle32;
use lf_input_frontend::montage_container::{
    AuxTag, CfgSubTag, CfgTag, ElemTag, InnerTag, ListTag, MontageWorld, OutTag, OutputTag, OwnTag,
    ProbeTag, SessionTag, SrcTag, Tag, TailTag, ViewTag,
};

/// The addresses a case works with, so the record can name them.
#[derive(Debug, Clone, Copy)]
pub struct Ctx {
    /// The container object.
    pub own: u32,
    /// The clip list object.
    pub list: u32,
    /// The auxiliary object.
    pub aux: u32,
    /// The configuration object.
    pub cfg: u32,
    /// The output object.
    pub out: u32,
    /// The view object.
    pub view: u32,
    /// The source object.
    pub src: u32,
    /// The opened session object.
    pub sess: u32,
    /// The refresh worker session object.
    pub sess2: u32,
    /// The list's parameter sub-object.
    pub tail: u32,
    /// The list's inner sub-object.
    pub inner: u32,
    /// The two probe objects.
    pub probes: [u32; 2],
    /// The configuration's sub-object.
    pub cfgsub: u32,
    /// The clip element pool.
    pub elems: [u32; 4],
    /// The helper answer pool (each writable past its flag byte).
    pub actives: [u32; 2],
    /// The blend target object.
    pub target: u32,
}

/// Names a value in the record: a known address by its role, a tag
/// constant by its letter, anything else by its hexadecimal value.
#[must_use]
pub fn sym(ctx: &Ctx, value: u32) -> String {
    if value == ctx.own {
        "own".to_string()
    } else if value == ctx.list {
        "list".to_string()
    } else if value == ctx.aux {
        "aux".to_string()
    } else if value == ctx.cfg {
        "cfg".to_string()
    } else if value == ctx.out {
        "out".to_string()
    } else if value == ctx.view {
        "view".to_string()
    } else if value == ctx.src {
        "src".to_string()
    } else if value == ctx.sess {
        "sess".to_string()
    } else if value == ctx.sess2 {
        "sess2".to_string()
    } else if value == ctx.tail {
        "tail".to_string()
    } else if value == ctx.inner {
        "inner".to_string()
    } else if value == ctx.cfgsub {
        "cfgsub".to_string()
    } else if value == ctx.target {
        "target".to_string()
    } else if let Some(index) = ctx.probes.iter().position(|&p| p == value) {
        format!("probe{index}")
    } else if let Some(index) = ctx.elems.iter().position(|&e| e == value) {
        format!("elem{index}")
    } else if let Some(index) = ctx.actives.iter().position(|&a| a == value) {
        format!("active{index}")
    } else {
        match value {
            0x00EF_AF04 => "tag_a".to_string(),
            0x00EF_AF14 => "tag_b".to_string(),
            0x00EF_AF24 => "tag_c".to_string(),
            0x00EF_AF44 => "tag_d".to_string(),
            0x00EF_AF50 => "tag_e".to_string(),
            0x00EF_AF60 => "tag_f".to_string(),
            0x00EF_AF80 => "tag_g".to_string(),
            0x00EF_AF90 => "tag_h".to_string(),
            0x00EF_AF9C => "tag_i".to_string(),
            0x00EF_AFAC => "tag_j".to_string(),
            _ => format!("{value:#x}"),
        }
    }
}

/// One callee or virtual slot of the rewrite, with the argument positions
/// that carry fixed values (a constant context, a fixed code, a trailing
/// zero) rather than inputs of the call.
#[derive(Debug, Clone, Copy)]
pub struct CallSpec {
    /// The callee number or virtual id the rewrite uses.
    pub id: u32,
    /// The name the lifted trait gives the call.
    pub name: &'static str,
    /// Argument positions dropped from the comparison.
    pub skip: &'static [usize],
}

/// The numbered callees of the state dispatcher.
pub const DISPATCH_CALLS: &[CallSpec] = &[
    CallSpec { id: 1, name: "open_session", skip: &[0] },
    CallSpec { id: 2, name: "tag_id", skip: &[] },
    CallSpec { id: 3, name: "activate", skip: &[0] },
    CallSpec { id: 4, name: "resolve_output", skip: &[0] },
    CallSpec { id: 5, name: "lookup", skip: &[0] },
    CallSpec { id: 6, name: "emit_a", skip: &[] },
    CallSpec { id: 7, name: "emit_b", skip: &[] },
    CallSpec { id: 8, name: "freshness", skip: &[] },
    CallSpec { id: 9, name: "note_teardown_mark", skip: &[0] },
    CallSpec { id: 10, name: "state_refresh", skip: &[0] },
    CallSpec { id: 11, name: "state_teardown", skip: &[0] },
    CallSpec { id: 12, name: "signal_out", skip: &[] },
    CallSpec { id: 13, name: "prepare", skip: &[1] },
    CallSpec { id: 14, name: "open_worker", skip: &[3] },
    CallSpec { id: 15, name: "note_rank", skip: &[] },
    CallSpec { id: 16, name: "note_session", skip: &[] },
    CallSpec { id: 17, name: "list_touch", skip: &[] },
    CallSpec { id: 18, name: "note_sync", skip: &[] },
    CallSpec { id: 19, name: "cfg_set", skip: &[] },
    CallSpec { id: 20, name: "blend_level", skip: &[] },
    CallSpec { id: 21, name: "release_output", skip: &[] },
    CallSpec { id: 22, name: "drop_pending", skip: &[] },
];

/// The virtual slots of the state dispatcher, by virtual id.
pub const DISPATCH_VIRTUALS: &[CallSpec] = &[
    CallSpec { id: 0x1001, name: "own_open", skip: &[] },
    CallSpec { id: 0x1002, name: "own_rearm", skip: &[] },
    CallSpec { id: 0x1003, name: "session_tag", skip: &[] },
    CallSpec { id: 0x1004, name: "session_first", skip: &[] },
    CallSpec { id: 0x1005, name: "session_mode", skip: &[] },
    CallSpec { id: 0x1006, name: "list_count", skip: &[] },
    CallSpec { id: 0x1007, name: "list_at", skip: &[] },
    CallSpec { id: 0x1008, name: "list_mode", skip: &[] },
    CallSpec { id: 0x1009, name: "list_rank", skip: &[] },
    CallSpec { id: 0x100A, name: "aux_set", skip: &[] },
    CallSpec { id: 0x100B, name: "tail_set", skip: &[] },
    CallSpec { id: 0x100C, name: "probe_open", skip: &[] },
    CallSpec { id: 0x100D, name: "prop_read", skip: &[] },
    CallSpec { id: 0x100E, name: "probe_prime", skip: &[] },
    CallSpec { id: 0x100F, name: "probe_set", skip: &[] },
    CallSpec { id: 0x1010, name: "view_set", skip: &[] },
    CallSpec { id: 0x1011, name: "cfg_sub_set", skip: &[] },
];

/// The answers a case scripts, by the call's name, popped in call order.
/// Both sides take their own copy.
pub type Script = HashMap<&'static str, VecDeque<u32>>;

/// The lifted side: records each call and answers from the script.
pub struct FakeWorld {
    /// The addresses the record names.
    pub ctx: Ctx,
    /// The scripted answers.
    pub script: Script,
    /// The recorded calls, in order.
    pub events: Vec<String>,
}

impl FakeWorld {
    /// A recorder over `ctx`, answering from `script`.
    #[must_use]
    pub fn new(ctx: Ctx, script: Script) -> Self {
        Self { ctx, script, events: Vec::new() }
    }

    fn answer(&mut self, name: &str) -> u32 {
        self.script.get_mut(name).and_then(VecDeque::pop_front).unwrap_or(0)
    }

    fn record(&mut self, name: &str, args: &[String]) {
        self.events.push(format!("{name}({})", args.join(",")));
    }

    fn word(&self, value: u32) -> String {
        sym(&self.ctx, value)
    }

    fn opt<T>(&self, handle: Option<Handle32<T>>) -> String {
        self.word(Handle32::raw_or_zero(handle))
    }
}

fn tag_name(tag: Tag) -> &'static str {
    match tag {
        Tag::A => "tag_a",
        Tag::B => "tag_b",
        Tag::C => "tag_c",
        Tag::D => "tag_d",
        Tag::E => "tag_e",
        Tag::F => "tag_f",
        Tag::G => "tag_g",
        Tag::H => "tag_h",
        Tag::I => "tag_i",
        Tag::J => "tag_j",
    }
}

impl MontageWorld for FakeWorld {
    fn open_session(&mut self, flag: u32) -> Option<Handle32<SessionTag>> {
        let f = self.word(flag);
        self.record("open_session", &[f]);
        Handle32::new(self.answer("open_session"))
    }
    fn own_open(&mut self, own: Handle32<OwnTag>) {
        let o = self.word(own.get());
        self.record("own_open", &[o]);
    }
    fn session_tag(&mut self, session: Option<Handle32<SessionTag>>) -> u32 {
        let s = self.opt(session);
        self.record("session_tag", &[s]);
        self.answer("session_tag")
    }
    fn tag_id(&mut self, tag: Tag) -> u32 {
        self.record("tag_id", &[tag_name(tag).to_string()]);
        self.answer("tag_id")
    }
    fn activate(&mut self, tag: Tag) {
        self.record("activate", &[tag_name(tag).to_string()]);
    }
    fn lookup(&mut self, tag: Tag) {
        self.record("lookup", &[tag_name(tag).to_string()]);
    }
    fn list_count(&mut self, list: Handle32<ListTag>) -> u32 {
        let l = self.word(list.get());
        self.record("list_count", &[l]);
        self.answer("list_count")
    }
    fn list_at(&mut self, list: Handle32<ListTag>, index: u32) -> Option<Handle32<ElemTag>> {
        let l = self.word(list.get());
        let i = self.word(index);
        self.record("list_at", &[l, i]);
        Handle32::new(self.answer("list_at"))
    }
    fn elem_word_a(&mut self, _elem: Option<Handle32<ElemTag>>) -> u32 {
        self.answer("elem_word_a")
    }
    fn elem_word_b(&mut self, _elem: Option<Handle32<ElemTag>>) -> u32 {
        self.answer("elem_word_b")
    }
    fn emit_a(&mut self, src: Handle32<SrcTag>, index: u32, word: u32) {
        let s = self.word(src.get());
        let i = self.word(index);
        let w = self.word(word);
        self.record("emit_a", &[s, i, w]);
    }
    fn emit_b(&mut self, src: Handle32<SrcTag>, index: u32, word: u32) {
        let s = self.word(src.get());
        let i = self.word(index);
        let w = self.word(word);
        self.record("emit_b", &[s, i, w]);
    }
    fn freshness(&mut self, src: Handle32<SrcTag>) -> u32 {
        let s = self.word(src.get());
        self.record("freshness", &[s]);
        self.answer("freshness")
    }
    fn note_teardown_mark(&mut self) {
        self.record("note_teardown_mark", &[]);
    }
    fn state_refresh(&mut self) {
        self.record("state_refresh", &[]);
    }
    fn state_teardown(&mut self) -> u32 {
        self.record("state_teardown", &[]);
        self.answer("state_teardown")
    }
    fn aux_set(&mut self, aux: Handle32<AuxTag>, mode: u32) {
        let a = self.word(aux.get());
        let m = self.word(mode);
        self.record("aux_set", &[a, m]);
    }
    fn list_tail(&mut self, _list: Handle32<ListTag>) -> Option<Handle32<TailTag>> {
        Handle32::new(self.answer("list_tail"))
    }
    fn tail_set(&mut self, tail: Option<Handle32<TailTag>>, mode: u32) {
        let t = self.opt(tail);
        let m = self.word(mode);
        self.record("tail_set", &[t, m]);
    }
    fn signal_out(&mut self, out: Handle32<OutTag>, flag: u32) {
        let o = self.word(out.get());
        let f = self.word(flag);
        self.record("signal_out", &[o, f]);
    }
    fn list_inner(&mut self, _list: Handle32<ListTag>) -> Option<Handle32<InnerTag>> {
        Handle32::new(self.answer("list_inner"))
    }
    fn probe_open(&mut self, inner: Option<Handle32<InnerTag>>) -> Option<Handle32<ProbeTag>> {
        let i = self.opt(inner);
        self.record("probe_open", &[i]);
        Handle32::new(self.answer("probe_open"))
    }
    fn probe_prime(&mut self, probe: Option<Handle32<ProbeTag>>) {
        let p = self.opt(probe);
        self.record("probe_prime", &[p]);
    }
    fn probe_set(&mut self, probe: Option<Handle32<ProbeTag>>, mode: u32) {
        let p = self.opt(probe);
        let m = self.word(mode);
        self.record("probe_set", &[p, m]);
    }
    fn prepare(&mut self, tag: Tag) {
        self.record("prepare", &[tag_name(tag).to_string()]);
    }
    fn open_worker(
        &mut self,
        own: Handle32<OwnTag>,
        first: Option<Handle32<SessionTag>>,
        handle: u32,
    ) -> Option<Handle32<SessionTag>> {
        let o = self.word(own.get());
        let f = self.opt(first);
        let h = self.word(handle);
        self.record("open_worker", &[o, f, h]);
        Handle32::new(self.answer("open_worker"))
    }
    fn session_first(&mut self, session: Option<Handle32<SessionTag>>) -> u32 {
        let s = self.opt(session);
        self.record("session_first", &[s]);
        self.answer("session_first")
    }
    fn list_rank(&mut self, list: Handle32<ListTag>, key: u32) -> u32 {
        let l = self.word(list.get());
        let k = self.word(key);
        self.record("list_rank", &[l, k]);
        self.answer("list_rank")
    }
    fn note_rank(&mut self, own: Handle32<OwnTag>, key: u32) {
        let o = self.word(own.get());
        let k = self.word(key);
        self.record("note_rank", &[o, k]);
    }
    fn note_session(&mut self, own: Handle32<OwnTag>, session: Option<Handle32<SessionTag>>) {
        let o = self.word(own.get());
        let s = self.opt(session);
        self.record("note_session", &[o, s]);
    }
    fn list_touch(&mut self, list: Handle32<ListTag>, key: u32, count: u32) {
        let l = self.word(list.get());
        let k = self.word(key);
        let c = self.word(count);
        self.record("list_touch", &[l, k, c]);
    }
    fn session_mode(&mut self, session: Option<Handle32<SessionTag>>, mode: u32) {
        let s = self.opt(session);
        let m = self.word(mode);
        self.record("session_mode", &[s, m]);
    }
    fn list_mode(&mut self, list: Handle32<ListTag>, mode: u32) {
        let l = self.word(list.get());
        let m = self.word(mode);
        self.record("list_mode", &[l, m]);
    }
    fn note_sync(&mut self, own: Handle32<OwnTag>) {
        let o = self.word(own.get());
        self.record("note_sync", &[o]);
    }
    fn prop_read(&mut self, inner: Option<Handle32<InnerTag>>) -> u32 {
        let i = self.opt(inner);
        self.record("prop_read", &[i]);
        self.answer("prop_read")
    }
    fn cfg_set(&mut self, cfg: Handle32<CfgTag>, prop: u32) {
        let c = self.word(cfg.get());
        let p = self.word(prop);
        self.record("cfg_set", &[c, p]);
    }
    fn blend_level(&mut self, src: Handle32<SrcTag>, prop: u32) -> u32 {
        let s = self.word(src.get());
        let p = self.word(prop);
        self.record("blend_level", &[s, p]);
        self.answer("blend_level")
    }
    fn resolve_output(&mut self, tag: Tag, blend: u32) -> Option<Handle32<OutputTag>> {
        let t = self.word(blend);
        self.record("resolve_output", &[tag_name(tag).to_string(), t]);
        Handle32::new(self.answer("resolve_output"))
    }
    fn release_output(&mut self, target: Option<Handle32<OutputTag>>) -> u32 {
        let t = self.opt(target);
        self.record("release_output", &[t]);
        self.answer("release_output")
    }
    fn view_set(&mut self, view: Handle32<ViewTag>, mode: u32) {
        let v = self.word(view.get());
        let m = self.word(mode);
        self.record("view_set", &[v, m]);
    }
    fn cfg_sub(&mut self, _cfg: Handle32<CfgTag>) -> Option<Handle32<CfgSubTag>> {
        Handle32::new(self.answer("cfg_sub"))
    }
    fn cfg_sub_set(&mut self, sub: Option<Handle32<CfgSubTag>>, mode: u32) {
        let s = self.opt(sub);
        let m = self.word(mode);
        self.record("cfg_sub_set", &[s, m]);
    }
    fn own_rearm(&mut self, own: Handle32<OwnTag>, mode: u32) -> u32 {
        let o = self.word(own.get());
        let m = self.word(mode);
        self.record("own_rearm", &[o, m]);
        self.answer("own_rearm")
    }
    fn drop_pending(&mut self, own: Handle32<OwnTag>, pending: u32) {
        let o = self.word(own.get());
        let p = self.word(pending);
        self.record("drop_pending", &[o, p]);
    }
}

/// Turns the rewrite's recorded calls into the lifted side's text.
///
/// `calls` lists each recorded call as (callee number or virtual id, raw
/// argument words), with `numbered` and `virtuals` naming them. Fixed
/// argument positions are dropped: they carry constants, which both sides
/// agree on by construction.
#[must_use]
pub fn normalise(
    ctx: &Ctx,
    numbered: &[CallSpec],
    virtuals: &[CallSpec],
    calls: &[(u32, Vec<u32>)],
) -> Vec<String> {
    calls
        .iter()
        .map(|(id, args)| {
            let spec = numbered
                .iter()
                .chain(virtuals.iter())
                .find(|c| c.id == *id)
                .unwrap_or_else(|| panic!("callee {id:#x} is not in the table"));
            let names: Vec<String> = args
                .iter()
                .enumerate()
                .filter(|(index, _)| !spec.skip.contains(index))
                .map(|(_, value)| sym(ctx, *value))
                .collect();
            format!("{}({})", spec.name, names.join(","))
        })
        .collect()
}
