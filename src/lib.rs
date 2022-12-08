use std::alloc::{alloc_zeroed, Layout, LayoutError};
use std::mem::transmute;
use std::ptr::slice_from_raw_parts_mut;

fn to_slice_mut<'a, T>(pointer: *mut T, len: usize) -> Box<&'a mut [T]> {
    let slice = unsafe {
        &mut *slice_from_raw_parts_mut(pointer, len)
    };
    Box::new(slice)
}

fn alloc_slice<'a, T>(len: usize) -> Result<Box<&'a mut [T]>, LayoutError> {
    unsafe {
        let p = alloc_zeroed(Layout::array::<T>(len)?);
        Ok(to_slice_mut(transmute(p), len))
    }
}

#[cfg(test)]
mod tests {
    use super::{to_slice_mut, alloc_slice};

    fn simulate_array_behind_ffi(a: &mut [i32]) -> (*mut i32, usize) {
        println!("ORIGINAL ARRAY: {a:?}");

        // convert slice into pointer and len
        let len = a.len();
        let pointer = a.as_mut_ptr();
        (pointer, len)
    }

    #[test]
    fn test_passing_ffi_boundary() {
        let mut a = [1, 2, 3];
        // read data from FFI
        let (pointer, len) = simulate_array_behind_ffi(&mut a);

        // --- here would be the FFI boundary ---

        // convert pointer and len to slice
        let b = to_slice_mut(pointer, len);
        // ... now enjoy the slice
        b[1] = 5;

        println!("   ... CHANGED: {:?}", a);
        assert_eq!([1,5,3], a);
        assert_eq!([1,5,3], *b);
    }

    #[test]
    fn test_alloc() {
        // convert pointer and len to slice
        let b = alloc_slice(5).unwrap();
        // ... now enjoy the slice
        b[1] = 5;

        println!("SET AFTER ALLOC: {b:?}");
        assert_eq!([0,5,0,0,0], *b);
    }
}
