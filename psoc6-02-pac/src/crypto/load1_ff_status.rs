#[doc = "Register `LOAD1_FF_STATUS` reader"]
pub type R = crate::R<Load1FfStatusSpec>;
#[doc = "Field `USED5` reader - See LOAD1_FF_STATUS.USED."]
pub type Used5R = crate::FieldReader;
#[doc = "Field `BUSY` reader - See LOAD1_FF_STATUS.BUSY."]
pub type BusyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - See LOAD1_FF_STATUS.USED."]
    #[inline(always)]
    pub fn used5(&self) -> Used5R {
        Used5R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 31 - See LOAD1_FF_STATUS.BUSY."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Load 1 FIFO status\n\nYou can [`read`](crate::Reg::read) this register and get [`load1_ff_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Load1FfStatusSpec;
impl crate::RegisterSpec for Load1FfStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`load1_ff_status::R`](R) reader structure"]
impl crate::Readable for Load1FfStatusSpec {}
#[doc = "`reset()` method sets LOAD1_FF_STATUS to value 0"]
impl crate::Resettable for Load1FfStatusSpec {
    const RESET_VALUE: u32 = 0;
}
