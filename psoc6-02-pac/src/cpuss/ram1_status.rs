#[doc = "Register `RAM1_STATUS` reader"]
pub type R = crate::R<Ram1StatusSpec>;
#[doc = "Field `WB_EMPTY` reader - See RAM0_STATUS."]
pub type WbEmptyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - See RAM0_STATUS."]
    #[inline(always)]
    pub fn wb_empty(&self) -> WbEmptyR {
        WbEmptyR::new((self.bits & 1) != 0)
    }
}
#[doc = "RAM 1 status\n\nYou can [`read`](crate::Reg::read) this register and get [`ram1_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ram1StatusSpec;
impl crate::RegisterSpec for Ram1StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram1_status::R`](R) reader structure"]
impl crate::Readable for Ram1StatusSpec {}
#[doc = "`reset()` method sets RAM1_STATUS to value 0x01"]
impl crate::Resettable for Ram1StatusSpec {
    const RESET_VALUE: u32 = 0x01;
}
