// original: 0x00be4c70 task_find_nearest_candidate (proposed)

/// Scan for the nearest candidate to a point, then report and reset.
///
/// `obj` (first stack word) points at four floats: the query point's x, y, z
/// and radius. They are published to the shared query globals, the shared
/// best pair (object, distance) is cleared, and the scan callee walks the
/// world with (float-block, candidate-callback, context, 16, 13), where the
/// float block repeats the query point's x, y, z, leaves its fourth word
/// unwritten (stale stack contents, skipped in the contract), and carries
/// the float 5.0 in its fifth, and the callback is this module's candidate
/// test. Whatever best object the scan leaves behind is returned. Afterwards
/// the best pair and the query point are all reset to zero, so no trace of
/// the scan remains in the globals.
///
/// Everything here is bit movement (no float arithmetic); the rewrite copies
/// words, and the float block it hands over carries the same words the
/// original stages in its own frame (compared by content, not address).
///
/// Original: 0x00be4c70 (cdecl, two stack words: object, context).
lf_checker_rt::export!(cdecl, rw_00be4c70(obj: u32, context: u32) -> u32 {
    unsafe {
        const QUERY: u32 = 0x01682ec0;
        const BEST_OBJ: u32 = 0x0167f5a4;
        const BEST_DIST: u32 = 0x0167f630;
        const CANDIDATE_FILE_VA: u32 = 0x00be4ba0;
        const FIFTH_BITS: u32 = 0x40a00000; // 5.0f
        const SCAN: u32 = 1;
        let f0 = (obj as *const u32).read_unaligned();
        let f1 = (obj.wrapping_add(4) as *const u32).read_unaligned();
        let f2 = (obj.wrapping_add(8) as *const u32).read_unaligned();
        let f3 = (obj.wrapping_add(12) as *const u32).read_unaligned();
        (lf_checker_rt::global::<u32>(QUERY) as *mut u32).write_unaligned(f0);
        (lf_checker_rt::global::<u32>(QUERY.wrapping_add(4)) as *mut u32).write_unaligned(f1);
        (lf_checker_rt::global::<u32>(QUERY.wrapping_add(8)) as *mut u32).write_unaligned(f2);
        (lf_checker_rt::global::<u32>(QUERY.wrapping_add(12)) as *mut u32).write_unaligned(f3);
        (lf_checker_rt::global::<u32>(BEST_OBJ) as *mut u32).write_unaligned(0);
        (lf_checker_rt::global::<u32>(BEST_DIST) as *mut u32).write_unaligned(0);
        // Word 3 is never written by the original (stale stack); the
        // contract snaps words 0-2 and 4 and skips it. Zero here stands in
        // for the unknowable.
        let block = [f0, f1, f2, 0, FIFTH_BITS];
        lf_checker_rt::callee_cdecl!(
            SCAN,
            u32,
            block.as_ptr() as u32,
            lf_checker_rt::relocated(CANDIDATE_FILE_VA),
            context,
            0x10,
            0x0d
        );
        let best = (lf_checker_rt::global::<u32>(BEST_OBJ) as *const u32).read_unaligned();
        (lf_checker_rt::global::<u32>(BEST_OBJ) as *mut u32).write_unaligned(0);
        (lf_checker_rt::global::<u32>(BEST_DIST) as *mut u32).write_unaligned(0);
        (lf_checker_rt::global::<u32>(QUERY.wrapping_add(8)) as *mut u32).write_unaligned(0);
        (lf_checker_rt::global::<u32>(QUERY.wrapping_add(4)) as *mut u32).write_unaligned(0);
        (lf_checker_rt::global::<u32>(QUERY) as *mut u32).write_unaligned(0);
        best
    }
});
