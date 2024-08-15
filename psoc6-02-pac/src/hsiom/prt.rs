#[repr(C)]
#[doc = "HSIOM port registers"]
#[doc(alias = "PRT")]
pub struct Prt {
    port_sel0: PortSel0,
    port_sel1: PortSel1,
}
impl Prt {
    #[doc = "0x00 - Port selection 0"]
    #[inline(always)]
    pub const fn port_sel0(&self) -> &PortSel0 {
        &self.port_sel0
    }
    #[doc = "0x04 - Port selection 1"]
    #[inline(always)]
    pub const fn port_sel1(&self) -> &PortSel1 {
        &self.port_sel1
    }
}
#[doc = "PORT_SEL0 (rw) register accessor: Port selection 0\n\nYou can [`read`](crate::Reg::read) this register and get [`port_sel0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`port_sel0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@port_sel0`]
module"]
#[doc(alias = "PORT_SEL0")]
pub type PortSel0 = crate::Reg<port_sel0::PortSel0Spec>;
#[doc = "Port selection 0"]
pub mod port_sel0;
#[doc = "PORT_SEL1 (rw) register accessor: Port selection 1\n\nYou can [`read`](crate::Reg::read) this register and get [`port_sel1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`port_sel1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@port_sel1`]
module"]
#[doc(alias = "PORT_SEL1")]
pub type PortSel1 = crate::Reg<port_sel1::PortSel1Spec>;
#[doc = "Port selection 1"]
pub mod port_sel1;
