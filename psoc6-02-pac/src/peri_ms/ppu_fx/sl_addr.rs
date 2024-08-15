#[doc = "Register `SL_ADDR` reader"]
pub type R = crate::R<SlAddrSpec>;
#[doc = "Field `ADDR30` reader - This field specifies the base address of the slave region. The region size is defined by SL_SIZE.REGION_SIZE. A region of n Bytes must be n Byte aligned. Therefore, some of the lesser significant address bits of ADDR30 must be '0's. E.g., a 64 KB address region (REGION_SIZE is '15') must be 64 KByte aligned, and ADDR30\\[13:0\\]
must be '0's."]
pub type Addr30R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 2:31 - This field specifies the base address of the slave region. The region size is defined by SL_SIZE.REGION_SIZE. A region of n Bytes must be n Byte aligned. Therefore, some of the lesser significant address bits of ADDR30 must be '0's. E.g., a 64 KB address region (REGION_SIZE is '15') must be 64 KByte aligned, and ADDR30\\[13:0\\]
must be '0's."]
    #[inline(always)]
    pub fn addr30(&self) -> Addr30R {
        Addr30R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
#[doc = "Slave region, base address\n\nYou can [`read`](crate::Reg::read) this register and get [`sl_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlAddrSpec;
impl crate::RegisterSpec for SlAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sl_addr::R`](R) reader structure"]
impl crate::Readable for SlAddrSpec {}
#[doc = "`reset()` method sets SL_ADDR to value 0"]
impl crate::Resettable for SlAddrSpec {
    const RESET_VALUE: u32 = 0;
}
