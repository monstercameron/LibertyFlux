// original: 0x00a32970 resolve_row_scan_report
/// Resolve the entity's indexed row, then scan and report matching rows.
///
/// `this` is the entity; `a0`/`a1` are report payload words, `a2` selects
/// the row, `a3` is the report's float word. The model index word at +0x2E
/// selects the table;
/// the row's score sets a flag, its link word yields a key, and then every
/// table row whose gate answers 1 and whose record matches the key (or is
/// unkeyed) is reported through the shared dispatcher (callee 7).
/// Returns the value the original leaves in `eax` on the taken exit path.
export!(thiscall, rw_00a32970(this: u32, a0: u32, a1: u32, a2: u32, a3: u32) -> u32 {
    unsafe {
        const PARAM_TABLE: u32 = 0x0129_5cd8;
        const DISPATCHER: u32 = 0x0171_DEE8;
        let slot = ((this as *const u8).add(0x2e) as *const u16).read();
        if slot == 0xffff {
            return 0xffff;
        }
        let table = global::<u32>(PARAM_TABLE).offset(slot as isize).read();
        if ((table as *const u8).add(0x5b).read()) & 2 == 0 {
            return table;
        }
        let mut flag: u32 = 0;
        let mut ebp: u32 = 0;
        let vt = (this as *const u32).read() as *const u32;
        let get: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(vt.add(0x28).read() as usize);
        let first = get(this);
        if first != 0 {
            let second = get(this);
            let edi: u32 = callee_thiscall!(2, u32, second);
            let arr = ((edi as *const u32).add(0x35).read()) as *const u32;
            let row = arr.add(a2 as usize).read();
            if row == 0 {
                return edi;
            }
            let mut eax = 0u32;
            let inner = ((first as *const u32).add(0x19).read()) as *const u32;
            if !inner.is_null() {
                let cl = ((row as *const u8).add(0x0c).read()) as usize;
                let farr = ((inner as *const u32).add(0x41).read()) as *const f32;
                if !(farr.add(cl).read() > 0.0) {
                    flag = 1;
                }
                eax = flag;
            }
            let cx = ((row as *const u8).add(0x0e) as *const i16).read() as i32;
            if cx == -1 {
                return eax;
            }
            let e1 = ((edi as *const u32).add(0x78).read()) as *const u32;
            if e1.is_null() {
                return 0;
            }
            let e2 = ((e1 as *const u32).add(1).read()) as *const u32;
            let base = e2.read() as *const u8;
            let target = base.offset(cx.wrapping_mul(0xe0) as isize);
            if target.is_null() {
                return 0;
            }
            ebp = ((target as *const u8).add(0x16) as *const u16).read() as u32;
        }
        // Scan section: runs whether or not the row resolved.
        let slot2 = ((this as *const u8).add(0x2e) as *const i16).read() as isize;
        let table2 = global::<u32>(PARAM_TABLE).offset(slot2).read();
        let mut n: u32 = callee_thiscall!(3, u32, table2);
        if (n as i32) <= 0 {
            return n;
        }
        let mut i: u32 = 0;
        loop {
            let item: u32 = callee_thiscall!(4, u32, table2, i);
            let vt2 = (item as *const u32).read() as *const u32;
            let gate: extern "thiscall" fn(u32) -> u32 =
                core::mem::transmute(vt2.add(1).read() as usize);
            if (gate(item) as u8) == 1 {
                let get2: extern "thiscall" fn(u32) -> u32 =
                    core::mem::transmute(vt2.add(7).read() as usize);
                let res = get2(item);
                let mut fire = true;
                if (flag as u8) != 0 && ((res as *const u8).add(0x38).read()) != 0 {
                    fire = false;
                }
                if fire && ((res as *const u32).add(9).read()) != 1 {
                    fire = false;
                }
                if fire {
                    let c = ((res as *const u32).add(10).read());
                    if c != ebp && c != 0xffff_ffff {
                        fire = false;
                    }
                }
                if fire {
                    let disp = lf_checker_rt::relocated(DISPATCHER);
                    let _: u32 =
                        callee_thiscall!(7, u32, disp, this, res, a0, a1, ebp, a3);
                }
            }
            i = i.wrapping_add(1);
            n = callee_thiscall!(3, u32, table2);
            if !((i as i32) < (n as i32)) {
                return n;
            }
        }
    }
});
