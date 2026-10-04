// original: 0x00be88d0 object_setup_with_transform (proposed)

/// Set up a sub-object: create it, initialise it, configure it with fixed
/// parameters, attach a 4x4 transform built from the vector at `arg1`, notify
/// through its virtual slot, and bind `arg0`.
///
/// Calls, in order: create (id 1, no arguments); if that returns null, return
/// null without further calls. Otherwise initialise (id 2, the object plus
/// four constant words the original pushes alongside it); configure (id 3,
/// constants `0`, `20.0`, `0`, `0xfa0`); build a 16-word row-major matrix on
/// the frame and hand it to id 4; call virtual slot `+8` on the object with
/// the global dword at `0x11735a4`; bind `arg0` through id 6. Returns the
/// created object.
///
/// The matrix is an identity rotation with the `arg1` vector (four words) as
/// its last row; words 3, 7 and 11 are copies the original takes from a frame
/// slot it never wrote, so under the checker's defined stack fill they read
/// as that fill (the contract sets it to zero).
///
/// Original: 0x00be88d0 (cdecl, two stack words; returns the object in `eax`).
lf_checker_rt::export!(cdecl, rw_00be88d0(arg0: u32, arg1: u32) -> u32 {
    unsafe {
        const GLOBAL_WORD: u32 = 0x11735a4;
        const ONE: u32 = 0x3f80_0000;
        const TWENTY: u32 = 0x41a0_0000;
        const COUNT: u32 = 0xfa0;
        const FILL: u32 = 0; // defined stack fill: the original's unread slot

        let edi: u32 = lf_checker_rt::callee_cdecl!(1, u32,);
        if edi == 0 {
            return 0;
        }
        let esi: u32 = lf_checker_rt::callee_stdcall!(2, u32, edi);
        lf_checker_rt::callee_thiscall!(3, u32, esi, 0u32, TWENTY, 0u32, COUNT);
        let mut m = [0u32; 16];
        m[0] = ONE;
        m[3] = FILL;
        m[5] = ONE;
        m[7] = FILL;
        m[10] = ONE;
        m[11] = FILL;
        m[12] = (arg1 as *const u32).read_unaligned();
        m[13] = ((arg1 + 4) as *const u32).read_unaligned();
        m[14] = ((arg1 + 8) as *const u32).read_unaligned();
        m[15] = ((arg1 + 12) as *const u32).read_unaligned();
        lf_checker_rt::callee_thiscall!(4, u32, esi, m.as_ptr() as u32);
        let vtable = (edi as *const u32).read_unaligned();
        let slot: u32 = ((vtable + 8) as *const u32).read_unaligned();
        let notify: extern "thiscall" fn(u32, u32) -> u32 =
            core::mem::transmute(slot as usize);
        let word = lf_checker_rt::global::<u32>(GLOBAL_WORD).read_unaligned();
        notify(edi, word);
        lf_checker_rt::callee_thiscall!(6, u32, esi, arg0);
        edi
    }
});
