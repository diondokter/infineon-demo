#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctl: Ctl,
    _reserved1: [u8; 0x0c],
    cmd: Cmd,
    _reserved2: [u8; 0x0c],
    seq_default: SeqDefault,
    _reserved3: [u8; 0x1c],
    seq_read_ctl_0: SeqReadCtl0,
    seq_read_ctl_1: SeqReadCtl1,
    seq_read_ctl_2: SeqReadCtl2,
    seq_read_ctl_3: SeqReadCtl3,
    seq_read_ctl_4: SeqReadCtl4,
    seq_read_ctl_5: SeqReadCtl5,
    _reserved9: [u8; 0x08],
    seq_program_ctl_0: SeqProgramCtl0,
    seq_program_ctl_1: SeqProgramCtl1,
    seq_program_ctl_2: SeqProgramCtl2,
    seq_program_ctl_3: SeqProgramCtl3,
    seq_program_ctl_4: SeqProgramCtl4,
    seq_program_ctl_5: SeqProgramCtl5,
}
impl RegisterBlock {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x10 - Command"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x20 - Sequencer Default value"]
    #[inline(always)]
    pub const fn seq_default(&self) -> &SeqDefault {
        &self.seq_default
    }
    #[doc = "0x40 - Sequencer read control 0"]
    #[inline(always)]
    pub const fn seq_read_ctl_0(&self) -> &SeqReadCtl0 {
        &self.seq_read_ctl_0
    }
    #[doc = "0x44 - Sequencer read control 1"]
    #[inline(always)]
    pub const fn seq_read_ctl_1(&self) -> &SeqReadCtl1 {
        &self.seq_read_ctl_1
    }
    #[doc = "0x48 - Sequencer read control 2"]
    #[inline(always)]
    pub const fn seq_read_ctl_2(&self) -> &SeqReadCtl2 {
        &self.seq_read_ctl_2
    }
    #[doc = "0x4c - Sequencer read control 3"]
    #[inline(always)]
    pub const fn seq_read_ctl_3(&self) -> &SeqReadCtl3 {
        &self.seq_read_ctl_3
    }
    #[doc = "0x50 - Sequencer read control 4"]
    #[inline(always)]
    pub const fn seq_read_ctl_4(&self) -> &SeqReadCtl4 {
        &self.seq_read_ctl_4
    }
    #[doc = "0x54 - Sequencer read control 5"]
    #[inline(always)]
    pub const fn seq_read_ctl_5(&self) -> &SeqReadCtl5 {
        &self.seq_read_ctl_5
    }
    #[doc = "0x60 - Sequencer program control 0"]
    #[inline(always)]
    pub const fn seq_program_ctl_0(&self) -> &SeqProgramCtl0 {
        &self.seq_program_ctl_0
    }
    #[doc = "0x64 - Sequencer program control 1"]
    #[inline(always)]
    pub const fn seq_program_ctl_1(&self) -> &SeqProgramCtl1 {
        &self.seq_program_ctl_1
    }
    #[doc = "0x68 - Sequencer program control 2"]
    #[inline(always)]
    pub const fn seq_program_ctl_2(&self) -> &SeqProgramCtl2 {
        &self.seq_program_ctl_2
    }
    #[doc = "0x6c - Sequencer program control 3"]
    #[inline(always)]
    pub const fn seq_program_ctl_3(&self) -> &SeqProgramCtl3 {
        &self.seq_program_ctl_3
    }
    #[doc = "0x70 - Sequencer program control 4"]
    #[inline(always)]
    pub const fn seq_program_ctl_4(&self) -> &SeqProgramCtl4 {
        &self.seq_program_ctl_4
    }
    #[doc = "0x74 - Sequencer program control 5"]
    #[inline(always)]
    pub const fn seq_program_ctl_5(&self) -> &SeqProgramCtl5 {
        &self.seq_program_ctl_5
    }
}
#[doc = "CTL (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "CTL")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "Control"]
pub mod ctl;
#[doc = "CMD (rw) register accessor: Command\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "Command"]
pub mod cmd;
#[doc = "SEQ_DEFAULT (rw) register accessor: Sequencer Default value\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_default::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_default::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_default`]
module"]
#[doc(alias = "SEQ_DEFAULT")]
pub type SeqDefault = crate::Reg<seq_default::SeqDefaultSpec>;
#[doc = "Sequencer Default value"]
pub mod seq_default;
#[doc = "SEQ_READ_CTL_0 (rw) register accessor: Sequencer read control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_read_ctl_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_read_ctl_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_read_ctl_0`]
module"]
#[doc(alias = "SEQ_READ_CTL_0")]
pub type SeqReadCtl0 = crate::Reg<seq_read_ctl_0::SeqReadCtl0Spec>;
#[doc = "Sequencer read control 0"]
pub mod seq_read_ctl_0;
#[doc = "SEQ_READ_CTL_1 (rw) register accessor: Sequencer read control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_read_ctl_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_read_ctl_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_read_ctl_1`]
module"]
#[doc(alias = "SEQ_READ_CTL_1")]
pub type SeqReadCtl1 = crate::Reg<seq_read_ctl_1::SeqReadCtl1Spec>;
#[doc = "Sequencer read control 1"]
pub mod seq_read_ctl_1;
#[doc = "SEQ_READ_CTL_2 (rw) register accessor: Sequencer read control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_read_ctl_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_read_ctl_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_read_ctl_2`]
module"]
#[doc(alias = "SEQ_READ_CTL_2")]
pub type SeqReadCtl2 = crate::Reg<seq_read_ctl_2::SeqReadCtl2Spec>;
#[doc = "Sequencer read control 2"]
pub mod seq_read_ctl_2;
#[doc = "SEQ_READ_CTL_3 (rw) register accessor: Sequencer read control 3\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_read_ctl_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_read_ctl_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_read_ctl_3`]
module"]
#[doc(alias = "SEQ_READ_CTL_3")]
pub type SeqReadCtl3 = crate::Reg<seq_read_ctl_3::SeqReadCtl3Spec>;
#[doc = "Sequencer read control 3"]
pub mod seq_read_ctl_3;
#[doc = "SEQ_READ_CTL_4 (rw) register accessor: Sequencer read control 4\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_read_ctl_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_read_ctl_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_read_ctl_4`]
module"]
#[doc(alias = "SEQ_READ_CTL_4")]
pub type SeqReadCtl4 = crate::Reg<seq_read_ctl_4::SeqReadCtl4Spec>;
#[doc = "Sequencer read control 4"]
pub mod seq_read_ctl_4;
#[doc = "SEQ_READ_CTL_5 (rw) register accessor: Sequencer read control 5\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_read_ctl_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_read_ctl_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_read_ctl_5`]
module"]
#[doc(alias = "SEQ_READ_CTL_5")]
pub type SeqReadCtl5 = crate::Reg<seq_read_ctl_5::SeqReadCtl5Spec>;
#[doc = "Sequencer read control 5"]
pub mod seq_read_ctl_5;
#[doc = "SEQ_PROGRAM_CTL_0 (rw) register accessor: Sequencer program control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_program_ctl_0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_program_ctl_0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_program_ctl_0`]
module"]
#[doc(alias = "SEQ_PROGRAM_CTL_0")]
pub type SeqProgramCtl0 = crate::Reg<seq_program_ctl_0::SeqProgramCtl0Spec>;
#[doc = "Sequencer program control 0"]
pub mod seq_program_ctl_0;
#[doc = "SEQ_PROGRAM_CTL_1 (rw) register accessor: Sequencer program control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_program_ctl_1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_program_ctl_1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_program_ctl_1`]
module"]
#[doc(alias = "SEQ_PROGRAM_CTL_1")]
pub type SeqProgramCtl1 = crate::Reg<seq_program_ctl_1::SeqProgramCtl1Spec>;
#[doc = "Sequencer program control 1"]
pub mod seq_program_ctl_1;
#[doc = "SEQ_PROGRAM_CTL_2 (rw) register accessor: Sequencer program control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_program_ctl_2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_program_ctl_2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_program_ctl_2`]
module"]
#[doc(alias = "SEQ_PROGRAM_CTL_2")]
pub type SeqProgramCtl2 = crate::Reg<seq_program_ctl_2::SeqProgramCtl2Spec>;
#[doc = "Sequencer program control 2"]
pub mod seq_program_ctl_2;
#[doc = "SEQ_PROGRAM_CTL_3 (rw) register accessor: Sequencer program control 3\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_program_ctl_3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_program_ctl_3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_program_ctl_3`]
module"]
#[doc(alias = "SEQ_PROGRAM_CTL_3")]
pub type SeqProgramCtl3 = crate::Reg<seq_program_ctl_3::SeqProgramCtl3Spec>;
#[doc = "Sequencer program control 3"]
pub mod seq_program_ctl_3;
#[doc = "SEQ_PROGRAM_CTL_4 (rw) register accessor: Sequencer program control 4\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_program_ctl_4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_program_ctl_4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_program_ctl_4`]
module"]
#[doc(alias = "SEQ_PROGRAM_CTL_4")]
pub type SeqProgramCtl4 = crate::Reg<seq_program_ctl_4::SeqProgramCtl4Spec>;
#[doc = "Sequencer program control 4"]
pub mod seq_program_ctl_4;
#[doc = "SEQ_PROGRAM_CTL_5 (rw) register accessor: Sequencer program control 5\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_program_ctl_5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_program_ctl_5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seq_program_ctl_5`]
module"]
#[doc(alias = "SEQ_PROGRAM_CTL_5")]
pub type SeqProgramCtl5 = crate::Reg<seq_program_ctl_5::SeqProgramCtl5Spec>;
#[doc = "Sequencer program control 5"]
pub mod seq_program_ctl_5;
