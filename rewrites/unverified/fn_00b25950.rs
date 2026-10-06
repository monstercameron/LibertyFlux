// original: 0x00b25950 setup_ccd_query
/// Collect candidate words, test each non-null one, then commit the query.
///
/// Reads the worker handle from the session singleton, asks the collector
/// to fill an 8-word frame buffer (it answers how many words it wrote),
/// passes every non-null written word to the tester with the handle, then
/// commits the incoming query word with fixed flags through the final
/// helper. Returns the final helper's answer.
export!(stdcall, rw_00b25950(query: u32) -> u32 {
    unsafe {
        let singleton = *(global::<u32>(0x012b9c7c) as *const u32);
        let handle = *((singleton + 8) as *const u32);
        let mut buf = [0u32; 8];
        let count = callee_thiscall!(1, u32, handle, query, buf.as_mut_ptr() as u32, 8) as i32;
        if count > 0 {
            for i in 0..count {
                let word = buf[i as usize];
                if word != 0 {
                    callee_thiscall!(2, u32, handle, word);
                }
            }
        }
        callee_thiscall!(3, u32, query, 0x2000, 0)
    }
});
