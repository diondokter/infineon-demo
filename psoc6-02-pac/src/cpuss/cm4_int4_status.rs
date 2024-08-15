#[doc = "Register `CM4_INT4_STATUS` reader"]
pub type R = crate::R<Cm4Int4StatusSpec>;
#[doc = "Field `SYSTEM_INT_IDX` reader - Lowest CM4 activated system interrupt index for CPU interrupt 4. See description of CM0_INT0_STATUS."]
pub type SystemIntIdxR = crate::FieldReader<u16>;
#[doc = "Field `SYSTEM_INT_VALID` reader - See description of CM0_INT0_STATUS."]
pub type SystemIntValidR = crate::BitReader;
impl R {
    #[doc = "Bits 0:9 - Lowest CM4 activated system interrupt index for CPU interrupt 4. See description of CM0_INT0_STATUS."]
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
#[doc = "CM4 interrupt 4 status\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_int4_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm4Int4StatusSpec;
impl crate::RegisterSpec for Cm4Int4StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm4_int4_status::R`](R) reader structure"]
impl crate::Readable for Cm4Int4StatusSpec {}
#[doc = "`reset()` method sets CM4_INT4_STATUS to value 0"]
impl crate::Resettable for Cm4Int4StatusSpec {
    const RESET_VALUE: u32 = 0;
}
