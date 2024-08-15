#[doc = "Register `AMUX_SPLIT_CTL[%s]` reader"]
pub type R = crate::R<AmuxSplitCtlSpec>;
#[doc = "Register `AMUX_SPLIT_CTL[%s]` writer"]
pub type W = crate::W<AmuxSplitCtlSpec>;
#[doc = "Field `SWITCH_AA_SL` reader - T-switch control for Left AMUXBUSA switch: '0': switch open. '1': switch closed."]
pub type SwitchAaSlR = crate::BitReader;
#[doc = "Field `SWITCH_AA_SL` writer - T-switch control for Left AMUXBUSA switch: '0': switch open. '1': switch closed."]
pub type SwitchAaSlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWITCH_AA_SR` reader - T-switch control for Right AMUXBUSA switch: '0': switch open. '1': switch closed."]
pub type SwitchAaSrR = crate::BitReader;
#[doc = "Field `SWITCH_AA_SR` writer - T-switch control for Right AMUXBUSA switch: '0': switch open. '1': switch closed."]
pub type SwitchAaSrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWITCH_AA_S0` reader - T-switch control for AMUXBUSA vssa/ground switch: '0': switch open. '1': switch closed."]
pub type SwitchAaS0R = crate::BitReader;
#[doc = "Field `SWITCH_AA_S0` writer - T-switch control for AMUXBUSA vssa/ground switch: '0': switch open. '1': switch closed."]
pub type SwitchAaS0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWITCH_BB_SL` reader - T-switch control for Left AMUXBUSB switch."]
pub type SwitchBbSlR = crate::BitReader;
#[doc = "Field `SWITCH_BB_SL` writer - T-switch control for Left AMUXBUSB switch."]
pub type SwitchBbSlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWITCH_BB_SR` reader - T-switch control for Right AMUXBUSB switch."]
pub type SwitchBbSrR = crate::BitReader;
#[doc = "Field `SWITCH_BB_SR` writer - T-switch control for Right AMUXBUSB switch."]
pub type SwitchBbSrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWITCH_BB_S0` reader - T-switch control for AMUXBUSB vssa/ground switch."]
pub type SwitchBbS0R = crate::BitReader;
#[doc = "Field `SWITCH_BB_S0` writer - T-switch control for AMUXBUSB vssa/ground switch."]
pub type SwitchBbS0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - T-switch control for Left AMUXBUSA switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub fn switch_aa_sl(&self) -> SwitchAaSlR {
        SwitchAaSlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - T-switch control for Right AMUXBUSA switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub fn switch_aa_sr(&self) -> SwitchAaSrR {
        SwitchAaSrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - T-switch control for AMUXBUSA vssa/ground switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub fn switch_aa_s0(&self) -> SwitchAaS0R {
        SwitchAaS0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - T-switch control for Left AMUXBUSB switch."]
    #[inline(always)]
    pub fn switch_bb_sl(&self) -> SwitchBbSlR {
        SwitchBbSlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - T-switch control for Right AMUXBUSB switch."]
    #[inline(always)]
    pub fn switch_bb_sr(&self) -> SwitchBbSrR {
        SwitchBbSrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - T-switch control for AMUXBUSB vssa/ground switch."]
    #[inline(always)]
    pub fn switch_bb_s0(&self) -> SwitchBbS0R {
        SwitchBbS0R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - T-switch control for Left AMUXBUSA switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    #[must_use]
    pub fn switch_aa_sl(&mut self) -> SwitchAaSlW<AmuxSplitCtlSpec> {
        SwitchAaSlW::new(self, 0)
    }
    #[doc = "Bit 1 - T-switch control for Right AMUXBUSA switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    #[must_use]
    pub fn switch_aa_sr(&mut self) -> SwitchAaSrW<AmuxSplitCtlSpec> {
        SwitchAaSrW::new(self, 1)
    }
    #[doc = "Bit 2 - T-switch control for AMUXBUSA vssa/ground switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    #[must_use]
    pub fn switch_aa_s0(&mut self) -> SwitchAaS0W<AmuxSplitCtlSpec> {
        SwitchAaS0W::new(self, 2)
    }
    #[doc = "Bit 4 - T-switch control for Left AMUXBUSB switch."]
    #[inline(always)]
    #[must_use]
    pub fn switch_bb_sl(&mut self) -> SwitchBbSlW<AmuxSplitCtlSpec> {
        SwitchBbSlW::new(self, 4)
    }
    #[doc = "Bit 5 - T-switch control for Right AMUXBUSB switch."]
    #[inline(always)]
    #[must_use]
    pub fn switch_bb_sr(&mut self) -> SwitchBbSrW<AmuxSplitCtlSpec> {
        SwitchBbSrW::new(self, 5)
    }
    #[doc = "Bit 6 - T-switch control for AMUXBUSB vssa/ground switch."]
    #[inline(always)]
    #[must_use]
    pub fn switch_bb_s0(&mut self) -> SwitchBbS0W<AmuxSplitCtlSpec> {
        SwitchBbS0W::new(self, 6)
    }
}
#[doc = "AMUX splitter cell control\n\nYou can [`read`](crate::Reg::read) this register and get [`amux_split_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amux_split_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AmuxSplitCtlSpec;
impl crate::RegisterSpec for AmuxSplitCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`amux_split_ctl::R`](R) reader structure"]
impl crate::Readable for AmuxSplitCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`amux_split_ctl::W`](W) writer structure"]
impl crate::Writable for AmuxSplitCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AMUX_SPLIT_CTL[%s]
to value 0"]
impl crate::Resettable for AmuxSplitCtlSpec {
    const RESET_VALUE: u32 = 0;
}
