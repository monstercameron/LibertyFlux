// original: 0x00ae2520 ui_float_pair_emit
/// Store an integer/float pair into a fresh record and register it.
///
/// Allocates the record through the block source, stores the two arguments at
/// `+0/+4`, and when the global registry flag is set drives the record
/// through the registry sink until the sink echoes it back. Returns the record.
export!(cdecl, rw_00ae2520(a0: u32, a1: u32) -> u32 {
    unsafe {
        let blk = callee_cdecl!(1, u32, relocated(0x1593BC4), 8);
        (blk as *mut u32).write(a0);
        ((blk + 4) as *mut u32).write(a1);
        if (global::<u8>(0x15B2B91)).read() == 0 {
            return blk;
        }
        let rec = blk.wrapping_add(8);
        loop {
            let r = callee_cdecl!(2, u32, relocated(0x15B2B78), rec, blk);
            if r == blk {
                break;
            }
        }
        blk
    }
});
