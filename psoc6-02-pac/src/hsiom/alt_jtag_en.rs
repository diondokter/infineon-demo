#[doc = "Register `ALT_JTAG_EN` reader"]
pub type R = crate::R<AltJtagEnSpec>;
#[doc = "Register `ALT_JTAG_EN` writer"]
pub type W = crate::W<AltJtagEnSpec>;
#[doc = "Field `ENABLE` reader - Provides the selection for alternate JTAG IF connectivity. 0: Primary JTAG interface is selected 1: Secondary (alternate) JTAG interface is selected. This connectivity works ONLY in ACTIVE mode."]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Provides the selection for alternate JTAG IF connectivity. 0: Primary JTAG interface is selected 1: Secondary (alternate) JTAG interface is selected. This connectivity works ONLY in ACTIVE mode."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - Provides the selection for alternate JTAG IF connectivity. 0: Primary JTAG interface is selected 1: Secondary (alternate) JTAG interface is selected. This connectivity works ONLY in ACTIVE mode."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Provides the selection for alternate JTAG IF connectivity. 0: Primary JTAG interface is selected 1: Secondary (alternate) JTAG interface is selected. This connectivity works ONLY in ACTIVE mode."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<AltJtagEnSpec> {
        EnableW::new(self, 31)
    }
}
#[doc = "Alternate JTAG IF selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`alt_jtag_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alt_jtag_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AltJtagEnSpec;
impl crate::RegisterSpec for AltJtagEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alt_jtag_en::R`](R) reader structure"]
impl crate::Readable for AltJtagEnSpec {}
#[doc = "`write(|w| ..)` method takes [`alt_jtag_en::W`](W) writer structure"]
impl crate::Writable for AltJtagEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALT_JTAG_EN to value 0"]
impl crate::Resettable for AltJtagEnSpec {
    const RESET_VALUE: u32 = 0;
}
