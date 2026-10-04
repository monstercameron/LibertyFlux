// original: 0x009510f0 subsystem_init_sequence
/// Run the start-up initialisation sequence for a group of subsystems.
///
/// Takes no arguments. Issues nineteen scripted helper calls in a fixed
/// order with fixed arguments: a four-word probe, float-scoped probes
/// that each take a pointer to a zeroed three-word scratch buffer plus
/// the constant 1000000.0 (one also takes three trailing words), context
/// calls carrying small integer handles, and parameter-free barriers.
/// The two pushes of the incoming ECX value only reserve stack slots
/// that are overwritten with the float constant before any call reads
/// them, so they contribute no behaviour. Returns the last helper's
/// answer.
export!(cdecl, rw_009510f0() -> u32 {
    unsafe {
        const RATE_BITS: u32 = 0x49742400; // 1000000.0f
        // The three context immediates carry reloc entries (verified
        // empirically: the worker maps them rebased), so derive them.
        let ctx_a = relocated(0x12E2420);
        let ctx_b = relocated(0x1668DB0);
        let ctx_c = relocated(0x1173750);
        const FLAG_NEG: u32 = 0xFFFFFFFF;
        const PARAM_CE: u32 = 0xCE;
        // Scratch buffer matching the original's zeroed frame window:
        // every pointer argument it passes points at three zero words.
        let buf = [0u32; 3];
        let scratch = buf.as_ptr() as u32;
        let _: u32 = callee_cdecl!(0, u32, 1, 0, 0, 0);
        let _: u32 = callee_cdecl!(1, u32, scratch, RATE_BITS);
        let _: u32 = callee_cdecl!(2, u32,);
        let _: u32 = callee_cdecl!(3, u32, scratch, RATE_BITS);
        let _: u32 = callee_cdecl!(4, u32, scratch, RATE_BITS, 0, 0, 1);
        let _: u32 = callee_thiscall!(5, u32, ctx_a, scratch, RATE_BITS);
        let _: u32 = callee_thiscall!(6, u32, ctx_b);
        let _: u32 = callee_cdecl!(7, u32,);
        let _: u32 = callee_cdecl!(8, u32,);
        let _: u32 = callee_cdecl!(9, u32, scratch, RATE_BITS);
        let _: u32 = callee_cdecl!(10, u32, FLAG_NEG, scratch, RATE_BITS);
        let _: u32 = callee_cdecl!(11, u32,);
        let _: u32 = callee_cdecl!(12, u32, scratch, RATE_BITS);
        let _: u32 = callee_cdecl!(13, u32,);
        let _: u32 = callee_cdecl!(14, u32,);
        let _: u32 = callee_cdecl!(15, u32,);
        let _: u32 = callee_cdecl!(16, u32, PARAM_CE);
        let _: u32 = callee_thiscall!(17, u32, ctx_c);
        callee_cdecl!(18, u32, 0)
    }
});
