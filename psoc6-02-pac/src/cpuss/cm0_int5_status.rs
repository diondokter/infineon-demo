#[doc = "Register `CM0_INT5_STATUS` reader"]
pub type R = crate::R<Cm0Int5StatusSpec>;
#[doc = "Field `SYSTEM_INT_IDX` reader - Lowest CM0+ activated system interrupt index for CPU interrupt 5. See description of CM0_INT0_STATUS."]
pub type SystemIntIdxR = crate::FieldReader<u16>;
#[doc = "Field `SYSTEM_INT_VALID` reader - See description of CM0_INT0_STATUS."]
pub type SystemIntValidR = crate::BitReader;
impl R {
    #[doc = "Bits 0:9 - Lowest CM0+ activated system interrupt index for CPU interrupt 5. See description of CM0_INT0_STATUS."]
    #[inline(always)]
    pub fn system_int_idx(&self) -> SystemIntIdxR {
        SystemIntIdxR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 31 - See description of CM0_INT0_STATUS."]
    #[inline(always)]
    pub fn system_int_valid(&self) -> SystemIntValidR {
        SystemIntValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "CM0+ interrupt 5 status\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_int5_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm0Int5StatusSpec;
impl crate::RegisterSpec for Cm0Int5StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm0_int5_status::R`](R) reader structure"]
impl crate::Readable for Cm0Int5StatusSpec {}
#[doc = "`reset()` method sets CM0_INT5_STATUS to value 0"]
impl crate::Resettable for Cm0Int5StatusSpec {
    const RESET_VALUE: u32 = 0;
}
