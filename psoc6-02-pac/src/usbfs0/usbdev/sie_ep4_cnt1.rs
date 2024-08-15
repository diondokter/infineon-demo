#[doc = "Register `SIE_EP4_CNT1` reader"]
pub type R = crate::R<SieEp4Cnt1Spec>;
#[doc = "Register `SIE_EP4_CNT1` writer"]
pub type W = crate::W<SieEp4Cnt1Spec>;
#[doc = "Field `DATA_COUNT` reader - These bits are the 8 LSb of a 11-bit counter. The 3 MSb bits are in the CNT0 register. The 11-bit count indicates the number of data bytes in a transaction."]
pub type DataCountR = crate::FieldReader;
#[doc = "Field `DATA_COUNT` writer - These bits are the 8 LSb of a 11-bit counter. The 3 MSb bits are in the CNT0 register. The 11-bit count indicates the number of data bytes in a transaction."]
pub type DataCountW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - These bits are the 8 LSb of a 11-bit counter. The 3 MSb bits are in the CNT0 register. The 11-bit count indicates the number of data bytes in a transaction."]
    #[inline(always)]
    pub fn data_count(&self) -> DataCountR {
        DataCountR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - These bits are the 8 LSb of a 11-bit counter. The 3 MSb bits are in the CNT0 register. The 11-bit count indicates the number of data bytes in a transaction."]
    #[inline(always)]
    #[must_use]
    pub fn data_count(&mut self) -> DataCountW<SieEp4Cnt1Spec> {
        DataCountW::new(self, 0)
    }
}
#[doc = "Non-control endpoint count register\n\nYou can [`read`](crate::Reg::read) this register and get [`sie_ep4_cnt1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sie_ep4_cnt1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SieEp4Cnt1Spec;
impl crate::RegisterSpec for SieEp4Cnt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sie_ep4_cnt1::R`](R) reader structure"]
impl crate::Readable for SieEp4Cnt1Spec {}
#[doc = "`write(|w| ..)` method takes [`sie_ep4_cnt1::W`](W) writer structure"]
impl crate::Writable for SieEp4Cnt1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIE_EP4_CNT1 to value 0"]
impl crate::Resettable for SieEp4Cnt1Spec {
    const RESET_VALUE: u32 = 0;
}
