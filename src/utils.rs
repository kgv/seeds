use opencv::{
    boxed_ref::BoxedRef,
    core::{Mat, MatTraitConst, MatTraitConstManual, ToInputArray, _InputArray, type_to_string},
    Result,
};
use std::{
    ffi::c_void,
    fmt::{self, Display, Formatter},
    hash::{Hash, Hasher},
};

/// Sync [`Mat`]
#[derive(Clone, Debug, Default)]
pub struct SyncMat(pub Mat);

impl Display for SyncMat {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("Matrix")
            .field(
                "type",
                &type_to_string(self.0.typ()).map_err(|_| fmt::Error)?,
            )
            .field("rows", &self.rows())
            .field("columns", &self.cols())
            .finish()
    }
}

impl Hash for SyncMat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.data_bytes().ok().hash(state)
    }
}

unsafe impl Sync for SyncMat {}

impl MatTraitConst for SyncMat {
    fn as_raw_Mat(&self) -> *const c_void {
        self.0.as_raw_Mat()
    }
}

impl ToInputArray for SyncMat {
    fn input_array(&self) -> Result<BoxedRef<_InputArray>> {
        self.0.input_array()
    }
}
