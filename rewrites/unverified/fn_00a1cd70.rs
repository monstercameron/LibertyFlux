// original: 0x00A1CD70 task_refresh_goal_and_range_flag (proposed)

/// Refresh the task's goal record from a candidate and maintain its
/// out-of-range flag.
///
/// `this` is the task object, `owner` is an owning object whose dword at
/// `+0x2e0` heads a type-tagged node list, and `cand` points at a 16-byte
/// candidate record (three floats and a dword).
///
/// If the task's mode at `+0x130` is 0, the candidate is copied verbatim to
/// the goal slot at `+0x200` first. Then the node list is walked (node type
/// at `+4`, link at `+0xc`) looking for type `0x2de`, with an order check:
/// the key `(node[8] >> 1) & 7` of the FIRST node is compared against each
/// later node's key, and when the first key is both below the current key
/// and at least 2 the search aborts with result 1. (The first node's key is
/// computed once, outside the loop-back edge; only the current key is
/// recomputed per node.) When a node of type `0x2de` is reached, the
/// candidate is copied to the goal slot again, the range flag (bit 0 of the
/// byte at `+0x38c`) is cleared and the function returns 0.
///
/// Otherwise the function returns 1, and unless the range flag is already
/// set it measures the distance from the candidate point to the goal point
/// (`sqrt((dy*dy + dx*dx) + dz*dz)`, subtractions `(candidate - goal)` in x,
/// y, z order) and sets the flag when the distance is strictly above the
/// tuned limit. The original's branch is unsigned-below-or-equal, which is
/// taken for unordered (NaN) results too, so a NaN distance leaves the flag
/// clear.
///
/// The low byte of the return value is the defined result (0 or 1); the
/// upper three bytes are register leftovers (part of `owner` or of the
/// candidate's last dword, depending on the path) and are not reproduced.
///
/// The limit is read through `relocated` because the original's reference to
/// it carries no relocation entry; the proof pins limit-straddling inputs
/// (0.9, 1.0, 1.0+1ulp, 1.1 against the file's 1.0) so any value other than
/// the file's constant at the original's address would fail the run.
///
/// Original: 0x00A1CD70 (thiscall, two stack words).
lf_checker_rt::export!(thiscall, rw_00A1CD70(this: u32, owner: u32, cand: u32) -> u32 {
    unsafe {
        const MODE: u32 = 0x130;
        const GOAL: u32 = 0x200;
        const LIST_HEAD: u32 = 0x2e0;
        const NODE_TYPE: u32 = 0x4;
        const NODE_KEY: u32 = 0x8;
        const NODE_LINK: u32 = 0xc;
        const WANT_TYPE: u32 = 0x2de;
        const RANGE_FLAG: u32 = 0x38c;
        const LIMIT_VA: u32 = 0x00fe88e8;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn add(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) + core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }
        #[inline(always)]
        unsafe fn copy_goal(this: u32, cand: u32) {
            unsafe {
                wr32(this + GOAL, rd32(cand));
                wr32(this + GOAL + 4, rd32(cand + 4));
                wr32(this + GOAL + 8, rd32(cand + 8));
                wr32(this + GOAL + 12, rd32(cand + 12));
            }
        }

        if rd32(this + MODE) == 0 {
            copy_goal(this, cand);
        }
        let mut node = rd32(owner + LIST_HEAD);
        let mut result = 1u32;
        if node != 0 {
            // The first node's key stays live for the whole walk; the loop
            // only recomputes the current node's key.
            let first_key = (rd32(node + NODE_KEY) >> 1) & 7;
            loop {
                let key = (rd32(node + NODE_KEY) >> 1) & 7;
                if first_key < key && first_key >= 2 {
                    break;
                }
                if rd32(node + NODE_TYPE) == WANT_TYPE {
                    copy_goal(this, cand);
                    let flag = (this as *mut u8).add(RANGE_FLAG as usize);
                    flag.write(flag.read() & !1);
                    result = 0;
                    break;
                }
                node = rd32(node + NODE_LINK);
                if node == 0 {
                    break;
                }
            }
        }
        let flag = (this as *mut u8).add(RANGE_FLAG as usize);
        if flag.read() & 1 == 0 {
            let dx = sub(rdf(cand), rdf(this + GOAL));
            let dy = sub(rdf(cand + 4), rdf(this + GOAL + 4));
            let dz = sub(rdf(cand + 8), rdf(this + GOAL + 8));
            let d = add(add(mul(dy, dy), mul(dx, dx)), mul(dz, dz)).sqrt();
            let limit = f32::from_bits(rd32(lf_checker_rt::relocated(LIMIT_VA)));
            if d > limit {
                flag.write(flag.read() | 1);
            }
        }
        result
    }
});
