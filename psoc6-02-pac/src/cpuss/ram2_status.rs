#[doc = "Register `RAM2_STATUS` reader"]
pub type R = crate::R<Ram2StatusSpec>;
#[doc = "Field `WB_EMPTY` reader - See RAM0_STATUS."]
pub type WbEmptyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - See RAM0_STATUS."]
    #[inline(always)]
    pub fn wb_empty(&self) -> WbEmptyR {
        WbEmptyR::new((self.bits & 1) != 0)
    }
}
#[doc = "RAM 2 status\n\nYou can [`read`](crate::Reg::read) this register and get [`ram2_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ram2StatusSpec;
impl crate::RegisterSpec for Ram2StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram2_status::R`](R) reader structure"]
impl crate::Readable for Ram2StatusSpec {}
#[doc = "`reset()` method sets RAM2_STATUS to value 0x01"]
impl crate::Resettable for Ram2StatusSpec {
    const RESET_VALUE: u32 = 0x01;
}
