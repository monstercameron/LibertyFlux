//! A coverage census over many programs: which opcodes, register files,
//! modifiers and declarations they use, and how many the translator
//! accepts, with the reason for every one it does not.
//!
//! The real-file test answers pass or fail for the game's shaders; the
//! census answers *what fraction translates, and what stops the rest* in
//! one run (`lf_dxso_dump --census` runs it over your own `.fxc` files).
//! Its numbers describe the game's files: keep the report on your own
//! machine; whether any figure from it is published is the project
//! owner's decision (`AGENTS.md`, rule 5).
//!
//! What is counted, and how exactly:
//!
//! * **The verdict is the translator's own** (Verified by the tests in
//!   this crate): every program is run through [`crate::translate_shader`]
//!   with the census's [`Options`], and a translated module through
//!   [`crate::validate::validate`]. A program counts as translated only
//!   when both accept it. Translation stops at the first construct it
//!   cannot handle, so a rejected program is counted once, under its first
//!   blocking reason; later unsupported constructs in the same program are
//!   not seen. [`Census::blocked_at`] names the instruction at the
//!   reported offset in listing syntax ([`crate::listing::mnemonic`]).
//! * Programs given as tokens or bytes that do not decode are counted
//!   with the decoder's error: [`Error::is_unsupported`] errors (other
//!   shader models, co-issue, pixel shader 1.x instructions) under
//!   [`Census::unsupported`], every other error under
//!   [`Census::decode_errors`]. They contribute nothing to the per-stage
//!   counts, which need a decoded program.
//! * Per-stage counts are static: each instruction once per program, as
//!   written, whether or not control flow reaches it. Register files count
//!   every operand (destination, predicate, sources, declared and defined
//!   registers) plus the `a0`/`aL` of every relative address;
//!   [`StageCensus::relative`] counts relatively addressed operands by the
//!   file they address. Keys are stable listing names (`texld`, `r#`,
//!   `abs neg`, `input texcoord8`), so two censuses diff cleanly.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;

use crate::binding;
use crate::decode::{
    Dcl, Opcode, Payload, RegType, Register, Shader, SrcMod, Stage, TextureType, decode,
};
use crate::listing::{mnemonic, register_file_name, register_name};
use crate::validate::validate;
use crate::{Error, Options, translate_shader};

/// Width of the key column in report tables.
const KEY_WIDTH: usize = 20;

/// How often something occurs: in total, and in how many programs.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Tally {
    /// Occurrences over all programs.
    pub uses: usize,
    /// Programs with at least one occurrence.
    pub programs: usize,
}

/// Static counts over the decoded programs of one stage.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct StageCensus {
    /// Decoded programs of this stage.
    pub programs: usize,
    /// Instructions in them (comments excluded, declarations included).
    pub instructions: usize,
    /// Instructions by opcode ([`Opcode::name`]).
    pub opcodes: BTreeMap<&'static str, Tally>,
    /// Operands by register file.
    pub register_files: BTreeMap<RegType, Tally>,
    /// Relatively addressed operands, by the file they address.
    pub relative: BTreeMap<RegType, Tally>,
    /// Source operands (and predicates) by modifier: `neg`, `abs`, `bx2`,
    /// `not`, ...; operands without a modifier are not counted.
    pub source_modifiers: BTreeMap<&'static str, Tally>,
    /// Instructions by result modifier: `sat`, `pp`, `centroid`, `shift`.
    pub result_modifiers: BTreeMap<&'static str, Tally>,
    /// `dcl` instructions by what they declare: `input texcoord0`,
    /// `output fog0`, `sampler cube`, `vFace`, ...
    pub declarations: BTreeMap<String, Tally>,
    /// Predicated instructions.
    pub predicated: Tally,
}

