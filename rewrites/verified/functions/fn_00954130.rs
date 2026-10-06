// original: 0x00954130 alloc_store_global (proposed)

/// Allocate a small object, construct it, publish it to a global slot.
///
/// Calls the allocator with `BLK` (0x1C) bytes. When it returns null,
/// writes 0 to `SLOT` and returns 0. Otherwise constructs the object
/// with the 3-argument thiscall constructor
/// (`CTOR_A0`, `CTOR_A1`, `CTOR_A2`; `CTOR_A1` is an image pointer and is
/// relocated), stores the constructed pointer in `SLOT` and returns it.
/// Original is cdecl/0, returns EAX.
lf_checker_rt::export!(cdecl, rw_00954130() -> u32 {
    const ALLOC: u32 = 1;
    const CTOR: u32 = 2;
    const SLOT: u32 = 0x011F6FEC;
    const BLK: u32 = 0x1C;
    const CTOR_A0: u32 = 0x5208;
    const CTOR_A1: u32 = 0xE8AB60;
    const CTOR_A2: u32 = 0x10;
    let blk = lf_checker_rt::callee_cdecl!(ALLOC, u32, BLK);
    if blk == 0 {
        unsafe { lf_checker_rt::global::<u32>(SLOT).write(0) };
        return 0;
    }
    let r = lf_checker_rt::callee_thiscall!(
        CTOR,
        u32,
        blk,
        CTOR_A0,
        lf_checker_rt::relocated(CTOR_A1),
        CTOR_A2
    );
    unsafe { lf_checker_rt::global::<u32>(SLOT).write(r) };
    r
});
