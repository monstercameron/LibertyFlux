// original: 0x00a95ca0 filemem_init_archive_manager

/// Initialise the archive manager: allocate its object and start it.
///
/// `a1` is stored to the manager slot at `0x01305344`; two state bytes at
/// `0x012fb3b1`/`0x012fb3b3` are cleared and the setup callee runs on the
/// static config at `0x0103e8d0`. The allocator callee reserves 0x3148
/// bytes; a null answer skips the build (and then faults on the null
/// object exactly as the original does). Otherwise the object takes the
/// manager vtable at `0x00ea3794` and eleven zero words, is published at
/// `0x01305348`, and is started through the vtable slot at `+0x4`. The
/// ready byte at `0x012fb3b2` is set, and unless the probe callee answers
/// zero (low byte), the warmup callee runs and control tail-jumps to the
/// enter callee with `a1` still on the stack.
///
/// Original: 0x00A95CA0 (cdecl, one stack word; four direct callees, one
/// indirect callee through the manager vtable, one tail jump).
lf_checker_rt::export!(cdecl, rw_00a95ca0(a1: u32) -> u32 {
    unsafe {
        /// Manager slot, state bytes, static config (file VAs).
        const MANAGER: u32 = 0x01305344;
        const STATE_A: u32 = 0x012fb3b1;
        const STATE_B: u32 = 0x012fb3b3;
        const CONFIG: u32 = 0x0103e8d0;
        /// Allocation size and manager vtable (file VA).
        const OBJ_SIZE: u32 = 0x3148;
        const VTABLE: u32 = 0x00ea3794;
        /// Zero-word offsets in the fresh object.
        const ZERO_OFFS: [u32; 11] = [
            0x30, 0x650, 0xc70, 0x1290, 0x18b0, 0x1ed0, 0x24f0, 0x2b10,
            0x313c, 0x3138, 0x3134,
        ];
        /// Published object slot, start slot, ready byte.
        const PUBLISH: u32 = 0x01305348;
        const VT_START: u32 = 0x4;
        const READY: u32 = 0x012fb3b2;
        const SETUP: u32 = 1;
        const ALLOC: u32 = 2;
        const PROBE: u32 = 4;
        const WARMUP: u32 = 5;
        const ENTER: u32 = 6;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        wr32(lf_checker_rt::relocated(MANAGER), a1);
        ((lf_checker_rt::relocated(STATE_A)) as *mut u8).write(0);
        ((lf_checker_rt::relocated(STATE_B)) as *mut u8).write(0);
        let _: u32 = lf_checker_rt::callee_thiscall!(SETUP, u32, lf_checker_rt::relocated(CONFIG));
        let obj: u32 = lf_checker_rt::callee_cdecl!(ALLOC, u32, OBJ_SIZE);
        if obj != 0 {
            wr32(obj, lf_checker_rt::relocated(VTABLE));
            for off in ZERO_OFFS {
                wr32(obj.wrapping_add(off), 0);
            }
        }
        wr32(lf_checker_rt::relocated(PUBLISH), obj);
        // Start through the manager vtable, exactly like the original
        // (faults on the null object on the null path, on both sides).
        let vt = rd32(obj);
        let sslot = rd32(vt.wrapping_add(VT_START));
        let start: extern "thiscall" fn(u32) -> u32 =
            core::mem::transmute(sslot as usize);
        let _: u32 = start(obj);
        ((lf_checker_rt::relocated(READY)) as *mut u8).write(1);
        let probe: u32 = lf_checker_rt::callee_cdecl!(PROBE, u32,);
        if (probe as u8) == 0 {
            return 0;
        }
        let _: u32 = lf_checker_rt::callee_cdecl!(WARMUP, u32,);
        // Tail call: the original jumps with `a1` as the pending argument.
        let _: u32 = lf_checker_rt::callee_cdecl!(ENTER, u32, a1);
        0
    }
});
