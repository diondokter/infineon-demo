#[doc = "Register `CLK_TRIM_PILO_CTL2` reader"]
pub type R = crate::R<ClkTrimPiloCtl2Spec>;
#[doc = "Register `CLK_TRIM_PILO_CTL2` writer"]
pub type W = crate::W<ClkTrimPiloCtl2Spec>;
#[doc = "Field `PILO_VREF_TRIM` reader - Trim for voltage reference"]
pub type PiloVrefTrimR = crate::FieldReader;
#[doc = "Field `PILO_VREF_TRIM` writer - Trim for voltage reference"]
pub type PiloVrefTrimW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PILO_IREFBM_TRIM` reader - Trim for beta-multiplier current reference"]
pub type PiloIrefbmTrimR = crate::FieldReader;
#[doc = "Field `PILO_IREFBM_TRIM` writer - Trim for beta-multiplier current reference"]
pub type PiloIrefbmTrimW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PILO_IREF_TRIM` reader - Trim for current reference"]
pub type PiloIrefTrimR = crate::FieldReader;
#[doc = "Field `PILO_IREF_TRIM` writer - Trim for current reference"]
pub type PiloIrefTrimW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Trim for voltage reference"]
    #[inline(always)]
    pub fn pilo_vref_trim(&self) -> PiloVrefTrimR {
        PiloVrefTrimR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - Trim for beta-multiplier current reference"]
    #[inline(always)]
    pub fn pilo_irefbm_trim(&self) -> PiloIrefbmTrimR {
        PiloIrefbmTrimR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - Trim for current reference"]
    #[inline(always)]
    pub fn pilo_iref_trim(&self) -> PiloIrefTrimR {
        PiloIrefTrimR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Trim for voltage reference"]
    #[inline(always)]
    #[must_use]
    pub fn pilo_vref_trim(&mut self) -> PiloVrefTrimW<ClkTrimPiloCtl2Spec> {
        PiloVrefTrimW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Trim for beta-multiplier current reference"]
    #[inline(always)]
    #[must_use]
    pub fn pilo_irefbm_trim(&mut self) -> PiloIrefbmTrimW<ClkTrimPiloCtl2Spec> {
        PiloIrefbmTrimW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Trim for current reference"]
    #[inline(always)]
    #[must_use]
    pub fn pilo_iref_trim(&mut self) -> PiloIrefTrimW<ClkTrimPiloCtl2Spec> {
        PiloIrefTrimW::new(self, 16)
    }
}
#[doc = "PILO Trim Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_trim_pilo_ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_trim_pilo_ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkTrimPiloCtl2Spec;
impl crate::RegisterSpec for ClkTrimPiloCtl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_trim_pilo_ctl2::R`](R) reader structure"]
impl crate::Readable for ClkTrimPiloCtl2Spec {}
#[doc = "`write(|w| ..)` method takes [`clk_trim_pilo_ctl2::W`](W) writer structure"]
impl crate::Writable for ClkTrimPiloCtl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_TRIM_PILO_CTL2 to value 0x00da_10e0"]
impl crate::Resettable for ClkTrimPiloCtl2Spec {
    const RESET_VALUE: u32 = 0x00da_10e0;
}
