// original: 0x00a979e0 filemem_touch_entry

/// Touch an entry unless it is skipped: resolve, bind, check kind, commit.
///
/// `arg` points to the entry. Returns at once when its flag word at
/// `+0x24` carries bit 0x4000000 (bit 26). Otherwise the resolve callee
/// (vtable slot `+0xa0`) names its subject, defaulting to the word at
/// `+0x38` on a null answer; the touch callee (vtable slot `+0x18` of the
/// object at subject `+0x4`) produces the handle, which the bind callee
/// files under the subject; and when the kind byte at handle `+0xc`
/// dereferenced `+0x4` is 0xc, the commit callee runs on that kind object.
///
/// Original: 0x00A979E0 (stdcall, one stack word; two indirect callees,
/// two direct callees).
lf_checker_rt::export!(stdcall, rw_00a979e0(arg: u32) -> u32 {
    unsafe {
        /// Skip flag bit (at +0x24) and resolve machinery.
        const SKIP_FLAG: u32 = 0x04000000;
        const FLAG_OFF: u32 = 0x24;
        const VT_RESOLVE: u32 = 0xa0;
        const SUB_OFF: u32 = 0x38;
        /// Touch target (at subject +0x4) and its vtable slot.
        const MID_OFF: u32 = 0x4;
        const VT_TOUCH: u32 = 0x18;
        /// Kind object (at handle +0xc), kind byte and wanted value.
        const KIND_OBJ_OFF: u32 = 0xc;
        const KIND_OFF: u32 = 0x4;
        const KIND_WANT: u8 = 0xc;
        const BIND: u32 = 3;
        const COMMIT: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }

        if rd32(arg.wrapping_add(FLAG_OFF)) & SKIP_FLAG != 0 {
            return 0;
        }
        let slot = rd32(rd32(arg).wrapping_add(VT_RESOLVE));
        let resolve: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(slot as usize);
        let mut sub = resolve(arg);
        if sub == 0 {
            sub = rd32(arg.wrapping_add(SUB_OFF));
        }
        let mid = rd32(sub.wrapping_add(MID_OFF));
        let tslot = rd32(rd32(mid).wrapping_add(VT_TOUCH));
        let touch: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(tslot as usize);
        let handle = touch(mid);
        let _: u32 = lf_checker_rt::callee_thiscall!(BIND, u32, sub, handle);
        let kobj = rd32(handle.wrapping_add(KIND_OBJ_OFF));
        if ((kobj.wrapping_add(KIND_OFF)) as *const u8).read() != KIND_WANT {
            return 0;
        }
        let _: u32 = lf_checker_rt::callee_thiscall!(COMMIT, u32, kobj);
        0
    }
});
