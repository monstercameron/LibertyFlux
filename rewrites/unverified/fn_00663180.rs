// original: 0x00663180 rage::snMigrateSessionTask::snMigrateSessionTask

/// Construct a session-migration task object.
///
/// `this` points to the object (at least 0x1698 bytes). The constructor
/// installs the class function table, zeroes the scalar fields, fills the
/// 33-entry request array at `+0xA0` (each entry 0x40 bytes: zeroed payload
/// words with -1 markers and zeroed tag words, leaving the three padding
/// word pairs untouched), then runs the member initialisers in order: the
/// base part, the two small members at `+0x920`/`+0x910`, the 32 peer slots
/// at `+0x958` (each 0x68 bytes, flag byte cleared after its own init), and
/// the tail member at `+0x1664`. Finally it stores the two code addresses the
/// task dispatches through and records the per-call answer of the shared
/// attribute hook for the tail member and for every peer slot.
///
/// The six callees are intercepted by the checker and answer from script;
/// their return values other than the hook's are ignored. Returns `this`.
/// Original: 0x00663180 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00663180(this: u32) -> u32 {
    unsafe { sn_migrate_ctor(this) }
});

unsafe fn sn_migrate_ctor(this: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00FE_34A0;
        const TAIL_DISPATCH: u32 = 0x0066_4430;
        const SLOT_DISPATCH: u32 = 0x0066_4640;
        const ENTRIES: u32 = 33;
        const SLOTS: u32 = 32;
        const ENTRY_STRIDE: u32 = 0x40;
        const SLOT_STRIDE: u32 = 0x68;
        const ARRAY_BASE: u32 = 0xA0;
        const SLOT_BASE: u32 = 0x958;
        const C_BASE: u32 = 1;
        const C_M920: u32 = 2;
        const C_M910: u32 = 3;
        const C_SLOT: u32 = 4;
        const C_TAIL: u32 = 5;
        const C_HOOK: u32 = 6;

        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(a: u32, v: u16) {
            unsafe { (a as *mut u16).write_unaligned(v) }
        }

        /// Initialise one 0x40-byte request entry: zero payload, -1 markers,
        /// zero tags. Byte pairs at +0x1A, +0x22 and +0x2A are padding the
        /// original never writes; they are left alone here too.
        #[inline(always)]
        unsafe fn init_entry(w: u32) {
            unsafe {
                wr32(w, 0);
                wr32(w + 0x04, 0);
                wr32(w + 0x08, 0);
                wr32(w + 0x0c, 0);
                wr32(w + 0x10, 0);
                wr32(w + 0x14, 0xffff_ffff);
                wr16(w + 0x18, 0);
                wr32(w + 0x1c, 0xffff_ffff);
                wr16(w + 0x20, 0);
                wr32(w + 0x24, 0xffff_ffff);
                wr16(w + 0x28, 0);
                wr32(w + 0x2c, 0);
                wr32(w + 0x30, 0);
                wr32(w + 0x34, 0);
                wr32(w + 0x38, 0xffff_ffff);
                wr32(w + 0x3c, 0xffff_ffff);
            }
        }

        let hook = lf_checker_rt::relocated(TAIL_DISPATCH);
        let slot_fn = lf_checker_rt::relocated(SLOT_DISPATCH);
        let vtable = lf_checker_rt::relocated(VTABLE);

        let _: u32 = lf_checker_rt::callee_thiscall!(C_BASE, u32, this);
        wr32(this + 0x60, 0);
        wr32(this + 0x64, 0);
        wr32(this + 0x68, 0);
        ((this + 0x6c) as *mut u8).write(((this + 0x6c) as *const u8).read() | 1);
        ((this + 0x6d) as *mut u8).write(0);
        wr32(this, vtable);
        wr32(this + 0x90, 0xffff_ffff);
        wr32(this + 0x94, 0);
        wr32(this + 0x98, 0);
        let mut i = 0u32;
        while i < ENTRIES {
            init_entry(this + ARRAY_BASE + i * ENTRY_STRIDE);
            i += 1;
        }
            wr32(this + 0x904, 0);
        wr32(this + 0x908, 0xffff_ffff);
        wr32(this + 0x918, 0);
        wr32(this + 0x91c, 0);
        let _: u32 = lf_checker_rt::callee_thiscall!(C_M920, u32, this + 0x920);
        let _: u32 = lf_checker_rt::callee_thiscall!(C_M910, u32, this + 0x910);
        let mut s = 0u32;
        while s < SLOTS {
            let slot = this + SLOT_BASE + s * SLOT_STRIDE;
            let _: u32 = lf_checker_rt::callee_thiscall!(C_SLOT, u32, slot);
            let flag = (slot + 0x64) as *mut u8;
            flag.write(flag.read() & 0xfe);
            wr32(slot + 0x64 - 0x0c, 0);
            wr32(slot + 0x64 - 0x08, 0);
            wr32(slot + 0x64 - 0x04, 0);
            s += 1;
        }
        wr32(this + 0x1658, 0);
        wr32(this + 0x165c, 0);
        wr32(this + 0x1660, 0);
        let _: u32 = lf_checker_rt::callee_thiscall!(C_TAIL, u32, this + 0x1664);
        wr32(this + 0x1694, 0);
        wr32(this + 0x1664, 0);
        wr32(this + 0x1668, hook);
        let tail_answer: u32 = lf_checker_rt::callee_thiscall!(C_HOOK, u32, this);
        wr32(this + 0x1664, tail_answer);
        let mut k = 0u32;
        while k < SLOTS {
            let slot = this + SLOT_BASE + k * SLOT_STRIDE;
            wr32(slot, 0);
            wr32(slot + 4, slot_fn);
            let answer: u32 = lf_checker_rt::callee_thiscall!(C_HOOK, u32, this);
            wr32(slot, answer);
            k += 1;
        }
        this
    }
}
