// original: 0x00e5fbb0 store_const_block_110ffc0
/// Write a 16-byte constant record to a fixed global address.
///
/// The record is four fixed words. Takes no arguments and returns nothing
/// meaningful (the original leaves its entry EAX untouched, so the return
/// channel is not compared). Two words are address constants, relocated
/// exactly as the worker relocates the original; the other two words are
/// zero, one written directly and one arriving via stack scratch that the
/// original never wrote (the checker fills that scratch with a defined
/// value, reproduced here as the same constant).
export!(cdecl, rw_00e5fbb0() -> u32 {
    unsafe {
        const DST: u32 = 0x110ffc0;
        let p = global::<u32>(DST);
        p.add(0).write(relocated(0x43ea90));
        p.add(1).write(0);
        p.add(2).write(0);
        p.add(3).write(relocated(0x409610));
        0
    }
});
