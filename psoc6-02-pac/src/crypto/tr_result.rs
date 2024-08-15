#[doc = "Register `TR_RESULT` reader"]
pub type R = crate::R<TrResultSpec>;
#[doc = "Register `TR_RESULT` writer"]
pub type W = crate::W<TrResultSpec>;
#[doc = "Field `DATA32` reader - Generated true random number. HW generates the number in the least significant bit positions (TR_CTL2.SIZE) of this field. The TR_DATA_AVAILABLE interrupt cause is activated when the number is generated. Note that SW can write this field. This functionality can be used prevent information leakage."]
pub type Data32R = crate::FieldReader<u32>;
#[doc = "Field `DATA32` writer - Generated true random number. HW generates the number in the least significant bit positions (TR_CTL2.SIZE) of this field. The TR_DATA_AVAILABLE interrupt cause is activated when the number is generated. Note that SW can write this field. This functionality can be used prevent information leakage."]
pub type Data32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Generated true random number. HW generates the number in the least significant bit positions (TR_CTL2.SIZE) of this field. The TR_DATA_AVAILABLE interrupt cause is activated when the number is generated. Note that SW can write this field. This functionality can be used prevent information leakage."]
    #[inline(always)]
    pub fn data32(&self) -> Data32R {
        Data32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Generated true random number. HW generates the number in the least significant bit positions (TR_CTL2.SIZE) of this field. The TR_DATA_AVAILABLE interrupt cause is activated when the number is generated. Note that SW can write this field. This functionality can be used prevent information leakage."]
    #[inline(always)]
    #[must_use]
    pub fn data32(&mut self) -> Data32W<TrResultSpec> {
        Data32W::new(self, 0)
    }
}
#[doc = "True random result\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_result::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_result::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrResultSpec;
impl crate::RegisterSpec for TrResultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr_result::R`](R) reader structure"]
impl crate::Readable for TrResultSpec {}
#[doc = "`write(|w| ..)` method takes [`tr_result::W`](W) writer structure"]
impl crate::Writable for TrResultSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TR_RESULT to value 0"]
impl crate::Resettable for TrResultSpec {
    const RESET_VALUE: u32 = 0;
}
