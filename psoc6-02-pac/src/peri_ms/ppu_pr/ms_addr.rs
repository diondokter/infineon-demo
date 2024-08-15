#[doc = "Register `MS_ADDR` reader"]
pub type R = crate::R<MsAddrSpec>;
#[doc = "Field `ADDR26` reader - This field specifies the base address of the master region. The base address of the region is the address of the SL_ADDR register."]
pub type Addr26R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 6:31 - This field specifies the base address of the master region. The base address of the region is the address of the SL_ADDR register."]
    #[inline(always)]
    pub fn addr26(&self) -> Addr26R {
        Addr26R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
#[doc = "Master region, base address\n\nYou can [`read`](crate::Reg::read) this register and get [`ms_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsAddrSpec;
impl crate::RegisterSpec for MsAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ms_addr::R`](R) reader structure"]
impl crate::Readable for MsAddrSpec {}
#[doc = "`reset()` method sets MS_ADDR to value 0"]
impl crate::Resettable for MsAddrSpec {
    const RESET_VALUE: u32 = 0;
}
