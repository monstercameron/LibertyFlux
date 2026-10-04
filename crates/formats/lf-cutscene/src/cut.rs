//! Parser for the `.cut` tag text format.
//!
//! See the [crate-level documentation](crate) for the format description.

use crate::Error;

/// Maximum accepted input size in bytes (64 MiB; shipped files are ≤ 36 KiB).
pub const MAX_INPUT: u64 = 64 * 1024 * 1024;

/// Default for the missing fourth `DRAW_DISTANCE` value, per the wiki.
pub const DRAW_DISTANCE_DEFAULT_UNKNOWN: f32 = 0.05;

/// One subtitle line: start time, on-screen length and `.gxt` key.
#[derive(Clone, Debug, PartialEq)]
pub struct TextEntry {
    /// Start time in milliseconds.
    pub start_ms: i32,
    /// On-screen duration in milliseconds.
    pub len_ms: i32,
    /// Localisation key into the `.gxt` text databases.
    pub key: String,
}

/// One `MODELS` row: an object taking part in the cutscene.
#[derive(Clone, Debug, PartialEq)]
pub struct ModelEntry {
    /// Object identifier used by the other tags.
    pub id: i32,
    /// Model name (cutscene models live in `cutsprops.img`).
    pub model: String,
    /// Animation name for this object inside the section's `.wad`.
    pub anim: String,
    /// Face animation name inside the `.wad`, when present.
    pub head_anim: Option<String>,
    /// Trailing value, meaning unknown (usually `0`).
    pub unknown: Option<i32>,
}

/// One `DRAW_DISTANCE` row.
#[derive(Clone, Debug, PartialEq)]
pub struct DrawDistance {
    /// Time in milliseconds at which this distance takes effect.
    pub time_ms: i32,
    /// Second value, ignored by the game. Shipped files use either a running
    /// index or an end time; both conventions are preserved as read.
    pub second: i32,
    /// Visibility distance.
    pub distance: f32,
    /// Fourth value ([`DRAW_DISTANCE_DEFAULT_UNKNOWN`] when the row omits it).
    pub unknown: f32,
}

/// One `BLOCKING_BOUNDS` row: a quadrilateral no-spawn zone.
#[derive(Clone, Debug, PartialEq)]
pub struct BlockingBounds {
    /// The four corners (AA, AB, BA, BB) as x/y/z triples.
    pub corners: [[f32; 3]; 4],
    /// Zone height.
    pub height: f32,
}

/// One `CAMCORDER` row: a camera-mode time window in milliseconds.
#[derive(Clone, Debug, PartialEq)]
pub struct Camcorder {
    /// Start time in milliseconds.
    pub start_ms: i32,
    /// End time in milliseconds.
    pub end_ms: i32,
}

/// One `VEHICLE_DETAILS` row: paint and trim for a vehicle model.
#[derive(Clone, Debug, PartialEq)]
pub struct VehicleDetails {
    /// Model identifier from the `MODELS` tag.
    pub vehicle_id: i32,
    /// Four colour identifiers from `carcols.dat`.
    pub colors: [i32; 4],
    /// Sixth value, meaning unknown (usually `0`).
    ///
    /// The wiki documents seven fields here; shipped files carry six at most.
    pub last: i32,
}

/// One `VEHICLE_REMOVAL` row: a vehicle part hidden during the cutscene.
#[derive(Clone, Debug, PartialEq)]
pub struct VehicleRemoval {
    /// Model identifier from the `MODELS` tag.
    pub model_index: i32,
    /// Bone identifier of the removed part.
    pub bone_id: i32,
}

/// One `PROPS` row.
#[derive(Clone, Debug, PartialEq)]
pub struct Prop {
    /// Model identifier from the `MODELS` tag.
    pub model_id: i32,
    /// Second value, meaning unknown.
    pub index: i32,
    /// Third value, meaning unknown.
    pub prop: i32,
}

/// One `VARIATION` row: four or five integers, meaning unknown.
#[derive(Clone, Debug, PartialEq)]
pub struct Variation {
    /// The row's integer values as read.
    pub values: Vec<i32>,
}

/// One `COMPRESSION` row: a per-model compression mode.
#[derive(Clone, Debug, PartialEq)]
pub struct Compression {
    /// Model identifier from the `MODELS` tag.
    pub model_id: i32,
    /// Mode letter (`A` on nearly every shipped row).
    pub mode: String,
}

/// One `ANIMRANGE` row: the frame window the section plays.
#[derive(Clone, Debug, PartialEq)]
pub struct AnimRange {
    /// First frame.
    pub start: f32,
    /// Last frame.
    pub end: f32,
}

/// One `EFFECTS` row: a particle effect trigger.
#[derive(Clone, Debug, PartialEq)]
pub struct Effect {
    /// Effect name, usually `emitter:effect` form.
    pub name: String,
    /// First object reference (`-1` when unattached).
    pub object_a: i32,
    /// Second object reference (`-1` when unattached).
    pub object_b: i32,
    /// Trigger time in milliseconds.
    pub start_ms: i32,
    /// End time in milliseconds.
    pub end_ms: i32,
    /// Trigger position.
    pub position: [f32; 3],
    /// Trigger rotation.
    pub rotation: [f32; 3],
    /// Twelfth value, meaning unknown.
    pub unknown: f32,
    /// Trailing tokens past the twelfth (two shipped rows carry two more).
    pub extra: Vec<String>,
}

/// One `REMOVE` row: a world object hidden during the cutscene.
#[derive(Clone, Debug, PartialEq)]
pub struct Remove {
    /// World position of the removed object.
    pub position: [f32; 3],
    /// Model name of the removed object.
    pub model: String,
    /// Trailing value, meaning unknown. Shipped files spell it both ways
    /// (`10` and `10.0`), so it is parsed as a float.
    pub unknown: Option<f32>,
}

/// A tag the parser does not know, preserved verbatim.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnknownSection {
    /// Tag name as read (without brackets).
    pub tag: String,
    /// Content lines as read.
    pub lines: Vec<String>,
    /// 1-based line number of the opening tag.
    pub line: usize,
}

/// A content row that failed typed parsing, preserved with its reason.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BadRow {
    /// Tag the row appeared under.
    pub tag: String,
    /// 1-based line number of the row.
    pub line: usize,
    /// The row as read.
    pub text: String,
    /// Why typed parsing failed.
    pub reason: String,
}

/// A content line found outside any tag, preserved with its line number.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrphanLine {
    /// 1-based line number.
    pub line: usize,
    /// The line as read.
    pub text: String,
}

