#[repr(C)]
#[doc = "MPU structure"]
#[doc(alias = "MPU_STRUCT")]
pub struct MpuStruct {
    addr: Addr,
    att: Att,
}
impl MpuStruct {
    #[doc = "0x00 - MPU region address"]
    #[inline(always)]
    pub const fn addr(&self) -> &Addr {
        &self.addr
    }
    #[doc = "0x04 - MPU region attrributes"]
    #[inline(always)]
    pub const fn att(&self) -> &Att {
        &self.att
    }
}
#[doc = "ADDR (rw) register accessor: MPU region address\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "MPU region address"]
pub mod addr;
#[doc = "ATT (rw) register accessor: MPU region attrributes\n\nYou can [`read`](crate::Reg::read) this register and get [`att::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`att::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@att`]
module"]
#[doc(alias = "ATT")]
pub type Att = crate::Reg<att::AttSpec>;
#[doc = "MPU region attrributes"]
pub mod att;
