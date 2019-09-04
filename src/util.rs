use std::{mem, ptr};

pub const fn ptr_size_bits() -> usize {
    mem::size_of::<usize>() * 8
}

pub fn map_in_place_2<T, U, F: FnOnce(U, T) -> T>(a: (U, &mut T), f: F) {
    unsafe {
        ptr::write(a.1, f(a.0, ptr::read(a.1)));
    }
}

pub unsafe fn change_lifetime_const<'a, 'b, T>(x: &'a T) -> &'b T {
    &*(x as *const T)
}

pub unsafe fn change_lifetime_mut<'a, 'b, T>(x: &'a mut T) -> &'b mut T {
    &mut *(x as *mut T)
}

pub unsafe fn to_mut<'a, T>(x: &'a T) -> &'a mut T {
    &mut *(x as *const T as *mut T)
}

pub unsafe fn swap_nonoverlapping<T>(x: *mut T, y: *mut T) {
    let (x, y) = (x as usize, y as usize);
    let item_size = mem::size_of::<T>();
    let mut i = 0;
    while i + 4 <= item_size {
        let x = (x + i) as *mut u32;
        let y = (y + i) as *mut u32;
        *x ^= *y;
        *y ^= *x;
        *x ^= *y;
        i += 4;
    }
    while i + 1 <= item_size {
        let x = (x + i) as *mut u8;
        let y = (y + i) as *mut u8;
        *x ^= *y;
        *y ^= *x;
        *x ^= *y;
        i += 1;
    }
}
