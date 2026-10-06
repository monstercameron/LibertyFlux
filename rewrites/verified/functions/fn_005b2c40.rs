// original: 0x005B2C40 apply_config_vectors

/// Fetch seven two-word config values by id, combine them with a combine
/// call each, then submit the words as float vectors to the cached sink
/// object through two wide setters and a finalizer.
///
/// Calling convention: cdecl, no stack arguments, no observed return value
/// (the single caller ignores `eax`, so the contract compares no return
/// channel).
///
/// Behaviour in order:
/// 1. If the `SINK` global is null, build the sink: call the TLS factory's
///    virtual slot 2 (a thiscall taking 0x520, 0x10 and 0) and, unless it
///    returns null, wrap the result through callee 2 and cache it in `SINK`
///    (a null factory result caches a null). A non-null `SINK` skips this.
/// 2. Fetch seven config pairs through callee 3 with ids 0x4e, 0x4f, 0x50,
///    0x51, 0x5d, 0x52 and 0x53 into seven stack slots, then run callee 4
///    as (2, 0, slot, 0) over six of the slots.
/// 3. Pick the submit path: the alternate path runs when `MODE_BYTE` is
///    0x6a or `ALT_FLAG` is non-zero, otherwise the main path. Both submit
///    the same slot words; the paths differ only in one constant argument
///    (2 on the alternate path, 0 on the main path) to each wide setter.
/// 4. Submit through callee 5 (thiscall, seven stack words: the flag, four
///    words from the first two slots, a constant 1, one word from the fifth
///    slot), callee 6 (thiscall, ten stack words: the flag, eight words
///    from the remaining four slots, a constant 1 in the middle) and callee
///    7 (thiscall, no stack arguments), all against the cached sink.
///
/// Every slot word is uninitialised stack (the scripted callees write
/// nothing), modelled here as zero words under the contract's `stack_fill`.
/// No floating-point arithmetic happens; the `movss` traffic is plain word
/// copies, passed on as `u32` words.
use lf_checker_rt::{callee_cdecl, callee_thiscall, relocated, tls_slot};

pub const SINK: u32 = 0x018B6E84;
pub const MODE_BYTE: u32 = 0x0116C250;
pub const ALT_FLAG: u32 = 0x0116C253;
pub const ALT_MODE: u8 = 0x6A;

#[inline(always)]
unsafe fn rd32(a: u32) -> u32 {
    unsafe { (a as *const u32).read_unaligned() }
}
#[inline(always)]
unsafe fn rd8(a: u32) -> u8 {
    unsafe { (a as *const u8).read() }
}

type FactoryFn = extern "thiscall" fn(u32, u32, u32, u32) -> u32;

unsafe fn factory(obj: u32) -> u32 {
    unsafe {
        let holder = ((obj + 8) as *const u32).read_unaligned();
        let vtable = (holder as *const u32).read_unaligned();
        let slot = ((vtable + 8) as *const u32).read_unaligned();
        let f: FactoryFn = core::mem::transmute(slot as usize);
        f(holder, 0x520, 0x10, 0)
    }
}

unsafe fn fetch(slot: &mut [u32; 2], id: u32) {
    callee_cdecl!(3, u32, slot.as_mut_ptr() as u32, id);
}

unsafe fn combine(slot: &mut [u32; 2]) {
    callee_cdecl!(4, u32, 2, 0, slot.as_mut_ptr() as u32, 0);
}

unsafe fn body() {
    unsafe {
        if rd32(relocated(SINK)) == 0 {
            let obj = tls_slot(0);
            let made = factory(obj);
            let cached = if made == 0 {
                0
            } else {
                callee_thiscall!(2, u32, made)
            };
            (relocated(SINK) as *mut u32).write_unaligned(cached);
        }
        let mut s4e = [0u32; 2];
        let mut s4f = [0u32; 2];
        let mut s50 = [0u32; 2];
        let mut s51 = [0u32; 2];
        let mut s5d = [0u32; 2];
        let mut s52 = [0u32; 2];
        let mut s53 = [0u32; 2];
        fetch(&mut s4e, 0x4E);
        fetch(&mut s4f, 0x4F);
        fetch(&mut s50, 0x50);
        fetch(&mut s51, 0x51);
        fetch(&mut s5d, 0x5D);
        fetch(&mut s52, 0x52);
        fetch(&mut s53, 0x53);
        combine(&mut s4f);
        combine(&mut s52);
        combine(&mut s53);
        combine(&mut s4e);
        combine(&mut s50);
        combine(&mut s51);
        let alt = rd8(relocated(MODE_BYTE)) == ALT_MODE
            || rd8(relocated(ALT_FLAG)) != 0;
        let flag: u32 = if alt { 2 } else { 0 };
        let sink = rd32(relocated(SINK));
        callee_thiscall!(5, u32, sink, flag, s4e[0], s4e[1], s4f[0], s4f[1], 1, s5d[0]);
        callee_thiscall!(
            6, u32, sink, flag, s50[0], s50[1], s51[0], s51[1], 1, s52[0], s52[1],
            s53[0], s53[1]
        );
        callee_thiscall!(7, u32, sink);
    }
}

lf_checker_rt::export!(cdecl, rw_005B2C40() -> u32 {
    unsafe { body() };
    0
});
