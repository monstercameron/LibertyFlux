// original: 0x006727E0 init_async_file_request
//! Initialize the request header's leading fields: write the invalid status
//! sentinel, clear the two following words and clear the byte flag at offset
//! `REQUEST_FLAG_OFFSET`. Return the same request pointer in EAX.
const REQUEST_STATUS_OFFSET: usize = 0;
const REQUEST_WORD_ONE_OFFSET: usize = 4;
const REQUEST_WORD_TWO_OFFSET: usize = 8;
const REQUEST_FLAG_OFFSET: usize = 12;
const INVALID_REQUEST_STATUS: u32 = u32::MAX;

#[no_mangle]
pub unsafe extern "thiscall" fn fn_006727E0(request: *mut u32) -> *mut u32 {
    let request_bytes = request.cast::<u8>();
    unsafe {
        core::ptr::write_volatile(request.add(REQUEST_STATUS_OFFSET / 4), INVALID_REQUEST_STATUS);
        core::ptr::write_volatile(request.add(REQUEST_WORD_ONE_OFFSET / 4), 0);
        core::ptr::write_volatile(request.add(REQUEST_WORD_TWO_OFFSET / 4), 0);
        core::ptr::write_volatile(request_bytes.add(REQUEST_FLAG_OFFSET), 0);
    }
    request
}
