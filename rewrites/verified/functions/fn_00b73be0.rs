// original: 0x00b73be0 task_ctor_state_dispatch (proposed)

/// Build a follow-up task for a state-driven ped task.
///
/// `this` is the task object: the current state number is read at `+STATE`,
/// the owning ped-ish host object at `+HOST`, a subtype byte at `+SUB`,
/// a flag byte at `+FLAG`, an optional extra object at `+EXTRA`, and the
/// created follow-up task is stored at `+TASK`. `arg` is the host-side
/// record the task is built against (callback object at `+CALLBACK`,
/// level word at `+LEVEL`, scale gate byte at `+NO_SCALE`).
///
/// Behaviour: the state (5..=8) selects a request code through a jump
/// table (5 gives `0x13F`, 7 gives `0x140`, each plus 4 when the flag byte
/// is non-zero; 6 gives `0x141`, 8 gives `0x142`; anything else gives -1).
/// The code is passed by reference to callee 0 (which answers in `ebx`
/// and overwrites the slot), together with the kind word read from the
/// host's table entry (`TABLE[idx]` at `+KIND`, where `idx` is the signed
/// word at `+MODEL`). Callee 1 then creates the follow-up task from
/// (`ebx`, overwritten slot, scale, -1), where scale is `K_LO` or `K_HI`
/// depending on the host's mode word at `+MODE`; callee 2 attaches it.
/// A sentinel word is stored at `+RESULT` when `ebx` answers `MAGIC_OUT`,
/// or when the host is present with its mode word set and no extra object
/// is attached.
/// Finally, unless the scale gate byte is set, a duration derived from
/// the level word (`K_BASE - level*K_A*K_B`, in that operand order) is
/// stored at `+DURATION`.
///
/// Returns the follow-up task pointer, except on the gated no-store path
/// without a tail store, where it returns the host pointer.
///
/// Original: 0x00b73be0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00b73be0(this: u32, arg: u32) -> u32 {
    unsafe {
        const STATE: u32 = 0x24;
        const HOST: u32 = 0x20;
        const TASK: u32 = 0x18;
        const EXTRA: u32 = 0x28;
        const SUB: u32 = 0x2c;
        const FLAG: u32 = 0x2d;
        const MODEL: u32 = 0x2e;
        const MODE: u32 = 0x1300;
        const KIND: u32 = 0xc4;
        const TABLE: u32 = 0x01295cd8;
        const CALLBACK: u32 = 0x78;
        const LEVEL: u32 = 0x2c;
        const NO_SCALE: u32 = 0x219;
        const RESULT: u32 = 0x3c;
        const DURATION: u32 = 0x54;
        const K_LO: u32 = 0x00fe8afc;
        const K_HI: u32 = 0x00fe8c58;
        const K_BASE: u32 = 0x00fe88e8;
        const K_A: u32 = 0x00fe8684;
        const K_B: u32 = 0x00fe879c;
        const VTBL: u32 = 0x00b6fe20;
        const MAGIC_OUT: u32 = 0x53;
        const SENTINEL: u32 = 0xc47a0000;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd16(a: u32) -> u32 {
            unsafe { (a as *const u16).read_unaligned() as u32 }
        }
        #[inline(always)]
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn rdf(a: u32) -> f32 {
            unsafe { f32::from_bits(rd32(a)) }
        }
        #[inline(always)]
        unsafe fn wrf(a: u32, v: f32) {
            unsafe { wr32(a, v.to_bits()) }
        }
        #[inline(always)]
        fn mul(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) * core::hint::black_box(b)
        }
        #[inline(always)]
        fn sub(a: f32, b: f32) -> f32 {
            core::hint::black_box(a) - core::hint::black_box(b)
        }

        let flagged = rd8(this + FLAG) != 0;
        let code: u32 = match rd32(this + STATE) {
            5 => 0x13f + 4 * (flagged as u32),
            6 => 0x141,
            7 => 0x140 + 4 * (flagged as u32),
            8 => 0x142,
            _ => 0xffff_ffff,
        };
        let host = rd32(this + HOST);
        let idx = rd16(host + MODEL) as u16 as i16 as i32;
        let entry = rd32(
            lf_checker_rt::relocated(TABLE).wrapping_add((idx as u32).wrapping_mul(4)),
        );
        let kind = rd32(entry + KIND);
        // The original passes a pointer to its code slot: the slot carries
        // the code into the call and the callee's answer out of it.
        let mut slot = code;
        let ebx: u32 = lf_checker_rt::callee_cdecl!(
            0,
            u32,
            kind,
            &mut slot as *mut u32 as u32,
            arg,
            host,
            rd8(this + SUB) as u32,
            0,
            0,
            1
        );
        let host = rd32(this + HOST);
        let mut scale = rdf(lf_checker_rt::relocated(K_LO));
        if host != 0 && rd32(host + MODE) == 1 {
            scale = rdf(lf_checker_rt::relocated(K_HI));
        }
        let task: u32 = lf_checker_rt::callee_thiscall!(
            1,
            u32,
            rd32(arg + CALLBACK),
            ebx,
            slot,
            scale.to_bits(),
            0xffff_ffff
        );
        wr32(this + TASK, task);
        lf_checker_rt::callee_thiscall!(
            2,
            u32,
            task,
            1,
            lf_checker_rt::relocated(VTBL),
            this
        );
        let host = rd32(this + HOST);
        let mut eax = host;
        if ebx == MAGIC_OUT || (host != 0 && rd32(host + MODE) == 1 && rd32(this + EXTRA) == 0)
        {
            let task = rd32(this + TASK);
            wr32(task + RESULT, SENTINEL);
            eax = task;
        }
        if rd8(arg + NO_SCALE) == 0 {
            let level = rd16(arg + LEVEL) as i32 as f32;
            let t = mul(mul(level, rdf(lf_checker_rt::relocated(K_A))), rdf(lf_checker_rt::relocated(K_B)));
            let f = sub(rdf(lf_checker_rt::relocated(K_BASE)), t);
            let task = rd32(this + TASK);
            wrf(task + DURATION, f);
            eax = task;
        }
        eax
    }
});
