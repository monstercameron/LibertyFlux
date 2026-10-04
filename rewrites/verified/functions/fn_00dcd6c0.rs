// original: 0x00dcd6c0 task_init_full_state (proposed)

/// Initialise a task object with its full default state (constructor).
///
/// `this` is the task object; the six stack words are `owner`, `template`,
/// `flags` and three float parameters `f0`, `f1`, `f2`. A base constructor
/// runs first, then the vtable pointer is installed at `+0x0` and the
/// arguments are stored (`owner` at `+0x14`, `f0` at `+0x40`, `f1` at `+0x5c`,
/// `f2` at `+0x60`, `flags` at `+0x8c`). A non-null `owner` is registered
/// through a helper taking `(owner, this+0x14)`.
///
/// Words `+0x20..+0x28` are cleared and `+0x2c` is written from the original's
/// unwritten stack locals; the contract pins the stack fill to zero so this
/// value is deterministically 0. A non-null `template` then overwrites
/// `+0x20..+0x2c` with its four words, and the block is duplicated to
/// `+0x30..+0x3c`. Bit 0 of the flag word at `+0xa4` is cleared, `f1`/`f2`
/// are stored again, two slots get -1.0f, four get -1, most of the remaining
/// state is zeroed (bytes at `+0x7c` and `+0xa1` included), and three slots
/// get 1, 3 and 5. Returns `this`.
///
/// Original: 0x00dcd6c0 (thiscall, six stack words).
lf_checker_rt::export!(thiscall, rw_00dcd6c0(this: u32, owner: u32, template: u32, flags: u32, f0: u32, f1: u32, f2: u32) -> u32 {
    unsafe {
        const VTABLE: u32 = 0x00ef9974;
        const OWNER_OFF: u32 = 0x14;
        const COPY_OFF: u32 = 0x20;
        const DUP_OFF: u32 = 0x30;
        const F0_OFF: u32 = 0x40;
        const NEG_ONE_A: u32 = 0x44;
        const NEG_ONE_B: u32 = 0x48;
        const F1_OFF: u32 = 0x5c;
        const F2_OFF: u32 = 0x60;
        const FLAGS_OFF: u32 = 0x8c;
        const FLAG_WORD: u32 = 0xa4;
        const NEG_ONE_F: u32 = 0xbf800000; // -1.0f
        const C_BASE_CTOR: u32 = 1;
        const C_REGISTER: u32 = 2;

        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }

        lf_checker_rt::callee_thiscall!(C_BASE_CTOR, u32, this);
        wr32(this, lf_checker_rt::relocated(VTABLE));
        wr32(this.wrapping_add(OWNER_OFF), owner);
        wr32(this.wrapping_add(F0_OFF), f0);
        wr32(this.wrapping_add(F1_OFF), f1);
        wr32(this.wrapping_add(F2_OFF), f2);
        wr32(this.wrapping_add(FLAGS_OFF), flags);
        if owner != 0 {
            lf_checker_rt::callee_thiscall!(
                C_REGISTER,
                u32,
                owner,
                this.wrapping_add(OWNER_OFF)
            );
        }
        wr32(this.wrapping_add(COPY_OFF), 0);
        wr32(this.wrapping_add(COPY_OFF + 4), 0);
        wr32(this.wrapping_add(COPY_OFF + 8), 0);
        // The original copies one word from its own unwritten stack locals
        // here; the contract pins the stack fill to zero, so it is 0.
        wr32(this.wrapping_add(COPY_OFF + 12), 0);
        if template != 0 {
            wr32(this.wrapping_add(COPY_OFF), rd32(template));
            wr32(this.wrapping_add(COPY_OFF + 4), rd32(template.wrapping_add(4)));
            wr32(this.wrapping_add(COPY_OFF + 8), rd32(template.wrapping_add(8)));
            wr32(this.wrapping_add(COPY_OFF + 12), rd32(template.wrapping_add(12)));
        }
        wr32(
            this.wrapping_add(DUP_OFF),
            rd32(this.wrapping_add(COPY_OFF)),
        );
        wr32(
            this.wrapping_add(DUP_OFF + 4),
            rd32(this.wrapping_add(COPY_OFF + 4)),
        );
        wr32(
            this.wrapping_add(DUP_OFF + 8),
            rd32(this.wrapping_add(COPY_OFF + 8)),
        );
        wr32(
            this.wrapping_add(DUP_OFF + 12),
            rd32(this.wrapping_add(COPY_OFF + 12)),
        );
        wr32(
            this.wrapping_add(FLAG_WORD),
            rd32(this.wrapping_add(FLAG_WORD)) & !1,
        );
        wr32(this.wrapping_add(F1_OFF), f1);
        wr32(this.wrapping_add(F2_OFF), f2);
        wr32(this.wrapping_add(NEG_ONE_A), NEG_ONE_F);
        wr32(this.wrapping_add(NEG_ONE_B), NEG_ONE_F);
        wr32(this.wrapping_add(0x4c), 0xffff_ffff);
        wr32(this.wrapping_add(0x50), 0xffff_ffff);
        wr32(this.wrapping_add(0x54), 0xffff_ffff);
        wr32(this.wrapping_add(0x58), 0xffff_ffff);
        wr32(this.wrapping_add(0x64), 0);
        wr32(this.wrapping_add(0x68), 0);
        wr32(this.wrapping_add(0x6c), 0);
        wr32(this.wrapping_add(0x70), 0);
        wr32(this.wrapping_add(0x74), 0);
        wr32(this.wrapping_add(0x78), 0);
        (this.wrapping_add(0x7c) as *mut u8).write(0);
        wr32(this.wrapping_add(0x80), 0);
        wr32(this.wrapping_add(0x84), 0);
        wr32(this.wrapping_add(0x88), 0);
        wr32(this.wrapping_add(0x9c), 0);
        wr32(this.wrapping_add(0x90), 1);
        wr32(this.wrapping_add(0x94), 3);
        wr32(this.wrapping_add(0x98), 5);
        wr32(this.wrapping_add(0xa8), 0);
        wr32(this.wrapping_add(0xac), 0);
        (this.wrapping_add(0xa1) as *mut u8).write(0);
        this
    }
});