/// A census of many programs; see the [module documentation](self).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Census {
    /// The options every program is translated with.
    pub options: Options,
    /// Programs added, decoded or not.
    pub programs: usize,
    /// Programs that translated and passed the structural validator.
    pub translated: usize,
    /// Static counts over decoded vertex programs.
    pub vertex: StageCensus,
    /// Static counts over decoded pixel programs.
    pub pixel: StageCensus,
    /// Programs the translator or decoder rejects as unsupported, by
    /// reason (the [`Error::Unsupported`] text, without the offset).
    pub unsupported: BTreeMap<String, usize>,
    /// The same programs by the instruction where translation stopped
    /// (`dcl_texcoord8`, `texbem`, ...; `version token` for another
    /// shader model).
    pub blocked_at: BTreeMap<String, usize>,
    /// Programs the translator rejects as invalid, by reason.
    pub invalid: BTreeMap<String, usize>,
    /// Token streams that do not decode, by reason (unsupported decoder
    /// errors are under [`Census::unsupported`] instead).
    pub decode_errors: BTreeMap<String, usize>,
    /// Translated modules the structural validator rejects, by message;
    /// any entry here is a translator bug.
    pub validator_rejected: BTreeMap<String, usize>,
}

impl Default for Census {
    fn default() -> Self {
        Census::new(Options::default())
    }
}

/// Keys already counted for the program being added.
#[derive(Default)]
struct Seen {
    opcodes: BTreeSet<&'static str>,
    files: BTreeSet<RegType>,
    relative: BTreeSet<RegType>,
    source_modifiers: BTreeSet<&'static str>,
    result_modifiers: BTreeSet<&'static str>,
    declarations: BTreeSet<String>,
    predicated: bool,
}

/// Count one occurrence of `key`, and the program the first time.
fn bump<K: Ord + Clone>(map: &mut BTreeMap<K, Tally>, seen: &mut BTreeSet<K>, key: K) {
    let tally = map.entry(key.clone()).or_default();
    tally.uses += 1;
    if seen.insert(key) {
        tally.programs += 1;
    }
}

impl Census {
    /// An empty census that translates with `options`.
    #[must_use]
    pub fn new(options: Options) -> Census {
        Census {
            options,
            programs: 0,
            translated: 0,
            vertex: StageCensus::default(),
            pixel: StageCensus::default(),
            unsupported: BTreeMap::new(),
            blocked_at: BTreeMap::new(),
            invalid: BTreeMap::new(),
            decode_errors: BTreeMap::new(),
            validator_rejected: BTreeMap::new(),
        }
    }

    /// The static counts of one stage.
    #[must_use]
    pub fn stage(&self, stage: Stage) -> &StageCensus {
        match stage {
            Stage::Vertex => &self.vertex,
            Stage::Pixel => &self.pixel,
        }
    }

    /// Programs rejected as unsupported, by the decoder or the translator.
    #[must_use]
    pub fn unsupported_programs(&self) -> usize {
        self.unsupported.values().sum()
    }

    /// Programs rejected as invalid by the translator.
    #[must_use]
    pub fn invalid_programs(&self) -> usize {
        self.invalid.values().sum()
    }

    /// Token streams that did not decode (unsupported versions and
    /// instructions excluded; they count as unsupported).
    #[must_use]
    pub fn decode_failures(&self) -> usize {
        self.decode_errors.values().sum()
    }

    /// Translated modules the structural validator rejected.
    #[must_use]
    pub fn validator_failures(&self) -> usize {
        self.validator_rejected.values().sum()
    }

    /// Add one decoded program: count it, translate it, record the verdict.
    pub fn add_shader(&mut self, shader: &Shader) {
        self.programs += 1;
        self.count_static(shader);
        match translate_shader(shader, &self.options) {
            Ok(module) => match validate(&module.words) {
                Ok(_) => self.translated += 1,
                Err(e) => *self.validator_rejected.entry(e.message).or_default() += 1,
            },
            Err(e) => {
                let site = shader
                    .instructions
                    .iter()
                    .find(|ins| Some(ins.offset) == error_offset(&e))
                    .map_or_else(|| "end of program".to_string(), mnemonic);
                self.record_error(&e, site);
            }
        }
    }