/// Kinds of recoverable problems recorded in [`CutsceneFile::warnings`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WarningKind {
    /// A close tag with no matching open tag.
    StrayClose,
    /// A close tag that names a different tag than the open one.
    MismatchedClose,
    /// An open tag while another tag was still open; the old one was closed.
    ImplicitClose,
    /// A bracket line that is not a well-formed tag.
    MalformedTag,
    /// A content line outside any tag.
    OrphanContent,
    /// A content row that failed typed parsing (also kept in `bad_rows`).
    BadRow,
}

/// One recoverable problem found while parsing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Warning {
    /// 1-based line number.
    pub line: usize,
    /// What kind of problem this is.
    pub kind: WarningKind,
    /// Human-readable detail.
    pub message: String,
}

/// One animation section: everything between a marker pair.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Section {
    /// `MODELS` rows.
    pub models: Vec<ModelEntry>,
    /// `COMPRESSION` rows.
    pub compression: Vec<Compression>,
    /// `LIGHTS` animation names.
    pub lights: Vec<String>,
    /// `EFFECTS` rows.
    pub effects: Vec<Effect>,
    /// `VEHICLE_DETAILS` rows.
    pub vehicle_details: Vec<VehicleDetails>,
    /// `VEHICLE_REMOVAL` rows.
    pub vehicle_removals: Vec<VehicleRemoval>,
    /// `ORIENT` headings in degrees.
    pub orient: Vec<f32>,
    /// Raw `TIME` lines (empty in every shipped file).
    pub time_lines: Vec<String>,
    /// `OFFSET` cutscene origins.
    pub offset: Vec<[f32; 3]>,
    /// `DURATION` section lengths in milliseconds.
    pub durations_ms: Vec<f32>,
    /// `AUDIO` bank names.
    pub audios: Vec<String>,
    /// `ANIM` animation-file stems.
    pub anims: Vec<String>,
    /// `ANIMRANGE` rows.
    pub anim_ranges: Vec<AnimRange>,
    /// `CAMERA` animation names.
    pub cameras: Vec<String>,
    /// `REMOVE` rows.
    pub removes: Vec<Remove>,
    /// `VARIATION` rows appearing inside the section (shipped files keep
    /// them at cutscene level instead).
    pub variations: Vec<Variation>,
    /// `PROPS` rows appearing inside the section (shipped files keep
    /// them at cutscene level instead).
    pub props: Vec<Prop>,
    /// Unknown tags appearing inside the section.
    pub unknown: Vec<UnknownSection>,
    /// Rows that failed typed parsing.
    pub bad_rows: Vec<BadRow>,
}

impl Section {
    /// Total of the `DURATION` values in milliseconds.
    #[must_use]
    pub fn duration_ms(&self) -> f32 {
        self.durations_ms.iter().sum()
    }
}

/// One cutscene: file-level tags plus animation sections.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Cutscene {
    /// `CUTSCENE_HEADER` frame boundaries (ascending).
    pub header_frames: Vec<i32>,
    /// `FLAGS` names.
    pub flags: Vec<String>,
    /// `BLOCKING_BOUNDS` rows.
    pub blocking_bounds: Vec<BlockingBounds>,
    /// `CAMCORDER` rows.
    pub camcorder: Vec<Camcorder>,
    /// `MISSION_TEXT_NAME` labels.
    pub mission_text_name: Vec<String>,
    /// `TEXT` subtitle rows.
    pub texts: Vec<TextEntry>,
    /// `PLAYER_START` positions.
    pub player_starts: Vec<[f32; 3]>,
    /// `VARIATION` rows.
    pub variations: Vec<Variation>,
    /// `PROPS` rows.
    pub props: Vec<Prop>,
    /// `DRAW_DISTANCE` rows.
    pub draw_distance: Vec<DrawDistance>,
    /// `MAX_PEDS` values.
    pub max_peds: Vec<i32>,
    /// `MAX_CARS` values.
    pub max_cars: Vec<i32>,
    /// `TIMECYCLE_MODIFIER_NAME` values.
    pub timecycle_modifiers: Vec<String>,
    /// Animation sections in file order.
    pub sections: Vec<Section>,
    /// Unknown tags at cutscene level.
    pub unknown: Vec<UnknownSection>,
    /// Rows that failed typed parsing.
    pub bad_rows: Vec<BadRow>,
}

impl Cutscene {
    /// Sum of the section durations in milliseconds.
    pub fn duration_ms(&self) -> f32 {
        self.sections.iter().map(Section::duration_ms).sum()
    }

    /// Number of `MODELS` rows across all sections.
    #[must_use]
    pub fn model_count(&self) -> usize {
        self.sections.iter().map(|s| s.models.len()).sum()
    }
}

/// A parsed `.cut` file.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CutsceneFile {
    /// Cutscenes in file order (split on `CUTSCENE_HEADER`).
    pub groups: Vec<Cutscene>,
    /// Recoverable problems found while parsing.
    pub warnings: Vec<Warning>,
    /// Content lines found outside any tag.
    pub orphans: Vec<OrphanLine>,
    /// Trailing bytes after the last text line (block padding and fill).
    pub trailing_slack_len: usize,
}

