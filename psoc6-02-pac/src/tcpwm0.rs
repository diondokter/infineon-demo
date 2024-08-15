#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrl: Ctrl,
    ctrl_clr: CtrlClr,
    ctrl_set: CtrlSet,
    cmd_capture: CmdCapture,
    cmd_reload: CmdReload,
    cmd_stop: CmdStop,
    cmd_start: CmdStart,
    intr_cause: IntrCause,
    _reserved8: [u8; 0xe0],
    cnt: [Cnt; 24],
}
impl RegisterBlock {
    #[doc = "0x00 - TCPWM control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - TCPWM control clear register"]
    #[inline(always)]
    pub const fn ctrl_clr(&self) -> &CtrlClr {
        &self.ctrl_clr
    }
    #[doc = "0x08 - TCPWM control set register"]
    #[inline(always)]
    pub const fn ctrl_set(&self) -> &CtrlSet {
        &self.ctrl_set
    }
    #[doc = "0x0c - TCPWM capture command register"]
    #[inline(always)]
    pub const fn cmd_capture(&self) -> &CmdCapture {
        &self.cmd_capture
    }
    #[doc = "0x10 - TCPWM reload command register"]
    #[inline(always)]
    pub const fn cmd_reload(&self) -> &CmdReload {
        &self.cmd_reload
    }
    #[doc = "0x14 - TCPWM stop command register"]
    #[inline(always)]
    pub const fn cmd_stop(&self) -> &CmdStop {
        &self.cmd_stop
    }
    #[doc = "0x18 - TCPWM start command register"]
    #[inline(always)]
    pub const fn cmd_start(&self) -> &CmdStart {
        &self.cmd_start
    }
    #[doc = "0x1c - TCPWM Counter interrupt cause register"]
    #[inline(always)]
    pub const fn intr_cause(&self) -> &IntrCause {
        &self.intr_cause
    }
    #[doc = "0x100..0x700 - Timer/Counter/PWM Counter Module"]
    #[inline(always)]
    pub const fn cnt(&self, n: usize) -> &Cnt {
        &self.cnt[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x700 - Timer/Counter/PWM Counter Module"]
    #[inline(always)]
    pub fn cnt_iter(&self) -> impl Iterator<Item = &Cnt> {
        self.cnt.iter()
    }
}
#[doc = "CTRL (rw) register accessor: TCPWM control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "TCPWM control register"]
pub mod ctrl;
#[doc = "CTRL_CLR (rw) register accessor: TCPWM control clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_clr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_clr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_clr`]
module"]
#[doc(alias = "CTRL_CLR")]
pub type CtrlClr = crate::Reg<ctrl_clr::CtrlClrSpec>;
#[doc = "TCPWM control clear register"]
pub mod ctrl_clr;
#[doc = "CTRL_SET (rw) register accessor: TCPWM control set register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl_set`]
module"]
#[doc(alias = "CTRL_SET")]
pub type CtrlSet = crate::Reg<ctrl_set::CtrlSetSpec>;
#[doc = "TCPWM control set register"]
pub mod ctrl_set;
#[doc = "CMD_CAPTURE (rw) register accessor: TCPWM capture command register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_capture::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd_capture::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_capture`]
module"]
#[doc(alias = "CMD_CAPTURE")]
pub type CmdCapture = crate::Reg<cmd_capture::CmdCaptureSpec>;
#[doc = "TCPWM capture command register"]
pub mod cmd_capture;
#[doc = "CMD_RELOAD (rw) register accessor: TCPWM reload command register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_reload::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd_reload::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_reload`]
module"]
#[doc(alias = "CMD_RELOAD")]
pub type CmdReload = crate::Reg<cmd_reload::CmdReloadSpec>;
#[doc = "TCPWM reload command register"]
pub mod cmd_reload;
#[doc = "CMD_STOP (rw) register accessor: TCPWM stop command register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_stop::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd_stop::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_stop`]
module"]
#[doc(alias = "CMD_STOP")]
pub type CmdStop = crate::Reg<cmd_stop::CmdStopSpec>;
#[doc = "TCPWM stop command register"]
pub mod cmd_stop;
#[doc = "CMD_START (rw) register accessor: TCPWM start command register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd_start::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd_start::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_start`]
module"]
#[doc(alias = "CMD_START")]
pub type CmdStart = crate::Reg<cmd_start::CmdStartSpec>;
#[doc = "TCPWM start command register"]
pub mod cmd_start;
#[doc = "INTR_CAUSE (r) register accessor: TCPWM Counter interrupt cause register\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_cause::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_cause`]
module"]
#[doc(alias = "INTR_CAUSE")]
pub type IntrCause = crate::Reg<intr_cause::IntrCauseSpec>;
#[doc = "TCPWM Counter interrupt cause register"]
pub mod intr_cause;
#[doc = "Timer/Counter/PWM Counter Module"]
pub use self::cnt::Cnt;
#[doc = r"Cluster"]
#[doc = "Timer/Counter/PWM Counter Module"]
pub mod cnt;