    /// Add one program as a token stream, decoding it first.
    pub fn add_words(&mut self, words: &[u32]) {
        match decode(words) {
            Ok(shader) => self.add_shader(&shader),
            Err(e) => {
                self.programs += 1;
                let site = decoder_site(&e, words);
                self.record_error(&e, site);
            }
        }
    }

    /// Add one program as little-endian bytes (a container's bytecode).
    pub fn add_bytes(&mut self, bytes: &[u8]) {
        if !bytes.len().is_multiple_of(4) {
            self.programs += 1;
            self.record_error(&Error::BadLength(bytes.len()), "input length".to_string());
            return;
        }
        let words: Vec<u32> = bytes
            .chunks_exact(4)
            .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
            .collect();
        self.add_words(&words);
    }

    fn record_error(&mut self, e: &Error, site: String) {
        let key = error_key(e);
        if e.is_unsupported() {
            *self.unsupported.entry(key).or_default() += 1;
            *self.blocked_at.entry(site).or_default() += 1;
        } else if matches!(e, Error::Invalid { .. }) {
            *self.invalid.entry(key).or_default() += 1;
        } else {
            *self.decode_errors.entry(key).or_default() += 1;
        }
    }

    fn count_static(&mut self, shader: &Shader) {
        let stage = shader.stage;
        let st = match stage {
            Stage::Vertex => &mut self.vertex,
            Stage::Pixel => &mut self.pixel,
        };
        st.programs += 1;
        st.instructions += shader.instructions.len();
        let mut seen = Seen::default();
        for ins in &shader.instructions {
            bump(&mut st.opcodes, &mut seen.opcodes, ins.opcode.name());
            let operands = ins
                .dst
                .iter()
                .map(|d| (d.reg, d.relative))
                .chain(ins.predicate.iter().map(|p| (p.reg, p.relative)))
                .chain(ins.src.iter().map(|s| (s.reg, s.relative)));
            for (reg, relative) in operands {
                bump(&mut st.register_files, &mut seen.files, reg.kind);
                if let Some(rel) = relative {
                    bump(&mut st.register_files, &mut seen.files, rel.reg.kind);
                    bump(&mut st.relative, &mut seen.relative, reg.kind);
                }
            }
            for s in ins.src.iter().chain(ins.predicate.iter()) {
                if let Some(name) = source_modifier_name(s.modifier) {
                    bump(&mut st.source_modifiers, &mut seen.source_modifiers, name);
                }
            }
            if let Some(d) = &ins.dst {
                let flags = [
                    (d.saturate, "sat"),
                    (d.partial_precision, "pp"),
                    (d.centroid, "centroid"),
                    (d.shift != 0, "shift"),
                ];
                for (_, name) in flags.iter().filter(|(set, _)| *set) {
                    bump(&mut st.result_modifiers, &mut seen.result_modifiers, name);
                }
            }
            if ins.predicate.is_some() {
                st.predicated.uses += 1;
                if !seen.predicated {
                    seen.predicated = true;
                    st.predicated.programs += 1;
                }
            }
            if let Payload::Dcl(dcl) = &ins.payload {
                let key = declaration_key(dcl, stage);
                bump(&mut st.declarations, &mut seen.declarations, key);
            }
        }
    }

