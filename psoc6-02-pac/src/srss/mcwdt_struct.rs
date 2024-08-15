#[repr(C)]
#[doc = "Multi-Counter Watchdog Timer"]
#[doc(alias = "MCWDT_STRUCT")]
pub struct McwdtStruct {
    _reserved0: [u8; 0x04],
    mcwdt_cntlow: McwdtCntlow,
    mcwdt_cnthigh: McwdtCnthigh,
    mcwdt_match: McwdtMatch,
    mcwdt_config: McwdtConfig,
    mcwdt_ctl: McwdtCtl,
    mcwdt_intr: McwdtIntr,
    mcwdt_intr_set: McwdtIntrSet,
    mcwdt_intr_mask: McwdtIntrMask,
    mcwdt_intr_masked: McwdtIntrMasked,
    mcwdt_lock: McwdtLock,
}
impl McwdtStruct {
    #[doc = "0x04 - Multi-Counter Watchdog Sub-counters 0/1"]
    #[inline(always)]
    pub const fn mcwdt_cntlow(&self) -> &McwdtCntlow {
        &self.mcwdt_cntlow
    }
    #[doc = "0x08 - Multi-Counter Watchdog Sub-counter 2"]
    #[inline(always)]
    pub const fn mcwdt_cnthigh(&self) -> &McwdtCnthigh {
        &self.mcwdt_cnthigh
    }
    #[doc = "0x0c - Multi-Counter Watchdog Counter Match Register"]
    #[inline(always)]
    pub const fn mcwdt_match(&self) -> &McwdtMatch {
        &self.mcwdt_match
    }
    #[doc = "0x10 - Multi-Counter Watchdog Counter Configuration"]
    #[inline(always)]
    pub const fn mcwdt_config(&self) -> &McwdtConfig {
        &self.mcwdt_config
    }
    #[doc = "0x14 - Multi-Counter Watchdog Counter Control"]
    #[inline(always)]
    pub const fn mcwdt_ctl(&self) -> &McwdtCtl {
        &self.mcwdt_ctl
    }
    #[doc = "0x18 - Multi-Counter Watchdog Counter Interrupt Register"]
    #[inline(always)]
    pub const fn mcwdt_intr(&self) -> &McwdtIntr {
        &self.mcwdt_intr
    }
    #[doc = "0x1c - Multi-Counter Watchdog Counter Interrupt Set Register"]
    #[inline(always)]
    pub const fn mcwdt_intr_set(&self) -> &McwdtIntrSet {
        &self.mcwdt_intr_set
    }
    #[doc = "0x20 - Multi-Counter Watchdog Counter Interrupt Mask Register"]
    #[inline(always)]
    pub const fn mcwdt_intr_mask(&self) -> &McwdtIntrMask {
        &self.mcwdt_intr_mask
    }
    #[doc = "0x24 - Multi-Counter Watchdog Counter Interrupt Masked Register"]
    #[inline(always)]
    pub const fn mcwdt_intr_masked(&self) -> &McwdtIntrMasked {
        &self.mcwdt_intr_masked
    }
    #[doc = "0x28 - Multi-Counter Watchdog Counter Lock Register"]
    #[inline(always)]
    pub const fn mcwdt_lock(&self) -> &McwdtLock {
        &self.mcwdt_lock
    }
}
#[doc = "MCWDT_CNTLOW (rw) register accessor: Multi-Counter Watchdog Sub-counters 0/1\n\nYou can [`read`](crate::Reg::read) this register and get [`mcwdt_cntlow::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcwdt_cntlow::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcwdt_cntlow`]
module"]
#[doc(alias = "MCWDT_CNTLOW")]
pub type McwdtCntlow = crate::Reg<mcwdt_cntlow::McwdtCntlowSpec>;
#[doc = "Multi-Counter Watchdog Sub-counters 0/1"]
pub mod mcwdt_cntlow;
#[doc = "MCWDT_CNTHIGH (rw) register accessor: Multi-Counter Watchdog Sub-counter 2\n\nYou can [`read`](crate::Reg::read) this register and get [`mcwdt_cnthigh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcwdt_cnthigh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcwdt_cnthigh`]
module"]
#[doc(alias = "MCWDT_CNTHIGH")]
pub type McwdtCnthigh = crate::Reg<mcwdt_cnthigh::McwdtCnthighSpec>;
#[doc = "Multi-Counter Watchdog Sub-counter 2"]
pub mod mcwdt_cnthigh;
#[doc = "MCWDT_MATCH (rw) register accessor: Multi-Counter Watchdog Counter Match Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcwdt_match::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcwdt_match::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcwdt_match`]
module"]
#[doc(alias = "MCWDT_MATCH")]
pub type McwdtMatch = crate::Reg<mcwdt_match::McwdtMatchSpec>;
#[doc = "Multi-Counter Watchdog Counter Match Register"]
pub mod mcwdt_match;
#[doc = "MCWDT_CONFIG (rw) register accessor: Multi-Counter Watchdog Counter Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcwdt_config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcwdt_config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcwdt_config`]
module"]
#[doc(alias = "MCWDT_CONFIG")]
pub type McwdtConfig = crate::Reg<mcwdt_config::McwdtConfigSpec>;
#[doc = "Multi-Counter Watchdog Counter Configuration"]
pub mod mcwdt_config;
#[doc = "MCWDT_CTL (rw) register accessor: Multi-Counter Watchdog Counter Control\n\nYou can [`read`](crate::Reg::read) this register and get [`mcwdt_ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcwdt_ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcwdt_ctl`]
module"]
#[doc(alias = "MCWDT_CTL")]
pub type McwdtCtl = crate::Reg<mcwdt_ctl::McwdtCtlSpec>;
#[doc = "Multi-Counter Watchdog Counter Control"]
pub mod mcwdt_ctl;
#[doc = "MCWDT_INTR (rw) register accessor: Multi-Counter Watchdog Counter Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcwdt_intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcwdt_intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcwdt_intr`]
module"]
#[doc(alias = "MCWDT_INTR")]
pub type McwdtIntr = crate::Reg<mcwdt_intr::McwdtIntrSpec>;
#[doc = "Multi-Counter Watchdog Counter Interrupt Register"]
pub mod mcwdt_intr;
#[doc = "MCWDT_INTR_SET (rw) register accessor: Multi-Counter Watchdog Counter Interrupt Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcwdt_intr_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcwdt_intr_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcwdt_intr_set`]
module"]
#[doc(alias = "MCWDT_INTR_SET")]
pub type McwdtIntrSet = crate::Reg<mcwdt_intr_set::McwdtIntrSetSpec>;
#[doc = "Multi-Counter Watchdog Counter Interrupt Set Register"]
pub mod mcwdt_intr_set;
#[doc = "MCWDT_INTR_MASK (rw) register accessor: Multi-Counter Watchdog Counter Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcwdt_intr_mask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcwdt_intr_mask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcwdt_intr_mask`]
module"]
#[doc(alias = "MCWDT_INTR_MASK")]
pub type McwdtIntrMask = crate::Reg<mcwdt_intr_mask::McwdtIntrMaskSpec>;
#[doc = "Multi-Counter Watchdog Counter Interrupt Mask Register"]
pub mod mcwdt_intr_mask;
#[doc = "MCWDT_INTR_MASKED (r) register accessor: Multi-Counter Watchdog Counter Interrupt Masked Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcwdt_intr_masked::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcwdt_intr_masked`]
module"]
#[doc(alias = "MCWDT_INTR_MASKED")]
pub type McwdtIntrMasked = crate::Reg<mcwdt_intr_masked::McwdtIntrMaskedSpec>;
#[doc = "Multi-Counter Watchdog Counter Interrupt Masked Register"]
pub mod mcwdt_intr_masked;
#[doc = "MCWDT_LOCK (rw) register accessor: Multi-Counter Watchdog Counter Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcwdt_lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcwdt_lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcwdt_lock`]
module"]
#[doc(alias = "MCWDT_LOCK")]
pub type McwdtLock = crate::Reg<mcwdt_lock::McwdtLockSpec>;
#[doc = "Multi-Counter Watchdog Counter Lock Register"]
pub mod mcwdt_lock;
