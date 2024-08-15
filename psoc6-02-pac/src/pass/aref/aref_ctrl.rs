#[doc = "Register `AREF_CTRL` reader"]
pub type R = crate::R<ArefCtrlSpec>;
#[doc = "Register `AREF_CTRL` writer"]
pub type W = crate::W<ArefCtrlSpec>;
#[doc = "Control bit to trade off AREF settling and noise performance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArefMode {
    #[doc = "0: Nominal noise normal startup mode (meets normal mode settling and noise specifications)"]
    Normal = 0,
    #[doc = "1: High noise fast startup mode (meets fast mode settling and noise specifications)"]
    FastStart = 1,
}
impl From<ArefMode> for bool {
    #[inline(always)]
    fn from(variant: ArefMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AREF_MODE` reader - Control bit to trade off AREF settling and noise performance"]
pub type ArefModeR = crate::BitReader<ArefMode>;
impl ArefModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ArefMode {
        match self.bits {
            false => ArefMode::Normal,
            true => ArefMode::FastStart,
        }
    }
    #[doc = "Nominal noise normal startup mode (meets normal mode settling and noise specifications)"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == ArefMode::Normal
    }
    #[doc = "High noise fast startup mode (meets fast mode settling and noise specifications)"]
    #[inline(always)]
    pub fn is_fast_start(&self) -> bool {
        *self == ArefMode::FastStart
    }
}
#[doc = "Field `AREF_MODE` writer - Control bit to trade off AREF settling and noise performance"]
pub type ArefModeW<'a, REG> = crate::BitWriter<'a, REG, ArefMode>;
impl<'a, REG> ArefModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Nominal noise normal startup mode (meets normal mode settling and noise specifications)"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(ArefMode::Normal)
    }
    #[doc = "High noise fast startup mode (meets fast mode settling and noise specifications)"]
    #[inline(always)]
    pub fn fast_start(self) -> &'a mut crate::W<REG> {
        self.variant(ArefMode::FastStart)
    }
}
#[doc = "Field `AREF_BIAS_SCALE` reader - BIAS Current Control for all AREF Amplifiers. (These are risk mitigation bits that should not be touched by the customer: the impact on IDDA/noise/startup still needs to be characterized) 0: 125nA (reduced bias: reduction in total AREF IDDA, higher noise and longer startup times) 1: 250nA ('default' setting to meet bandgap performance (noise/startup) and IDDA specifications) 2: 375nA (increased bias: increase in total AREF IDDA, lower noise and shorter startup times) 3: 500nA (further increased bias: increase in total AREF IDDA, lower noise and shorter startup times)"]
pub type ArefBiasScaleR = crate::FieldReader;
#[doc = "Field `AREF_BIAS_SCALE` writer - BIAS Current Control for all AREF Amplifiers. (These are risk mitigation bits that should not be touched by the customer: the impact on IDDA/noise/startup still needs to be characterized) 0: 125nA (reduced bias: reduction in total AREF IDDA, higher noise and longer startup times) 1: 250nA ('default' setting to meet bandgap performance (noise/startup) and IDDA specifications) 2: 375nA (increased bias: increase in total AREF IDDA, lower noise and shorter startup times) 3: 500nA (further increased bias: increase in total AREF IDDA, lower noise and shorter startup times)"]
pub type ArefBiasScaleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AREF_RMB` reader - AREF control signals (RMB). Bit 0: Manual VBG startup circuit enable 0: normal VBG startup circuit operation 1: VBG startup circuit is forced 'always on' Bit 1: Manual disable of IPTAT2 DAC 0: normal IPTAT2 DAC operation 1: PTAT2 DAC is disabled while VBG startup is active Bit 2: Manual enable of VBG offset correction DAC 0: normal VBG offset correction DAC operation 1: VBG offset correction DAC is enabled while VBG startup is active"]
pub type ArefRmbR = crate::FieldReader;
#[doc = "Field `AREF_RMB` writer - AREF control signals (RMB). Bit 0: Manual VBG startup circuit enable 0: normal VBG startup circuit operation 1: VBG startup circuit is forced 'always on' Bit 1: Manual disable of IPTAT2 DAC 0: normal IPTAT2 DAC operation 1: PTAT2 DAC is disabled while VBG startup is active Bit 2: Manual enable of VBG offset correction DAC 0: normal VBG offset correction DAC operation 1: VBG offset correction DAC is enabled while VBG startup is active"]
pub type ArefRmbW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CTB_IPTAT_SCALE` reader - CTB IPTAT current scaler. This bit must be set in order to operate the CTB amplifiers in the lowest power mode. This bit is chip-wide (controls all CTB amplifiers). 0: 1uA 1: 100nA"]
pub type CtbIptatScaleR = crate::BitReader;
#[doc = "Field `CTB_IPTAT_SCALE` writer - CTB IPTAT current scaler. This bit must be set in order to operate the CTB amplifiers in the lowest power mode. This bit is chip-wide (controls all CTB amplifiers). 0: 1uA 1: 100nA"]
pub type CtbIptatScaleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTB_IPTAT_REDIRECT` reader - Re-direct the CTB IPTAT output current. This can be used to reduce amplifier bias glitches during power mode transitions (for PSoC4A/B DSAB backwards compatibility). 0: Opamp&lt;n>.IPTAT = AREF.IPTAT and Opamp&lt;n>.IZTAT= AREF.IZTAT 1: Opamp&lt;n>.IPTAT = HiZ and Opamp&lt;n>.IZTAT= AREF.IPTAT *Note that in Deep Sleep, the AREF IZTAT and/or IPTAT currents can be disabled and therefore the corresponding Opamp&lt;n>.IZTAT/IPTAT will be HiZ."]
pub type CtbIptatRedirectR = crate::FieldReader;
#[doc = "Field `CTB_IPTAT_REDIRECT` writer - Re-direct the CTB IPTAT output current. This can be used to reduce amplifier bias glitches during power mode transitions (for PSoC4A/B DSAB backwards compatibility). 0: Opamp&lt;n>.IPTAT = AREF.IPTAT and Opamp&lt;n>.IZTAT= AREF.IZTAT 1: Opamp&lt;n>.IPTAT = HiZ and Opamp&lt;n>.IZTAT= AREF.IPTAT *Note that in Deep Sleep, the AREF IZTAT and/or IPTAT currents can be disabled and therefore the corresponding Opamp&lt;n>.IZTAT/IPTAT will be HiZ."]
pub type CtbIptatRedirectW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "iztat current select control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IztatSel {
    #[doc = "0: Use 250nA IZTAT from SRSS"]
    Srss = 0,
    #[doc = "1: Use locally generated 250nA"]
    Local = 1,
}
impl From<IztatSel> for bool {
    #[inline(always)]
    fn from(variant: IztatSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IZTAT_SEL` reader - iztat current select control"]
pub type IztatSelR = crate::BitReader<IztatSel>;
impl IztatSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IztatSel {
        match self.bits {
            false => IztatSel::Srss,
            true => IztatSel::Local,
        }
    }
    #[doc = "Use 250nA IZTAT from SRSS"]
    #[inline(always)]
    pub fn is_srss(&self) -> bool {
        *self == IztatSel::Srss
    }
    #[doc = "Use locally generated 250nA"]
    #[inline(always)]
    pub fn is_local(&self) -> bool {
        *self == IztatSel::Local
    }
}
#[doc = "Field `IZTAT_SEL` writer - iztat current select control"]
pub type IztatSelW<'a, REG> = crate::BitWriter<'a, REG, IztatSel>;
impl<'a, REG> IztatSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use 250nA IZTAT from SRSS"]
    #[inline(always)]
    pub fn srss(self) -> &'a mut crate::W<REG> {
        self.variant(IztatSel::Srss)
    }
    #[doc = "Use locally generated 250nA"]
    #[inline(always)]
    pub fn local(self) -> &'a mut crate::W<REG> {
        self.variant(IztatSel::Local)
    }
}
#[doc = "Field `CLOCK_PUMP_PERI_SEL` reader - CTBm charge pump clock source select. This field has nothing to do with the AREF. 0: Use the dedicated pump clock from SRSS (default) 1: Use one of the CLK_PERI dividers"]
pub type ClockPumpPeriSelR = crate::BitReader;
#[doc = "Field `CLOCK_PUMP_PERI_SEL` writer - CTBm charge pump clock source select. This field has nothing to do with the AREF. 0: Use the dedicated pump clock from SRSS (default) 1: Use one of the CLK_PERI dividers"]
pub type ClockPumpPeriSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "bandgap voltage select control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VrefSel {
    #[doc = "0: Use 0.8V Vref from SRSS"]
    Srss = 0,
    #[doc = "1: Use locally generated Vref"]
    Local = 1,
    #[doc = "2: Use externally supplied Vref (aref_ext_vref)"]
    External = 2,
}
impl From<VrefSel> for u8 {
    #[inline(always)]
    fn from(variant: VrefSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VrefSel {
    type Ux = u8;
}
impl crate::IsEnum for VrefSel {}
#[doc = "Field `VREF_SEL` reader - bandgap voltage select control"]
pub type VrefSelR = crate::FieldReader<VrefSel>;
impl VrefSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VrefSel> {
        match self.bits {
            0 => Some(VrefSel::Srss),
            1 => Some(VrefSel::Local),
            2 => Some(VrefSel::External),
            _ => None,
        }
    }
    #[doc = "Use 0.8V Vref from SRSS"]
    #[inline(always)]
    pub fn is_srss(&self) -> bool {
        *self == VrefSel::Srss
    }
    #[doc = "Use locally generated Vref"]
    #[inline(always)]
    pub fn is_local(&self) -> bool {
        *self == VrefSel::Local
    }
    #[doc = "Use externally supplied Vref (aref_ext_vref)"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == VrefSel::External
    }
}
#[doc = "Field `VREF_SEL` writer - bandgap voltage select control"]
pub type VrefSelW<'a, REG> = crate::FieldWriter<'a, REG, 2, VrefSel>;
impl<'a, REG> VrefSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use 0.8V Vref from SRSS"]
    #[inline(always)]
    pub fn srss(self) -> &'a mut crate::W<REG> {
        self.variant(VrefSel::Srss)
    }
    #[doc = "Use locally generated Vref"]
    #[inline(always)]
    pub fn local(self) -> &'a mut crate::W<REG> {
        self.variant(VrefSel::Local)
    }
    #[doc = "Use externally supplied Vref (aref_ext_vref)"]
    #[inline(always)]
    pub fn external(self) -> &'a mut crate::W<REG> {
        self.variant(VrefSel::External)
    }
}
#[doc = "AREF DeepSleep Operation Modes (only applies if DEEPSLEEP_ON = 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DeepsleepMode {
    #[doc = "0: All blocks 'OFF' in DeepSleep"]
    Off = 0,
    #[doc = "1: IPTAT bias generator 'ON' in DeepSleep (used for fast AREF wakeup only: IPTAT outputs not available)"]
    Iptat = 1,
    #[doc = "2: IPTAT bias generator and outputs 'ON' in DeepSleep (used for biasing the CTBm with a PTAT current only in deep sleep) *Note that this mode also requires that the CTB_IPTAT_REDIRECT be set if the CTBm opamp is to operate in DeepSleep"]
    IptatIztat = 2,
    #[doc = "3: IPTAT, VREF, and IZTAT generators 'ON' in DeepSleep. This mode provides identical AREF functionality in DeepSleep as in the Active mode."]
    IptatIztatVref = 3,
}
impl From<DeepsleepMode> for u8 {
    #[inline(always)]
    fn from(variant: DeepsleepMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DeepsleepMode {
    type Ux = u8;
}
impl crate::IsEnum for DeepsleepMode {}
#[doc = "Field `DEEPSLEEP_MODE` reader - AREF DeepSleep Operation Modes (only applies if DEEPSLEEP_ON = 1)"]
pub type DeepsleepModeR = crate::FieldReader<DeepsleepMode>;
impl DeepsleepModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DeepsleepMode {
        match self.bits {
            0 => DeepsleepMode::Off,
            1 => DeepsleepMode::Iptat,
            2 => DeepsleepMode::IptatIztat,
            3 => DeepsleepMode::IptatIztatVref,
            _ => unreachable!(),
        }
    }
    #[doc = "All blocks 'OFF' in DeepSleep"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == DeepsleepMode::Off
    }
    #[doc = "IPTAT bias generator 'ON' in DeepSleep (used for fast AREF wakeup only: IPTAT outputs not available)"]
    #[inline(always)]
    pub fn is_iptat(&self) -> bool {
        *self == DeepsleepMode::Iptat
    }
    #[doc = "IPTAT bias generator and outputs 'ON' in DeepSleep (used for biasing the CTBm with a PTAT current only in deep sleep) *Note that this mode also requires that the CTB_IPTAT_REDIRECT be set if the CTBm opamp is to operate in DeepSleep"]
    #[inline(always)]
    pub fn is_iptat_iztat(&self) -> bool {
        *self == DeepsleepMode::IptatIztat
    }
    #[doc = "IPTAT, VREF, and IZTAT generators 'ON' in DeepSleep. This mode provides identical AREF functionality in DeepSleep as in the Active mode."]
    #[inline(always)]
    pub fn is_iptat_iztat_vref(&self) -> bool {
        *self == DeepsleepMode::IptatIztatVref
    }
}
#[doc = "Field `DEEPSLEEP_MODE` writer - AREF DeepSleep Operation Modes (only applies if DEEPSLEEP_ON = 1)"]
pub type DeepsleepModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, DeepsleepMode, crate::Safe>;
impl<'a, REG> DeepsleepModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "All blocks 'OFF' in DeepSleep"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(DeepsleepMode::Off)
    }
    #[doc = "IPTAT bias generator 'ON' in DeepSleep (used for fast AREF wakeup only: IPTAT outputs not available)"]
    #[inline(always)]
    pub fn iptat(self) -> &'a mut crate::W<REG> {
        self.variant(DeepsleepMode::Iptat)
    }
    #[doc = "IPTAT bias generator and outputs 'ON' in DeepSleep (used for biasing the CTBm with a PTAT current only in deep sleep) *Note that this mode also requires that the CTB_IPTAT_REDIRECT be set if the CTBm opamp is to operate in DeepSleep"]
    #[inline(always)]
    pub fn iptat_iztat(self) -> &'a mut crate::W<REG> {
        self.variant(DeepsleepMode::IptatIztat)
    }
    #[doc = "IPTAT, VREF, and IZTAT generators 'ON' in DeepSleep. This mode provides identical AREF functionality in DeepSleep as in the Active mode."]
    #[inline(always)]
    pub fn iptat_iztat_vref(self) -> &'a mut crate::W<REG> {
        self.variant(DeepsleepMode::IptatIztatVref)
    }
}
#[doc = "Field `DEEPSLEEP_ON` reader - - 0: AREF IP disabled/off during DeepSleep power mode - 1: AREF IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
pub type DeepsleepOnR = crate::BitReader;
#[doc = "Field `DEEPSLEEP_ON` writer - - 0: AREF IP disabled/off during DeepSleep power mode - 1: AREF IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
pub type DeepsleepOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED` reader - Disable AREF"]
pub type EnabledR = crate::BitReader;
#[doc = "Field `ENABLED` writer - Disable AREF"]
pub type EnabledW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Control bit to trade off AREF settling and noise performance"]
    #[inline(always)]
    pub fn aref_mode(&self) -> ArefModeR {
        ArefModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - BIAS Current Control for all AREF Amplifiers. (These are risk mitigation bits that should not be touched by the customer: the impact on IDDA/noise/startup still needs to be characterized) 0: 125nA (reduced bias: reduction in total AREF IDDA, higher noise and longer startup times) 1: 250nA ('default' setting to meet bandgap performance (noise/startup) and IDDA specifications) 2: 375nA (increased bias: increase in total AREF IDDA, lower noise and shorter startup times) 3: 500nA (further increased bias: increase in total AREF IDDA, lower noise and shorter startup times)"]
    #[inline(always)]
    pub fn aref_bias_scale(&self) -> ArefBiasScaleR {
        ArefBiasScaleR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - AREF control signals (RMB). Bit 0: Manual VBG startup circuit enable 0: normal VBG startup circuit operation 1: VBG startup circuit is forced 'always on' Bit 1: Manual disable of IPTAT2 DAC 0: normal IPTAT2 DAC operation 1: PTAT2 DAC is disabled while VBG startup is active Bit 2: Manual enable of VBG offset correction DAC 0: normal VBG offset correction DAC operation 1: VBG offset correction DAC is enabled while VBG startup is active"]
    #[inline(always)]
    pub fn aref_rmb(&self) -> ArefRmbR {
        ArefRmbR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - CTB IPTAT current scaler. This bit must be set in order to operate the CTB amplifiers in the lowest power mode. This bit is chip-wide (controls all CTB amplifiers). 0: 1uA 1: 100nA"]
    #[inline(always)]
    pub fn ctb_iptat_scale(&self) -> CtbIptatScaleR {
        CtbIptatScaleR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Re-direct the CTB IPTAT output current. This can be used to reduce amplifier bias glitches during power mode transitions (for PSoC4A/B DSAB backwards compatibility). 0: Opamp&lt;n>.IPTAT = AREF.IPTAT and Opamp&lt;n>.IZTAT= AREF.IZTAT 1: Opamp&lt;n>.IPTAT = HiZ and Opamp&lt;n>.IZTAT= AREF.IPTAT *Note that in Deep Sleep, the AREF IZTAT and/or IPTAT currents can be disabled and therefore the corresponding Opamp&lt;n>.IZTAT/IPTAT will be HiZ."]
    #[inline(always)]
    pub fn ctb_iptat_redirect(&self) -> CtbIptatRedirectR {
        CtbIptatRedirectR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - iztat current select control"]
    #[inline(always)]
    pub fn iztat_sel(&self) -> IztatSelR {
        IztatSelR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - CTBm charge pump clock source select. This field has nothing to do with the AREF. 0: Use the dedicated pump clock from SRSS (default) 1: Use one of the CLK_PERI dividers"]
    #[inline(always)]
    pub fn clock_pump_peri_sel(&self) -> ClockPumpPeriSelR {
        ClockPumpPeriSelR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - bandgap voltage select control"]
    #[inline(always)]
    pub fn vref_sel(&self) -> VrefSelR {
        VrefSelR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 28:29 - AREF DeepSleep Operation Modes (only applies if DEEPSLEEP_ON = 1)"]
    #[inline(always)]
    pub fn deepsleep_mode(&self) -> DeepsleepModeR {
        DeepsleepModeR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - - 0: AREF IP disabled/off during DeepSleep power mode - 1: AREF IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub fn deepsleep_on(&self) -> DeepsleepOnR {
        DeepsleepOnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Disable AREF"]
    #[inline(always)]
    pub fn enabled(&self) -> EnabledR {
        EnabledR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control bit to trade off AREF settling and noise performance"]
    #[inline(always)]
    #[must_use]
    pub fn aref_mode(&mut self) -> ArefModeW<ArefCtrlSpec> {
        ArefModeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - BIAS Current Control for all AREF Amplifiers. (These are risk mitigation bits that should not be touched by the customer: the impact on IDDA/noise/startup still needs to be characterized) 0: 125nA (reduced bias: reduction in total AREF IDDA, higher noise and longer startup times) 1: 250nA ('default' setting to meet bandgap performance (noise/startup) and IDDA specifications) 2: 375nA (increased bias: increase in total AREF IDDA, lower noise and shorter startup times) 3: 500nA (further increased bias: increase in total AREF IDDA, lower noise and shorter startup times)"]
    #[inline(always)]
    #[must_use]
    pub fn aref_bias_scale(&mut self) -> ArefBiasScaleW<ArefCtrlSpec> {
        ArefBiasScaleW::new(self, 2)
    }
    #[doc = "Bits 4:6 - AREF control signals (RMB). Bit 0: Manual VBG startup circuit enable 0: normal VBG startup circuit operation 1: VBG startup circuit is forced 'always on' Bit 1: Manual disable of IPTAT2 DAC 0: normal IPTAT2 DAC operation 1: PTAT2 DAC is disabled while VBG startup is active Bit 2: Manual enable of VBG offset correction DAC 0: normal VBG offset correction DAC operation 1: VBG offset correction DAC is enabled while VBG startup is active"]
    #[inline(always)]
    #[must_use]
    pub fn aref_rmb(&mut self) -> ArefRmbW<ArefCtrlSpec> {
        ArefRmbW::new(self, 4)
    }
    #[doc = "Bit 7 - CTB IPTAT current scaler. This bit must be set in order to operate the CTB amplifiers in the lowest power mode. This bit is chip-wide (controls all CTB amplifiers). 0: 1uA 1: 100nA"]
    #[inline(always)]
    #[must_use]
    pub fn ctb_iptat_scale(&mut self) -> CtbIptatScaleW<ArefCtrlSpec> {
        CtbIptatScaleW::new(self, 7)
    }
    #[doc = "Bits 8:15 - Re-direct the CTB IPTAT output current. This can be used to reduce amplifier bias glitches during power mode transitions (for PSoC4A/B DSAB backwards compatibility). 0: Opamp&lt;n>.IPTAT = AREF.IPTAT and Opamp&lt;n>.IZTAT= AREF.IZTAT 1: Opamp&lt;n>.IPTAT = HiZ and Opamp&lt;n>.IZTAT= AREF.IPTAT *Note that in Deep Sleep, the AREF IZTAT and/or IPTAT currents can be disabled and therefore the corresponding Opamp&lt;n>.IZTAT/IPTAT will be HiZ."]
    #[inline(always)]
    #[must_use]
    pub fn ctb_iptat_redirect(&mut self) -> CtbIptatRedirectW<ArefCtrlSpec> {
        CtbIptatRedirectW::new(self, 8)
    }
    #[doc = "Bit 16 - iztat current select control"]
    #[inline(always)]
    #[must_use]
    pub fn iztat_sel(&mut self) -> IztatSelW<ArefCtrlSpec> {
        IztatSelW::new(self, 16)
    }
    #[doc = "Bit 19 - CTBm charge pump clock source select. This field has nothing to do with the AREF. 0: Use the dedicated pump clock from SRSS (default) 1: Use one of the CLK_PERI dividers"]
    #[inline(always)]
    #[must_use]
    pub fn clock_pump_peri_sel(&mut self) -> ClockPumpPeriSelW<ArefCtrlSpec> {
        ClockPumpPeriSelW::new(self, 19)
    }
    #[doc = "Bits 20:21 - bandgap voltage select control"]
    #[inline(always)]
    #[must_use]
    pub fn vref_sel(&mut self) -> VrefSelW<ArefCtrlSpec> {
        VrefSelW::new(self, 20)
    }
    #[doc = "Bits 28:29 - AREF DeepSleep Operation Modes (only applies if DEEPSLEEP_ON = 1)"]
    #[inline(always)]
    #[must_use]
    pub fn deepsleep_mode(&mut self) -> DeepsleepModeW<ArefCtrlSpec> {
        DeepsleepModeW::new(self, 28)
    }
    #[doc = "Bit 30 - - 0: AREF IP disabled/off during DeepSleep power mode - 1: AREF IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    #[must_use]
    pub fn deepsleep_on(&mut self) -> DeepsleepOnW<ArefCtrlSpec> {
        DeepsleepOnW::new(self, 30)
    }
    #[doc = "Bit 31 - Disable AREF"]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> EnabledW<ArefCtrlSpec> {
        EnabledW::new(self, 31)
    }
}
#[doc = "global AREF control\n\nYou can [`read`](crate::Reg::read) this register and get [`aref_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aref_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArefCtrlSpec;
impl crate::RegisterSpec for ArefCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aref_ctrl::R`](R) reader structure"]
impl crate::Readable for ArefCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`aref_ctrl::W`](W) writer structure"]
impl crate::Writable for ArefCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AREF_CTRL to value 0"]
impl crate::Resettable for ArefCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
