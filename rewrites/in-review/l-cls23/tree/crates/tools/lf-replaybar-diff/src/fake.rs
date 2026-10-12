//! The recorder both sides of a differential case write their calls into.
//!
//! The lifted methods run against [`FakeWorld`], which records each
//! collaborator call as text with its arguments named by [`sym`]. The
//! verified rewrite runs against the runtime's stubs, whose recorded
//! numbered calls are turned into the same text by [`normalise`]. Both
//! sides then answer from the same scripted answers, so a difference in
//! order, arguments or results shows up as a difference in the text.

use std::collections::{HashMap, VecDeque};

use lf_core::Handle32;
use lf_input_frontend::replay_progress_bar::{
    BarTag, InnerTag, OwnedTag, Part, ReplayBarWorld,
};

/// One scripted answer: the call's return word, and the words a call writes
/// through its out-pointer arguments, in order.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Answer {
    /// The call's return word.
    pub ret: u32,
    /// The words written through the out-pointer arguments, in order.
    pub outs: [u32; 3],
}

/// The addresses a case works with, so the record can name them.
#[derive(Debug, Clone, Copy)]
pub struct Ctx {
    /// The progress bar object.
    pub bar: u32,
    /// The sub-object at the bar's offset `0x1c`.
    pub sub1c: u32,
    /// The inner object.
    pub inner: u32,
    /// The five owned collaborator slots' values (zero when empty).
    pub owned: [u32; 5],
    /// The token argument cell, passed by the teardown's open call.
    pub token_cell: u32,
}

/// Names a value in the record: a known address by its role, anything else
/// by its hexadecimal value.
#[must_use]
pub fn sym(ctx: &Ctx, value: u32) -> String {
    if value == ctx.bar {
        "bar".to_string()
    } else if value == ctx.sub1c {
        "sub1c".to_string()
    } else if value == ctx.inner {
        "inner".to_string()
    } else if value == ctx.token_cell {
        "tok".to_string()
    } else if let Some(index) = ctx.owned.iter().position(|&o| o != 0 && o == value) {
        format!("own{index}")
    } else {
        format!("{value:#x}")
    }
}

/// One numbered callee of a rewrite, with the argument positions that are
/// out-pointers (results, not inputs) in that call.
#[derive(Debug, Clone, Copy)]
pub struct CallSpec {
    /// The callee number the rewrite uses.
    pub id: u32,
    /// The name the lifted trait gives the call.
    pub name: &'static str,
    /// Argument positions that carry out-pointers; their values are the
    /// words written there, not inputs of the call.
    pub outs: &'static [usize],
}

/// The numbered callees of the teardown routine.
pub const TEARDOWN_CALLS: &[CallSpec] = &[
    CallSpec { id: 1, name: "base_teardown", outs: &[] },
    CallSpec { id: 2, name: "open_token", outs: &[] },
    CallSpec { id: 3, name: "attach_token", outs: &[] },
    CallSpec { id: 4, name: "destruct", outs: &[] },
    CallSpec { id: 5, name: "free_owned", outs: &[] },
    CallSpec { id: 6, name: "close_token", outs: &[] },
    CallSpec { id: 7, name: "finalize_a", outs: &[] },
    CallSpec { id: 8, name: "finalize_b", outs: &[] },
];

/// The numbered callees of the deleting destructor, apart from its
/// teardown call (callee 1, which runs the teardown rewrite).
pub const DELETING_CALLS: &[CallSpec] = &[CallSpec {
    id: 2,
    name: "release_storage",
    outs: &[],
}];

