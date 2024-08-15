#[doc = "Register `ADMA_SA_LOW_R` reader"]
pub type R = crate::R<AdmaSaLowRSpec>;
#[doc = "Register `ADMA_SA_LOW_R` writer"]
pub type W = crate::W<AdmaSaLowRSpec>;
#[doc = "Field `ADMA_SA_LOW` reader - ADMA System Address These bits indicate the lower 32 bits of the ADMA system address. - SDMA: If Host Version 4 Enable is set to 1, this register stores the system address of the data location - ADMA2: This register stores the byte address of the executing command of the descriptor table - ADMA3: This register is set by ADMA3. ADMA2 increments the address of this register that points to the next line, every time a Descriptor line is fetched."]
pub type AdmaSaLowR = crate::FieldReader<u32>;
#[doc = "Field `ADMA_SA_LOW` writer - ADMA System Address These bits indicate the lower 32 bits of the ADMA system address. - SDMA: If Host Version 4 Enable is set to 1, this register stores the system address of the data location - ADMA2: This register stores the byte address of the executing command of the descriptor table - ADMA3: This register is set by ADMA3. ADMA2 increments the address of this register that points to the next line, every time a Descriptor line is fetched."]
pub type AdmaSaLowW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - ADMA System Address These bits indicate the lower 32 bits of the ADMA system address. - SDMA: If Host Version 4 Enable is set to 1, this register stores the system address of the data location - ADMA2: This register stores the byte address of the executing command of the descriptor table - ADMA3: This register is set by ADMA3. ADMA2 increments the address of this register that points to the next line, every time a Descriptor line is fetched."]
    #[inline(always)]
    pub fn adma_sa_low(&self) -> AdmaSaLowR {
        AdmaSaLowR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - ADMA System Address These bits indicate the lower 32 bits of the ADMA system address. - SDMA: If Host Version 4 Enable is set to 1, this register stores the system address of the data location - ADMA2: This register stores the byte address of the executing command of the descriptor table - ADMA3: This register is set by ADMA3. ADMA2 increments the address of this register that points to the next line, every time a Descriptor line is fetched."]
    #[inline(always)]
    #[must_use]
    pub fn adma_sa_low(&mut self) -> AdmaSaLowW<AdmaSaLowRSpec> {
        AdmaSaLowW::new(self, 0)
    }
}
#[doc = "ADMA System Address Register - Low\n\nYou can [`read`](crate::Reg::read) this register and get [`adma_sa_low_r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adma_sa_low_r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdmaSaLowRSpec;
impl crate::RegisterSpec for AdmaSaLowRSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adma_sa_low_r::R`](R) reader structure"]
impl crate::Readable for AdmaSaLowRSpec {}
#[doc = "`write(|w| ..)` method takes [`adma_sa_low_r::W`](W) writer structure"]
impl crate::Writable for AdmaSaLowRSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADMA_SA_LOW_R to value 0"]
impl crate::Resettable for AdmaSaLowRSpec {
    const RESET_VALUE: u32 = 0;
}
