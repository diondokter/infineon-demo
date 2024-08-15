#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `CNT_DURATION` reader - This field specifies if events (edges) or a duration of the monitor signal is counted. '0': Events are monitored. An edge detection is done. All edges of the selected monitor signal are counted. '1': A duration is monitored. No edge detection is done. The monitored signal is taken as a (high active) level signal for enabling the profiling counter. Note: All monitor signals which only can represent events are edge encoded in hardware, therefore a selection of CTL.CNT_DURATION=1 will not produce meaningful results."]
pub type CntDurationR = crate::BitReader;
#[doc = "Field `CNT_DURATION` writer - This field specifies if events (edges) or a duration of the monitor signal is counted. '0': Events are monitored. An edge detection is done. All edges of the selected monitor signal are counted. '1': A duration is monitored. No edge detection is done. The monitored signal is taken as a (high active) level signal for enabling the profiling counter. Note: All monitor signals which only can represent events are edge encoded in hardware, therefore a selection of CTL.CNT_DURATION=1 will not produce meaningful results."]
pub type CntDurationW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "This field specifies the reference clock used for a counting time base when counting durations. Has no effect when CTL.CNT_DURATION=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RefClkSel {
    #[doc = "0: Timer clock (divided or undivided high frequency clock, e.g. from IMO). Selection is done in SRSS register CLK_TIMER_CTL.TIMER_SEL."]
    ClkTimer = 0,
    #[doc = "1: IMO - Internal Main Oscillator"]
    ClkImo = 1,
    #[doc = "2: ECO - External-Crystal Oscillator"]
    ClkEco = 2,
    #[doc = "3: Low frequency clock (ILO, WCO or ALTLF). Selection is done in SRSS register CLK_SELECT.LFCLK_SEL."]
    ClkLf = 3,
    #[doc = "4: High frequuency clock ('clk_hfx')."]
    ClkHf = 4,
    #[doc = "5: Peripheral clock ('clk_peri')."]
    ClkPeri = 5,
    #[doc = "6: N/A"]
    Rsvd6 = 6,
    #[doc = "7: N/A"]
    Rsvd7 = 7,
}
impl From<RefClkSel> for u8 {
    #[inline(always)]
    fn from(variant: RefClkSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RefClkSel {
    type Ux = u8;
}
impl crate::IsEnum for RefClkSel {}
#[doc = "Field `REF_CLK_SEL` reader - This field specifies the reference clock used for a counting time base when counting durations. Has no effect when CTL.CNT_DURATION=0."]
pub type RefClkSelR = crate::FieldReader<RefClkSel>;
impl RefClkSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RefClkSel {
        match self.bits {
            0 => RefClkSel::ClkTimer,
            1 => RefClkSel::ClkImo,
            2 => RefClkSel::ClkEco,
            3 => RefClkSel::ClkLf,
            4 => RefClkSel::ClkHf,
            5 => RefClkSel::ClkPeri,
            6 => RefClkSel::Rsvd6,
            7 => RefClkSel::Rsvd7,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer clock (divided or undivided high frequency clock, e.g. from IMO). Selection is done in SRSS register CLK_TIMER_CTL.TIMER_SEL."]
    #[inline(always)]
    pub fn is_clk_timer(&self) -> bool {
        *self == RefClkSel::ClkTimer
    }
    #[doc = "IMO - Internal Main Oscillator"]
    #[inline(always)]
    pub fn is_clk_imo(&self) -> bool {
        *self == RefClkSel::ClkImo
    }
    #[doc = "ECO - External-Crystal Oscillator"]
    #[inline(always)]
    pub fn is_clk_eco(&self) -> bool {
        *self == RefClkSel::ClkEco
    }
    #[doc = "Low frequency clock (ILO, WCO or ALTLF). Selection is done in SRSS register CLK_SELECT.LFCLK_SEL."]
    #[inline(always)]
    pub fn is_clk_lf(&self) -> bool {
        *self == RefClkSel::ClkLf
    }
    #[doc = "High frequuency clock ('clk_hfx')."]
    #[inline(always)]
    pub fn is_clk_hf(&self) -> bool {
        *self == RefClkSel::ClkHf
    }
    #[doc = "Peripheral clock ('clk_peri')."]
    #[inline(always)]
    pub fn is_clk_peri(&self) -> bool {
        *self == RefClkSel::ClkPeri
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_rsvd_6(&self) -> bool {
        *self == RefClkSel::Rsvd6
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_rsvd_7(&self) -> bool {
        *self == RefClkSel::Rsvd7
    }
}
#[doc = "Field `REF_CLK_SEL` writer - This field specifies the reference clock used for a counting time base when counting durations. Has no effect when CTL.CNT_DURATION=0."]
pub type RefClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 3, RefClkSel, crate::Safe>;
impl<'a, REG> RefClkSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer clock (divided or undivided high frequency clock, e.g. from IMO). Selection is done in SRSS register CLK_TIMER_CTL.TIMER_SEL."]
    #[inline(always)]
    pub fn clk_timer(self) -> &'a mut crate::W<REG> {
        self.variant(RefClkSel::ClkTimer)
    }
    #[doc = "IMO - Internal Main Oscillator"]
    #[inline(always)]
    pub fn clk_imo(self) -> &'a mut crate::W<REG> {
        self.variant(RefClkSel::ClkImo)
    }
    #[doc = "ECO - External-Crystal Oscillator"]
    #[inline(always)]
    pub fn clk_eco(self) -> &'a mut crate::W<REG> {
        self.variant(RefClkSel::ClkEco)
    }
    #[doc = "Low frequency clock (ILO, WCO or ALTLF). Selection is done in SRSS register CLK_SELECT.LFCLK_SEL."]
    #[inline(always)]
    pub fn clk_lf(self) -> &'a mut crate::W<REG> {
        self.variant(RefClkSel::ClkLf)
    }
    #[doc = "High frequuency clock ('clk_hfx')."]
    #[inline(always)]
    pub fn clk_hf(self) -> &'a mut crate::W<REG> {
        self.variant(RefClkSel::ClkHf)
    }
    #[doc = "Peripheral clock ('clk_peri')."]
    #[inline(always)]
    pub fn clk_peri(self) -> &'a mut crate::W<REG> {
        self.variant(RefClkSel::ClkPeri)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd_6(self) -> &'a mut crate::W<REG> {
        self.variant(RefClkSel::Rsvd6)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd_7(self) -> &'a mut crate::W<REG> {
        self.variant(RefClkSel::Rsvd7)
    }
}
#[doc = "Field `MON_SEL` reader - This field specifies the montior input signal to be observed by the profiling counter. The monitor signals are product specific, see product definition spreadsheet tab 'Monitor' for details."]
pub type MonSelR = crate::FieldReader;
#[doc = "Field `MON_SEL` writer - This field specifies the montior input signal to be observed by the profiling counter. The monitor signals are product specific, see product definition spreadsheet tab 'Monitor' for details."]
pub type MonSelW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ENABLED` reader - Enables the profiling counter: '0': Disabled. '1': Enabled."]
pub type EnabledR = crate::BitReader;
#[doc = "Field `ENABLED` writer - Enables the profiling counter: '0': Disabled. '1': Enabled."]
pub type EnabledW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This field specifies if events (edges) or a duration of the monitor signal is counted. '0': Events are monitored. An edge detection is done. All edges of the selected monitor signal are counted. '1': A duration is monitored. No edge detection is done. The monitored signal is taken as a (high active) level signal for enabling the profiling counter. Note: All monitor signals which only can represent events are edge encoded in hardware, therefore a selection of CTL.CNT_DURATION=1 will not produce meaningful results."]
    #[inline(always)]
    pub fn cnt_duration(&self) -> CntDurationR {
        CntDurationR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - This field specifies the reference clock used for a counting time base when counting durations. Has no effect when CTL.CNT_DURATION=0."]
    #[inline(always)]
    pub fn ref_clk_sel(&self) -> RefClkSelR {
        RefClkSelR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 16:22 - This field specifies the montior input signal to be observed by the profiling counter. The monitor signals are product specific, see product definition spreadsheet tab 'Monitor' for details."]
    #[inline(always)]
    pub fn mon_sel(&self) -> MonSelR {
        MonSelR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Enables the profiling counter: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> EnabledR {
        EnabledR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This field specifies if events (edges) or a duration of the monitor signal is counted. '0': Events are monitored. An edge detection is done. All edges of the selected monitor signal are counted. '1': A duration is monitored. No edge detection is done. The monitored signal is taken as a (high active) level signal for enabling the profiling counter. Note: All monitor signals which only can represent events are edge encoded in hardware, therefore a selection of CTL.CNT_DURATION=1 will not produce meaningful results."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_duration(&mut self) -> CntDurationW<CtlSpec> {
        CntDurationW::new(self, 0)
    }
    #[doc = "Bits 4:6 - This field specifies the reference clock used for a counting time base when counting durations. Has no effect when CTL.CNT_DURATION=0."]
    #[inline(always)]
    #[must_use]
    pub fn ref_clk_sel(&mut self) -> RefClkSelW<CtlSpec> {
        RefClkSelW::new(self, 4)
    }
    #[doc = "Bits 16:22 - This field specifies the montior input signal to be observed by the profiling counter. The monitor signals are product specific, see product definition spreadsheet tab 'Monitor' for details."]
    #[inline(always)]
    #[must_use]
    pub fn mon_sel(&mut self) -> MonSelW<CtlSpec> {
        MonSelW::new(self, 16)
    }
    #[doc = "Bit 31 - Enables the profiling counter: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> EnabledW<CtlSpec> {
        EnabledW::new(self, 31)
    }
}
#[doc = "Profile counter configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
