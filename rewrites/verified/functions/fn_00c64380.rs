// original: 0x00c64380 CCutsceneObject::~CCutsceneObject

/// Destroy the cutscene object and hand off to the base destructor.
///
/// `this` is the cutscene object. The table pointer for the destruction
/// phase (`DTOR_VTABLE`) is stored first, always. When `MODE (+0x314)` is
/// zero the members are torn down: the handles at `+0x310` and `+0x290` get
/// their deleting destructor (slot 0, flag 1) when non-null, the blocks at
/// `+0x29c`, `+0x298` and `+0x294` get the teardown helper 2 plus the free
/// helper 3 when non-null, and every freed slot is zeroed. Helper 4 is then
/// asked whether the registry step runs: a signed word index from `+0x2e`
/// selects a registry entry from `REGISTRY_G`, whose signed word at `+0x58`
/// must not be -1 while `+0xd4` is nonzero and helper 5 agrees; then helper
/// 6 runs on the fixed context with a zero word, helper 7 runs on `+0xd0`
/// with word 3, and helper 8 is told the registry word. When the mode is 1
/// instead, bit 26 of `+0x24` is set. The byte at `+0x2ac` is cleared, and
/// the base destructor (tail (an instruction of the original)`this` in ECX) finishes; its answer is
/// the result.
///
/// All mode and flag tests are equality tests. The registry index and word
/// are read signed (`movsx`), matching the original.
///
/// Original: thiscall, no stack arguments, direct and member-table calls
/// plus a tail jump, word result in EAX.
lf_checker_rt::export!(thiscall, rw_00c64380(this: u32) -> u32 {
    const DTOR_VTABLE: u32 = 0x00ec_b99c;
    const MODE: u32 = 0x314;
    const HANDLE_A: u32 = 0x310;
    const HANDLE_B: u32 = 0x290;
    const BLOCK_0: u32 = 0x29c;
    const BLOCK_1: u32 = 0x298;
    const BLOCK_2: u32 = 0x294;
    const FLAGS_24: u32 = 0x24;
    const MODE1_BIT: u32 = 0x0400_0000;
    const DONE_2AC: u32 = 0x2ac;
    const REG_INDEX: u32 = 0x2e;
    const REG_ENTRY_58: u32 = 0x58;
    const REGISTRY_G: u32 = 0x0129_5cd8;
    const GATE_D4: u32 = 0xd4;
    const CTX_D0: u32 = 0xd0;
    const FIXED_CTX: u32 = 0x0117_37d0;
    const MEMBER_DTOR: u32 = 1;
    const TEARDOWN: u32 = 2;
    const FREE: u32 = 3;
    const ASK_REGISTRY: u32 = 4;
    const ASK_GATE: u32 = 5;
    const REG_RUN: u32 = 6;
    const CTX_RUN: u32 = 7;
    const REG_TELL: u32 = 8;
    const BASE_DTOR: u32 = 9;
    unsafe {
        #[inline(always)]
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        #[inline(always)]
        unsafe fn wr32(a: u32, v: u32) {
            unsafe { (a as *mut u32).write_unaligned(v) }
        }
        wr32(this, lf_checker_rt::relocated(DTOR_VTABLE));
        let mode = rd32(this + MODE);
        if mode == 0 {
            let ha = rd32(this + HANDLE_A);
            if ha != 0 {
                let slot = ((ha as *const u32).read_unaligned()
                    as *const u32)
                    .read_unaligned();
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(ha, 1);
                wr32(this + HANDLE_A, 0);
            }
            for off in [BLOCK_0, BLOCK_1, BLOCK_2] {
                let blk = rd32(this + off);
                if blk != 0 {
                    lf_checker_rt::callee_thiscall!(TEARDOWN, u32, blk);
                    lf_checker_rt::callee_cdecl!(FREE, u32, blk);
                    wr32(this + off, 0);
                }
            }
            let hb = rd32(this + HANDLE_B);
            if hb != 0 {
                let slot = ((hb as *const u32).read_unaligned()
                    as *const u32)
                    .read_unaligned();
                let f: extern "thiscall" fn(u32, u32) -> u32 =
                    core::mem::transmute(slot as usize);
                f(hb, 1);
                wr32(this + HANDLE_B, 0);
            }
            let ask = lf_checker_rt::callee_thiscall!(ASK_REGISTRY, u32, this);
            if ask & 0xff != 0 {
                let idx =
                    ((this + REG_INDEX) as *const i16).read_unaligned() as i32;
                let base = lf_checker_rt::relocated(REGISTRY_G);
                let entry = rd32(base.wrapping_add((idx * 4) as u32));
                let word =
                    ((entry + REG_ENTRY_58) as *const i16).read_unaligned()
                        as i32;
                if word != -1
                    && rd32(this + GATE_D4) != 0
                    && lf_checker_rt::callee_cdecl!(
                        ASK_GATE, u32, word as u32)
                        & 0xff
                        != 0
                {
                    lf_checker_rt::callee_thiscall!(
                        REG_RUN, u32, lf_checker_rt::relocated(FIXED_CTX), 0);
                    let ctx = rd32(this + CTX_D0);
                    lf_checker_rt::callee_thiscall!(CTX_RUN, u32, ctx, 3);
                    lf_checker_rt::callee_cdecl!(REG_TELL, u32, word as u32);
                }
            }
        }
        if mode == 1 {
            let f = rd32(this + FLAGS_24);
            wr32(this + FLAGS_24, f | MODE1_BIT);
        }
        ((this + DONE_2AC) as *mut u8).write_unaligned(0);
        lf_checker_rt::callee_thiscall!(BASE_DTOR, u32, this)
    }
});
