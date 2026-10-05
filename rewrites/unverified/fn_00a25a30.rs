// original: 0x00a25a30 ped_task_state_init (proposed)

/// Initialise a ped task-state object to its default (unbound) state.
///
/// `this` points to the object (about 0x2e0 bytes). The routine zeroes most
/// fields, plants a few default constants (1.0 floats, two 0x42340000 words,
/// 0x3fc90fdb, 0x3e99999a, the word 0x0609), copies four configuration bytes
/// and two configuration floats from globals, stores the address of the
/// task-runner helper as a callback, and issues five intercepted calls:
/// an initializer on the object itself, a one-argument query on the linked
/// object at `+0x12c`, a getter whose `+0x24` float seeds two fields, and two
/// calls whose argument words overlap on the stack (the second callee pops
/// the first callee's leftover words together with its own).
///
/// Two fields depend on the mode global (whether it equals 2): the kind byte
/// at `+0x210` (0x0c or 0) and the low bit of the flag byte at `+0x200`
/// (xored with a config byte, else cleared). Three flag bytes keep their
/// incoming bits under a mask (`+0x216` or-ed with 3, `+0x1a8` anded twice,
/// `+0x200` anded then re-set); everything else is fully overwritten.
///
/// Returns 1 in the low byte; the upper three bytes are whatever the last
/// callee returned (the original only writes `al`).
///
/// Original: 0x00a25a30 (thiscall, no stack arguments).
lf_checker_rt::export!(thiscall, rw_00a25a30(this: u32) -> u32 {
    unsafe {
        const LINK: u32 = 0x12c;
        const KIND: u32 = 0x210;
        const KIND_WORD: u32 = 0x211;
        const CFG_A: u32 = 0x213;
        const CFG_B: u32 = 0x214;
        const CFG_C: u32 = 0x215;
        const FLAGS216: u32 = 0x216;
        const FLAG200: u32 = 0x200;
        const FLAG1A8: u32 = 0x1a8;
        const SEED_SRC: u32 = 0x204;
        const SEED_DST0: u32 = 0x208;
        const SEED_DST1: u32 = 0x20c;
        const CB_PTR: u32 = 0x25c;
        const FLOAT_A: u32 = 0x244;
        const FLOAT_B: u32 = 0x248;
        const KIND_ACTIVE: u8 = 0x0c;
        const MODE_ACTIVE: u32 = 2;
        const G_MODE: u32 = 0x011d6fd4;
        const G_BYTE_A: u32 = 0x012dd5ec;
        const G_BYTE_B: u32 = 0x0103c0d9;
        const G_BYTE_C: u32 = 0x0103c0da;
        const G_BYTE_D: u32 = 0x0103c111;
        const G_FLOAT_A: u32 = 0x0128e3d8;
        const G_FLOAT_B: u32 = 0x0128e3dc;
        const ONE: u32 = 0x3f800000;
        const BIG: u32 = 0x42340000;
        const RATE: u32 = 0x3fc90fdb;
        const SMALL: u32 = 0x3e99999a;
        const KIND_WORD_VAL: u16 = 0x0609;
        const CALLBACK: u32 = 0x00a25e60;

        #[inline(always)]
        unsafe fn wr32(base: u32, off: u32, v: u32) {
            unsafe { ((base + off) as *mut u32).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr16(base: u32, off: u32, v: u16) {
            unsafe { ((base + off) as *mut u16).write_unaligned(v) }
        }
        #[inline(always)]
        unsafe fn wr8(base: u32, off: u32, v: u8) {
            unsafe { ((base + off) as *mut u8).write(v) }
        }
        #[inline(always)]
        unsafe fn rd32(base: u32, off: u32) -> u32 {
            unsafe { ((base + off) as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn rd8(base: u32, off: u32) -> u8 {
            unsafe { ((base + off) as *const u8).read() }
        }
        #[inline(always)]
        unsafe fn g32(va: u32) -> u32 {
            unsafe { lf_checker_rt::global::<u32>(va).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn g8(va: u32) -> u8 {
            unsafe { lf_checker_rt::global::<u8>(va).read() }
        }

        // Head block: defaults and zeroing.
        wr32(this, 0x2d4, 0);
        wr32(this, 0x2d0, 0);
        wr8(this, 0x2d8, 0);
        wr32(this, 0x180, BIG);
        wr32(this, 0x2cc, 0);
        wr32(this, 0x2c8, 0);
        wr16(this, 0x2c4, 0);
        wr32(this, 0x2b8, 0);
        wr32(this, 0x2b4, 0);
        wr32(this, 0x2b0, 0);
        wr32(this, 0x2c0, ONE);
        wr32(this, 0x2a0, ONE);
        wr32(this, 0x298, 0);
        wr32(this, 0x294, 0);
        wr32(this, 0x290, 0);
        wr32(this, 0x288, 0);
        wr32(this, 0x284, 0);
        wr32(this, 0x280, 0);
        wr8(this, FLAGS216, rd8(this, FLAGS216) | 3);
        wr32(this, 0x274, 0);
        wr32(this, 0x270, 0);
        wr32(this, 0x26c, ONE);
        wr32(this, 0x268, ONE);
        // Kind byte follows the mode global.
        let mode = g32(G_MODE);
        wr8(this, KIND, if mode == MODE_ACTIVE { KIND_ACTIVE } else { 0 });
        wr16(this, KIND_WORD, KIND_WORD_VAL);
        wr8(this, CFG_A, g8(G_BYTE_A));
        wr8(this, CFG_B, g8(G_BYTE_B));
        wr8(this, CFG_C, g8(G_BYTE_C));
        // Middle block.
        wr32(this, 0x1f4, 0);
        wr32(this, 0x168, 0);
        wr32(this, 0x164, 0);
        wr32(this, 0x160, 0);
        wr32(this, SEED_SRC, 0);
        wr32(this, 0x148, 0);
        wr32(this, 0x144, 0);
        wr32(this, 0x140, 0);
        wr32(this, 0x158, 0);
        wr32(this, 0x154, 0);
        wr32(this, 0x150, 0);
        wr32(this, 0x218, 0);
        wr32(this, 0x21c, 0);
        wr32(this, 0x224, 0);
        wr32(this, 0x220, 0);
        wr32(this, 0x228, ONE);
        wr8(this, 0x23c, 0);
        wr32(this, 0x238, 0);
        wr32(this, 0x240, 0);
        wr32(this, FLOAT_A, g32(G_FLOAT_A));
        wr32(this, FLOAT_B, g32(G_FLOAT_B));
        // Calls: init, linked query, seed getter, then the overlapping pair.
        lf_checker_rt::callee_thiscall!(1, u32, this);
        let kind_arg = rd8(this, KIND) as i8 as i32 as u32;
        lf_checker_rt::callee_thiscall!(2, u32, rd32(this, LINK), kind_arg);
        wr32(this, 0x258, 0);
        wr32(this, CB_PTR, lf_checker_rt::relocated(CALLBACK));
        wr32(this, 0x178, 0);
        wr32(this, 0x174, 0);
        wr32(this, 0x170, 0);
        let seed_obj = lf_checker_rt::callee_thiscall!(3, u32, this);
        let seed = rd32(seed_obj, 0x24);
        wr32(this, SEED_DST0, seed);
        wr32(this, SEED_DST1, seed);
        // Seven words pushed; the callee pops none and the caller drops two.
        let r4 = lf_checker_rt::callee_cdecl!(4, u32, rd32(this, SEED_SRC), 0, 1, 1, 0, 0, 0);
        // The second callee pops all seven of its words: the two fresh ones
        // plus the five left over above.
        let r5 = lf_checker_rt::callee_thiscall!(5, u32, this, 0, r4, 1, 1, 0, 0, 0);
        // Tail block.
        wr8(this, FLAG1A8, rd8(this, FLAG1A8) & 0xfc);
        wr32(this, 0x188, 0);
        wr32(this, 0x184, RATE);
        wr32(this, 0x18c, 0);
        wr32(this, 0x190, 0);
        wr32(this, 0x194, 0);
        wr32(this, 0x198, 0);
        wr32(this, 0x19c, ONE);
        wr32(this, 0x1a0, 0);
        wr32(this, 0x1b8, 0);
        wr32(this, 0x1b4, 0);
        wr32(this, 0x1b0, 0);
        wr32(this, 0x1c8, 0);
        wr32(this, 0x1c4, 0);
        wr32(this, 0x1c0, 0);
        wr32(this, 0x1d8, 0);
        wr32(this, 0x1d4, 0);
        wr32(this, 0x1d0, 0);
        wr8(this, FLAG200, rd8(this, FLAG200) & 0xfd);
        let flag = rd8(this, FLAG200);
        wr8(this, FLAG1A8, rd8(this, FLAG1A8) & 0xf3);
        wr32(this, 0x1e0, ONE);
        wr32(this, 0x1e4, ONE);
        wr32(this, 0x1a4, 0);
        wr32(this, 0x1e8, 0);
        wr32(this, 0x1ec, 0);
        wr32(this, 0x1f0, SMALL);
        if mode == MODE_ACTIVE {
            wr8(this, FLAG200, ((flag ^ g8(G_BYTE_D)) & 1) ^ flag);
        } else {
            wr8(this, FLAG200, flag & 0xfe);
        }
        wr32(this, 0x1f8, ONE);
        wr8(this, 0x234, 0);
        wr32(this, 0x22c, 0);
        wr32(this, 0x230, 0);
        wr32(this, 0x1fc, BIG);
        wr32(this, 0x24c, 0);
        wr32(this, 0x250, 0);
        wr32(this, 0x254, 0);
        wr32(this, 0x260, 0);
        wr32(this, 0x264, 0);
        wr8(this, 0x2d9, 0);
        (r5 & 0xffff_ff00) | 1
    }
});
