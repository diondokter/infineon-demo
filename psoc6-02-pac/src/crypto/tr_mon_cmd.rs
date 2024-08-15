#[doc = "Register `TR_MON_CMD` reader"]
pub type R = crate::R<TrMonCmdSpec>;
#[doc = "Register `TR_MON_CMD` writer"]
pub type W = crate::W<TrMonCmdSpec>;
#[doc = "Field `START_AP` reader - Adaptive proportion (AP) test enable: '0': Stopped. '1': Started. On a AP detection, HW sets this field to '0' and sets INTR.TR_AP_DETECT to '1."]
pub type StartApR = crate::BitReader;
#[doc = "Field `START_AP` writer - Adaptive proportion (AP) test enable: '0': Stopped. '1': Started. On a AP detection, HW sets this field to '0' and sets INTR.TR_AP_DETECT to '1."]
pub type StartApW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START_RC` reader - Repetition count (RC) test enable: '0': Disabled. '1': Enabled. On a RC detection, HW sets this field to '0' and sets INTR.TR_RC_DETECT to '1."]
pub type StartRcR = crate::BitReader;
#[doc = "Field `START_RC` writer - Repetition count (RC) test enable: '0': Disabled. '1': Enabled. On a RC detection, HW sets this field to '0' and sets INTR.TR_RC_DETECT to '1."]
pub type StartRcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Adaptive proportion (AP) test enable: '0': Stopped. '1': Started. On a AP detection, HW sets this field to '0' and sets INTR.TR_AP_DETECT to '1."]
    #[inline(always)]
    pub fn start_ap(&self) -> StartApR {
        StartApR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Repetition count (RC) test enable: '0': Disabled. '1': Enabled. On a RC detection, HW sets this field to '0' and sets INTR.TR_RC_DETECT to '1."]
    #[inline(always)]
    pub fn start_rc(&self) -> StartRcR {
        StartRcR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Adaptive proportion (AP) test enable: '0': Stopped. '1': Started. On a AP detection, HW sets this field to '0' and sets INTR.TR_AP_DETECT to '1."]
    #[inline(always)]
    #[must_use]
    pub fn start_ap(&mut self) -> StartApW<TrMonCmdSpec> {
        StartApW::new(self, 0)
    }
    #[doc = "Bit 1 - Repetition count (RC) test enable: '0': Disabled. '1': Enabled. On a RC detection, HW sets this field to '0' and sets INTR.TR_RC_DETECT to '1."]
    #[inline(always)]
    #[must_use]
    pub fn start_rc(&mut self) -> StartRcW<TrMonCmdSpec> {
        StartRcW::new(self, 1)
    }
}
#[doc = "True random monitor command\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_mon_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_mon_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrMonCmdSpec;
impl crate::RegisterSpec for TrMonCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr_mon_cmd::R`](R) reader structure"]
impl crate::Readable for TrMonCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`tr_mon_cmd::W`](W) writer structure"]
impl crate::Writable for TrMonCmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TR_MON_CMD to value 0"]
impl crate::Resettable for TrMonCmdSpec {
    const RESET_VALUE: u32 = 0;
}
