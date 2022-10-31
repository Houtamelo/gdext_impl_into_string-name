/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

// Stub for various other built-in classes, which are currently incomplete, but whose types
// are required for codegen
use crate::builtin::GodotString;
use godot_ffi as sys;
use sys::{ffi_methods, GodotFfi};

// TODO: Swap more inner math types with glam types
// Note: ordered by enum ord in extension JSON
impl_builtin_stub!(Rect2, OpaqueRect2);
impl_builtin_stub!(Rect2i, OpaqueRect2i);
impl_builtin_stub!(Plane, OpaquePlane);
impl_builtin_stub!(Quaternion, OpaqueQuaternion);
impl_builtin_stub!(AABB, OpaqueAABB);
impl_builtin_stub!(Basis, OpaqueBasis);
impl_builtin_stub!(Transform2D, OpaqueTransform2D);
impl_builtin_stub!(Transform3D, OpaqueTransform3D);
impl_builtin_stub!(Projection, OpaqueProjection);
impl_builtin_stub!(NodePath, OpaqueNodePath);
impl_builtin_stub!(RID, OpaqueRID);
impl_builtin_stub!(Callable, OpaqueCallable);
impl_builtin_stub!(Signal, OpaqueSignal);
impl_builtin_stub!(Dictionary, OpaqueDictionary);

impl From<&GodotString> for NodePath {
    fn from(path: &GodotString) -> Self {
        unsafe {
            Self::from_sys_init(|self_ptr| {
                let ctor = sys::method_table().node_path_from_string;
                let args = [path.sys()];
                ctor(self_ptr, args.as_ptr());
            })
        }
    }
}

impl From<&str> for NodePath {
    fn from(path: &str) -> Self {
        Self::from(&GodotString::from(path))
    }
}
