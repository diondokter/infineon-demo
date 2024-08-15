#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `WCO_EN` reader - Watch-crystal oscillator (WCO) enable. If there is a write in progress when this bit is cleared, the WCO will be internally kept on until the write completes. After enabling the WCO software must wait until STATUS.WCO_OK=1 before configuring any component that depends on clk_lf/clk_bak, like for example RTC or WDTs. Follow the procedure in BACKUP_RTC_RW to access this bit."]
pub type WcoEnR = crate::BitReader;
#[doc = "Field `WCO_EN` writer - Watch-crystal oscillator (WCO) enable. If there is a write in progress when this bit is cleared, the WCO will be internally kept on until the write completes. After enabling the WCO software must wait until STATUS.WCO_OK=1 before configuring any component that depends on clk_lf/clk_bak, like for example RTC or WDTs. Follow the procedure in BACKUP_RTC_RW to access this bit."]
pub type WcoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Clock select for BAK clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkSel {
    #[doc = "0: Watch-crystal oscillator input."]
    Wco = 0,
    #[doc = "1: This allows to use the LFCLK selection as an alternate backup domain clock. Note that LFCLK is not available in all power modes, and clock glitches can propagate into the backup logic when the clock is stopped. For this reason, if the WCO is intended as the clock source then choose it directly instead of routing through LFCLK."]
    Altbak = 1,
}
impl From<ClkSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkSel {
    type Ux = u8;
}
impl crate::IsEnum for ClkSel {}
#[doc = "Field `CLK_SEL` reader - Clock select for BAK clock"]
pub type ClkSelR = crate::FieldReader<ClkSel>;
impl ClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkSel> {
        match self.bits {
            0 => Some(ClkSel::Wco),
            1 => Some(ClkSel::Altbak),
            _ => None,
        }
    }
    #[doc = "Watch-crystal oscillator input."]
    #[inline(always)]
    pub fn is_wco(&self) -> bool {
        *self == ClkSel::Wco
    }
    #[doc = "This allows to use the LFCLK selection as an alternate backup domain clock. Note that LFCLK is not available in all power modes, and clock glitches can propagate into the backup logic when the clock is stopped. For this reason, if the WCO is intended as the clock source then choose it directly instead of routing through LFCLK."]
    #[inline(always)]
    pub fn is_altbak(&self) -> bool {
        *self == ClkSel::Altbak
    }
}
#[doc = "Field `CLK_SEL` writer - Clock select for BAK clock"]
pub type ClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkSel>;
impl<'a, REG> ClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Watch-crystal oscillator input."]
    #[inline(always)]
    pub fn wco(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSel::Wco)
    }
    #[doc = "This allows to use the LFCLK selection as an alternate backup domain clock. Note that LFCLK is not available in all power modes, and clock glitches can propagate into the backup logic when the clock is stopped. For this reason, if the WCO is intended as the clock source then choose it directly instead of routing through LFCLK."]
    #[inline(always)]
    pub fn altbak(self) -> &'a mut crate::W<REG> {
        self.variant(ClkSel::Altbak)
    }
}
#[doc = "Field `PRESCALER` reader - N/A"]
pub type PrescalerR = crate::FieldReader;
#[doc = "Field `PRESCALER` writer - N/A"]
pub type PrescalerW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WCO_BYPASS` reader - Configures the WCO for different board-level connections to the WCO pins. For example, this can be used to connect an external watch crystal oscillator instead of a watch crystal. In all cases, the two related GPIO pins (WCO input and output pins) must be configured as analog connections using GPIO registers, and they must be hooked at the board level as described below. Configure this field before enabling the WCO, and do not change this setting when WCO_EN=1. 0: Watch crystal. Connect a 32.768 kHz watch crystal between WCO input and output pins. 1: Clock signal, either a square wave or sine wave. See PRESCALER field for connection information."]
pub type WcoBypassR = crate::BitReader;
#[doc = "Field `WCO_BYPASS` writer - Configures the WCO for different board-level connections to the WCO pins. For example, this can be used to connect an external watch crystal oscillator instead of a watch crystal. In all cases, the two related GPIO pins (WCO input and output pins) must be configured as analog connections using GPIO registers, and they must be hooked at the board level as described below. Configure this field before enabling the WCO, and do not change this setting when WCO_EN=1. 0: Watch crystal. Connect a 32.768 kHz watch crystal between WCO input and output pins. 1: Clock signal, either a square wave or sine wave. See PRESCALER field for connection information."]
pub type WcoBypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDBAK_CTL` reader - Controls the behavior of the switch that generates vddbak from vbackup or vddd. 0: automatically select vddd if its brownout detector says it is valid. If the brownout says its not valid, then use vmax which is the highest of vddd or vbackup. 1,2,3: force vddbak and vmax to select vbackup, regardless of its voltage."]
pub type VddbakCtlR = crate::FieldReader;
#[doc = "Field `VDDBAK_CTL` writer - Controls the behavior of the switch that generates vddbak from vbackup or vddd. 0: automatically select vddd if its brownout detector says it is valid. If the brownout says its not valid, then use vmax which is the highest of vddd or vbackup. 1,2,3: force vddbak and vmax to select vbackup, regardless of its voltage."]
pub type VddbakCtlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VBACKUP_MEAS` reader - Connect vbackup supply to the vbackup_meas output for measurement by an ADC attached to amuxbusa_adft_vddd. The vbackup_meas signal is scaled to 10 percent of vbackup, so it is within the supply range of the ADC."]
pub type VbackupMeasR = crate::BitReader;
#[doc = "Field `VBACKUP_MEAS` writer - Connect vbackup supply to the vbackup_meas output for measurement by an ADC attached to amuxbusa_adft_vddd. The vbackup_meas signal is scaled to 10 percent of vbackup, so it is within the supply range of the ADC."]
pub type VbackupMeasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_CHARGE_KEY` reader - When set to 3C, the supercap charger circuit is enabled. Any other code disables the supercap charger. THIS CHARGING CIRCUIT IS FOR A SUPERCAP ONLY AND CANNOT SAFELY CHARGE A BATTERY. DO NOT WRITE THIS KEY WHEN VBACKUP IS CONNECTED TO A BATTERY."]
pub type EnChargeKeyR = crate::FieldReader;
#[doc = "Field `EN_CHARGE_KEY` writer - When set to 3C, the supercap charger circuit is enabled. Any other code disables the supercap charger. THIS CHARGING CIRCUIT IS FOR A SUPERCAP ONLY AND CANNOT SAFELY CHARGE A BATTERY. DO NOT WRITE THIS KEY WHEN VBACKUP IS CONNECTED TO A BATTERY."]
pub type EnChargeKeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 3 - Watch-crystal oscillator (WCO) enable. If there is a write in progress when this bit is cleared, the WCO will be internally kept on until the write completes. After enabling the WCO software must wait until STATUS.WCO_OK=1 before configuring any component that depends on clk_lf/clk_bak, like for example RTC or WDTs. Follow the procedure in BACKUP_RTC_RW to access this bit."]
    #[inline(always)]
    pub fn wco_en(&self) -> WcoEnR {
        WcoEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Clock select for BAK clock"]
    #[inline(always)]
    pub fn clk_sel(&self) -> ClkSelR {
        ClkSelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - N/A"]
    #[inline(always)]
    pub fn prescaler(&self) -> PrescalerR {
        PrescalerR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 16 - Configures the WCO for different board-level connections to the WCO pins. For example, this can be used to connect an external watch crystal oscillator instead of a watch crystal. In all cases, the two related GPIO pins (WCO input and output pins) must be configured as analog connections using GPIO registers, and they must be hooked at the board level as described below. Configure this field before enabling the WCO, and do not change this setting when WCO_EN=1. 0: Watch crystal. Connect a 32.768 kHz watch crystal between WCO input and output pins. 1: Clock signal, either a square wave or sine wave. See PRESCALER field for connection information."]
    #[inline(always)]
    pub fn wco_bypass(&self) -> WcoBypassR {
        WcoBypassR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Controls the behavior of the switch that generates vddbak from vbackup or vddd. 0: automatically select vddd if its brownout detector says it is valid. If the brownout says its not valid, then use vmax which is the highest of vddd or vbackup. 1,2,3: force vddbak and vmax to select vbackup, regardless of its voltage."]
    #[inline(always)]
    pub fn vddbak_ctl(&self) -> VddbakCtlR {
        VddbakCtlR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - Connect vbackup supply to the vbackup_meas output for measurement by an ADC attached to amuxbusa_adft_vddd. The vbackup_meas signal is scaled to 10 percent of vbackup, so it is within the supply range of the ADC."]
    #[inline(always)]
    pub fn vbackup_meas(&self) -> VbackupMeasR {
        VbackupMeasR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 24:31 - When set to 3C, the supercap charger circuit is enabled. Any other code disables the supercap charger. THIS CHARGING CIRCUIT IS FOR A SUPERCAP ONLY AND CANNOT SAFELY CHARGE A BATTERY. DO NOT WRITE THIS KEY WHEN VBACKUP IS CONNECTED TO A BATTERY."]
    #[inline(always)]
    pub fn en_charge_key(&self) -> EnChargeKeyR {
        EnChargeKeyR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 3 - Watch-crystal oscillator (WCO) enable. If there is a write in progress when this bit is cleared, the WCO will be internally kept on until the write completes. After enabling the WCO software must wait until STATUS.WCO_OK=1 before configuring any component that depends on clk_lf/clk_bak, like for example RTC or WDTs. Follow the procedure in BACKUP_RTC_RW to access this bit."]
    #[inline(always)]
    #[must_use]
    pub fn wco_en(&mut self) -> WcoEnW<CtlSpec> {
        WcoEnW::new(self, 3)
    }
    #[doc = "Bits 8:9 - Clock select for BAK clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sel(&mut self) -> ClkSelW<CtlSpec> {
        ClkSelW::new(self, 8)
    }
    #[doc = "Bits 12:13 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PrescalerW<CtlSpec> {
        PrescalerW::new(self, 12)
    }
    #[doc = "Bit 16 - Configures the WCO for different board-level connections to the WCO pins. For example, this can be used to connect an external watch crystal oscillator instead of a watch crystal. In all cases, the two related GPIO pins (WCO input and output pins) must be configured as analog connections using GPIO registers, and they must be hooked at the board level as described below. Configure this field before enabling the WCO, and do not change this setting when WCO_EN=1. 0: Watch crystal. Connect a 32.768 kHz watch crystal between WCO input and output pins. 1: Clock signal, either a square wave or sine wave. See PRESCALER field for connection information."]
    #[inline(always)]
    #[must_use]
    pub fn wco_bypass(&mut self) -> WcoBypassW<CtlSpec> {
        WcoBypassW::new(self, 16)
    }
    #[doc = "Bits 17:18 - Controls the behavior of the switch that generates vddbak from vbackup or vddd. 0: automatically select vddd if its brownout detector says it is valid. If the brownout says its not valid, then use vmax which is the highest of vddd or vbackup. 1,2,3: force vddbak and vmax to select vbackup, regardless of its voltage."]
    #[inline(always)]
    #[must_use]
    pub fn vddbak_ctl(&mut self) -> VddbakCtlW<CtlSpec> {
        VddbakCtlW::new(self, 17)
    }
    #[doc = "Bit 19 - Connect vbackup supply to the vbackup_meas output for measurement by an ADC attached to amuxbusa_adft_vddd. The vbackup_meas signal is scaled to 10 percent of vbackup, so it is within the supply range of the ADC."]
    #[inline(always)]
    #[must_use]
    pub fn vbackup_meas(&mut self) -> VbackupMeasW<CtlSpec> {
        VbackupMeasW::new(self, 19)
    }
    #[doc = "Bits 24:31 - When set to 3C, the supercap charger circuit is enabled. Any other code disables the supercap charger. THIS CHARGING CIRCUIT IS FOR A SUPERCAP ONLY AND CANNOT SAFELY CHARGE A BATTERY. DO NOT WRITE THIS KEY WHEN VBACKUP IS CONNECTED TO A BATTERY."]
    #[inline(always)]
    #[must_use]
    pub fn en_charge_key(&mut self) -> EnChargeKeyW<CtlSpec> {
        EnChargeKeyW::new(self, 24)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
