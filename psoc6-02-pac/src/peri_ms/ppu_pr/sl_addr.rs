#[doc = "Register `SL_ADDR` reader"]
pub type R = crate::R<SlAddrSpec>;
#[doc = "Register `SL_ADDR` writer"]
pub type W = crate::W<SlAddrSpec>;
#[doc = "Field `ADDR30` reader - This field specifies the base address of the slave region. The region size is defined by SL_SIZE.REGION_SIZE. A region of n Bytes must be n Byte aligned. Therefore, some of the lesser significant address bits of ADDR30 must be '0's. E.g., a 64 KB address region (REGION_SIZE is '15') must be 64 KByte aligned, and ADDR30\\[13:0\\]
must be '0's."]
pub type Addr30R = crate::FieldReader<u32>;
#[doc = "Field `ADDR30` writer - This field specifies the base address of the slave region. The region size is defined by SL_SIZE.REGION_SIZE. A region of n Bytes must be n Byte aligned. Therefore, some of the lesser significant address bits of ADDR30 must be '0's. E.g., a 64 KB address region (REGION_SIZE is '15') must be 64 KByte aligned, and ADDR30\\[13:0\\]
must be '0's."]
pub type Addr30W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - This field specifies the base address of the slave region. The region size is defined by SL_SIZE.REGION_SIZE. A region of n Bytes must be n Byte aligned. Therefore, some of the lesser significant address bits of ADDR30 must be '0's. E.g., a 64 KB address region (REGION_SIZE is '15') must be 64 KByte aligned, and ADDR30\\[13:0\\]
must be '0's."]
    #[inline(always)]
    pub fn addr30(&self) -> Addr30R {
        Addr30R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - This field specifies the base address of the slave region. The region size is defined by SL_SIZE.REGION_SIZE. A region of n Bytes must be n Byte aligned. Therefore, some of the lesser significant address bits of ADDR30 must be '0's. E.g., a 64 KB address region (REGION_SIZE is '15') must be 64 KByte aligned, and ADDR30\\[13:0\\]
must be '0's."]
    #[inline(always)]
    #[must_use]
    pub fn addr30(&mut self) -> Addr30W<SlAddrSpec> {
        Addr30W::new(self, 2)
    }
}
#[doc = "Slave region, base address\n\nYou can [`read`](crate::Reg::read) this register and get [`sl_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sl_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlAddrSpec;
impl crate::RegisterSpec for SlAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sl_addr::R`](R) reader structure"]
impl crate::Readable for SlAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`sl_addr::W`](W) writer structure"]
impl crate::Writable for SlAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SL_ADDR to value 0"]
impl crate::Resettable for SlAddrSpec {
    const RESET_VALUE: u32 = 0;
}
