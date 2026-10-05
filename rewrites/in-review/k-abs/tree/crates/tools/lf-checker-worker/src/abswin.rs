//! The preferred-base window: unrelocated absolute accesses.
//!
//! The worker maps the original image away from its preferred base and
//! applies the relocation table, so a rewrite that forgets the relocation
//! is caught. An instruction whose absolute operand has no relocation entry
//! (the executable has some: it never needs relocating in the game) then
//! still points at the preferred base, which in the worker is unrelated
//! memory. Before version 5 whatever the worker had allocated there (heap
//! blocks, its own copy of the executable file) was read silently: the
//! original computed with a wrong constant and a rewrite matching the file
//! failed (devlog entry "The checker disagreed with the file on one
//! constant").
//!
//! Version 5 holds that window. At start-up the worker reserves it (no
//! commit, so any access faults); the original side's unrelocated accesses
//! then fault loudly and deterministically, and the fault detail names the
//! file address. With the contract option `abs_shadow` the worker also
//! commits a read-only shadow there holding the file's unrelocated headers
//! and read-only sections (what the game sees at its preferred base),
//! readable only while the original runs: a rewrite that reads it faults,
//! because rewrites must derive addresses through the relocated base.
//! Writable sections stay uncommitted in the window: an unrelocated access
//! to writable data still faults.
//!
//! This module plans the window; the worker performs the reservations and
//! protections.

/// Allocation granularity of a reservation.
pub const ABS_CHUNK: usize = 0x1_0000;
/// Page size of a commit or protection change.
pub const PAGE: usize = 0x1000;
/// Start of the window reserved at worker start-up: the conventional
/// preferred base of a 32-bit executable (the rewrite runtime's
/// `relocated` assumes the same base).
pub const EARLY_LO: usize = 0x0040_0000;
/// Length reserved at start-up, before the executable is read, so the
/// worker's own allocations (the file copies among them) cannot land in
/// the window. The setup step extends the reservation if the image is
/// larger.
pub const EARLY_LEN: usize = 0x0300_0000;

/// What the shadow needs to know about one section.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SectionInfo {
    /// Section RVA.
    pub vaddr: usize,
    /// Bytes the section spans in memory (the larger of virtual and raw).
    pub vsize: usize,
    /// The section is writable (its pages stay uncommitted in the shadow).
    pub writable: bool,
}

/// Round `v` down to a multiple of `g` (a power of two).
#[must_use]
pub fn align_down(v: usize, g: usize) -> usize {
    v & !(g - 1)
}

/// Round `v` up to a multiple of `g` (a power of two).
#[must_use]
pub fn align_up(v: usize, g: usize) -> usize {
    v.saturating_add(g - 1) & !(g - 1)
}

/// The window an image with this preferred base and size occupies,
/// widened to reservation granularity: `[lo, hi)`.
#[must_use]
pub fn window(image_base: usize, image_size: usize) -> (usize, usize) {
    (
        align_down(image_base, ABS_CHUNK),
        align_up(image_base.saturating_add(image_size), ABS_CHUNK),
    )
}

/// Whether `addr` lies in `[lo, hi)`.
#[must_use]
pub fn in_window(addr: usize, lo: usize, hi: usize) -> bool {
    lo <= addr && addr < hi
}

/// RVA runs the shadow commits: the headers and every non-writable
/// section, page-aligned, clipped to the image size and merged where they
/// touch. Writable sections are left out so unrelocated access to writable
/// data keeps faulting.
#[must_use]
pub fn shadow_runs(
    headers_len: usize,
    sections: &[SectionInfo],
    image_size: usize,
) -> Vec<(usize, usize)> {
    let end = align_up(image_size, PAGE);
    let mut runs: Vec<(usize, usize)> = Vec::new();
    if headers_len > 0 {
        runs.push((0, align_up(headers_len, PAGE).min(end)));
    }
    for s in sections.iter().filter(|s| !s.writable && s.vsize > 0) {
        let lo = align_down(s.vaddr, PAGE);
        let hi = align_up(s.vaddr.saturating_add(s.vsize), PAGE).min(end);
        if lo < hi {
            runs.push((lo, hi));
        }
    }
    runs.sort_unstable();
    let mut merged: Vec<(usize, usize)> = Vec::new();
    for (lo, hi) in runs {
        match merged.last_mut() {
            Some(last) if lo <= last.1 => last.1 = last.1.max(hi),
            _ => merged.push((lo, hi)),
        }
    }
    merged
}

/// Split absolute runs at the boundaries of the reservations that hold
/// them (a commit or protection change must stay inside one reservation).
///
/// # Errors
/// Returns the first `[lo, hi)` range not inside any held region: the
/// shadow cannot be built there.
pub fn split_runs(
    runs: &[(usize, usize)],
    regions: &[(usize, usize)],
) -> Result<Vec<(usize, usize)>, (usize, usize)> {
    let mut out = Vec::new();
    for &(lo, hi) in runs {
        let mut at = lo;
        while at < hi {
            let Some(&(_, rhi)) = regions.iter().find(|&&(rlo, rhi)| rlo <= at && at < rhi) else {
                // Report the uncovered stretch up to the next held region.
                let next = regions
                    .iter()
                    .map(|r| r.0)
                    .filter(|&rlo| rlo > at)
                    .min()
                    .unwrap_or(hi)
                    .min(hi);
                return Err((at, next));
            };
            let end = hi.min(rhi);
            out.push((at, end));
            at = end;
        }
    }
    Ok(out)
}

