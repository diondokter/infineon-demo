#[doc = "Register `CLK_TRIM_CCO_CTL2` reader"]
pub type R = crate::R<ClkTrimCcoCtl2Spec>;
#[doc = "Register `CLK_TRIM_CCO_CTL2` writer"]
pub type W = crate::W<ClkTrimCcoCtl2Spec>;
#[doc = "Field `CCO_FCTRIM1` reader - CCO frequency 1st range calibration"]
pub type CcoFctrim1R = crate::FieldReader;
#[doc = "Field `CCO_FCTRIM1` writer - CCO frequency 1st range calibration"]
pub type CcoFctrim1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CCO_FCTRIM2` reader - CCO frequency 2nd range calibration"]
pub type CcoFctrim2R = crate::FieldReader;
#[doc = "Field `CCO_FCTRIM2` writer - CCO frequency 2nd range calibration"]
pub type CcoFctrim2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CCO_FCTRIM3` reader - CCO frequency 3rd range calibration"]
pub type CcoFctrim3R = crate::FieldReader;
#[doc = "Field `CCO_FCTRIM3` writer - CCO frequency 3rd range calibration"]
pub type CcoFctrim3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CCO_FCTRIM4` reader - CCO frequency 4th range calibration"]
pub type CcoFctrim4R = crate::FieldReader;
#[doc = "Field `CCO_FCTRIM4` writer - CCO frequency 4th range calibration"]
pub type CcoFctrim4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CCO_FCTRIM5` reader - CCO frequency 5th range calibration"]
pub type CcoFctrim5R = crate::FieldReader;
#[doc = "Field `CCO_FCTRIM5` writer - CCO frequency 5th range calibration"]
pub type CcoFctrim5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - CCO frequency 1st range calibration"]
    #[inline(always)]
    pub fn cco_fctrim1(&self) -> CcoFctrim1R {
        CcoFctrim1R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - CCO frequency 2nd range calibration"]
    #[inline(always)]
    pub fn cco_fctrim2(&self) -> CcoFctrim2R {
        CcoFctrim2R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - CCO frequency 3rd range calibration"]
    #[inline(always)]
    pub fn cco_fctrim3(&self) -> CcoFctrim3R {
        CcoFctrim3R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - CCO frequency 4th range calibration"]
    #[inline(always)]
    pub fn cco_fctrim4(&self) -> CcoFctrim4R {
        CcoFctrim4R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - CCO frequency 5th range calibration"]
    #[inline(always)]
    pub fn cco_fctrim5(&self) -> CcoFctrim5R {
        CcoFctrim5R::new(((self.bits >> 20) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - CCO frequency 1st range calibration"]
    #[inline(always)]
    #[must_use]
    pub fn cco_fctrim1(&mut self) -> CcoFctrim1W<ClkTrimCcoCtl2Spec> {
        CcoFctrim1W::new(self, 0)
    }
    #[doc = "Bits 5:9 - CCO frequency 2nd range calibration"]
    #[inline(always)]
    #[must_use]
    pub fn cco_fctrim2(&mut self) -> CcoFctrim2W<ClkTrimCcoCtl2Spec> {
        CcoFctrim2W::new(self, 5)
    }
    #[doc = "Bits 10:14 - CCO frequency 3rd range calibration"]
    #[inline(always)]
    #[must_use]
    pub fn cco_fctrim3(&mut self) -> CcoFctrim3W<ClkTrimCcoCtl2Spec> {
        CcoFctrim3W::new(self, 10)
    }
    #[doc = "Bits 15:19 - CCO frequency 4th range calibration"]
    #[inline(always)]
    #[must_use]
    pub fn cco_fctrim4(&mut self) -> CcoFctrim4W<ClkTrimCcoCtl2Spec> {
        CcoFctrim4W::new(self, 15)
    }
    #[doc = "Bits 20:24 - CCO frequency 5th range calibration"]
    #[inline(always)]
    #[must_use]
    pub fn cco_fctrim5(&mut self) -> CcoFctrim5W<ClkTrimCcoCtl2Spec> {
        CcoFctrim5W::new(self, 20)
    }
}
#[doc = "CCO Trim Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_trim_cco_ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_trim_cco_ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkTrimCcoCtl2Spec;
impl crate::RegisterSpec for ClkTrimCcoCtl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_trim_cco_ctl2::R`](R) reader structure"]
impl crate::Readable for ClkTrimCcoCtl2Spec {}
#[doc = "`write(|w| ..)` method takes [`clk_trim_cco_ctl2::W`](W) writer structure"]
impl crate::Writable for ClkTrimCcoCtl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_TRIM_CCO_CTL2 to value 0x0088_4110"]
impl crate::Resettable for ClkTrimCcoCtl2Spec {
    const RESET_VALUE: u32 = 0x0088_4110;
}
