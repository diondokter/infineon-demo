#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `TR_EN` reader - Trigger output enable: '0': Disabled. The trigger output 'tr_fault' is '0'. '1': Enabled. The trigger output 'tr_fault' reflects STATUS.VALID. The trigger can be used to initiate a Datawire transfer of the FAULT data (FAULT_DATA0 through FAULT_DATA3)."]
pub type TrEnR = crate::BitReader;
#[doc = "Field `TR_EN` writer - Trigger output enable: '0': Disabled. The trigger output 'tr_fault' is '0'. '1': Enabled. The trigger output 'tr_fault' reflects STATUS.VALID. The trigger can be used to initiate a Datawire transfer of the FAULT data (FAULT_DATA0 through FAULT_DATA3)."]
pub type TrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EN` reader - IO output signal enable: '0': Disabled. The IO output signal 'fault_out' is '0'. The IO output enable signal 'fault_out_en' is '0'. '1': Enabled. The IO output signal 'fault_out' reflects STATUS.VALID. The IO output enable signal 'fault_out_en' is '1'."]
pub type OutEnR = crate::BitReader;
#[doc = "Field `OUT_EN` writer - IO output signal enable: '0': Disabled. The IO output signal 'fault_out' is '0'. The IO output enable signal 'fault_out_en' is '0'. '1': Enabled. The IO output signal 'fault_out' reflects STATUS.VALID. The IO output enable signal 'fault_out_en' is '1'."]
pub type OutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_REQ_EN` reader - Reset request enable: '0': Disabled. '1': Enabled. The output reset request signal 'fault_reset_req' reflects STATUS.VALID. This reset causes a warm/soft/core reset. This warm/soft/core reset does not affect the fault logic STATUS, DATA0, ..., DATA3 registers (allowing for post soft reset failure analysis). The 'fault_reset_req' signals of the individual fault report structures are combined (logically OR'd) into a single SRSS 'fault_reset_req' signal."]
pub type ResetReqEnR = crate::BitReader;
#[doc = "Field `RESET_REQ_EN` writer - Reset request enable: '0': Disabled. '1': Enabled. The output reset request signal 'fault_reset_req' reflects STATUS.VALID. This reset causes a warm/soft/core reset. This warm/soft/core reset does not affect the fault logic STATUS, DATA0, ..., DATA3 registers (allowing for post soft reset failure analysis). The 'fault_reset_req' signals of the individual fault report structures are combined (logically OR'd) into a single SRSS 'fault_reset_req' signal."]
pub type ResetReqEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Trigger output enable: '0': Disabled. The trigger output 'tr_fault' is '0'. '1': Enabled. The trigger output 'tr_fault' reflects STATUS.VALID. The trigger can be used to initiate a Datawire transfer of the FAULT data (FAULT_DATA0 through FAULT_DATA3)."]
    #[inline(always)]
    pub fn tr_en(&self) -> TrEnR {
        TrEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO output signal enable: '0': Disabled. The IO output signal 'fault_out' is '0'. The IO output enable signal 'fault_out_en' is '0'. '1': Enabled. The IO output signal 'fault_out' reflects STATUS.VALID. The IO output enable signal 'fault_out_en' is '1'."]
    #[inline(always)]
    pub fn out_en(&self) -> OutEnR {
        OutEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset request enable: '0': Disabled. '1': Enabled. The output reset request signal 'fault_reset_req' reflects STATUS.VALID. This reset causes a warm/soft/core reset. This warm/soft/core reset does not affect the fault logic STATUS, DATA0, ..., DATA3 registers (allowing for post soft reset failure analysis). The 'fault_reset_req' signals of the individual fault report structures are combined (logically OR'd) into a single SRSS 'fault_reset_req' signal."]
    #[inline(always)]
    pub fn reset_req_en(&self) -> ResetReqEnR {
        ResetReqEnR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trigger output enable: '0': Disabled. The trigger output 'tr_fault' is '0'. '1': Enabled. The trigger output 'tr_fault' reflects STATUS.VALID. The trigger can be used to initiate a Datawire transfer of the FAULT data (FAULT_DATA0 through FAULT_DATA3)."]
    #[inline(always)]
    #[must_use]
    pub fn tr_en(&mut self) -> TrEnW<CtlSpec> {
        TrEnW::new(self, 0)
    }
    #[doc = "Bit 1 - IO output signal enable: '0': Disabled. The IO output signal 'fault_out' is '0'. The IO output enable signal 'fault_out_en' is '0'. '1': Enabled. The IO output signal 'fault_out' reflects STATUS.VALID. The IO output enable signal 'fault_out_en' is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn out_en(&mut self) -> OutEnW<CtlSpec> {
        OutEnW::new(self, 1)
    }
    #[doc = "Bit 2 - Reset request enable: '0': Disabled. '1': Enabled. The output reset request signal 'fault_reset_req' reflects STATUS.VALID. This reset causes a warm/soft/core reset. This warm/soft/core reset does not affect the fault logic STATUS, DATA0, ..., DATA3 registers (allowing for post soft reset failure analysis). The 'fault_reset_req' signals of the individual fault report structures are combined (logically OR'd) into a single SRSS 'fault_reset_req' signal."]
    #[inline(always)]
    #[must_use]
    pub fn reset_req_en(&mut self) -> ResetReqEnW<CtlSpec> {
        ResetReqEnW::new(self, 2)
    }
}
#[doc = "Fault control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0;
}
