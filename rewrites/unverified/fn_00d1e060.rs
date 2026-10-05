// original: 0x00d1e060 CTaskComplexSeekCover::vf1
/// Select a cover-seek sub-task by the low 3 bits of the flag word, run it,
/// and copy flag bit 3 from this task into the selected sub-task.
///
/// `this` is the task object. The flag word at `+0x90` routes: low bits 0, 1
/// or 2 fetch a worker object from the pool behind the `POOL` global through
/// callee 1 (thiscall, no stack args) and then invoke one of callees 2..4
/// (thiscall on that worker, 7/7/6 stack args) with fields of this task:
/// arg0 the word at `+0x30`, arg1 a pointer to `+0x20`, arg2 the byte at
/// `+0x5c`, arg3 bit 0 of the byte at `+0x94`, arg4 the word at `+0x74`
/// (route 2), a pointer to `+0x80` (route 1) or the float at `+0x98`
/// (route 0), then the float at `+0x98` and the sign-extended bits 5..3 of
/// the flag word (routes 2 and 1). Any other low-bits value, or a null
/// worker, skips the call and leaves the result null.
///
/// The tail copies bit 3 (`FLAG_SYNC`): it xors the byte at `+0x94` of this
/// task with that of the result, masks to bit 3 and xors it back into the
/// result, so the result's bit 3 ends up equal to this task's. With a null
/// result this faults on the read, exactly like the original.
///
/// The float argument is pushed by overwriting a scratch stack slot; only the
/// float value is behaviour. Returns the selected sub-task (or null).
/// Original: 0x00d1e060 (thiscall, no stack args).
lf_checker_rt::export!(thiscall, rw_00d1e060(this: u32) -> u32 {
    unsafe {
        const FLAG_WORD: u32 = 0x90;
        const FLAG_BYTE: u32 = 0x94;
        const SPEED: u32 = 0x98;
        const ARG0_OFF: u32 = 0x30;
        const OBJ_ARG: u32 = 0x20;
        const MODE_BYTE: u32 = 0x5c;
        const EXTRA_WORD: u32 = 0x74;
        const ALT_OBJ: u32 = 0x80;
        const FLAG_SYNC: u8 = 8;
        const POOL: u32 = 0x0167e2a0;
        const FETCH_WORKER: u32 = 1;
        const RUN_ROUTE2: u32 = 2;
        const RUN_ROUTE1: u32 = 3;
        const RUN_ROUTE0: u32 = 4;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }

        let route = rd32(this + FLAG_WORD) & 7;
        let picked = if route <= 2 {
            let pool = (lf_checker_rt::global::<u32>(POOL) as *const u32).read();
            let worker: u32 = lf_checker_rt::callee_thiscall!(FETCH_WORKER, u32, pool);
            if worker == 0 {
                0
            } else {
                let flags = rd32(this + FLAG_WORD);
                let lane = (((flags << 26) as i32) >> 29) as u32;
                let speed = rdf(this + SPEED).to_bits();
                let a0 = rd32(this + ARG0_OFF);
                let a1 = this + OBJ_ARG;
                let a2 = rd8(this + MODE_BYTE) as u32;
                let a3 = (rd8(this + FLAG_BYTE) & 1) as u32;
                if route == 2 {
                    let a4 = rd32(this + EXTRA_WORD);
                    lf_checker_rt::callee_thiscall!(RUN_ROUTE2, u32, worker, a0, a1, a2, a3, a4, speed, lane)
                } else if route == 1 {
                    let a4 = this + ALT_OBJ;
                    lf_checker_rt::callee_thiscall!(RUN_ROUTE1, u32, worker, a0, a1, a2, a3, a4, speed, lane)
                } else {
                    lf_checker_rt::callee_thiscall!(RUN_ROUTE0, u32, worker, a0, a1, a2, a3, speed, lane)
                }
            }
        } else {
            0
        };
        let mine = rd8(this + FLAG_BYTE);
        let theirs = ((picked + FLAG_BYTE) as *const u8).read();
        let flip = (mine ^ theirs) & FLAG_SYNC;
        ((picked + FLAG_BYTE) as *mut u8).write(theirs ^ flip);
        picked
    }
});