impl CutsceneFile {
    /// Parse a `.cut` payload.
    ///
    /// Never fails on tag-level problems: those become [`Warning`]s, unknown
    /// tags stay in `unknown` and mistyped rows stay in `bad_rows`. Only an
    /// input larger than [`MAX_INPUT`] is a hard error.
    ///
    /// # Errors
    ///
    /// Returns [`Error::TooLarge`] when the input exceeds [`MAX_INPUT`]
    /// bytes; every other malformed shape becomes warnings, not an error.
    ///
    /// # Panics
    ///
    /// Never panics: every index comes from a length just pushed, every
    /// `expect` unwraps a `Some` established by the same state
    /// transition (open tag, section entry, or just-pushed unknown
    /// section), and string slicing only cuts at ASCII delimiters.
    // One single-pass line parser; splitting it would scatter the open-tag,
    // group and section state across helpers.
    #[allow(clippy::too_many_lines)]
    pub fn parse(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() as u64 > MAX_INPUT {
            return Err(Error::TooLarge {
                size: bytes.len() as u64,
            });
        }
        let mut file = CutsceneFile::default();
        // Split into lines on LF, tracking byte offsets.
        let mut start = 0usize;
        let mut line_no = 0usize;
        let mut last_text_end = 0usize;
        // Parser state.
        let mut group_idx: Option<usize> = None;
        let mut section_idx: Option<usize> = None;
        // Currently open tag: (name, is_unknown, unknown_index).
        let mut open: Option<OpenTag> = None;

        let mut lines: Vec<(usize, &[u8])> = Vec::new();
        for (i, b) in bytes.iter().enumerate() {
            if *b == b'\n' {
                lines.push((start, &bytes[start..i]));
                start = i + 1;
            }
        }
        if start < bytes.len() {
            lines.push((start, &bytes[start..]));
        }

        // Pre-scan: index of the last tag line. Stray bytes after it are
        // block padding, so non-tag lines out there are skipped silently
        // instead of being reported as orphans.
        let mut last_tag_idx: Option<usize> = None;
        for (idx, (_, raw)) in lines.iter().enumerate() {
            let trimmed_end = trim_trailing_nulls(raw);
            if trimmed_end.contains(&0) {
                continue;
            }
            let Ok(text) = std::str::from_utf8(trimmed_end) else {
                continue;
            };
            let line = text.strip_suffix('\r').unwrap_or(text).trim();
            if !line.is_empty() && parse_tag_line(line).is_some() {
                last_tag_idx = Some(idx);
            }
        }

        for (idx, (offset, raw)) in lines.iter().enumerate() {
            let (offset, raw) = (*offset, *raw);
            line_no += 1;
            // Binary slack rule: interior NUL or invalid UTF-8 ends parsing.
            // Padding is the normal end of file, so this is silent; the
            // skipped tail is counted in `trailing_slack_len`.
            let trimmed_end = trim_trailing_nulls(raw);
            if trimmed_end.contains(&0) {
                break;
            }
            let Ok(text) = std::str::from_utf8(trimmed_end) else {
                break;
            };
            let line = text.strip_suffix('\r').unwrap_or(text).trim();
            if line.is_empty() {
                continue;
            }
            let tag = parse_tag_line(line);
            if tag.is_none()
                && (group_idx.is_none() || open.is_none())
                && last_tag_idx.is_some_and(|li| idx > li)
            {
                // Stray bytes after the final tag: block padding. Skip
                // silently (without moving `last_text_end`, so the bytes
                // count as slack).
                continue;
            }
            let mut content_len = trimmed_end.len();
            if trimmed_end.ends_with(b"\r") {
                content_len -= 1;
            }
            let newline = usize::from(offset + raw.len() < bytes.len());
            last_text_end = offset + content_len + newline;
            if let Some(tag) = tag {
                match tag {
                    TagLine::Open(name) => {
                        if let Some(prev) = open.take() {
                            file.warn(
                                line_no,
                                WarningKind::ImplicitClose,
                                format!("{} implicitly closes {}", name, prev.name),
                            );
                        }
                        match name {
                            "CUTSCENE_HEADER" => {
                                file.groups.push(Cutscene::default());
                                group_idx = Some(file.groups.len() - 1);
                                section_idx = None;
                                open = Some(OpenTag::known(name));
                            }
                            "SECTION_START" => {
                                let gi = ensure_group(&mut file, &mut group_idx, line_no);
                                file.groups[gi].sections.push(Section::default());
                                section_idx =
                                    Some(file.groups[gi].sections.len().saturating_sub(1));
                                open = None;
                            }
                            "SECTION_END" => {
                                section_idx = None;
                                open = None;
                            }
                            _ => {
                                let gi = ensure_group(&mut file, &mut group_idx, line_no);
                                if is_known_tag(name) {
                                    open = Some(OpenTag::known(name));
                                } else {
                                    let unk = UnknownSection {
                                        tag: name.to_string(),
                                        lines: Vec::new(),
                                        line: line_no,
                                    };
                                    match section_idx {
                                        Some(si) => {
                                            file.groups[gi].sections[si].unknown.push(unk);
                                        }
                                        None => file.groups[gi].unknown.push(unk),
                                    }
                                    open = Some(OpenTag::unknown(name));
                                }
                            }
                        }
                    }
                    TagLine::Close(name) => match open.take() {
                        None => {
                            file.warn(
                                line_no,
                                WarningKind::StrayClose,
                                format!("close tag {name} with nothing open"),
                            );
                        }
                        Some(prev) => {
                            if prev.name != name {
                                file.warn(
                                    line_no,
                                    WarningKind::MismatchedClose,
                                    format!("close tag {name} ends {0}", prev.name),
                                );
                            }
                        }
                    },
                    TagLine::Malformed => {
                        if let Some(prev) = open.take() {
                            file.warn(
                                line_no,
                                WarningKind::ImplicitClose,
                                format!("malformed tag line implicitly closes {}", prev.name),
                            );
                        }
                        file.warn(
                            line_no,
                            WarningKind::MalformedTag,
                            format!("malformed tag line {line:?}"),
                        );
                        let gi = ensure_group(&mut file, &mut group_idx, line_no);
                        let unk = UnknownSection {
                            tag: line.to_string(),
                            lines: Vec::new(),
                            line: line_no,
                        };
                        match section_idx {
                            Some(si) => file.groups[gi].sections[si].unknown.push(unk),
                            None => file.groups[gi].unknown.push(unk),
                        }
                        open = Some(OpenTag::unknown(line));
                    }
                }
            } else if let (Some(gi), true) = (group_idx, open.is_some()) {
                let mut taken = open.take();
                let tag = taken
                    .as_ref()
                    .map(|o| o.name.clone())
                    .expect("open tag taken");
                push_content(&mut file, gi, section_idx, &mut taken, &tag, line, line_no);
                open = taken;
            } else {
                file.warn(
                    line_no,
                    WarningKind::OrphanContent,
                    format!("content outside any tag: {line:?}"),
                );
                file.orphans.push(OrphanLine {
                    line: line_no,
                    text: line.to_string(),
                });
            }
        }
        file.trailing_slack_len = bytes.len().saturating_sub(last_text_end);
        Ok(file)
    }

    /// Record a warning.
    fn warn(&mut self, line: usize, kind: WarningKind, message: impl Into<String>) {
        self.warnings.push(Warning {
            line,
            kind,
            message: message.into(),
        });
    }
}

/// Lazily create the first group so headerless input still parses.
fn ensure_group(file: &mut CutsceneFile, group_idx: &mut Option<usize>, line: usize) -> usize {
    if let Some(gi) = *group_idx {
        return gi;
    }
    file.groups.push(Cutscene::default());
    let gi = file.groups.len() - 1;
    *group_idx = Some(gi);
    file.warn(
        line,
        WarningKind::ImplicitClose,
        "content before the first CUTSCENE_HEADER starts an implicit cutscene",
    );
    gi
}

/// Route one content line to its typed target.
fn push_content(
    file: &mut CutsceneFile,
    group_idx: usize,
    section_idx: Option<usize>,
    open: &mut Option<OpenTag>,
    tag: &str,
    line: &str,
    line_no: usize,
) {
    debug_assert!(open.is_some());
    if matches!(open, Some(o) if o.unknown) {
        let lines = match section_idx {
            Some(si) => {
                &mut file.groups[group_idx].sections[si]
                    .unknown
                    .last_mut()
                    .expect("unknown section just opened")
                    .lines
            }
            None => {
                &mut file.groups[group_idx]
                    .unknown
                    .last_mut()
                    .expect("unknown section just opened")
                    .lines
            }
        };
        lines.push(line.to_string());
        return;
    }
    let target = RowTarget {
        tag,
        line,
        in_section: section_idx.is_some(),
    };
    if let Err(reason) = target.apply(file, group_idx, section_idx) {
        file.warn(line_no, WarningKind::BadRow, format!("{tag}: {reason}"));
        let bad = BadRow {
            tag: tag.to_string(),
            line: line_no,
            text: line.to_string(),
            reason,
        };
        match section_idx {
            Some(si) => file.groups[group_idx].sections[si].bad_rows.push(bad),
            None => file.groups[group_idx].bad_rows.push(bad),
        }
    }
}

