//! Pointed-to snapshots: validation and call-log placement.
//!
//! A callee's `snap` entries ask the recorder stub to copy words from the
//! memory a pointer argument (a stack argument, ECX or EDX) points at, at
//! call time, into the call log, where they compare by value. Each entry is
//! `{kind, idx, n, at}`: `n` words starting `at` bytes from the pointer
//! (`at` defaults to 0 and may be negative).
//!
//! Version 4 allowed 8 words per callee, at offset 0 only. The first 8
//! words still land in the base log entry exactly as before, so existing
//! contracts emit byte-identical stubs; words 8 and up land in a parallel
//! extension log whose entries use the same stride, so the stub reaches
//! them from the base entry pointer with one constant displacement. The
//! per-callee total is capped at [`SNAP_CAP_WORDS`] and setup fails loudly
//! above it: a snapshot is never truncated.

/// Most snapshot words one callee may declare in total.
pub const SNAP_CAP_WORDS: usize = 64;
/// Snapshot words that fit in the base log entry.
pub const SNAP_BASE_WORDS: usize = 8;
/// Byte offset of the first snapshot word in a base log entry.
pub const LOG_SNAP_OFF: usize = 192;
/// Byte stride of both the base and the extension call logs.
pub const LOG_ENTRY: usize = 272;
/// Largest `|at|` accepted, in bytes (a larger offset is a contract error,
/// not a layout to honour).
pub const SNAP_AT_LIMIT: i64 = 0x1_0000;

/// Where the stub finds the pointer a snapshot reads through.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnapBase {
    /// Stack argument `idx` of the call.
    Arg(usize),
    /// ECX at the call.
    Ecx,
    /// EDX at the call.
    Edx,
}

/// One validated snapshot entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SnapSpec {
    /// Pointer source.
    pub base: SnapBase,
    /// Words to copy.
    pub n: usize,
    /// Byte offset from the pointer to the first word.
    pub at: i32,
}

impl SnapSpec {
    /// Build one entry from its contract fields (`kind` is `"arg"`, `"ecx"`
    /// or `"edx"`; anything else is an error, never a silent default).
    ///
    /// # Errors
    /// Returns a message for an unknown kind or an `at` beyond
    /// [`SNAP_AT_LIMIT`].
    pub fn new(kind: &str, idx: usize, n: usize, at: i64) -> Result<SnapSpec, String> {
        let base = match kind {
            "arg" => SnapBase::Arg(idx),
            "ecx" => SnapBase::Ecx,
            "edx" => SnapBase::Edx,
            other => return Err(format!("snap kind {other:?} (expected arg, ecx or edx)")),
        };
        if at.abs() > SNAP_AT_LIMIT {
            return Err(format!(
                "snap at {at} is beyond the {SNAP_AT_LIMIT}-byte limit"
            ));
        }
        // Bounded by SNAP_AT_LIMIT above, so the narrowing is exact.
        let at = i32::try_from(at).map_err(|_| format!("snap at {at} out of range"))?;
        Ok(SnapSpec { base, n, at })
    }

    /// The stub's legacy kind code (0 stack argument, 1 ECX, 2 EDX) and
    /// argument index.
    #[must_use]
    pub fn kind_code(&self) -> (u8, usize) {
        match self.base {
            SnapBase::Arg(i) => (0, i),
            SnapBase::Ecx => (1, 0),
            SnapBase::Edx => (2, 0),
        }
    }

    /// Displacement from the pointer to word `j` of this entry, as the
    /// 32-bit two's-complement value the stub encodes (`at` 0 gives
    /// `4 * j`, the version 4 encoding).
    #[must_use]
    pub fn disp(&self, j: usize) -> u32 {
        // j < SNAP_CAP_WORDS, so 4 * j fits easily in i32.
        let j = i32::try_from(j * 4).unwrap_or(i32::MAX);
        self.at.wrapping_add(j).cast_unsigned()
    }
}

/// Check one callee's snapshot entries against the per-callee cap.
///
/// # Errors
/// Returns a message naming the callee when the words total more than
/// [`SNAP_CAP_WORDS`].
pub fn total_words(id: u32, specs: &[SnapSpec]) -> Result<usize, String> {
    let total: usize = specs.iter().map(|s| s.n).sum();
    if total > SNAP_CAP_WORDS {
        return Err(format!(
            "callee {id} snap declares {total} words; the cap is {SNAP_CAP_WORDS} per callee"
        ));
    }
    Ok(total)
}