    /// A plain-text report: totals and the translated fraction, the
    /// rejection reasons by frequency, then each stage's tables sorted by
    /// frequency.
    #[must_use]
    pub fn report(&self) -> String {
        let mut out = String::new();
        let decoded = self.vertex.programs + self.pixel.programs;
        let _ = writeln!(
            out,
            "census of {} programs: {} vertex and {} pixel decoded, {} not decoded",
            self.programs,
            self.vertex.programs,
            self.pixel.programs,
            self.programs.saturating_sub(decoded)
        );
        let _ = writeln!(
            out,
            "translated and validated: {} of {} ({})",
            self.translated,
            self.programs,
            percent(self.translated, self.programs)
        );
        let unsupported = self.unsupported_programs();
        let _ = writeln!(
            out,
            "unsupported: {unsupported} ({})",
            percent(unsupported, self.programs)
        );
        let _ = writeln!(out, "invalid: {}", self.invalid_programs());
        let _ = writeln!(out, "decode errors: {}", self.decode_failures());
        let _ = writeln!(
            out,
            "rejected by the structural validator: {}",
            self.validator_failures()
        );
        for (title, map) in [
            ("unsupported, by reason", &self.unsupported),
            (
                "unsupported, by the instruction where translation stopped",
                &self.blocked_at,
            ),
            ("invalid, by reason", &self.invalid),
            ("decode errors, by reason", &self.decode_errors),
            ("structural validator, by message", &self.validator_rejected),
        ] {
            if map.is_empty() {
                continue;
            }
            let _ = writeln!(out, "\n{title} (programs):");
            let mut rows: Vec<(&String, &usize)> = map.iter().collect();
            rows.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
            for (reason, n) in rows {
                let _ = writeln!(out, "  {n:>8}  {reason}");
            }
        }
        for stage in [Stage::Vertex, Stage::Pixel] {
            stage_report(&mut out, stage, self.stage(stage));
        }
        out
    }
}

/// One stage's section of [`Census::report`].
fn stage_report(out: &mut String, stage: Stage, st: &StageCensus) {
    if st.programs == 0 {
        return;
    }
    let name = match stage {
        Stage::Vertex => "vertex",
        Stage::Pixel => "pixel",
    };
    let _ = writeln!(
        out,
        "\n{name} programs: {} decoded, {} instructions, {} predicated instructions in {} programs",
        st.programs, st.instructions, st.predicated.uses, st.predicated.programs
    );
    let opcodes: Vec<(String, Tally)> = st
        .opcodes
        .iter()
        .map(|(k, t)| ((*k).to_string(), *t))
        .collect();
    let files = |map: &BTreeMap<RegType, Tally>| -> Vec<(String, Tally)> {
        map.iter()
            .map(|(k, t)| (register_file_name(*k, stage).to_string(), *t))
            .collect()
    };
    let named = |map: &BTreeMap<&'static str, Tally>| -> Vec<(String, Tally)> {
        map.iter().map(|(k, t)| ((*k).to_string(), *t)).collect()
    };
    let declarations: Vec<(String, Tally)> = st
        .declarations
        .iter()
        .map(|(k, t)| (k.clone(), *t))
        .collect();
    for (title, rows) in [
        ("opcodes (instructions, programs)", opcodes),
        (
            "register files (operands, programs)",
            files(&st.register_files),
        ),
        (
            "relative addressing (operands, programs)",
            files(&st.relative),
        ),
        (
            "source modifiers (operands, programs)",
            named(&st.source_modifiers),
        ),
        (
            "result modifiers (instructions, programs)",
            named(&st.result_modifiers),
        ),
        ("declarations (instructions, programs)", declarations),
    ] {
        if rows.is_empty() {
            continue;
        }
        let _ = writeln!(out, "  {title}:");
        let mut rows = rows;
        rows.sort_by(|a, b| b.1.uses.cmp(&a.1.uses).then_with(|| a.0.cmp(&b.0)));
        for (key, t) in rows {
            let _ = writeln!(out, "    {key:<KEY_WIDTH$} {:>8} {:>8}", t.uses, t.programs);
        }
    }
}

/// `66.6%` (rounded down to a tenth), or `n/a` for an empty census.
fn percent(part: usize, whole: usize) -> String {
    if whole == 0 {
        return "n/a".to_string();
    }
    let per_mille = part.saturating_mul(1000) / whole;
    format!("{}.{}%", per_mille / 10, per_mille % 10)
}