/// One typed row awaiting parsing.
struct RowTarget<'a> {
    tag: &'a str,
    line: &'a str,
    in_section: bool,
}

impl RowTarget<'_> {
    // One match arm per row tag; splitting it would scatter tag handling.
    #[allow(clippy::too_many_lines)]
    fn apply(
        self,
        file: &mut CutsceneFile,
        group_idx: usize,
        section_idx: Option<usize>,
    ) -> Result<(), String> {
        let f: Vec<&str> = self.line.split_whitespace().collect();
        if !self.in_section
            && matches!(
                self.tag,
                "MODELS"
                    | "COMPRESSION"
                    | "LIGHTS"
                    | "EFFECTS"
                    | "VEHICLE_DETAILS"
                    | "VEHICLE_REMOVAL"
                    | "ORIENT"
                    | "TIME"
                    | "OFFSET"
                    | "DURATION"
                    | "AUDIO"
                    | "ANIM"
                    | "ANIMRANGE"
                    | "CAMERA"
                    | "REMOVE"
            )
        {
            return Err(format!("{} outside a section", self.tag));
        }
        let group = &mut file.groups[group_idx];
        match self.tag {
            "CUTSCENE_HEADER" => {
                group.header_frames = parse_ints(&f)?;
            }
            "FLAGS" => {
                group.flags.push(exactly_one(&f, "flag name")?.to_string());
            }
            "BLOCKING_BOUNDS" => group.blocking_bounds.push(parse_blocking(&f)?),
            "CAMCORDER" => group.camcorder.push(parse_camcorder(&f)?),
            "MISSION_TEXT_NAME" => group
                .mission_text_name
                .push(exactly_one(&f, "mission text name")?.to_string()),
            "TEXT" => group.texts.push(parse_text(&f)?),
            "PLAYER_START" => group.player_starts.push(parse_vec3(&f)?),
            "DRAW_DISTANCE" => group.draw_distance.push(parse_draw_distance(&f)?),
            "MAX_PEDS" => group.max_peds.push(parse_one_int(&f, "MAX_PEDS")?),
            "MAX_CARS" => group.max_cars.push(parse_one_int(&f, "MAX_CARS")?),
            "TIMECYCLE_MODIFIER_NAME" => group
                .timecycle_modifiers
                .push(exactly_one(&f, "timecycle modifier")?.to_string()),
            "VARIATION" => {
                let v = parse_variation(&f)?;
                match section_idx {
                    Some(si) => group.sections[si].variations.push(v),
                    None => group.variations.push(v),
                }
            }
            "PROPS" => {
                let p = parse_prop(&f)?;
                match section_idx {
                    Some(si) => group.sections[si].props.push(p),
                    None => group.props.push(p),
                }
            }
            "MODELS" => group.sections[section_idx.expect("checked")]
                .models
                .push(parse_model(&f)?),
            "COMPRESSION" => group.sections[section_idx.expect("checked")]
                .compression
                .push(parse_compression(&f)?),
            "LIGHTS" => group.sections[section_idx.expect("checked")]
                .lights
                .push(exactly_one(&f, "light name")?.to_string()),
            "EFFECTS" => group.sections[section_idx.expect("checked")]
                .effects
                .push(parse_effect(&f)?),
            "VEHICLE_DETAILS" => group.sections[section_idx.expect("checked")]
                .vehicle_details
                .push(parse_vehicle_details(&f)?),
            "VEHICLE_REMOVAL" => group.sections[section_idx.expect("checked")]
                .vehicle_removals
                .push(parse_vehicle_removal(&f)?),
            "ORIENT" => group.sections[section_idx.expect("checked")]
                .orient
                .push(parse_one_float(&f, "ORIENT")?),
            "TIME" => group.sections[section_idx.expect("checked")]
                .time_lines
                .push(self.line.to_string()),
            "OFFSET" => group.sections[section_idx.expect("checked")]
                .offset
                .push(parse_vec3(&f)?),
            "DURATION" => group.sections[section_idx.expect("checked")]
                .durations_ms
                .push(parse_one_float(&f, "DURATION")?),
            "AUDIO" => group.sections[section_idx.expect("checked")]
                .audios
                .push(exactly_one(&f, "audio name")?.to_string()),
            "ANIM" => group.sections[section_idx.expect("checked")]
                .anims
                .push(exactly_one(&f, "anim name")?.to_string()),
            "ANIMRANGE" => group.sections[section_idx.expect("checked")]
                .anim_ranges
                .push(parse_anim_range(&f)?),
            "CAMERA" => group.sections[section_idx.expect("checked")]
                .cameras
                .push(exactly_one(&f, "camera name")?.to_string()),
            "REMOVE" => group.sections[section_idx.expect("checked")]
                .removes
                .push(parse_remove(&f)?),
            other => return Err(format!("no typed parser for {other}")),
        }
        Ok(())
    }
}

fn parse_ints(f: &[&str]) -> Result<Vec<i32>, String> {
    if f.is_empty() {
        return Err("expected at least one integer".to_string());
    }
    f.iter()
        .map(|t| t.parse::<i32>().map_err(|_| format!("bad integer {t:?}")))
        .collect()
}

fn parse_one_int(f: &[&str], what: &str) -> Result<i32, String> {
    if f.len() != 1 {
        return Err(format!("{what} expects 1 field, found {}", f.len()));
    }
    f[0].parse::<i32>()
        .map_err(|_| format!("bad integer {0:?}", f[0]))
}

fn parse_one_float(f: &[&str], what: &str) -> Result<f32, String> {
    if f.len() != 1 {
        return Err(format!("{what} expects 1 field, found {}", f.len()));
    }
    f[0].parse::<f32>()
        .map_err(|_| format!("bad float {0:?}", f[0]))
}

fn exactly_one<'a>(f: &[&'a str], what: &str) -> Result<&'a str, String> {
    if f.len() != 1 {
        return Err(format!("{what} expects 1 field, found {}", f.len()));
    }
    Ok(f[0])
}

fn parse_vec3(f: &[&str]) -> Result<[f32; 3], String> {
    if f.len() != 3 {
        return Err(format!("expected 3 floats, found {}", f.len()));
    }
    let mut out = [0.0f32; 3];
    for (i, t) in f.iter().enumerate() {
        out[i] = t.parse::<f32>().map_err(|_| format!("bad float {t:?}"))?;
    }
    Ok(out)
}

