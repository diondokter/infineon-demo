#[doc = "Register `CM0_PC_CTL` reader"]
pub type R = crate::R<Cm0PcCtlSpec>;
#[doc = "Register `CM0_PC_CTL` writer"]
pub type W = crate::W<Cm0PcCtlSpec>;
#[doc = "Field `VALID` reader - Valid fields for the protection context handler CM0_PCi_HANDLER registers: Bit 0: Valid field for CM0_PC0_HANDLER. Bit 1: Valid field for CM0_PC1_HANDLER. Bit 2: Valid field for CM0_PC2_HANDLER. Bit 3: Valid field for CM0_PC3_HANDLER."]
pub type ValidR = crate::FieldReader;
#[doc = "Field `VALID` writer - Valid fields for the protection context handler CM0_PCi_HANDLER registers: Bit 0: Valid field for CM0_PC0_HANDLER. Bit 1: Valid field for CM0_PC1_HANDLER. Bit 2: Valid field for CM0_PC2_HANDLER. Bit 3: Valid field for CM0_PC3_HANDLER."]
pub type ValidW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Valid fields for the protection context handler CM0_PCi_HANDLER registers: Bit 0: Valid field for CM0_PC0_HANDLER. Bit 1: Valid field for CM0_PC1_HANDLER. Bit 2: Valid field for CM0_PC2_HANDLER. Bit 3: Valid field for CM0_PC3_HANDLER."]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Valid fields for the protection context handler CM0_PCi_HANDLER registers: Bit 0: Valid field for CM0_PC0_HANDLER. Bit 1: Valid field for CM0_PC1_HANDLER. Bit 2: Valid field for CM0_PC2_HANDLER. Bit 3: Valid field for CM0_PC3_HANDLER."]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> ValidW<Cm0PcCtlSpec> {
        ValidW::new(self, 0)
    }
}
#[doc = "CM0+ protection context control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_pc_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_pc_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm0PcCtlSpec;
impl crate::RegisterSpec for Cm0PcCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm0_pc_ctl::R`](R) reader structure"]
impl crate::Readable for Cm0PcCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`cm0_pc_ctl::W`](W) writer structure"]
impl crate::Writable for Cm0PcCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CM0_PC_CTL to value 0"]
impl crate::Resettable for Cm0PcCtlSpec {
    const RESET_VALUE: u32 = 0;
}
