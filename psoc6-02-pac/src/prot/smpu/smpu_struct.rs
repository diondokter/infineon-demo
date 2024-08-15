#[repr(C)]
#[doc = "SMPU structure"]
#[doc(alias = "SMPU_STRUCT")]
pub struct SmpuStruct {
    addr0: Addr0,
    att0: Att0,
    _reserved2: [u8; 0x18],
    addr1: Addr1,
    att1: Att1,
}
impl SmpuStruct {
    #[doc = "0x00 - SMPU region address 0 (slave structure)"]
    #[inline(always)]
    pub const fn addr0(&self) -> &Addr0 {
        &self.addr0
    }
    #[doc = "0x04 - SMPU region attributes 0 (slave structure)"]
    #[inline(always)]
    pub const fn att0(&self) -> &Att0 {
        &self.att0
    }
    #[doc = "0x20 - SMPU region address 1 (master structure)"]
    #[inline(always)]
    pub const fn addr1(&self) -> &Addr1 {
        &self.addr1
    }
    #[doc = "0x24 - SMPU region attributes 1 (master structure)"]
    #[inline(always)]
    pub const fn att1(&self) -> &Att1 {
        &self.att1
    }
}
#[doc = "ADDR0 (rw) register accessor: SMPU region address 0 (slave structure)\n\nYou can [`read`](crate::Reg::read) this register and get [`addr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr0`]
module"]
#[doc(alias = "ADDR0")]
pub type Addr0 = crate::Reg<addr0::Addr0Spec>;
#[doc = "SMPU region address 0 (slave structure)"]
pub mod addr0;
#[doc = "ATT0 (rw) register accessor: SMPU region attributes 0 (slave structure)\n\nYou can [`read`](crate::Reg::read) this register and get [`att0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`att0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@att0`]
module"]
#[doc(alias = "ATT0")]
pub type Att0 = crate::Reg<att0::Att0Spec>;
#[doc = "SMPU region attributes 0 (slave structure)"]
pub mod att0;
#[doc = "ADDR1 (r) register accessor: SMPU region address 1 (master structure)\n\nYou can [`read`](crate::Reg::read) this register and get [`addr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr1`]
module"]
#[doc(alias = "ADDR1")]
pub type Addr1 = crate::Reg<addr1::Addr1Spec>;
#[doc = "SMPU region address 1 (master structure)"]
pub mod addr1;
#[doc = "ATT1 (rw) register accessor: SMPU region attributes 1 (master structure)\n\nYou can [`read`](crate::Reg::read) this register and get [`att1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`att1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@att1`]
module"]
#[doc(alias = "ATT1")]
pub type Att1 = crate::Reg<att1::Att1Spec>;
#[doc = "SMPU region attributes 1 (master structure)"]
pub mod att1;
