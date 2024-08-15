#[doc = "Register `DESCR_STATUS` reader"]
pub type R = crate::R<DescrStatusSpec>;
#[doc = "Field `VALID` reader - Indicates whether the descriptor information present in DESCR_CTL, DESCR_SRC, DESCR_DST, DESCR_X_SIZE, DESCR_X_INCR, DESCR_Y_SIZE, DESCR_Y_INCR, DESCR_NEXT status registers is valid or not."]
pub type ValidR = crate::BitReader;
impl R {
    #[doc = "Bit 31 - Indicates whether the descriptor information present in DESCR_CTL, DESCR_SRC, DESCR_DST, DESCR_X_SIZE, DESCR_X_INCR, DESCR_Y_SIZE, DESCR_Y_INCR, DESCR_NEXT status registers is valid or not."]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Channel descriptor status\n\nYou can [`read`](crate::Reg::read) this register and get [`descr_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DescrStatusSpec;
impl crate::RegisterSpec for DescrStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`descr_status::R`](R) reader structure"]
impl crate::Readable for DescrStatusSpec {}
#[doc = "`reset()` method sets DESCR_STATUS to value 0"]
impl crate::Resettable for DescrStatusSpec {
    const RESET_VALUE: u32 = 0;
}
