use ic_kit::ic::{stable_read, stable_write, StableSize};

/// Address to a place in stable memory.
pub struct Address(pub(crate) u64);

// Reads a struct from memory.
pub fn read_struct<T>(addr: u64) -> T {
    let mut t: T = unsafe { core::mem::zeroed() };
    let t_slice = unsafe {
        core::slice::from_raw_parts_mut(&mut t as *mut _ as *mut u8, core::mem::size_of::<T>())
    };
    stable_read(addr as StableSize, t_slice);
    t
}

// Writes a struct to memory.
pub fn write_struct<T>(t: &T, addr: u64) {
    let slice = unsafe {
        core::slice::from_raw_parts(t as *const _ as *const u8, core::mem::size_of::<T>())
    };
    stable_write(addr as StableSize, slice);
}
