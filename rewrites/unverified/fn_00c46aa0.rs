// original: 0x00c46aa0 CCamFinal::vf1 (symbols)
/// Attach the final camera to its owner and register eight entries.
///
/// When the owner at `this + OWNER` does not point back at `this`
/// there is nothing to do. Otherwise marks `this` active and, for
/// each of eight entry kinds, asks the owner for the entry object
/// (callees 1-8, called with the kind, 0 and `this`) and activates it
/// through its vtable slot 4 (callee 11). Two entries are first
/// cleared of `FLAG_X` at `MARK`, the fifth entry additionally runs
/// its own setup (callee 9) before activation, and the sixth entry
/// gets `FLAG_Y` set at `MARK` before the seventh is requested. The
/// sixth entry is activated once more at the end. Returns 1 in the
/// low byte on every path.
///
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00c46aa0(this: u32) -> u32 {
    const OWNER: u32 = 0x114;
    const BACKLINK: u32 = 4;
    const ACTIVE: u32 = 0x1b4;
    const MARK: u32 = 0x13c;
    const FLAG_X: u8 = 4;
    const FLAG_Y: u8 = 4;
    const VT_SLOT_ACTIVATE: u32 = 4;
    const LOOKUP_IDS: [u32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    const KINDS: [u32; 8] = [7, 0x20, 6, 0x0f, 0x27, 0x1d, 0x14, 0x25];
    const ENTRY_SETUP: u32 = 9;
    const ACTIVATE: u32 = 11;
    unsafe fn activate(obj: u32) {
        unsafe {
            let vtable = (obj as *const u32).read_unaligned();
            let f: extern "thiscall" fn(u32) -> u32 = core::mem::transmute(
                ((vtable + VT_SLOT_ACTIVATE) as *const u32).read_unaligned() as usize,
            );
            f(obj);
        }
    }
    unsafe {
        let owner = ((this + OWNER) as *const u32).read_unaligned();
        if ((owner + BACKLINK) as *const u32).read_unaligned() != this {
            return 1;
        }
        ((this + ACTIVE) as *mut u8).write(1);
        let mut k = 0usize;
        while k < 4 {
            let ans =
                lf_checker_rt::callee_thiscall!(LOOKUP_IDS[k], u32, owner, KINDS[k], 0, this);
            if k >= 2 {
                let m = (ans + MARK) as *mut u8;
                m.write(m.read() & !FLAG_X);
            }
            activate(ans);
            k += 1;
        }
        let fifth = lf_checker_rt::callee_thiscall!(LOOKUP_IDS[4], u32, owner, KINDS[4], 0, this);
        lf_checker_rt::callee_thiscall!(ENTRY_SETUP, u32, fifth);
        let m = (fifth + MARK) as *mut u8;
        m.write(m.read() & !FLAG_X);
        activate(fifth);
        let sixth = lf_checker_rt::callee_thiscall!(LOOKUP_IDS[5], u32, owner, KINDS[5], 0, this);
        let m6 = (sixth + MARK) as *mut u8;
        m6.write((m6.read() & 0xf7) | FLAG_Y);
        let ans = lf_checker_rt::callee_thiscall!(LOOKUP_IDS[6], u32, owner, KINDS[6], 0, this);
        activate(ans);
        let ans = lf_checker_rt::callee_thiscall!(LOOKUP_IDS[7], u32, owner, KINDS[7], 0, this);
        activate(ans);
        activate(sixth);
    }
    1
});