/// Where snapshot word `k` (counted across a callee's entries) is stored.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnapSlot {
    /// Byte offset inside the base log entry.
    Base(usize),
    /// Byte offset inside the extension log entry.
    Ext(usize),
}

/// Placement of snapshot word `k`: the first [`SNAP_BASE_WORDS`] in the
/// base entry (as in version 4), the rest from the start of the extension
/// entry.
#[must_use]
pub fn slot(k: usize) -> SnapSlot {
    if k < SNAP_BASE_WORDS {
        SnapSlot::Base(LOG_SNAP_OFF + k * 4)
    } else {
        SnapSlot::Ext((k - SNAP_BASE_WORDS) * 4)
    }
}

/// Bytes of the extension entry used by snapshot words at the cap.
pub const EXT_SNAP_BYTES: usize = (SNAP_CAP_WORDS - SNAP_BASE_WORDS) * 4;

// The extension words must fit in one extension entry.
const _: () = assert!(EXT_SNAP_BYTES <= LOG_ENTRY);
// The base words must end inside the base entry, before the xmm0 slot.
const _: () = assert!(LOG_SNAP_OFF + SNAP_BASE_WORDS * 4 <= 224);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kinds_parse_and_unknown_kind_is_an_error() {
        assert_eq!(SnapSpec::new("arg", 3, 2, 0).unwrap().kind_code(), (0, 3));
        assert_eq!(SnapSpec::new("ecx", 9, 2, 0).unwrap().kind_code(), (1, 0));
        assert_eq!(SnapSpec::new("edx", 9, 2, 0).unwrap().kind_code(), (2, 0));
        assert!(SnapSpec::new("esi", 0, 1, 0).is_err());
    }

    #[test]
    fn offsets_encode_as_twos_complement() {
        let zero = SnapSpec::new("ecx", 0, 4, 0).unwrap();
        // at 0 reproduces the version 4 displacements exactly.
        assert_eq!(
            (0..4).map(|j| zero.disp(j)).collect::<Vec<_>>(),
            vec![0, 4, 8, 12]
        );
        let neg = SnapSpec::new("ecx", 0, 3, -8).unwrap();
        assert_eq!(neg.disp(0), 0xFFFF_FFF8);
        assert_eq!(neg.disp(2), 0);
        let pos = SnapSpec::new("arg", 1, 2, 0x1D8).unwrap();
        assert_eq!(pos.disp(1), 0x1DC);
    }

    #[test]
    fn offset_limit_is_enforced() {
        assert!(SnapSpec::new("ecx", 0, 1, SNAP_AT_LIMIT).is_ok());
        assert!(SnapSpec::new("ecx", 0, 1, -SNAP_AT_LIMIT).is_ok());
        assert!(SnapSpec::new("ecx", 0, 1, SNAP_AT_LIMIT + 4).is_err());
        assert!(SnapSpec::new("ecx", 0, 1, -SNAP_AT_LIMIT - 4).is_err());
    }

    #[test]
    fn cap_fails_loudly_instead_of_truncating() {
        let a = SnapSpec::new("ecx", 0, 40, 0).unwrap();
        let b = SnapSpec::new("arg", 0, 24, -16).unwrap();
        assert_eq!(total_words(1, &[a, b]).unwrap(), 64);
        let c = SnapSpec::new("edx", 0, 1, 0).unwrap();
        let e = total_words(7, &[a, b, c]).unwrap_err();
        assert!(e.contains("callee 7") && e.contains("65"), "{e}");
        assert_eq!(total_words(1, &[]).unwrap(), 0);
    }

    #[test]
    fn placement_keeps_the_first_eight_in_the_base_entry() {
        assert_eq!(slot(0), SnapSlot::Base(192));
        assert_eq!(slot(7), SnapSlot::Base(220));
        assert_eq!(slot(8), SnapSlot::Ext(0));
        assert_eq!(slot(SNAP_CAP_WORDS - 1), SnapSlot::Ext(EXT_SNAP_BYTES - 4));
    }
}
