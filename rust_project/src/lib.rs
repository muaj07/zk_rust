use std::ffi::c_void;
pub mod libstark_tests;

#[cxx::bridge(namespace = "bair_witness_bridge")]
mod ffi {
    
    unsafe extern "C++" {
        include!("bair_witness_bridge.h");

        type BairWitness;

        pub unsafe fn new_bair_witness() -> *mut BairWitness;
        pub unsafe fn delete_bair_witness(witness: *mut BairWitness);
    }
}

pub struct BairWitness {
    inner: *mut ffi::BairWitness,
}

impl BairWitness {
    pub fn new() -> Self {
        unsafe {
        let inner = ffi::new_bair_witness();
        Self { inner }
        }
    }
}

impl Drop for BairWitness {
    fn drop(&mut self) {
        unsafe { ffi::delete_bair_witness(self.inner) }
    }
}
