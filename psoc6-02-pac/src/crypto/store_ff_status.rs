#[doc = "Register `STORE_FF_STATUS` reader"]
pub type R = crate::R<StoreFfStatusSpec>;
#[doc = "Field `USED5` reader - Number of Bytes in the FIFO. The value of this field is in the range \\[0, 16\\]."]
pub type Used5R = crate::FieldReader;
#[doc = "Field `BUSY` reader - Reflects the state of the FIFO: '0': FIFO store engine is idle and a new FIFO instruction can be accepted (USED is '0'). '1': FIFO store engine is busy and NO new FIFO instruction can be accepted."]
pub type BusyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - Number of Bytes in the FIFO. The value of this field is in the range \\[0, 16\\]."]
    #[inline(always)]
    pub fn used5(&self) -> Used5R {
        Used5R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Reflects the state of the FIFO: '0': FIFO store engine is idle and a new FIFO instruction can be accepted (USED is '0'). '1': FIFO store engine is busy and NO new FIFO instruction can be accepted."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Store FIFO status\n\nYou can [`read`](crate::Reg::read) this register and get [`store_ff_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StoreFfStatusSpec;
impl crate::RegisterSpec for StoreFfStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`store_ff_status::R`](R) reader structure"]
impl crate::Readable for StoreFfStatusSpec {}
#[doc = "`reset()` method sets STORE_FF_STATUS to value 0"]
impl crate::Resettable for StoreFfStatusSpec {
    const RESET_VALUE: u32 = 0;
}
