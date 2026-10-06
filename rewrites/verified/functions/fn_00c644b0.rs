// original: 0x00c644b0 CCutsceneObject::~CCutsceneObject__deleting

/// Destroy the cutscene object, freeing it when the flag asks.
///
/// `this` is the cutscene object and `flag` is one stack word. The real
/// destructor (helper 1) is called with `this` in ECX; then, only when the
/// low bit of `flag` is set, the allocator word is loaded from the global
/// `ALLOC_G` and the freeing helper 2 is called with it in ECX and `this`
/// as its one stack word. The object pointer itself is returned in EAX
/// either way.
///
/// Original: thiscall, one stack word, callee pops it (the callee pops 4 bytes), address
/// result in EAX.
lf_checker_rt::export!(thiscall, rw_00c644b0(this: u32, flag: u32) -> u32 {
    const DTOR: u32 = 1;
    const FREE: u32 = 2;
    const ALLOC_G: u32 = 0x0163_2c60;
    unsafe {
        lf_checker_rt::callee_thiscall!(DTOR, u32, this);
        if flag & 1 != 0 {
            let alloc = lf_checker_rt::global::<u32>(ALLOC_G).read_unaligned();
            lf_checker_rt::callee_thiscall!(FREE, u32, alloc, this);
        }
        this
    }
});