/// Chunk starts in `[lo, hi)` (chunk-aligned) that no held region covers.
#[must_use]
pub fn uncovered_chunks(lo: usize, hi: usize, regions: &[(usize, usize)]) -> Vec<usize> {
    (lo..hi)
        .step_by(ABS_CHUNK)
        .filter(|&c| {
            !regions
                .iter()
                .any(|&(rlo, rhi)| rlo <= c && c + ABS_CHUNK <= rhi)
        })
        .collect()
}

/// Diagnostic note for a fault whose address lies in the window, naming
/// the file address and the likely cause. `None` outside the window.
#[must_use]
pub fn fault_note(badva: usize, lo: usize, hi: usize, is_rw: bool, shadow: bool) -> Option<String> {
    if !in_window(badva, lo, hi) {
        return None;
    }
    Some(if is_rw {
        format!(
            "abs-window: the rewrite accessed file VA {badva:#x} directly; derive addresses with relocated()/global()"
        )
    } else if shadow {
        format!(
            "abs-window: unrelocated absolute access to file VA {badva:#x} outside the read-only shadow (writable data, or a write)"
        )
    } else {
        format!(
            "abs-window: unrelocated absolute access to file VA {badva:#x}; declare abs_shadow for read-only data"
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alignment_and_window() {
        assert_eq!(align_down(0x40_1234, PAGE), 0x40_1000);
        assert_eq!(align_up(0x40_1234, PAGE), 0x40_2000);
        assert_eq!(align_up(0x40_1000, PAGE), 0x40_1000);
        assert_eq!(window(0x40_0000, 0x1_2345), (0x40_0000, 0x42_0000));
        assert!(in_window(0x40_0000, 0x40_0000, 0x42_0000));
        assert!(!in_window(0x42_0000, 0x40_0000, 0x42_0000));
    }

    #[test]
    fn shadow_takes_headers_and_read_only_sections() {
        let secs = [
            SectionInfo {
                vaddr: 0x1000,
                vsize: 0x2345,
                writable: false,
            }, // code
            SectionInfo {
                vaddr: 0x4000,
                vsize: 0x1000,
                writable: false,
            }, // read-only data, touching the code run once rounded
            SectionInfo {
                vaddr: 0x5000,
                vsize: 0x3000,
                writable: true,
            }, // writable data: left out
            SectionInfo {
                vaddr: 0x9000,
                vsize: 0,
                writable: false,
            }, // empty: left out
        ];
        let r = shadow_runs(0x400, &secs, 0x8000);
        assert_eq!(r, vec![(0, 0x5000)]);
        // A read-only section past a writable one is its own run, clipped.
        let secs2 = [
            SectionInfo {
                vaddr: 0x2000,
                vsize: 0x1000,
                writable: true,
            },
            SectionInfo {
                vaddr: 0x3000,
                vsize: 0x9000,
                writable: false,
            },
        ];
        assert_eq!(
            shadow_runs(0x200, &secs2, 0x6000),
            vec![(0, 0x1000), (0x3000, 0x6000)]
        );
    }

    #[test]
    fn runs_split_at_reservations_and_report_gaps() {
        let regions = [(0x40_0000, 0x41_0000), (0x41_0000, 0x43_0000)];
        let r = split_runs(&[(0x40_f000, 0x41_2000)], &regions).unwrap();
        assert_eq!(r, vec![(0x40_f000, 0x41_0000), (0x41_0000, 0x41_2000)]);
        let holed = [(0x40_0000, 0x41_0000), (0x42_0000, 0x43_0000)];
        assert_eq!(
            split_runs(&[(0x40_f000, 0x42_1000)], &holed),
            Err((0x41_0000, 0x42_0000))
        );
        assert_eq!(split_runs(&[], &holed), Ok(vec![]));
    }

    #[test]
    fn uncovered_chunks_are_listed() {
        let regions = [(0x40_0000, 0x42_0000), (0x43_0000, 0x44_0000)];
        assert_eq!(
            uncovered_chunks(0x40_0000, 0x45_0000, &regions),
            vec![0x42_0000, 0x44_0000]
        );
        assert!(uncovered_chunks(0x40_0000, 0x42_0000, &regions).is_empty());
    }

    #[test]
    fn fault_notes_name_the_side_and_mode() {
        assert!(fault_note(0x3F_FFFF, 0x40_0000, 0x50_0000, false, false).is_none());
        let o = fault_note(0x40_003C, 0x40_0000, 0x50_0000, false, false).unwrap();
        assert!(o.contains("0x40003c") && o.contains("abs_shadow"), "{o}");
        let s = fault_note(0x40_003C, 0x40_0000, 0x50_0000, false, true).unwrap();
        assert!(s.contains("outside the read-only shadow"), "{s}");
        let w = fault_note(0x40_003C, 0x40_0000, 0x50_0000, true, true).unwrap();
        assert!(w.contains("relocated()"), "{w}");
    }
}
