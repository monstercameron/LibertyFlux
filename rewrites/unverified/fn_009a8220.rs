// original: 0x009a8220 script_slot_release
/// Release script slot `arg0` through the three-stage teardown.
///
/// Returns 1 on every path; the observable behaviour is which of the
/// three teardown calls fire. When the dword at `this+0x34cc` is zero
/// nothing fires. Otherwise the opener (stubbed, thiscall/1, and it
/// preserves ECX so the object pointer survives into the next call)
/// runs: a false answer ends the sequence. A true answer continues
/// only when the dword at `this+0x2e70` is zero; then the releaser
/// (stubbed, thiscall/1) runs, and on its true answer the same zero
/// word is re-checked (it cannot have changed, the stubs write
/// nothing) before the closer (stubbed, thiscall/0) runs. Thiscall,
/// two stack words (the second is never read), dword result always 1.
export!(thiscall, rw_009A8220(this: u32, arg0: u32, _arg1: u32) -> u32 {
    unsafe {
        if ((this + 0x34cc) as *const u32).read_unaligned() == 0 {
            return 1;
        }
        let open: u32 = callee_thiscall!(1, u32, this, arg0);
        if open as u8 == 0 {
            return 1;
        }
        if ((this + 0x2e70) as *const u32).read_unaligned() != 0 {
            return 1;
        }
        let rel: u32 = callee_thiscall!(2, u32, this, arg0);
        if rel as u8 == 0 {
            return 1;
        }
        if ((this + 0x2e70) as *const u32).read_unaligned() != 0 {
            return 1;
        }
        let _: u32 = callee_thiscall!(3, u32, this);
        1
    }
});
