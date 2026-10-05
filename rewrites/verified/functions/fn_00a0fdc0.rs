// original: 0x00a0fdc0 quad_slot_verify (proposed)
/// Verify four slots through staged checks, reporting full agreement.
///
/// Returns 0 (low byte) when the flag at `obj + 0x76` is clear, when the
/// first gate answers non-zero, or when the second gate answers zero. Then,
/// for `i` in 0..4, fetches slot `i` for (`p`, `q`): a null slot, a type
/// other than 0x100 (bits 6-9 of the word at `+0x28`), or a clear byte at
/// `+0x22b` skips it, otherwise the slot counts and the confirmer (with two
/// fixed constants) decides whether it also agrees. Returns 1
/// when at least one slot counted and every counted slot agreed. Cdecl.
export!(cdecl, rw_00a0fdc0(obj: u32, p: u32, q: u32) -> u32 {
    unsafe {
        const GATE_A: u32 = 1;
        const GATE_B: u32 = 2;
        const FETCH: u32 = 3;
        const CONFIRM: u32 = 4;
        const FLAG_OFF: u32 = 0x76;
        const TYPE_OFF: u32 = 0x28;
        const TYPE_MASK: u32 = 0x3c0;
        const WANT_TYPE: u32 = 0x100;
        const READY_OFF: u32 = 0x22b;
        const K0: u32 = 0x3c23d70a;
        const K1: u32 = 0x3cf5c28f;
        if ((obj + FLAG_OFF) as *const u8).read() == 0 {
            return 0;
        }
        if (callee_thiscall!(GATE_A, u32, obj, p, q) & 0xff) != 0 {
            return 0;
        }
        if (callee_thiscall!(GATE_B, u32, obj, p, q) & 0xff) == 0 {
            return 0;
        }
        let mut counted: u32 = 0;
        let mut agreed: u32 = 0;
        for i in 0..4u32 {
            let slot = callee_thiscall!(FETCH, u32, obj, p, q, i);
            if slot == 0 {
                continue;
            }
            if ((slot + TYPE_OFF) as *const u32).read_unaligned() & TYPE_MASK != WANT_TYPE
            {
                continue;
            }
            if ((slot + READY_OFF) as *const u8).read() == 0 {
                continue;
            }
            counted += 1;
            if (callee_thiscall!(CONFIRM, u32, slot, K0, K1) & 0xff) != 0 {
                agreed += 1;
            }
        }
        (counted > 0 && counted == agreed) as u32
    }
});
