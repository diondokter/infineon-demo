#[doc = "Register `TOUT_CTRL_R` reader"]
pub type R = crate::R<ToutCtrlRSpec>;
#[doc = "Register `TOUT_CTRL_R` writer"]
pub type W = crate::W<ToutCtrlRSpec>;
#[doc = "Field `TOUT_CNT` reader - N/A"]
pub type ToutCntR = crate::FieldReader;
#[doc = "Field `TOUT_CNT` writer - N/A"]
pub type ToutCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    pub fn tout_cnt(&self) -> ToutCntR {
        ToutCntR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn tout_cnt(&mut self) -> ToutCntW<ToutCtrlRSpec> {
        ToutCntW::new(self, 0)
    }
}
#[doc = "Timeout Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tout_ctrl_r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tout_ctrl_r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ToutCtrlRSpec;
impl crate::RegisterSpec for ToutCtrlRSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tout_ctrl_r::R`](R) reader structure"]
impl crate::Readable for ToutCtrlRSpec {}
#[doc = "`write(|w| ..)` method takes [`tout_ctrl_r::W`](W) writer structure"]
impl crate::Writable for ToutCtrlRSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets TOUT_CTRL_R to value 0"]
impl crate::Resettable for ToutCtrlRSpec {
    const RESET_VALUE: u8 = 0;
}
