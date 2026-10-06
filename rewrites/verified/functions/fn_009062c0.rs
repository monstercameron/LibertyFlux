// original: 0x009062c0 input_cell_refresh (proposed)
/// Refresh one grid cell after validating its coordinates.
///
/// Returns entry `eax` when `x` is negative, the dimension when `x` or
/// `y` fails any later bound test (all signed against `limit - 1`), where
/// `limit` is the static dimension.
/// Otherwise looks the cell up in the static table (adding the static
/// offset when the mode byte is set) and returns on a -1 entry; then runs
/// the check callee on `(cell, flag)` (low byte must be set), the recheck
/// callee on `(cell)` (low byte must be set) and the settle callee on
/// `(cell)` (full answer must be zero), and finally runs the commit callee
/// on `(cell, flag)` and returns its answer. Cdecl, two stack words.
export!(cdecl, rw_009062c0(x: u32, y: u32) -> u32 {
    unsafe {
        /// Static grid dimension (file VA).
        const DIM: u32 = 0x010344E4;
        /// Static mode byte (file VA).
        const MODE: u32 = 0x011609F6;
        /// Static index offset added when the mode byte is set (file VA).
        const OFF: u32 = 0x010344EC;
        /// Static cell table base (file VA).
        const BASE: u32 = 0x0118F4E8;
        /// Static flag word passed to two callees (file VA).
        const FLAG: u32 = 0x01032F58;
        const CHECK_ID: u32 = 1;
        const RECHECK_ID: u32 = 2;
        const SETTLE_ID: u32 = 3;
        const COMMIT_ID: u32 = 4;
        let n = (global::<u32>(DIM)).read_unaligned() as i32;
        let lim = n.wrapping_sub(1);
        // Signed bounds (js/jg). Only the first failing test returns entry
        // eax; the rest return the dimension still in eax.
        if (x as i32) < 0 {
            return 0;
        }
        if (x as i32) > lim || (y as i32) < 0 || (y as i32) > lim {
            return n as u32;
        }
        let mut idx = (n as u32).wrapping_mul(y).wrapping_add(x);
        if (global::<u8>(MODE)).read() != 0 {
            idx = idx.wrapping_add((global::<u32>(OFF)).read_unaligned());
        }
        let base = (global::<u32>(BASE)).read_unaligned();
        let cell = ((base.wrapping_add(idx.wrapping_mul(4))) as *const u32).read_unaligned();
        if cell == 0xFFFFFFFF {
            return cell;
        }
        let flag = (global::<u32>(FLAG)).read_unaligned();
        let r1: u32 = callee_cdecl!(CHECK_ID, u32, cell, flag);
        if (r1 as u8) == 0 {
            return r1;
        }
        let cell2 = ((base.wrapping_add(idx.wrapping_mul(4))) as *const u32).read_unaligned();
        let r2: u32 = callee_cdecl!(RECHECK_ID, u32, cell2);
        if (r2 as u8) == 0 {
            return r2;
        }
        let r3: u32 = callee_cdecl!(SETTLE_ID, u32, cell2);
        if r3 != 0 {
            return r3;
        }
        callee_cdecl!(COMMIT_ID, u32, cell2, flag)
    }
});
