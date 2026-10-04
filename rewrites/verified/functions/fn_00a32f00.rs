// original: 0x00a32f00 model_table_filtered_setup
/// Scan the entity's model table, filter each row and conditionally arm it.
///
/// `this` is the entity. The model index word at +0x2E selects the table;
/// rows whose classifier answers 2 are resolved, scored and vector-checked,
/// and the first row that passes every gate runs the shared 19-argument
/// setup call plus two gate calls, then sets the armed bit (0x100000) on the
/// entity (and on its chain object when the mode bits say so).
/// Returns the value the original leaves in `eax` on the taken exit path.
export!(thiscall, rw_00a32f00(this: u32) -> u32 {
    unsafe {
        const PARAM_TABLE: u32 = 0x0129_5cd8;
        const FLAG_TABLE: u32 = 0x012F_8498;

        /// Second gate of the scan: the entity flag word and the flag byte table.
        /// Returns `Some(exit value)` when the setup path runs to completion.
        #[inline(always)]
        fn check_gates(this: u32, res: u32) -> Option<u32> {
            unsafe {
                let flags = ((this as *const u32).add(10)).read();
                let idx = ((res as *const u32).add(8)).read();
                if (flags & 0x0010_0000) != 0 {
                    let b = global::<u8>(FLAG_TABLE)
                        .offset((idx as i32).wrapping_mul(0x70) as isize)
                        .read();
                    if b == 0 {
                        return None;
                    }
                }
                Some(run_setup(this, res))
            }
        }

        /// Terminal setup path of the scan. Always exits the function with the
        /// original's `eax` value for the taken path.
        #[inline(always)]
        fn run_setup(this: u32, res: u32) -> u32 {
            unsafe {
                let flags = ((this as *const u32).add(10)).read();
                let mut edx = 0u32;
                if (flags & 0x3c0) == 0x100 {
                    let d = ((this as *const u32).add(0x9f)).read();
                    if d != 0 {
                        edx = d;
                    } else {
                        let e = ((this as *const u32).add(0xa0)).read();
                        if e != 0 {
                            let ef = ((e as *const u32).add(10)).read();
                            if (ef & 0x3c0) == 0x100 {
                                edx = ((e as *const u32).add(0x9f)).read();
                            }
                        }
                    }
                }
                let m = ((this as *const u32).add(8)).read();
                let edi = if m != 0 {
                    m.wrapping_add(0x30)
                } else {
                    this.wrapping_add(0x10)
                };
                let mut vec = [0f32, 0f32, 1f32];
                let isnull = if edx == 0 { 1u32 } else { 0u32 };
                let idx2 = ((res as *const u32).add(8)).read();
                let _: u32 = callee_cdecl!(
                    6, u32, 0, 0, idx2, 0x3f80_0000u32, edi, 0, 0, 1, 0xbf80_0000u32, 0, 0,
                    vec.as_mut_ptr() as u32, this, 0, 0, isnull, edx, 0, 0xffff_ffffu32
                );
                let a7: u32 = callee_cdecl!(7, u32,);
                if (a7 as u8) != 0 {
                    let a8: u32 = callee_cdecl!(8, u32,);
                    if (a8 as u8) == 0 {
                        return a8;
                    }
                }
                let f = (this as *mut u32).add(10);
                f.write(f.read() | 0x0010_0000);
                let masked = ((this as *const u32).add(10)).read() & 0x3c0;
                if masked != 0x100 {
                    return masked;
                }
                let e = ((this as *const u32).add(0xa0)).read();
                if e == 0 {
                    return 0;
                }
                let ef = (e as *mut u32).add(10);
                ef.write(ef.read() | 0x0010_0000);
                e
            }
        }
        let slot = ((this as *const u8).add(0x2e) as *const u16).read();
        if slot == 0xffff {
            return 0xffff;
        }
        let table = global::<u32>(PARAM_TABLE).offset(slot as isize).read();
        if ((table as *const u8).add(0x5b).read()) & 0x1c == 0 {
            // eax still holds the sign-extended index here, not the table.
            return slot as u32;
        }
        let mut n: u32 = callee_thiscall!(1, u32, table);
        if (n as i32) <= 0 {
            return n;
        }
        let mut i: u32 = 0;
        loop {
            let item: u32 = callee_thiscall!(2, u32, table, i);
            let vt = (item as *const u32).read() as *const u32;
            let classify: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(vt.add(1).read() as usize);
            if (classify(item) as u8) != 2 {
                // Next row.
            } else {
                let resolve: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(vt.add(9).read() as usize);
                let res = resolve(item);
                let val = ((res as *const u32).add(10).read()) as i32;
                if val >= 0 {
                    // The original passes a one-byte frame flag whose three
                    // neighbour bytes are the table pointer's low bytes; the
                    // checker snapshots the whole word, so mirror it exactly.
                    let mut flag: u32 = 1 | ((table & 0x00ff_ffff) << 8);
                    let r: u32 = callee_cdecl!(
                        5, u32, this, val as u32, 1, &mut flag as *mut u32 as u32
                    );
                    if r != 0 {
                        let f = r as *const f32;
                        let x = f.read();
                        let y = f.add(1).read();
                        let z = f.add(2).read();
                        if !(x == 0.0 && y == 0.0 && z == 0.0) {
                            // Falls into the same flag gate as the
                            // negative-score path below.
                            let arm = check_gates(this, res);
                            if let Some(v) = arm {
                                return v;
                            }
                        }
                    }
                } else {
                    let arm = check_gates(this, res);
                    if let Some(v) = arm {
                        return v;
                    }
                }
            }
            i = i.wrapping_add(1);
            n = callee_thiscall!(1, u32, table);
            if (i as i32) >= (n as i32) {
                return n;
            }
        }
    }
});