/// The numbered callees of the update.
pub const UPDATE_CALLS: &[CallSpec] = &[
    CallSpec { id: 1, name: "setup_inner", outs: &[] },
    CallSpec { id: 2, name: "produce", outs: &[1, 2, 3] },
    CallSpec { id: 3, name: "fill_pair", outs: &[0, 1] },
    CallSpec { id: 4, name: "consume", outs: &[] },
    CallSpec { id: 5, name: "measure", outs: &[] },
    CallSpec { id: 6, name: "use_a", outs: &[] },
    CallSpec { id: 7, name: "use_b", outs: &[] },
    CallSpec { id: 8, name: "use_c", outs: &[] },
    CallSpec { id: 9, name: "use_d", outs: &[] },
    CallSpec { id: 10, name: "use_e", outs: &[] },
    CallSpec { id: 11, name: "combine", outs: &[] },
    CallSpec { id: 12, name: "step", outs: &[] },
    CallSpec { id: 13, name: "compare", outs: &[] },
    CallSpec { id: 14, name: "signal", outs: &[] },
    CallSpec { id: 15, name: "push_s4", outs: &[] },
    CallSpec { id: 16, name: "gate_a", outs: &[] },
    CallSpec { id: 17, name: "gate_b", outs: &[] },
    CallSpec { id: 18, name: "gate_c", outs: &[] },
    CallSpec { id: 19, name: "finish", outs: &[] },
    CallSpec { id: 20, name: "base_update", outs: &[] },
];

/// The answers a case scripts, by the call's name, popped in call order.
/// Both sides take their own copy.
pub type Script = HashMap<&'static str, VecDeque<Answer>>;

/// The lifted side: records each call and answers from the script.
pub struct FakeWorld {
    /// The addresses the record names.
    pub ctx: Ctx,
    /// The inner object's flag byte, as the case set it in memory.
    pub inner_flag: bool,
    /// The scripted answers.
    pub script: Script,
    /// The recorded calls, in order.
    pub events: Vec<String>,
}

impl FakeWorld {
    /// A recorder over `ctx`, answering from `script`.
    #[must_use]
    pub fn new(ctx: Ctx, inner_flag: bool, script: Script) -> Self {
        Self {
            ctx,
            inner_flag,
            script,
            events: Vec::new(),
        }
    }

    fn answer(&mut self, name: &str) -> Answer {
        self.script
            .get_mut(name)
            .and_then(VecDeque::pop_front)
            .unwrap_or_default()
    }

    fn record(&mut self, name: &str, args: &[String]) {
        self.events.push(format!("{name}({})", args.join(",")));
    }

    fn word(&self, value: u32) -> String {
        sym(&self.ctx, value)
    }

    fn handle_bar(&self, bar: Handle32<BarTag>) -> String {
        self.word(bar.get())
    }

    fn handle_owned(&self, owned: Handle32<OwnedTag>) -> String {
        self.word(owned.get())
    }
}

fn part_name(part: Part) -> &'static str {
    match part {
        Part::Whole => "bar",
        Part::Sub1c => "sub1c",
    }
}

impl ReplayBarWorld for FakeWorld {
    fn base_teardown(&mut self, bar: Handle32<BarTag>) {
        let a = self.handle_bar(bar);
        self.record("base_teardown", &[a]);
    }

    fn open_token(&mut self) -> u32 {
        let tok = self.word(self.ctx.token_cell);
        self.record("open_token", &[tok]);
        self.answer("open_token").ret
    }

    fn attach_token(&mut self, token: u32) {
        let t = self.word(token);
        self.record("attach_token", &[t]);
    }

    fn destruct(&mut self, owned: Handle32<OwnedTag>) {
        let o = self.handle_owned(owned);
        self.record("destruct", &[o]);
    }

    fn free_owned(&mut self, owned: Handle32<OwnedTag>) {
        let o = self.handle_owned(owned);
        self.record("free_owned", &[o]);
    }

    fn close_token(&mut self, token: u32) {
        let t = self.word(token);
        self.record("close_token", &[t]);
    }

    fn finalize_a(&mut self) {
        self.record("finalize_a", &[]);
    }

    fn finalize_b(&mut self) {
        self.record("finalize_b", &[]);
    }

    fn release_storage(&mut self, bar: Handle32<BarTag>) {
        let a = self.handle_bar(bar);
        self.record("release_storage", &[a]);
    }

    fn setup_inner(&mut self, inner: Handle32<InnerTag>, arg: u32) {
        let i = self.word(inner.get());
        let z = self.word(arg);
        self.record("setup_inner", &[i, z]);
    }

    fn produce(&mut self, bar: Handle32<BarTag>) -> [u32; 3] {
        let a = self.handle_bar(bar);
        self.record("produce", &[a]);
        self.answer("produce").outs
    }

