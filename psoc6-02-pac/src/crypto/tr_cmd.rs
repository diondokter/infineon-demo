#[doc = "Register `TR_CMD` reader"]
pub type R = crate::R<TrCmdSpec>;
#[doc = "Register `TR_CMD` writer"]
pub type W = crate::W<TrCmdSpec>;
#[doc = "Field `START` reader - True random command. On completion of the command, HW sets this field to '0' and sets INTR.TR_DATA_AVAILABLE to '1 when: - A random number is generated in TR_RESULT. - All ring oscillators are off (per TR_CTL1). - A repetition count (RC) or adaptive proportion (AP) error is detected during the random number generation (INTR.TR_RC/AP_DETECT_ERROR). Note: On completion of the command, SW should check TR_CTL1 and INTR.TR_RC/AP_DETECT_ERROR to ensure that no unexpected error occurred during random number generation."]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - True random command. On completion of the command, HW sets this field to '0' and sets INTR.TR_DATA_AVAILABLE to '1 when: - A random number is generated in TR_RESULT. - All ring oscillators are off (per TR_CTL1). - A repetition count (RC) or adaptive proportion (AP) error is detected during the random number generation (INTR.TR_RC/AP_DETECT_ERROR). Note: On completion of the command, SW should check TR_CTL1 and INTR.TR_RC/AP_DETECT_ERROR to ensure that no unexpected error occurred during random number generation."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - True random command. On completion of the command, HW sets this field to '0' and sets INTR.TR_DATA_AVAILABLE to '1 when: - A random number is generated in TR_RESULT. - All ring oscillators are off (per TR_CTL1). - A repetition count (RC) or adaptive proportion (AP) error is detected during the random number generation (INTR.TR_RC/AP_DETECT_ERROR). Note: On completion of the command, SW should check TR_CTL1 and INTR.TR_RC/AP_DETECT_ERROR to ensure that no unexpected error occurred during random number generation."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - True random command. On completion of the command, HW sets this field to '0' and sets INTR.TR_DATA_AVAILABLE to '1 when: - A random number is generated in TR_RESULT. - All ring oscillators are off (per TR_CTL1). - A repetition count (RC) or adaptive proportion (AP) error is detected during the random number generation (INTR.TR_RC/AP_DETECT_ERROR). Note: On completion of the command, SW should check TR_CTL1 and INTR.TR_RC/AP_DETECT_ERROR to ensure that no unexpected error occurred during random number generation."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<TrCmdSpec> {
        StartW::new(self, 0)
    }
}
#[doc = "True random command\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrCmdSpec;
impl crate::RegisterSpec for TrCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr_cmd::R`](R) reader structure"]
impl crate::Readable for TrCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`tr_cmd::W`](W) writer structure"]
impl crate::Writable for TrCmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TR_CMD to value 0"]
impl crate::Resettable for TrCmdSpec {
    const RESET_VALUE: u32 = 0;
}
