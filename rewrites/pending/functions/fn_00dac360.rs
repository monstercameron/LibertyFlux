// original: 0x00dac360 flee_plan_chain
/// Run the chained flee-feasibility checks and report whether all pass.
///
/// Six checks run in a fixed order with early exits: an area check, the
/// probe gate (whose answer also feeds the final check), two fallback checks
/// when the probe declines, a clearance check unless skipped by flag, and a
/// final weighted check. Returns 1 when the chain accepts, 0 otherwise.
export!(cdecl, rw_00dac360(a0: u32, a1: u32, a2: f32, a3: u32, a4: u32, a5: u32) -> u32 {
    unsafe {
        /// Weight passed to the clearance check.
        const CLEAR_W: u32 = 0x3ff33333;
        let a2b = a2.to_bits();
        let r1: u32 = callee_cdecl!(1, u32, a0, a1, a2b);
        if r1 as u8 == 0 {
            return 0;
        }
        let probe: u32 = callee_cdecl!(2, u32, a0, a5);
        let pal = probe as u8;
        if pal == 0 {
            let r3: u32 = callee_cdecl!(3, u32, a0, a1, a2b);
            if r3 as u8 == 0 {
                return 0;
            }
            let r4: u32 = callee_cdecl!(4, u32, a0, a1, a2b);
            if r4 as u8 == 0 {
                return 0;
            }
        }
        // The original tests the low bytes of the flag words.
        if a4 as u8 == 0 {
            let r5: u32 = callee_cdecl!(5, u32, a0, a1, a2b, CLEAR_W);
            if a3 as u8 != 0 {
                return (r5 as u8 != 0) as u32;
            }
            if r5 as u8 != 0 {
                return 1;
            }
        }
        // The original pushes the saved probe byte as a full word, leaving
        // the upper bytes as entry-register residue; the contract compares
        // only the low byte of this argument.
        let r6: u32 = callee_cdecl!(6, u32, a0, a1, a2b, 0, pal as u32);
        (r6 as u8 != 0) as u32
    }
});
