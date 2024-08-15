#[doc = "Register `FM_PL_WRDATA_ALL` reader"]
pub type R = crate::R<FmPlWrdataAllSpec>;
#[doc = "Register `FM_PL_WRDATA_ALL` writer"]
pub type W = crate::W<FmPlWrdataAllSpec>;
#[doc = "Field `DATA32` reader - Write all high Voltage page latches with the same 32-bit data in a single write cycle Read always returns 0."]
pub type Data32R = crate::FieldReader<u32>;
#[doc = "Field `DATA32` writer - Write all high Voltage page latches with the same 32-bit data in a single write cycle Read always returns 0."]
pub type Data32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write all high Voltage page latches with the same 32-bit data in a single write cycle Read always returns 0."]
    #[inline(always)]
    pub fn data32(&self) -> Data32R {
        Data32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write all high Voltage page latches with the same 32-bit data in a single write cycle Read always returns 0."]
    #[inline(always)]
    #[must_use]
    pub fn data32(&mut self) -> Data32W<FmPlWrdataAllSpec> {
        Data32W::new(self, 0)
    }
}
#[doc = "Flash macro write page latches all\n\nYou can [`read`](crate::Reg::read) this register and get [`fm_pl_wrdata_all::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fm_pl_wrdata_all::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmPlWrdataAllSpec;
impl crate::RegisterSpec for FmPlWrdataAllSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fm_pl_wrdata_all::R`](R) reader structure"]
impl crate::Readable for FmPlWrdataAllSpec {}
#[doc = "`write(|w| ..)` method takes [`fm_pl_wrdata_all::W`](W) writer structure"]
impl crate::Writable for FmPlWrdataAllSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FM_PL_WRDATA_ALL to value 0"]
impl crate::Resettable for FmPlWrdataAllSpec {
    const RESET_VALUE: u32 = 0;
}
