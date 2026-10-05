// original: 0x00c47030 CCamScript::vf4 (symbols)
/// Refresh every registered script camera from this camera's matrix.
///
/// Resets the two shared scales to 1.0, then asks the gate (callee 1);
/// when it declines there is nothing to do. Otherwise reads table slot
/// `TABLE_GLOBAL[index]` where `index` is the word stored at
/// `TABLE_GLOBAL` itself (index 0 always selects word 0, which is the
/// index, so it refreshes nothing). For each of the `count` entries
/// (a halfword at `COUNT_OFF`) in the chosen table: fetches the
/// element, asks its vtable slot `0x34` whether it accepts (callee 11)
/// and skips it on refusal; otherwise asks slot `0x2c` for the target
/// object (callee 12) and skips on null; otherwise copies this
/// camera's 4-word matrix at `MATRIX` into the target at `DST_A` and
/// `DST_B`, clears the target's flag byte at `FLAG_OFF`, and applies
/// the update through the target's vtable slot 8 (callee 13, called
/// with 0 and -2). Returns 1 in the low byte on every path.
///
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00c47030(this: u32) -> u32 {
    const SCALE_A: u32 = 0x01032350;
    const SCALE_B: u32 = 0x0103234c;
    const TABLE_GLOBAL: u32 = 0x0118d818;
    const ONE: u32 = 0x3f80_0000;
    const ARRAY_OFF: u32 = 0x400;
    const COUNT_OFF: u32 = 0x404;
    const MATRIX: u32 = 0x40;
    const DST_A: u32 = 0x20;
    const DST_B: u32 = 0x10;
    const FLAG_OFF: u32 = 0x3c;
    const VT_ACCEPT: u32 = 0x34;
    const VT_FETCH: u32 = 0x2c;
    const VT_APPLY: u32 = 8;
    const GATE: u32 = 1;
    const ACCEPT: u32 = 11;
    const FETCH: u32 = 12;
    const APPLY: u32 = 13;
    unsafe fn accept(elem: u32) -> u32 {
        unsafe {
            let vtable = (elem as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                ((vtable + VT_ACCEPT) as *const u32).read_unaligned() as usize,
            );
            f(elem)
        }
    }
    unsafe fn fetch(elem: u32) -> u32 {
        unsafe {
            let vtable = (elem as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                ((vtable + VT_FETCH) as *const u32).read_unaligned() as usize,
            );
            f(elem)
        }
    }
    unsafe fn apply(target: u32) {
        unsafe {
            let vtable = (target as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32, u32, u32) -> u32 = core::mem::transmute(
                ((vtable + VT_APPLY) as *const u32).read_unaligned() as usize,
            );
            f(target, 0, 0xffff_fffe);
        }
    }
    unsafe {
        lf_checker_rt::global::<u32>(SCALE_A).write_unaligned(ONE);
        lf_checker_rt::global::<u32>(SCALE_B).write_unaligned(ONE);
        if lf_checker_rt::callee_thiscall!(GATE, u32, this) & 0xff == 0 {
            return 1;
        }
        let idx = lf_checker_rt::global::<u32>(TABLE_GLOBAL).read_unaligned();
        let table =
            lf_checker_rt::global::<u32>(TABLE_GLOBAL + idx.wrapping_mul(4)).read_unaligned();
        if table == 0 {
            return 1;
        }
        let count = ((table + COUNT_OFF) as *const u16).read_unaligned() as u32;
        let mut i = 0u32;
        while i < count {
            let array = ((table + ARRAY_OFF) as *const u32).read_unaligned();
            let elem = ((array + i.wrapping_mul(4)) as *const u32).read_unaligned();
            if accept(elem) & 0xff == 0 {
                i += 1;
                continue;
            }
            let array = ((table + ARRAY_OFF) as *const u32).read_unaligned();
            let elem = ((array + i.wrapping_mul(4)) as *const u32).read_unaligned();
            let target = fetch(elem);
            if target == 0 {
                i += 1;
                continue;
            }
            let m0 = ((this + MATRIX) as *const u32).read_unaligned();
            let m1 = ((this + MATRIX + 4) as *const u32).read_unaligned();
            let m2 = ((this + MATRIX + 8) as *const u32).read_unaligned();
            let m3 = ((this + MATRIX + 12) as *const u32).read_unaligned();
            ((target + DST_A) as *mut u32).write_unaligned(m0);
            ((target + DST_A + 4) as *mut u32).write_unaligned(m1);
            ((target + DST_A + 8) as *mut u32).write_unaligned(m2);
            ((target + DST_A + 12) as *mut u32).write_unaligned(m3);
            ((target + DST_B) as *mut u32).write_unaligned(m0);
            ((target + DST_B + 4) as *mut u32).write_unaligned(m1);
            ((target + DST_B + 8) as *mut u32).write_unaligned(m2);
            ((target + DST_B + 12) as *mut u32).write_unaligned(m3);
            ((target + FLAG_OFF) as *mut u8).write(0);
            apply(target);
            i += 1;
        }
    }
    1
});
