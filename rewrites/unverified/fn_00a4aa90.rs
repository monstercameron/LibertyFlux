// original: 0x00A4AA90 vehicle_id_in_table_b (proposed)

/// True when the argument equals any of seven global id words, else defers to
/// the thirteen-word check via a tail jump.
///
/// Compares the argument against seven dword globals in order; on a match
/// returns 1 without calling. Otherwise tail-jumps to the thirteen-word
/// check with the same argument and returns its result. The checker
/// intercepts the tail jump as a call: the rewrite invokes the callee stub
/// and forwards its answer.
///
/// Original: 0x00A4AA90 (cdecl, one stack word; E9 tail jump at the end).
lf_checker_rt::export!(cdecl, rw_00A4AA90(val: u32) -> u32 {
    unsafe {
        const IDS: [u32; 7] = [
            0x012FA428, 0x012FA56C, 0x012F9E34, 0x012FA59C, 0x012F9E88,
            0x012FA374, 0x012F9F30,
        ];
        const TAIL_CALLEE: u32 = 1;
        for file_va in IDS {
            if lf_checker_rt::global::<u32>(file_va).read_unaligned() == val {
                return 1;
            }
        }
        lf_checker_rt::callee_cdecl!(TAIL_CALLEE, u32, val)
    }
});
