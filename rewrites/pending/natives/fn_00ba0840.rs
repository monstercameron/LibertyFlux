// original: 0x00BA0840 LOCATE_CHAR_IN_CAR_OBJECT_3D
// Rewrite of native handler LOCATE_CHAR_IN_CAR_OBJECT_3D.
//
// Passes character and object handles, a radius plus a flag; stores the low byte.
// The call context holds the return-slot pointer at +0 and the
// argument-array pointer at +8.
// The flag argument reproduces the original's codegen quirk: the 0/1 byte
// is written over the low byte of the incoming context-pointer slot, so the
// engine observes `(ctx & !0xFF) | flag` and reads only the low byte.
// Returns the return-slot pointer, matching the value the original leaves in EAX.
export!(cdecl, rw_00ba0840(ctx: u32) -> u32 {
    unsafe {
        let argv = *((ctx + 8) as *const u32) as *const u32;
        let a0 = *argv.add(0);
        let a1 = *argv.add(1);
        let a2 = *argv.add(2);
        let a3 = *argv.add(3);
        let a4 = *argv.add(4);
        let a5 = *argv.add(5);
        let engine: extern "cdecl" fn(u32, u32, u32, u32, u32, u32) -> u32 =
            core::mem::transmute(callee_addr(1) as usize);
        let ret = *(ctx as *const u32) as *mut u32;
        let ans = engine(a0, a1, a2, a3, a4, (ctx & 0xFFFFFF00) | ((a5 != 0) as u32));
        *ret = ans & 0xFF;
        ret as u32
    }
});
