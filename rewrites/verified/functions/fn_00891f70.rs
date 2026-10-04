// original: 0x00891f70 aud_sound_start
/// Starts a sound through the dispatch tables.
///
/// Records the argument, stamps state 2 and probes the chain predicate: when
/// it reports false the shared finish tail runs. Otherwise the word at 0x3c
/// feeds the index callee and the dispatch table entry selected by the byte
/// at 0x3b runs with this object, that answer and zero; an answer of 1 also
/// takes the finish tail, any other answer sets disable bit 1 and returns it.
/// The finish tail re-records the argument, runs its callee, then either
/// records state 1 (when the word at 0x50 is set) or state 2 plus the second
/// table's handler.
export!(thiscall, rw_00891f70(this: *mut u8, arg: u32) -> u32 {
    unsafe {
        let _: u32 = callee_thiscall!(1, u32, this as u32);
        *(this.add(0x84) as *mut u32) = arg;
        let _: u32 = callee_thiscall!(2, u32, this as u32);
        let flag50 = *(this.add(0x50) as *const u32);
        *(this.add(6) as *mut u16) = 2;
        let probe: u32 = callee_thiscall!(3, u32, this as u32);
        if probe & 0xff == 0 {
            return rw_00891f70_tail(this, arg, flag50);
        }
        let w = *(this.add(0x3c) as *const i16) as i32 as u32;
        let v: u32 = callee_cdecl!(4, u32, w);
        let idx = *(this.add(0x3b)) as u32;
        let base = relocated(0x115d654);
        let callee = *((base.wrapping_add(idx.wrapping_mul(4))) as *const u32);
        let f: extern "cdecl" fn(u32, u32, u32) -> u32 =
            core::mem::transmute(callee as usize);
        let r = f(this as u32, v, 0);
        if r == 1 {
            return rw_00891f70_tail(this, arg, flag50);
        }
        *(this.add(0x39)) |= 2;
        r
    }
});

/// Shared finish tail of `rw_00891f70` (a single code block reached by two
/// jumps in the original).
unsafe fn rw_00891f70_tail(this: *mut u8, arg: u32, flag50: u32) -> u32 {
    unsafe {
        *(this.add(0x84) as *mut u32) = arg;
        let _: u32 = callee_thiscall!(5, u32, this as u32, arg);
        if flag50 != 0 {
            *(this.add(0x42) as *mut u16) = 1;
            return 1;
        }
        *(this.add(0x42) as *mut u16) = 2;
        let idx = *(this.add(0x3b)) as u32;
        let base = relocated(0x115d594);
        let callee = *((base.wrapping_add(idx.wrapping_mul(4))) as *const u32);
        let f: extern "cdecl" fn(u32, u32) -> u32 =
            core::mem::transmute(callee as usize);
        f(this as u32, arg)
    }
}