    fn fill_pair(&mut self) -> (u32, u32, u32) {
        self.record("fill_pair", &[]);
        let answer = self.answer("fill_pair");
        (answer.ret, answer.outs[0], answer.outs[1])
    }

    fn consume(
        &mut self,
        bar: Handle32<BarTag>,
        token: u32,
        _first: &mut u32,
        _second: &mut u32,
    ) {
        let a = self.handle_bar(bar);
        let t = self.word(token);
        self.record("consume", &[a, t]);
    }

    fn measure(&mut self) -> u32 {
        self.record("measure", &[]);
        self.answer("measure").ret
    }

    fn use_a(&mut self, a: u32) {
        let x = self.word(a);
        self.record("use_a", &[x]);
    }

    fn use_b(&mut self, bar: Handle32<BarTag>, a: u32, second: u32) -> u32 {
        let b = self.handle_bar(bar);
        let x = self.word(a);
        let z = self.word(second);
        self.record("use_b", &[b, x, z]);
        self.answer("use_b").ret
    }

    fn use_c(&mut self, b: u32) {
        let x = self.word(b);
        self.record("use_c", &[x]);
    }

    fn use_d(&mut self, c: u32, b: u32) {
        let x = self.word(c);
        let y = self.word(b);
        self.record("use_d", &[x, y]);
    }

    fn use_e(&mut self, d: u32) {
        let x = self.word(d);
        self.record("use_e", &[x]);
    }

    fn combine(&mut self, part: Part, first: u32, second: u32) {
        let f = self.word(first);
        let s = self.word(second);
        self.record("combine", &[part_name(part).to_string(), f, s]);
    }

    fn step(&mut self, bar: Handle32<BarTag>) {
        let a = self.handle_bar(bar);
        self.record("step", &[a]);
    }

    fn compare(&mut self) -> u32 {
        self.record("compare", &[]);
        self.answer("compare").ret
    }

    fn signal(&mut self, code: u32) {
        let c = self.word(code);
        self.record("signal", &[c]);
    }

    fn push_s4(&mut self, bar: Handle32<BarTag>, word: u32) {
        let a = self.handle_bar(bar);
        let w = self.word(word);
        self.record("push_s4", &[a, w]);
    }

    fn inner_flag_set(&mut self, _inner: Handle32<InnerTag>) -> bool {
        self.inner_flag
    }

    fn gate_a(&mut self, bar: Handle32<BarTag>) {
        let a = self.handle_bar(bar);
        self.record("gate_a", &[a]);
    }

    fn gate_b(&mut self, bar: Handle32<BarTag>) {
        let a = self.handle_bar(bar);
        self.record("gate_b", &[a]);
    }

    fn gate_c(&mut self, bar: Handle32<BarTag>) {
        let a = self.handle_bar(bar);
        self.record("gate_c", &[a]);
    }

    fn finish(&mut self, bar: Handle32<BarTag>) {
        let a = self.handle_bar(bar);
        self.record("finish", &[a]);
    }

    fn base_update(&mut self, bar: Handle32<BarTag>) -> u32 {
        let a = self.handle_bar(bar);
        self.record("base_update", &[a]);
        self.answer("base_update").ret
    }
}

/// Turns the rewrite's recorded numbered calls into the lifted side's text.
///
/// `calls` lists each recorded call as (callee number, raw argument words),
/// with `table` naming the callees of that rewrite. Out-pointer arguments
/// are dropped: they carry results, which both sides compare through the
/// returned words and the globals.
#[must_use]
pub fn normalise(ctx: &Ctx, table: &[CallSpec], calls: &[(u32, Vec<u32>)]) -> Vec<String> {
    calls
        .iter()
        .map(|(id, args)| {
            let spec = table
                .iter()
                .find(|c| c.id == *id)
                .unwrap_or_else(|| panic!("callee {id} is not in the table"));
            let names: Vec<String> = args
                .iter()
                .enumerate()
                .filter(|(index, _)| !spec.outs.contains(index) && !(spec.name == "consume" && **index >= 2))
                .map(|(_, value)| sym(ctx, *value))
                .collect();
            format!("{}({})", spec.name, names.join(","))
        })
        .collect()
}
