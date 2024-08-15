#[doc = "Register `ADMA_ID_LOW_R` reader"]
pub type R = crate::R<AdmaIdLowRSpec>;
#[doc = "Register `ADMA_ID_LOW_R` writer"]
pub type W = crate::W<AdmaIdLowRSpec>;
#[doc = "Field `ADMA_ID_LOW` reader - ADMA Integrated Descriptor Address These bits indicate the lower 32-bit of the ADMA Integrated Descriptor address. The start address of Integrated Descriptor is set to these register bits. The ADMA3 fetches one Descriptor Address and increments these bits to indicate the next Descriptor address."]
pub type AdmaIdLowR = crate::FieldReader<u32>;
#[doc = "Field `ADMA_ID_LOW` writer - ADMA Integrated Descriptor Address These bits indicate the lower 32-bit of the ADMA Integrated Descriptor address. The start address of Integrated Descriptor is set to these register bits. The ADMA3 fetches one Descriptor Address and increments these bits to indicate the next Descriptor address."]
pub type AdmaIdLowW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ADMA Integrated Descriptor Address These bits indicate the lower 32-bit of the ADMA Integrated Descriptor address. The start address of Integrated Descriptor is set to these register bits. The ADMA3 fetches one Descriptor Address and increments these bits to indicate the next Descriptor address."]
    #[inline(always)]
    pub fn adma_id_low(&self) -> AdmaIdLowR {
        AdmaIdLowR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ADMA Integrated Descriptor Address These bits indicate the lower 32-bit of the ADMA Integrated Descriptor address. The start address of Integrated Descriptor is set to these register bits. The ADMA3 fetches one Descriptor Address and increments these bits to indicate the next Descriptor address."]
    #[inline(always)]
    #[must_use]
    pub fn adma_id_low(&mut self) -> AdmaIdLowW<AdmaIdLowRSpec> {
        AdmaIdLowW::new(self, 0)
    }
}
#[doc = "ADMA3 Integrated Descriptor Address Register - Low\n\nYou can [`read`](crate::Reg::read) this register and get [`adma_id_low_r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adma_id_low_r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdmaIdLowRSpec;
impl crate::RegisterSpec for AdmaIdLowRSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adma_id_low_r::R`](R) reader structure"]
impl crate::Readable for AdmaIdLowRSpec {}
#[doc = "`write(|w| ..)` method takes [`adma_id_low_r::W`](W) writer structure"]
impl crate::Writable for AdmaIdLowRSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADMA_ID_LOW_R to value 0"]
impl crate::Resettable for AdmaIdLowRSpec {
    const RESET_VALUE: u32 = 0;
}
