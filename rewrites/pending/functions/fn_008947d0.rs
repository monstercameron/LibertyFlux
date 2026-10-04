// original: 0x008947d0 resolve_and_create
/// Resolve the first argument through a helper, then build an entry.
///
/// The helper answer becomes the key passed to a five-argument constructor
/// together with the remaining arguments and a zero options word.
/// Returns the constructor's result.
crate::rt::export!(thiscall, rs17_008947d0(this: u32, a1: u32, a2: u32, a3: u32, a4: u32) -> u32 {
    let key = crate::callee_thiscall!(1, u32, this, a1);
    crate::callee_thiscall!(2, u32, this, key, a2, a3, a4, 0)
});
