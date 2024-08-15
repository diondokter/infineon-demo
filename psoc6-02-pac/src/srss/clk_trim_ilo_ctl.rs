#[doc = "Register `CLK_TRIM_ILO_CTL` reader"]
pub type R = crate::R<ClkTrimIloCtlSpec>;
#[doc = "Register `CLK_TRIM_ILO_CTL` writer"]
pub type W = crate::W<ClkTrimIloCtlSpec>;
#[doc = "Field `ILO_FTRIM` reader - ILO frequency trims. LSB step size is 1.5 percent (typical) of the frequency."]
pub type IloFtrimR = crate::FieldReader;
#[doc = "Field `ILO_FTRIM` writer - ILO frequency trims. LSB step size is 1.5 percent (typical) of the frequency."]
pub type IloFtrimW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - ILO frequency trims. LSB step size is 1.5 percent (typical) of the frequency."]
    #[inline(always)]
    pub fn ilo_ftrim(&self) -> IloFtrimR {
        IloFtrimR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - ILO frequency trims. LSB step size is 1.5 percent (typical) of the frequency."]
    #[inline(always)]
    #[must_use]
    pub fn ilo_ftrim(&mut self) -> IloFtrimW<ClkTrimIloCtlSpec> {
        IloFtrimW::new(self, 0)
    }
}
#[doc = "ILO Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clk_trim_ilo_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clk_trim_ilo_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkTrimIloCtlSpec;
impl crate::RegisterSpec for ClkTrimIloCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_trim_ilo_ctl::R`](R) reader structure"]
impl crate::Readable for ClkTrimIloCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_trim_ilo_ctl::W`](W) writer structure"]
impl crate::Writable for ClkTrimIloCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_TRIM_ILO_CTL to value 0x2c"]
impl crate::Resettable for ClkTrimIloCtlSpec {
    const RESET_VALUE: u32 = 0x2c;
}
