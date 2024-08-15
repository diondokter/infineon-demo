#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    struct_: (),
}
impl RegisterBlock {
    #[doc = "0x00..0x1a0 - Fault structure"]
    #[inline(always)]
    pub const fn struct_(&self, n: usize) -> &Struct {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(0)
                .add(256 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x1a0 - Fault structure"]
    #[inline(always)]
    pub fn struct__iter(&self) -> impl Iterator<Item = &Struct> {
        (0..2).map(move |n| unsafe {
            &*(self as *const Self)
                .cast::<u8>()
                .add(0)
                .add(256 * n)
                .cast()
        })
    }
}
#[doc = "Fault structure"]
pub use self::struct_::Struct;
#[doc = r"Cluster"]
#[doc = "Fault structure"]
pub mod struct_;
