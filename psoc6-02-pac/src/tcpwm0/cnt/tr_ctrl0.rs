#[doc = "Register `TR_CTRL0` reader"]
pub type R = crate::R<TrCtrl0Spec>;
#[doc = "Register `TR_CTRL0` writer"]
pub type W = crate::W<TrCtrl0Spec>;
#[doc = "Field `CAPTURE_SEL` reader - Selects one of the 16 input triggers as a capture trigger. Input trigger 0 is always '0' and input trigger is always '1'. In the PWM, PWM_DT and PWM_PR modes this trigger is used to switch the values if the compare and period registers with their buffer counterparts."]
pub type CaptureSelR = crate::FieldReader;
#[doc = "Field `CAPTURE_SEL` writer - Selects one of the 16 input triggers as a capture trigger. Input trigger 0 is always '0' and input trigger is always '1'. In the PWM, PWM_DT and PWM_PR modes this trigger is used to switch the values if the compare and period registers with their buffer counterparts."]
pub type CaptureSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `COUNT_SEL` reader - Selects one of the 16 input triggers as a count trigger. In QUAD mode, this is the first phase (phi A). Default setting selects input trigger 1, which is always '1'."]
pub type CountSelR = crate::FieldReader;
#[doc = "Field `COUNT_SEL` writer - Selects one of the 16 input triggers as a count trigger. In QUAD mode, this is the first phase (phi A). Default setting selects input trigger 1, which is always '1'."]
pub type CountSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RELOAD_SEL` reader - Selects one of the 16 input triggers as a reload trigger. In QUAD mode, this is the index or revolution pulse. In this mode, it will update the counter with 0x8000 (counter midpoint)."]
pub type ReloadSelR = crate::FieldReader;
#[doc = "Field `RELOAD_SEL` writer - Selects one of the 16 input triggers as a reload trigger. In QUAD mode, this is the index or revolution pulse. In this mode, it will update the counter with 0x8000 (counter midpoint)."]
pub type ReloadSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `STOP_SEL` reader - Selects one of the 16 input triggers as a stop trigger. In PWM, PWM_DT and PWM_PR modes, this is the kill trigger. In these modes, the kill trigger is used to either temporarily block the PWM outputs (PWM_STOP_ON_KILL is '0') or stop the functionality (PWM_STOP_ON_KILL is '1'). For the PWM and PWM_DT modes, the blocking of the output signals can be asynchronous (STOP_EDGE should be NO_EDGE_DET) in which case the blocking is as long as the trigger is '1' or synchronous (STOP_EDGE should be RISING_EDGE) in which case it extends till the next terminal count event."]
pub type StopSelR = crate::FieldReader;
#[doc = "Field `STOP_SEL` writer - Selects one of the 16 input triggers as a stop trigger. In PWM, PWM_DT and PWM_PR modes, this is the kill trigger. In these modes, the kill trigger is used to either temporarily block the PWM outputs (PWM_STOP_ON_KILL is '0') or stop the functionality (PWM_STOP_ON_KILL is '1'). For the PWM and PWM_DT modes, the blocking of the output signals can be asynchronous (STOP_EDGE should be NO_EDGE_DET) in which case the blocking is as long as the trigger is '1' or synchronous (STOP_EDGE should be RISING_EDGE) in which case it extends till the next terminal count event."]
pub type StopSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `START_SEL` reader - Selects one of the 16 input triggers as a start trigger. In QUAD mode, this is the second phase (phi B)."]
pub type StartSelR = crate::FieldReader;
#[doc = "Field `START_SEL` writer - Selects one of the 16 input triggers as a start trigger. In QUAD mode, this is the second phase (phi B)."]
pub type StartSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Selects one of the 16 input triggers as a capture trigger. Input trigger 0 is always '0' and input trigger is always '1'. In the PWM, PWM_DT and PWM_PR modes this trigger is used to switch the values if the compare and period registers with their buffer counterparts."]
    #[inline(always)]
    pub fn capture_sel(&self) -> CaptureSelR {
        CaptureSelR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Selects one of the 16 input triggers as a count trigger. In QUAD mode, this is the first phase (phi A). Default setting selects input trigger 1, which is always '1'."]
    #[inline(always)]
    pub fn count_sel(&self) -> CountSelR {
        CountSelR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Selects one of the 16 input triggers as a reload trigger. In QUAD mode, this is the index or revolution pulse. In this mode, it will update the counter with 0x8000 (counter midpoint)."]
    #[inline(always)]
    pub fn reload_sel(&self) -> ReloadSelR {
        ReloadSelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Selects one of the 16 input triggers as a stop trigger. In PWM, PWM_DT and PWM_PR modes, this is the kill trigger. In these modes, the kill trigger is used to either temporarily block the PWM outputs (PWM_STOP_ON_KILL is '0') or stop the functionality (PWM_STOP_ON_KILL is '1'). For the PWM and PWM_DT modes, the blocking of the output signals can be asynchronous (STOP_EDGE should be NO_EDGE_DET) in which case the blocking is as long as the trigger is '1' or synchronous (STOP_EDGE should be RISING_EDGE) in which case it extends till the next terminal count event."]
    #[inline(always)]
    pub fn stop_sel(&self) -> StopSelR {
        StopSelR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Selects one of the 16 input triggers as a start trigger. In QUAD mode, this is the second phase (phi B)."]
    #[inline(always)]
    pub fn start_sel(&self) -> StartSelR {
        StartSelR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Selects one of the 16 input triggers as a capture trigger. Input trigger 0 is always '0' and input trigger is always '1'. In the PWM, PWM_DT and PWM_PR modes this trigger is used to switch the values if the compare and period registers with their buffer counterparts."]
    #[inline(always)]
    #[must_use]
    pub fn capture_sel(&mut self) -> CaptureSelW<TrCtrl0Spec> {
        CaptureSelW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Selects one of the 16 input triggers as a count trigger. In QUAD mode, this is the first phase (phi A). Default setting selects input trigger 1, which is always '1'."]
    #[inline(always)]
    #[must_use]
    pub fn count_sel(&mut self) -> CountSelW<TrCtrl0Spec> {
        CountSelW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Selects one of the 16 input triggers as a reload trigger. In QUAD mode, this is the index or revolution pulse. In this mode, it will update the counter with 0x8000 (counter midpoint)."]
    #[inline(always)]
    #[must_use]
    pub fn reload_sel(&mut self) -> ReloadSelW<TrCtrl0Spec> {
        ReloadSelW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Selects one of the 16 input triggers as a stop trigger. In PWM, PWM_DT and PWM_PR modes, this is the kill trigger. In these modes, the kill trigger is used to either temporarily block the PWM outputs (PWM_STOP_ON_KILL is '0') or stop the functionality (PWM_STOP_ON_KILL is '1'). For the PWM and PWM_DT modes, the blocking of the output signals can be asynchronous (STOP_EDGE should be NO_EDGE_DET) in which case the blocking is as long as the trigger is '1' or synchronous (STOP_EDGE should be RISING_EDGE) in which case it extends till the next terminal count event."]
    #[inline(always)]
    #[must_use]
    pub fn stop_sel(&mut self) -> StopSelW<TrCtrl0Spec> {
        StopSelW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Selects one of the 16 input triggers as a start trigger. In QUAD mode, this is the second phase (phi B)."]
    #[inline(always)]
    #[must_use]
    pub fn start_sel(&mut self) -> StartSelW<TrCtrl0Spec> {
        StartSelW::new(self, 16)
    }
}
#[doc = "Counter trigger control register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrCtrl0Spec;
impl crate::RegisterSpec for TrCtrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr_ctrl0::R`](R) reader structure"]
impl crate::Readable for TrCtrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`tr_ctrl0::W`](W) writer structure"]
impl crate::Writable for TrCtrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TR_CTRL0 to value 0x10"]
impl crate::Resettable for TrCtrl0Spec {
    const RESET_VALUE: u32 = 0x10;
}