/// An error as a histogram key: its message without the offset.
fn error_key(e: &Error) -> String {
    match e {
        Error::BadLength(_) => "input length is not a whole number of tokens".to_string(),
        Error::BadVersion(v) => format!("bad version token 0x{v:08x}"),
        Error::UnsupportedVersion { major, minor } => {
            format!("shader model {major}.{minor} (3.0 only)")
        }
        Error::MissingEnd => "no end token".to_string(),
        Error::Truncated { .. } => "a token runs past the end".to_string(),
        Error::UnknownOpcode { opcode, .. } => format!("unknown opcode {opcode}"),
        Error::BadInstruction { reason, .. } => format!("malformed instruction: {reason}"),
        Error::Invalid { reason, .. } => reason.clone(),
        Error::Unsupported { what, .. } => what.clone(),
    }
}

/// The offset an error names, if it names one.
fn error_offset(e: &Error) -> Option<usize> {
    match e {
        Error::Truncated { offset }
        | Error::UnknownOpcode { offset, .. }
        | Error::BadInstruction { offset, .. }
        | Error::Invalid { offset, .. }
        | Error::Unsupported { offset, .. } => Some(*offset),
        Error::BadLength(_)
        | Error::BadVersion(_)
        | Error::UnsupportedVersion { .. }
        | Error::MissingEnd => None,
    }
}

/// Where the decoder stopped: the opcode of the instruction token at the
/// error's offset, or what part of the stream it rejected.
fn decoder_site(e: &Error, words: &[u32]) -> String {
    match e {
        Error::UnsupportedVersion { .. } | Error::BadVersion(_) => "version token".to_string(),
        _ => error_offset(e)
            .and_then(|at| words.get(at))
            .and_then(|&token| Opcode::from_raw((token & 0xFFFF) as u16))
            .map_or_else(|| "token stream".to_string(), |o| o.name().to_string()),
    }
}

/// `neg`, `abs neg`, `bx2`, ...; `None` for no modifier.
fn source_modifier_name(modifier: SrcMod) -> Option<&'static str> {
    Some(match modifier {
        SrcMod::None => return None,
        SrcMod::Neg => "neg",
        SrcMod::Bias => "bias",
        SrcMod::BiasNeg => "bias neg",
        SrcMod::Sign => "bx2",
        SrcMod::SignNeg => "bx2 neg",
        SrcMod::Comp => "comp",
        SrcMod::X2 => "x2",
        SrcMod::X2Neg => "x2 neg",
        SrcMod::Dz => "dz",
        SrcMod::Dw => "dw",
        SrcMod::Abs => "abs",
        SrcMod::AbsNeg => "abs neg",
        SrcMod::Not => "not",
    })
}

/// `input texcoord1`, `output position0`, `sampler cube`, `vFace`, ...
fn declaration_key(dcl: &Dcl, stage: Stage) -> String {
    let reg: Register = dcl.dst.reg;
    match reg.kind {
        RegType::Input | RegType::Output => {
            let direction = if reg.kind == RegType::Input {
                "input"
            } else {
                "output"
            };
            match binding::usage_name(dcl.usage) {
                "unknown" => format!("{direction} usage{}_{}", dcl.usage, dcl.usage_index),
                name => format!("{direction} {name}{}", dcl.usage_index),
            }
        }
        RegType::Sampler => match dcl.texture_type {
            TextureType::Tex2d => "sampler 2d".to_string(),
            TextureType::Cube => "sampler cube".to_string(),
            TextureType::Volume => "sampler volume".to_string(),
            TextureType::Unknown(t) => format!("sampler type {t}"),
        },
        RegType::MiscType => register_name(reg, stage),
        other => format!("dcl of {}", register_file_name(other, stage)),
    }
}
