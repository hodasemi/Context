//! Basic helper functions

use std::fs;
use std::rc::Rc;
use std::sync::Arc;

use std::ffi::CStr;
use std::os::raw::c_char;

use cgmath;
use cgmath::prelude::Transform;

use crate::errortype::VerboseResult;

pub fn erase_by_ptr<T>(vector: &mut Vec<T>, object: &T) -> bool {
    match vector
        .iter()
        .position(|t| t as *const T == object as *const T)
    {
        Some(i) => {
            vector.remove(i);
            true
        }
        None => false,
    }
}

pub fn erase_arc<T: ?Sized>(vector: &mut Vec<Arc<T>>, object: &Arc<T>) -> bool {
    match vector.iter().position(|t| Arc::ptr_eq(t, object)) {
        Some(i) => {
            vector.remove(i);
            true
        }
        None => false,
    }
}

pub fn erase_rc<T: ?Sized>(vector: &mut Vec<Rc<T>>, object: &Rc<T>) -> bool {
    match vector.iter().position(|t| Rc::ptr_eq(t, object)) {
        Some(i) => {
            vector.remove(i);
            true
        }
        None => false,
    }
}

pub fn ortho(
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
    z_near: f32,
    z_far: f32,
) -> cgmath::Matrix4<f32> {
    let mut mat = cgmath::Matrix4::one();

    mat[0][0] = 2.0 / (right - left);
    mat[1][1] = -2.0 / (top - bottom);
    mat[2][2] = -2.0 / (z_far - z_near);
    mat[3][0] = -(right + left) / (right - left);
    mat[3][1] = -(top + bottom) / (top - bottom);
    mat[3][2] = -(z_far + z_near) / (z_far - z_near);

    mat
}

pub fn search_dir_recursively(base_dir: &str, suffix: &str) -> VerboseResult<Vec<String>> {
    let mut files = Vec::new();

    let dir_content = fs::read_dir(base_dir)?;

    for fs_object in dir_content {
        let obj = fs_object?;

        let path = obj.path();

        if let Some(string) = path.to_str() {
            if path.is_file() {
                if string.ends_with(suffix) {
                    files.push(string.to_string());
                }
            } else {
                let more_files = search_dir_recursively(string, suffix)?;
                files.extend(more_files);
            }
        }
    }

    Ok(files)
}

pub unsafe fn c_char_to_string(cchar: *const c_char) -> String {
    CStr::from_ptr(cchar).to_str().unwrap().to_string()
}

#[inline]
pub fn perspective(fov_y: f32, aspect: f32, z_near: f32, z_far: f32) -> cgmath::Matrix4<f32> {
    debug_assert!(z_near != 0.0);

    let zero = 0.0;
    let one = 1.0;
    let two = 2.0;
    let q = one / (fov_y / two).tan();
    let a = q / aspect;
    let b = (z_near + z_far) / (z_near - z_far);
    let c = (two * z_near * z_far) / (z_near - z_far);

    #[cfg_attr(rustfmt, rustfmt_skip)]
    cgmath::Matrix4::new(
        a,    zero, zero, zero,
        zero,   -q, zero, zero,
        zero, zero,    b, zero - one,
        zero, zero,    c, zero,
    )
}

#[inline]
pub fn rotate_z(v: cgmath::Vector3<f32>, angle: f32) -> cgmath::Vector3<f32> {
    let mut result = v;
    let cos = angle.cos();
    let sin = angle.sin();

    result.x = v.x * cos - v.y * sin;
    result.y = v.x * sin + v.y * cos;

    result
}