fn parse_text(f: &[&str]) -> Result<TextEntry, String> {
    if f.len() != 3 {
        return Err(format!("TEXT expects 3 fields, found {}", f.len()));
    }
    Ok(TextEntry {
        start_ms: f[0]
            .parse::<i32>()
            .map_err(|_| format!("bad integer {0:?}", f[0]))?,
        len_ms: f[1]
            .parse::<i32>()
            .map_err(|_| format!("bad integer {0:?}", f[1]))?,
        key: f[2].to_string(),
    })
}

fn parse_model(f: &[&str]) -> Result<ModelEntry, String> {
    if !(3..=5).contains(&f.len()) {
        return Err(format!("MODELS expects 3-5 fields, found {}", f.len()));
    }
    Ok(ModelEntry {
        id: f[0]
            .parse::<i32>()
            .map_err(|_| format!("bad integer {0:?}", f[0]))?,
        model: f[1].to_string(),
        anim: f[2].to_string(),
        head_anim: f.get(3).map(ToString::to_string),
        unknown: f
            .get(4)
            .map(|t| t.parse::<i32>().map_err(|_| format!("bad integer {t:?}")))
            .transpose()?,
    })
}

fn parse_draw_distance(f: &[&str]) -> Result<DrawDistance, String> {
    if f.len() != 3 && f.len() != 4 {
        return Err(format!(
            "DRAW_DISTANCE expects 3-4 fields, found {}",
            f.len()
        ));
    }
    Ok(DrawDistance {
        time_ms: f[0]
            .parse::<i32>()
            .map_err(|_| format!("bad integer {0:?}", f[0]))?,
        second: f[1]
            .parse::<i32>()
            .map_err(|_| format!("bad integer {0:?}", f[1]))?,
        distance: f[2]
            .parse::<f32>()
            .map_err(|_| format!("bad float {0:?}", f[2]))?,
        unknown: match f.get(3) {
            Some(t) => t.parse::<f32>().map_err(|_| format!("bad float {t:?}"))?,
            None => DRAW_DISTANCE_DEFAULT_UNKNOWN,
        },
    })
}

fn parse_blocking(f: &[&str]) -> Result<BlockingBounds, String> {
    if f.len() != 13 {
        return Err(format!(
            "BLOCKING_BOUNDS expects 13 fields, found {}",
            f.len()
        ));
    }
    let mut corners = [[0.0f32; 3]; 4];
    for c in 0..4 {
        for k in 0..3 {
            corners[c][k] = f[c * 3 + k]
                .parse::<f32>()
                .map_err(|_| format!("bad float {0:?}", f[c * 3 + k]))?;
        }
    }
    Ok(BlockingBounds {
        corners,
        height: f[12]
            .parse::<f32>()
            .map_err(|_| format!("bad float {0:?}", f[12]))?,
    })
}

fn parse_camcorder(f: &[&str]) -> Result<Camcorder, String> {
    if f.len() != 2 {
        return Err(format!("CAMCORDER expects 2 fields, found {}", f.len()));
    }
    Ok(Camcorder {
        start_ms: f[0]
            .parse::<i32>()
            .map_err(|_| format!("bad integer {0:?}", f[0]))?,
        end_ms: f[1]
            .parse::<i32>()
            .map_err(|_| format!("bad integer {0:?}", f[1]))?,
    })
}

fn parse_variation(f: &[&str]) -> Result<Variation, String> {
    if f.len() != 4 && f.len() != 5 {
        return Err(format!("VARIATION expects 4-5 fields, found {}", f.len()));
    }
    Ok(Variation {
        values: parse_ints(f)?,
    })
}

fn parse_prop(f: &[&str]) -> Result<Prop, String> {
    if f.len() != 3 {
        return Err(format!("PROPS expects 3 fields, found {}", f.len()));
    }
    let mut v = [0i32; 3];
    for (i, t) in f.iter().enumerate() {
        v[i] = t.parse::<i32>().map_err(|_| format!("bad integer {t:?}"))?;
    }
    Ok(Prop {
        model_id: v[0],
        index: v[1],
        prop: v[2],
    })
}

fn parse_compression(f: &[&str]) -> Result<Compression, String> {
    if f.len() != 2 {
        return Err(format!("COMPRESSION expects 2 fields, found {}", f.len()));
    }
    Ok(Compression {
        model_id: f[0]
            .parse::<i32>()
            .map_err(|_| format!("bad integer {0:?}", f[0]))?,
        mode: f[1].to_string(),
    })
}

fn parse_anim_range(f: &[&str]) -> Result<AnimRange, String> {
    if f.len() != 4 || f[0] != "range" || f[1] != "on" {
        return Err("ANIMRANGE expects `range on <start> <end>`".to_string());
    }
    Ok(AnimRange {
        start: f[2]
            .parse::<f32>()
            .map_err(|_| format!("bad float {0:?}", f[2]))?,
        end: f[3]
            .parse::<f32>()
            .map_err(|_| format!("bad float {0:?}", f[3]))?,
    })
}

fn parse_effect(f: &[&str]) -> Result<Effect, String> {
    if f.len() < 12 {
        return Err(format!("EFFECTS expects 12+ fields, found {}", f.len()));
    }
    let mut ints = [0i32; 4];
    for (i, t) in f[1..5].iter().enumerate() {
        ints[i] = t.parse::<i32>().map_err(|_| format!("bad integer {t:?}"))?;
    }
    let mut floats = [0.0f32; 7];
    for (i, t) in f[5..12].iter().enumerate() {
        floats[i] = t.parse::<f32>().map_err(|_| format!("bad float {t:?}"))?;
    }
    Ok(Effect {
        name: f[0].to_string(),
        object_a: ints[0],
        object_b: ints[1],
        start_ms: ints[2],
        end_ms: ints[3],
        position: [floats[0], floats[1], floats[2]],
        rotation: [floats[3], floats[4], floats[5]],
        unknown: floats[6],
        extra: f[12..].iter().map(ToString::to_string).collect(),
    })
}

fn parse_vehicle_details(f: &[&str]) -> Result<VehicleDetails, String> {
    if f.len() != 6 {
        return Err(format!(
            "VEHICLE_DETAILS expects 6 fields, found {}",
            f.len()
        ));
    }
    let mut v = [0i32; 6];
    for (i, t) in f.iter().enumerate() {
        v[i] = t.parse::<i32>().map_err(|_| format!("bad integer {t:?}"))?;
    }
    Ok(VehicleDetails {
        vehicle_id: v[0],
        colors: [v[1], v[2], v[3], v[4]],
        last: v[5],
    })
}

