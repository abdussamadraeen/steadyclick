//! Standard C-ABI WebAssembly exports for zero-copy linear memory interop.

use alloc::boxed::Box;
use alloc::vec::Vec;
use crate::kinematics::one_euro::Point2DFilter;
use crate::kinematics::voronoi::{find_captured_target, TargetAABB};
use crate::linguistics::dale_chall::is_familiar_word;
use crate::perceptual::apca::calculate_apca;

/// Allocates raw byte buffer in WASM linear memory for host (JavaScript) data transfer.
#[no_mangle]
pub extern "C" fn centennial_alloc(size: usize) -> *mut u8 {
    let mut vec = Vec::<u8>::with_capacity(size);
    let ptr = vec.as_mut_ptr();
    core::mem::forget(vec);
    ptr
}

/// Deallocates raw byte buffer in WASM linear memory.
#[no_mangle]
pub unsafe extern "C" fn centennial_dealloc(ptr: *mut u8, size: usize) {
    if !ptr.is_null() {
        let _ = Vec::from_raw_parts(ptr, 0, size);
    }
}

/// Creates a new 1€ 2D Point Filter.
#[no_mangle]
pub extern "C" fn one_euro_new(min_cutoff: f64, beta: f64, d_cutoff: f64) -> *mut Point2DFilter {
    let filter = Box::new(Point2DFilter::new(min_cutoff, beta, d_cutoff));
    Box::into_raw(filter)
}

/// Frees an existing 1€ 2D Point Filter.
#[no_mangle]
pub unsafe extern "C" fn one_euro_free(ptr: *mut Point2DFilter) {
    if !ptr.is_null() {
        drop(Box::from_raw(ptr));
    }
}

/// Steps the 1€ 2D Point Filter with new raw coordinates.
/// Output is written to out_x and out_y pointers.
#[no_mangle]
pub unsafe extern "C" fn one_euro_step(
    filter_ptr: *mut Point2DFilter,
    x: f64,
    y: f64,
    timestamp_sec: f64,
    out_x: *mut f64,
    out_y: *mut f64,
) {
    if filter_ptr.is_null() || out_x.is_null() || out_y.is_null() {
        return;
    }
    let filter = &mut *filter_ptr;
    let (fx, fy) = filter.filter(x, y, timestamp_sec);
    *out_x = fx;
    *out_y = fy;
}

/// Resets the 1€ 2D Point Filter history.
#[no_mangle]
pub unsafe extern "C" fn one_euro_reset(filter_ptr: *mut Point2DFilter) {
    if !filter_ptr.is_null() {
        (*filter_ptr).reset();
    }
}

/// Evaluates targets on screen and calculates Voronoi Bubble Cursor capture.
///
/// * `targets_packed`: Array of f64 with length = target_count * 5.
///   Each element consists of: [id as f64, x0, y0, x1, y1]
/// * `out_result`: Pointer to an array of 5 f64 values:
///   [captured_id, distance_primary, distance_secondary, snap_x, snap_y]
#[no_mangle]
pub unsafe extern "C" fn voronoi_query(
    cursor_x: f64,
    cursor_y: f64,
    targets_packed: *const f64,
    target_count: usize,
    max_magnetic_radius: f64,
    out_result: *mut f64,
) {
    if targets_packed.is_null() || out_result.is_null() {
        return;
    }

    let mut targets = Vec::with_capacity(target_count);
    for i in 0..target_count {
        let offset = i * 5;
        let id = *targets_packed.add(offset) as u32;
        let x0 = *targets_packed.add(offset + 1);
        let y0 = *targets_packed.add(offset + 2);
        let x1 = *targets_packed.add(offset + 3);
        let y1 = *targets_packed.add(offset + 4);
        targets.push(TargetAABB::new(id, x0, y0, x1, y1));
    }

    let res = find_captured_target(cursor_x, cursor_y, &targets, max_magnetic_radius);

    *out_result = res.captured_target_id.map(|id| id as f64).unwrap_or(-1.0);
    *out_result.add(1) = res.distance_to_primary;
    *out_result.add(2) = res.distance_to_secondary;
    *out_result.add(3) = res.snap_point.0;
    *out_result.add(4) = res.snap_point.1;
}

/// Computes APCA Lightness Contrast (Lc) value between text (r,g,b) and background (r,g,b).
#[no_mangle]
pub extern "C" fn apca_evaluate(
    txt_r: u32,
    txt_g: u32,
    txt_b: u32,
    bg_r: u32,
    bg_g: u32,
    bg_b: u32,
) -> f64 {
    let res = calculate_apca(
        txt_r as u8,
        txt_g as u8,
        txt_b as u8,
        bg_r as u8,
        bg_g as u8,
        bg_b as u8,
    );
    res.lc
}

/// Checks if a lowercase UTF-8 string is in the Dale-Chall familiar vocabulary.
/// Returns 1 if familiar, 0 if unfamiliar.
#[no_mangle]
pub unsafe extern "C" fn dale_chall_lookup(word_ptr: *const u8, word_len: usize) -> i32 {
    if word_ptr.is_null() || word_len == 0 {
        return 0;
    }
    let slice = core::slice::from_raw_parts(word_ptr, word_len);
    if let Ok(word_str) = core::str::from_utf8(slice) {
        if is_familiar_word(word_str) {
            1
        } else {
            0
        }
    } else {
        0
    }
}
