#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    id: Id,
    divider: Divider,
    control: Control,
    _reserved3: [u8; 0xf4],
    data0: [Data0; 8],
    _reserved4: [u8; 0xe0],
    data1: [Data1; 8],
    _reserved5: [u8; 0xe0],
    data2: [Data2; 8],
    _reserved6: [u8; 0xe0],
    data3: [Data3; 8],
}
impl RegisterBlock {
    #[doc = "0x00 - ID &amp; Revision"]
    #[inline(always)]
    pub const fn id(&self) -> &Id {
        &self.id
    }
    #[doc = "0x04 - LCD Divider Register"]
    #[inline(always)]
    pub const fn divider(&self) -> &Divider {
        &self.divider
    }
    #[doc = "0x08 - LCD Configuration Register"]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x100..0x120 - LCD Pin Data Registers"]
    #[inline(always)]
    pub const fn data0(&self, n: usize) -> &Data0 {
        &self.data0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x120 - LCD Pin Data Registers"]
    #[inline(always)]
    pub fn data0_iter(&self) -> impl Iterator<Item = &Data0> {
        self.data0.iter()
    }
    #[doc = "0x200..0x220 - LCD Pin Data Registers"]
    #[inline(always)]
    pub const fn data1(&self, n: usize) -> &Data1 {
        &self.data1[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x220 - LCD Pin Data Registers"]
    #[inline(always)]
    pub fn data1_iter(&self) -> impl Iterator<Item = &Data1> {
        self.data1.iter()
    }
    #[doc = "0x300..0x320 - LCD Pin Data Registers"]
    #[inline(always)]
    pub const fn data2(&self, n: usize) -> &Data2 {
        &self.data2[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x300..0x320 - LCD Pin Data Registers"]
    #[inline(always)]
    pub fn data2_iter(&self) -> impl Iterator<Item = &Data2> {
        self.data2.iter()
    }
    #[doc = "0x400..0x420 - LCD Pin Data Registers"]
    #[inline(always)]
    pub const fn data3(&self, n: usize) -> &Data3 {
        &self.data3[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x400..0x420 - LCD Pin Data Registers"]
    #[inline(always)]
    pub fn data3_iter(&self) -> impl Iterator<Item = &Data3> {
        self.data3.iter()
    }
}
#[doc = "ID (r) register accessor: ID &amp; Revision\n\nYou can [`read`](crate::Reg::read) this register and get [`id::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
#[doc(alias = "ID")]
pub type Id = crate::Reg<id::IdSpec>;
#[doc = "ID &amp; Revision"]
pub mod id;
#[doc = "DIVIDER (rw) register accessor: LCD Divider Register\n\nYou can [`read`](crate::Reg::read) this register and get [`divider::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divider::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divider`]
module"]
#[doc(alias = "DIVIDER")]
pub type Divider = crate::Reg<divider::DividerSpec>;
#[doc = "LCD Divider Register"]
pub mod divider;
#[doc = "CONTROL (rw) register accessor: LCD Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "LCD Configuration Register"]
pub mod control;
#[doc = "DATA0 (rw) register accessor: LCD Pin Data Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`data0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data0`]
module"]
#[doc(alias = "DATA0")]
pub type Data0 = crate::Reg<data0::Data0Spec>;
#[doc = "LCD Pin Data Registers"]
pub mod data0;
#[doc = "DATA1 (rw) register accessor: LCD Pin Data Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`data1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data1`]
module"]
#[doc(alias = "DATA1")]
pub type Data1 = crate::Reg<data1::Data1Spec>;
#[doc = "LCD Pin Data Registers"]
pub mod data1;
#[doc = "DATA2 (rw) register accessor: LCD Pin Data Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`data2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data2`]
module"]
#[doc(alias = "DATA2")]
pub type Data2 = crate::Reg<data2::Data2Spec>;
#[doc = "LCD Pin Data Registers"]
pub mod data2;
#[doc = "DATA3 (rw) register accessor: LCD Pin Data Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`data3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data3`]
module"]
#[doc(alias = "DATA3")]
pub type Data3 = crate::Reg<data3::Data3Spec>;
#[doc = "LCD Pin Data Registers"]
pub mod data3;