fn parse_vehicle_removal(f: &[&str]) -> Result<VehicleRemoval, String> {
    if f.len() != 2 {
        return Err(format!(
            "VEHICLE_REMOVAL expects 2 fields, found {}",
            f.len()
        ));
    }
    Ok(VehicleRemoval {
        model_index: f[0]
            .parse::<i32>()
            .map_err(|_| format!("bad integer {0:?}", f[0]))?,
        bone_id: f[1]
            .parse::<i32>()
            .map_err(|_| format!("bad integer {0:?}", f[1]))?,
    })
}

fn parse_remove(f: &[&str]) -> Result<Remove, String> {
    if f.len() != 4 && f.len() != 5 {
        return Err(format!("REMOVE expects 4-5 fields, found {}", f.len()));
    }
    let mut position = [0.0f32; 3];
    for (i, t) in f[..3].iter().enumerate() {
        position[i] = t.parse::<f32>().map_err(|_| format!("bad float {t:?}"))?;
    }
    Ok(Remove {
        position,
        model: f[3].to_string(),
        unknown: f
            .get(4)
            .map(|t| t.parse::<f32>().map_err(|_| format!("bad float {t:?}")))
            .transpose()?,
    })
}

/// A currently open tag.
struct OpenTag {
    name: String,
    unknown: bool,
}

impl OpenTag {
    fn known(name: &str) -> Self {
        OpenTag {
            name: name.to_string(),
            unknown: false,
        }
    }

    fn unknown(name: &str) -> Self {
        OpenTag {
            name: name.to_string(),
            unknown: true,
        }
    }
}

/// Classification of one non-blank text line.
enum TagLine<'a> {
    /// `[NAME]`: opens a section or marks a boundary.
    Open(&'a str),
    /// `[/NAME]`: closes a section.
    Close(&'a str),
    /// Starts with `[` but is not a well-formed tag.
    Malformed,
}

/// Classify a trimmed line: tag, close tag, malformed bracket line or content.
///
/// Tag names are ASCII alphanumerics and underscores. Matching is
/// case-sensitive: shipped files use uppercase throughout.
fn parse_tag_line(line: &str) -> Option<TagLine<'_>> {
    if !line.starts_with('[') {
        return None;
    }
    let inner = line.strip_prefix('[')?;
    let (inner, is_close) = match inner.strip_prefix('/') {
        Some(rest) => (rest, true),
        None => (inner, false),
    };
    let Some(name) = inner.strip_suffix(']') else {
        return Some(TagLine::Malformed);
    };
    if name.is_empty() || !name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_') {
        return Some(TagLine::Malformed);
    }
    if is_close {
        Some(TagLine::Close(name))
    } else {
        Some(TagLine::Open(name))
    }
}

/// Strip trailing NUL padding bytes from one raw line.
///
/// Shipped payloads are NUL-padded to a block boundary; one file ends its
/// final marker with NULs on the same line instead of a newline.
fn trim_trailing_nulls(raw: &[u8]) -> &[u8] {
    let mut end = raw.len();
    while end > 0 && raw[end - 1] == 0 {
        end -= 1;
    }
    &raw[..end]
}

