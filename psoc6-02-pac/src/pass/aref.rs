#[repr(C)]
#[doc = "AREF configuration"]
#[doc(alias = "AREF")]
pub struct Aref {
    aref_ctrl: ArefCtrl,
}
impl Aref {
    #[doc = "0x00 - global AREF control"]
    #[inline(always)]
    pub const fn aref_ctrl(&self) -> &ArefCtrl {
        &self.aref_ctrl
    }
}
#[doc = "AREF_CTRL (rw) register accessor: global AREF control\n\nYou can [`read`](crate::Reg::read) this register and get [`aref_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aref_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aref_ctrl`]
module"]
#[doc(alias = "AREF_CTRL")]
pub type ArefCtrl = crate::Reg<aref_ctrl::ArefCtrlSpec>;
#[doc = "global AREF control"]
pub mod aref_ctrl;