/// True for tags with a typed row parser (markers included).
fn is_known_tag(name: &str) -> bool {
    matches!(
        name,
        "CUTSCENE_HEADER"
            | "FLAGS"
            | "BLOCKING_BOUNDS"
            | "CAMCORDER"
            | "DRAW_DISTANCE"
            | "MISSION_TEXT_NAME"
            | "TEXT"
            | "PLAYER_START"
            | "VARIATION"
            | "PROPS"
            | "MAX_PEDS"
            | "MAX_CARS"
            | "TIMECYCLE_MODIFIER_NAME"
            | "MODELS"
            | "COMPRESSION"
            | "LIGHTS"
            | "EFFECTS"
            | "VEHICLE_DETAILS"
            | "VEHICLE_REMOVAL"
            | "ORIENT"
            | "REMOVE"
            | "TIME"
            | "OFFSET"
            | "DURATION"
            | "AUDIO"
            | "ANIM"
            | "ANIMRANGE"
            | "CAMERA"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A small but complete hand-built cutscene exercising every common tag.
    /// All names and numbers are invented for the test.
    const FULL: &str = "\
[CUTSCENE_HEADER]\r
100\t900\r
[/CUTSCENE_HEADER]\r
\r
[FLAGS]\r
LONG_FADE_OUT\r
[/FLAGS]\r
\r
[BLOCKING_BOUNDS]\r
1.0 2.0 3.0 4.0 5.0 6.0 7.0 8.0 9.0 10.0 11.0 12.0 13.0\r
[/BLOCKING_BOUNDS]\r
\r
[CAMCORDER]\r
1000 2000\r
[/CAMCORDER]\r
\r
[DRAW_DISTANCE]\r
0 0 60.00000\r
5000 1 200.00000 0.07000\r
[/DRAW_DISTANCE]\r
\r
[MISSION_TEXT_NAME]\r
SAMPLEAUD\r
[/MISSION_TEXT_NAME]\r
\r
[TEXT]\r
100\t500\tSAMPLE_KEY_1\r
700\t300\tSAMPLE_KEY_2\r
[/TEXT]\r
\r
[PLAYER_START]\r
1.5 -2.5 10.0\r
[/PLAYER_START]\r
\r
[VARIATION]\r
0 1 0 0 0\r
[/VARIATION]\r
\r
[PROPS]\r
1 0 1\r
[/PROPS]\r
\r
[MAX_PEDS]\r
5\r
[/MAX_PEDS]\r
\r
[MAX_CARS]\r
2\r
[/MAX_CARS]\r
\r
[TIMECYCLE_MODIFIER_NAME]\r
sample_mod\r
[/TIMECYCLE_MODIFIER_NAME]\r
\r
[SECTION_START]\r
\r
[MODELS]\r
0 hero_m hero_0 hero_head_0 0\r
1 hero_car hero_car_0\r
[/MODELS]\r
\r
[COMPRESSION]\r
0 A\r
[/COMPRESSION]\r
\r
[LIGHTS]\r
fill_0\r
[/LIGHTS]\r
\r
[EFFECTS]\r
FX_sample:effect_0 -1 -1 1000 2000 1.0 2.0 3.0 0.0 0.0 1.0 0.0\r
[/EFFECTS]\r
\r
[VEHICLE_DETAILS]\r
1 10 20 30 40 0\r
[/VEHICLE_DETAILS]\r
\r
[VEHICLE_REMOVAL]\r
1 15301\r
[/VEHICLE_REMOVAL]\r
\r
[ORIENT]\r
180.000000\r
[/ORIENT]\r
\r
[REMOVE]\r
1.0 2.0 3.0 sample_prop 7\r
[/REMOVE]\r
\r
[TIME]\r
[/TIME]\r
\r
[OFFSET]\r
-1.0 -2.0 3.5\r
[/OFFSET]\r
\r
[DURATION]\r
30000.000000\r
[/DURATION]\r
\r
[AUDIO]\r
sample_audio\r
[/AUDIO]\r
\r
[ANIM]\r
sample_anim_0\r
[/ANIM]\r
\r
[ANIMRANGE]\r
range on 100.000000 900.000000\r
[/ANIMRANGE]\r
\r
[CAMERA]\r
camera_0\r
[/CAMERA]\r
\r
[SECTION_END]\r
";

    #[test]
    // Parser test: the text spellings must decode to exactly these floats.
    #[allow(clippy::float_cmp)]
    fn full_fixture_parses_cleanly() {
        let file = CutsceneFile::parse(FULL.as_bytes()).unwrap();
        assert!(file.warnings.is_empty(), "{:?}", file.warnings);
        assert!(file.orphans.is_empty());
        assert_eq!(file.groups.len(), 1);
        let g = &file.groups[0];
        assert_eq!(g.header_frames, vec![100, 900]);
        assert_eq!(g.flags, vec!["LONG_FADE_OUT".to_string()]);
        assert_eq!(g.blocking_bounds.len(), 1);
        assert_eq!(g.blocking_bounds[0].height, 13.0);
        assert_eq!(
            g.camcorder,
            vec![Camcorder {
                start_ms: 1000,
                end_ms: 2000
            }]
        );
        assert_eq!(g.draw_distance.len(), 2);
        assert_eq!(g.draw_distance[0].unknown, DRAW_DISTANCE_DEFAULT_UNKNOWN);
        assert_eq!(g.draw_distance[1].unknown, 0.07);
        assert_eq!(g.mission_text_name, vec!["SAMPLEAUD".to_string()]);
        assert_eq!(g.texts.len(), 2);
        assert_eq!(g.texts[0].key, "SAMPLE_KEY_1");
        assert_eq!(g.player_starts, vec![[1.5, -2.5, 10.0]]);
        assert_eq!(g.variations.len(), 1);
        assert_eq!(g.props.len(), 1);
        assert_eq!(g.max_peds, vec![5]);
        assert_eq!(g.max_cars, vec![2]);
        assert_eq!(g.timecycle_modifiers, vec!["sample_mod".to_string()]);
        assert_eq!(g.sections.len(), 1);
        let s = &g.sections[0];
        assert_eq!(s.models.len(), 2);
        assert_eq!(s.models[0].head_anim.as_deref(), Some("hero_head_0"));
        assert_eq!(s.models[0].unknown, Some(0));
        assert_eq!(s.models[1].head_anim, None);
        assert_eq!(s.compression.len(), 1);
        assert_eq!(s.lights, vec!["fill_0".to_string()]);
        assert_eq!(s.effects.len(), 1);
        assert_eq!(s.effects[0].start_ms, 1000);
        assert!(s.effects[0].extra.is_empty());
        assert_eq!(s.vehicle_details.len(), 1);
        assert_eq!(s.vehicle_details[0].colors, [10, 20, 30, 40]);
        assert_eq!(s.vehicle_removals.len(), 1);
        assert_eq!(s.orient, vec![180.0]);
        assert!(s.time_lines.is_empty());
        assert_eq!(s.offset, vec![[-1.0, -2.0, 3.5]]);
        assert_eq!(s.durations_ms, vec![30000.0]);
        assert_eq!(s.audios, vec!["sample_audio".to_string()]);
        assert_eq!(s.anims, vec!["sample_anim_0".to_string()]);
        assert_eq!(s.anim_ranges.len(), 1);
        assert_eq!(s.cameras, vec!["camera_0".to_string()]);
        assert_eq!(s.removes.len(), 1);
        assert_eq!(s.removes[0].unknown, Some(7.0));
        assert_eq!(g.duration_ms(), 30000.0);
        assert_eq!(g.model_count(), 2);
    }

    #[test]
    fn lf_endings_parse() {
        let lf = FULL.replace("\r\n", "\n");
        let file = CutsceneFile::parse(lf.as_bytes()).unwrap();
        assert_eq!(file.groups.len(), 1);
        assert_eq!(file.groups[0].sections.len(), 1);
        assert!(file.warnings.is_empty());
    }

    #[test]
    fn repeated_open_implicitly_closes() {
        let bytes = b"[CUTSCENE_HEADER]\n1 2\n[/CUTSCENE_HEADER]\n[TEXT]\n1 2 KEY_A\n[TEXT]\n3 4 KEY_B\n[/TEXT]\n";
        let file = CutsceneFile::parse(bytes).unwrap();
        assert_eq!(file.groups[0].texts.len(), 2);
        assert_eq!(file.groups[0].texts[1].key, "KEY_B");
        assert!(
            file.warnings
                .iter()
                .any(|w| w.kind == WarningKind::ImplicitClose),
            "{:?}",
            file.warnings
        );
    }

    #[test]
    fn unclosed_section_runs_to_next_tag() {
        let bytes = b"[CUTSCENE_HEADER]\n1 2\n[/CUTSCENE_HEADER]\n[TEXT]\n1 2 KEY_A\n[SECTION_START]\n[MODELS]\n0 m a\n[/MODELS]\n[SECTION_END]\n";
        let file = CutsceneFile::parse(bytes).unwrap();
        assert_eq!(file.groups[0].texts.len(), 1);
        assert_eq!(file.groups[0].sections.len(), 1);
        assert_eq!(file.groups[0].sections[0].models.len(), 1);
    }

    #[test]
    fn unclosed_final_section_kept() {
        let bytes = b"[CUTSCENE_HEADER]\n1 2\n[/CUTSCENE_HEADER]\n[SECTION_START]\n[MODELS]\n0 m a\n[/MODELS]\n";
        let file = CutsceneFile::parse(bytes).unwrap();
        assert_eq!(file.groups[0].sections.len(), 1);
        assert_eq!(file.groups[0].sections[0].models.len(), 1);
    }

    #[test]
    fn stray_close_warns() {
        let bytes = b"[CUTSCENE_HEADER]\n1 2\n[/CUTSCENE_HEADER]\n[/TEXT]\n";
        let file = CutsceneFile::parse(bytes).unwrap();
        assert!(
            file.warnings
                .iter()
                .any(|w| w.kind == WarningKind::StrayClose),
            "{:?}",
            file.warnings
        );
    }

    #[test]
    fn mismatched_close_warns_but_closes() {
        let bytes = b"[CUTSCENE_HEADER]\n1 2\n[/CUTSCENE_HEADER]\n[TEXT]\n1 2 KEY_A\n[/FLAGS]\n[FLAGS]\nok_flag\n[/FLAGS]\n";
        let file = CutsceneFile::parse(bytes).unwrap();
        assert_eq!(file.groups[0].texts.len(), 1);
        assert_eq!(file.groups[0].flags, vec!["ok_flag".to_string()]);
        assert!(
            file.warnings
                .iter()
                .any(|w| w.kind == WarningKind::MismatchedClose),
            "{:?}",
            file.warnings
        );
    }

    #[test]
    fn malformed_and_unknown_tags_preserved() {
        let bytes = b"[CUTSCENE_HEADER]\n1 2\n[/CUTSCENE_HEADER]\n[BROKEN_TAG\nkept line\n[/BROKEN_TAG]\n[FUTURE_TAG]\nfuture line\n[/FUTURE_TAG]\n";
        let file = CutsceneFile::parse(bytes).unwrap();
        assert!(
            file.warnings
                .iter()
                .any(|w| w.kind == WarningKind::MalformedTag),
            "{:?}",
            file.warnings
        );
        // Malformed opener becomes an unknown section; its lines are kept.
        assert_eq!(file.groups[0].unknown.len(), 2);
        assert_eq!(
            file.groups[0].unknown[0].lines,
            vec!["kept line".to_string()]
        );
        assert_eq!(file.groups[0].unknown[1].tag, "FUTURE_TAG");
        assert_eq!(
            file.groups[0].unknown[1].lines,
            vec!["future line".to_string()]
        );
    }

    #[test]
    fn orphan_content_preserved() {
        // Mid-file orphan (lines after the final tag count as slack).
        let bytes =
            b"[CUTSCENE_HEADER]\n1 2\n[/CUTSCENE_HEADER]\nlost line\n[TEXT]\n1 2 KEY\n[/TEXT]\n";
        let file = CutsceneFile::parse(bytes).unwrap();
        assert_eq!(file.orphans.len(), 1);
        assert_eq!(file.orphans[0].text, "lost line");
        assert!(
            file.warnings
                .iter()
                .any(|w| w.kind == WarningKind::OrphanContent),
            "{:?}",
            file.warnings
        );
    }

    #[test]
    fn bad_row_kept_with_reason() {
        let bytes =
            b"[CUTSCENE_HEADER]\n1 2\n[/CUTSCENE_HEADER]\n[TEXT]\nnot a row\n1 2 KEY_OK\n[/TEXT]\n";
        let file = CutsceneFile::parse(bytes).unwrap();
        assert_eq!(file.groups[0].texts.len(), 1);
        assert_eq!(file.groups[0].bad_rows.len(), 1);
        assert_eq!(file.groups[0].bad_rows[0].tag, "TEXT");
        assert!(
            file.warnings.iter().any(|w| w.kind == WarningKind::BadRow),
            "{:?}",
            file.warnings
        );
    }

    #[test]
    fn binary_slack_stops_parsing_silently() {
        // Padding is the normal end of file: parsing stops, the tail is
        // counted, and no warning is emitted.
        let mut bytes = FULL.as_bytes().to_vec();
        bytes.extend_from_slice(b"\nZ\n\xcd\xcd\xcd\xcd binary \x00\x00\x00");
        bytes.extend_from_slice(&[0u8; 100]);
        let file = CutsceneFile::parse(&bytes).unwrap();
        assert_eq!(file.groups.len(), 1);
        assert_eq!(file.groups[0].sections.len(), 1);
        assert!(file.trailing_slack_len > 100);
        assert!(file.orphans.is_empty());
        assert!(file.warnings.is_empty(), "{:?}", file.warnings);
    }

    #[test]
    fn nul_glued_final_tag_parses() {
        // Regression: one shipped file ends "[SECTION_END]" + NULs, no newline.
        let bytes = b"[CUTSCENE_HEADER]\n1 2\n[/CUTSCENE_HEADER]\n[SECTION_START]\n[MODELS]\n0 m a\n[/MODELS]\n[SECTION_END]\x00\x00\x00\x00";
        let file = CutsceneFile::parse(bytes).unwrap();
        assert_eq!(file.groups[0].sections.len(), 1);
        assert!(file.warnings.is_empty(), "{:?}", file.warnings);
        assert!(file.trailing_slack_len >= 4);
    }

    #[test]
    // Parser test: the text spellings must decode to exactly these floats.
    #[allow(clippy::float_cmp)]
    fn multi_group_file() {
        let bytes = b"[CUTSCENE_HEADER]\n1 2\n[/CUTSCENE_HEADER]\n[SECTION_START]\n[DURATION]\n1000.0\n[/DURATION]\n[SECTION_END]\n[CUTSCENE_HEADER]\n3 4 5\n[/CUTSCENE_HEADER]\n[SECTION_START]\n[DURATION]\n2000.0\n[/DURATION]\n[SECTION_END]\n[SECTION_START]\n[DURATION]\n3000.0\n[/DURATION]\n[SECTION_END]\n";
        let file = CutsceneFile::parse(bytes).unwrap();
        assert_eq!(file.groups.len(), 2);
        assert_eq!(file.groups[0].duration_ms(), 1000.0);
        assert_eq!(file.groups[1].duration_ms(), 5000.0);
    }

    #[test]
    fn section_tag_outside_section_is_bad_row() {
        let bytes = b"[CUTSCENE_HEADER]\n1 2\n[/CUTSCENE_HEADER]\n[MODELS]\n0 m a\n[/MODELS]\n";
        let file = CutsceneFile::parse(bytes).unwrap();
        assert_eq!(file.groups[0].bad_rows.len(), 1);
        assert!(
            file.groups[0].bad_rows[0]
                .reason
                .contains("outside a section")
        );
    }

    #[test]
    fn too_large_rejected() {
        let over = usize::try_from(MAX_INPUT + 1).expect("test limit fits");
        assert!(matches!(
            CutsceneFile::parse(&vec![b' '; over]),
            Err(Error::TooLarge { .. })
        ));
    }

    #[test]
    fn remove_accepts_int_and_float_spellings() {
        let bytes = b"[CUTSCENE_HEADER]\n1 2\n[/CUTSCENE_HEADER]\n[SECTION_START]\n[REMOVE]\n1.0 2.0 3.0 prop_a 10\n4.0 5.0 6.0 prop_b 5.0\n7.0 8.0 9.0 prop_c\n[/REMOVE]\n[SECTION_END]\n";
        let file = CutsceneFile::parse(bytes).unwrap();
        assert!(file.warnings.is_empty(), "{:?}", file.warnings);
        let removes = &file.groups[0].sections[0].removes;
        assert_eq!(removes.len(), 3);
        assert_eq!(removes[0].unknown, Some(10.0));
        assert_eq!(removes[1].unknown, Some(5.0));
        assert_eq!(removes[2].unknown, None);
    }

    #[test]
    fn empty_input_parses_empty() {
        let file = CutsceneFile::parse(b"").unwrap();
        assert!(file.groups.is_empty());
        assert!(file.warnings.is_empty());
    }
}
