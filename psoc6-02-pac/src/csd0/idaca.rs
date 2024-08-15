#[doc = "Register `IDACA` reader"]
pub type R = crate::R<IdacaSpec>;
#[doc = "Register `IDACA` writer"]
pub type W = crate::W<IdacaSpec>;
#[doc = "Field `VAL` reader - Current value setting for this IDAC (7 bits)."]
pub type ValR = crate::FieldReader;
#[doc = "Field `VAL` writer - Current value setting for this IDAC (7 bits)."]
pub type ValW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Polarity is dynamic, this bit does not influence the logic in the SoftIP, it only goes to the HardIP.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PolDyn {
    #[doc = "0: Static polarity. Polarity is expected to be stable, so to save power this avoids the shunting of the unused polarity, at the expense of response time."]
    Static = 0,
    #[doc = "1: Dynamic polarity. Polarity is expected to change frequently (e.g. invert after every csd_sense phase), so to improve response time this keeps the shunt of the unused polarity on at the expense of power."]
    Dynamic = 1,
}
impl From<PolDyn> for bool {
    #[inline(always)]
    fn from(variant: PolDyn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POL_DYN` reader - Polarity is dynamic, this bit does not influence the logic in the SoftIP, it only goes to the HardIP."]
pub type PolDynR = crate::BitReader<PolDyn>;
impl PolDynR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PolDyn {
        match self.bits {
            false => PolDyn::Static,
            true => PolDyn::Dynamic,
        }
    }
    #[doc = "Static polarity. Polarity is expected to be stable, so to save power this avoids the shunting of the unused polarity, at the expense of response time."]
    #[inline(always)]
    pub fn is_static(&self) -> bool {
        *self == PolDyn::Static
    }
    #[doc = "Dynamic polarity. Polarity is expected to change frequently (e.g. invert after every csd_sense phase), so to improve response time this keeps the shunt of the unused polarity on at the expense of power."]
    #[inline(always)]
    pub fn is_dynamic(&self) -> bool {
        *self == PolDyn::Dynamic
    }
}
#[doc = "Field `POL_DYN` writer - Polarity is dynamic, this bit does not influence the logic in the SoftIP, it only goes to the HardIP."]
pub type PolDynW<'a, REG> = crate::BitWriter<'a, REG, PolDyn>;
impl<'a, REG> PolDynW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Static polarity. Polarity is expected to be stable, so to save power this avoids the shunting of the unused polarity, at the expense of response time."]
    #[inline(always)]
    pub fn static_(self) -> &'a mut crate::W<REG> {
        self.variant(PolDyn::Static)
    }
    #[doc = "Dynamic polarity. Polarity is expected to change frequently (e.g. invert after every csd_sense phase), so to improve response time this keeps the shunt of the unused polarity on at the expense of power."]
    #[inline(always)]
    pub fn dynamic(self) -> &'a mut crate::W<REG> {
        self.variant(PolDyn::Dynamic)
    }
}
#[doc = "Selects the polarity of the IDAC (sensing operation). Normally the actual polarity depends on this bit, optionally mixed with DSI (see DSI_CTRL_EN) and if LEG1_MODE==CSD also mixed with the CSD configuration and operation. However in mutual cap mode with one IDAC (config.mutual_cap=1 &amp; config.csx_dual_idac=0) the polarity of the IDAC is controlled by csd_sense.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity {
    #[doc = "0: Normal: switch between Vssa and Cmod. For non-CSD application, IDAC will source current."]
    VssaSrc = 0,
    #[doc = "1: Inverted: switch between Vdda and Cmod. For non-CSD application, IDAC will sink current."]
    VddaSnk = 1,
    #[doc = "2: The polarity of the IDAC will follow the csd_sense signal (POL_DYN bit should be set too). The intended usage is for CSX using a single IDAC."]
    Sense = 2,
    #[doc = "3: The polarity of the IDAC will follow the inverted csd_sense signal (POL_DYN bit should be set too). The intended usage is for CSX using a single IDAC."]
    SenseInv = 3,
}
impl From<Polarity> for u8 {
    #[inline(always)]
    fn from(variant: Polarity) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity {
    type Ux = u8;
}
impl crate::IsEnum for Polarity {}
#[doc = "Field `POLARITY` reader - Selects the polarity of the IDAC (sensing operation). Normally the actual polarity depends on this bit, optionally mixed with DSI (see DSI_CTRL_EN) and if LEG1_MODE==CSD also mixed with the CSD configuration and operation. However in mutual cap mode with one IDAC (config.mutual_cap=1 &amp; config.csx_dual_idac=0) the polarity of the IDAC is controlled by csd_sense."]
pub type PolarityR = crate::FieldReader<Polarity>;
impl PolarityR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity {
        match self.bits {
            0 => Polarity::VssaSrc,
            1 => Polarity::VddaSnk,
            2 => Polarity::Sense,
            3 => Polarity::SenseInv,
            _ => unreachable!(),
        }
    }
    #[doc = "Normal: switch between Vssa and Cmod. For non-CSD application, IDAC will source current."]
    #[inline(always)]
    pub fn is_vssa_src(&self) -> bool {
        *self == Polarity::VssaSrc
    }
    #[doc = "Inverted: switch between Vdda and Cmod. For non-CSD application, IDAC will sink current."]
    #[inline(always)]
    pub fn is_vdda_snk(&self) -> bool {
        *self == Polarity::VddaSnk
    }
    #[doc = "The polarity of the IDAC will follow the csd_sense signal (POL_DYN bit should be set too). The intended usage is for CSX using a single IDAC."]
    #[inline(always)]
    pub fn is_sense(&self) -> bool {
        *self == Polarity::Sense
    }
    #[doc = "The polarity of the IDAC will follow the inverted csd_sense signal (POL_DYN bit should be set too). The intended usage is for CSX using a single IDAC."]
    #[inline(always)]
    pub fn is_sense_inv(&self) -> bool {
        *self == Polarity::SenseInv
    }
}
#[doc = "Field `POLARITY` writer - Selects the polarity of the IDAC (sensing operation). Normally the actual polarity depends on this bit, optionally mixed with DSI (see DSI_CTRL_EN) and if LEG1_MODE==CSD also mixed with the CSD configuration and operation. However in mutual cap mode with one IDAC (config.mutual_cap=1 &amp; config.csx_dual_idac=0) the polarity of the IDAC is controlled by csd_sense."]
pub type PolarityW<'a, REG> = crate::FieldWriter<'a, REG, 2, Polarity, crate::Safe>;
impl<'a, REG> PolarityW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal: switch between Vssa and Cmod. For non-CSD application, IDAC will source current."]
    #[inline(always)]
    pub fn vssa_src(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity::VssaSrc)
    }
    #[doc = "Inverted: switch between Vdda and Cmod. For non-CSD application, IDAC will sink current."]
    #[inline(always)]
    pub fn vdda_snk(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity::VddaSnk)
    }
    #[doc = "The polarity of the IDAC will follow the csd_sense signal (POL_DYN bit should be set too). The intended usage is for CSX using a single IDAC."]
    #[inline(always)]
    pub fn sense(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity::Sense)
    }
    #[doc = "The polarity of the IDAC will follow the inverted csd_sense signal (POL_DYN bit should be set too). The intended usage is for CSX using a single IDAC."]
    #[inline(always)]
    pub fn sense_inv(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity::SenseInv)
    }
}
#[doc = "Balancing mode: only applies to legs configured as CSD.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BalMode {
    #[doc = "0: enabled from start of Phi2 until disabled by CSDCMP. Intended usage: legacy CSD for balancing over a full csd_sense period (non-overlap should be turned off)"]
    Full = 0,
    #[doc = "1: enabled from start of Phi1 and disabled by CSDCMP or at end of Phi1. Enables dual IDAC CSX or Full-Wave, one for sourcing and the other for sinking."]
    Phi1 = 1,
    #[doc = "2: enabled from start of Phi2 and disabled by CSDCMP or at end of Phi2. Intended usage: CSD Low EMI or dual IDAC CSX or Full-Wave."]
    Phi2 = 2,
    #[doc = "3: enabled from start of both Phi1 and Phi2 and disabled by CSDCMP or at end of Phi1 or Phi2 (if non-overlap enabled). Intended usage: single IDAC CSX, or Full-Wave."]
    Phi1_2 = 3,
}
impl From<BalMode> for u8 {
    #[inline(always)]
    fn from(variant: BalMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BalMode {
    type Ux = u8;
}
impl crate::IsEnum for BalMode {}
#[doc = "Field `BAL_MODE` reader - Balancing mode: only applies to legs configured as CSD."]
pub type BalModeR = crate::FieldReader<BalMode>;
impl BalModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BalMode {
        match self.bits {
            0 => BalMode::Full,
            1 => BalMode::Phi1,
            2 => BalMode::Phi2,
            3 => BalMode::Phi1_2,
            _ => unreachable!(),
        }
    }
    #[doc = "enabled from start of Phi2 until disabled by CSDCMP. Intended usage: legacy CSD for balancing over a full csd_sense period (non-overlap should be turned off)"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == BalMode::Full
    }
    #[doc = "enabled from start of Phi1 and disabled by CSDCMP or at end of Phi1. Enables dual IDAC CSX or Full-Wave, one for sourcing and the other for sinking."]
    #[inline(always)]
    pub fn is_phi1(&self) -> bool {
        *self == BalMode::Phi1
    }
    #[doc = "enabled from start of Phi2 and disabled by CSDCMP or at end of Phi2. Intended usage: CSD Low EMI or dual IDAC CSX or Full-Wave."]
    #[inline(always)]
    pub fn is_phi2(&self) -> bool {
        *self == BalMode::Phi2
    }
    #[doc = "enabled from start of both Phi1 and Phi2 and disabled by CSDCMP or at end of Phi1 or Phi2 (if non-overlap enabled). Intended usage: single IDAC CSX, or Full-Wave."]
    #[inline(always)]
    pub fn is_phi1_2(&self) -> bool {
        *self == BalMode::Phi1_2
    }
}
#[doc = "Field `BAL_MODE` writer - Balancing mode: only applies to legs configured as CSD."]
pub type BalModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, BalMode, crate::Safe>;
impl<'a, REG> BalModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "enabled from start of Phi2 until disabled by CSDCMP. Intended usage: legacy CSD for balancing over a full csd_sense period (non-overlap should be turned off)"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(BalMode::Full)
    }
    #[doc = "enabled from start of Phi1 and disabled by CSDCMP or at end of Phi1. Enables dual IDAC CSX or Full-Wave, one for sourcing and the other for sinking."]
    #[inline(always)]
    pub fn phi1(self) -> &'a mut crate::W<REG> {
        self.variant(BalMode::Phi1)
    }
    #[doc = "enabled from start of Phi2 and disabled by CSDCMP or at end of Phi2. Intended usage: CSD Low EMI or dual IDAC CSX or Full-Wave."]
    #[inline(always)]
    pub fn phi2(self) -> &'a mut crate::W<REG> {
        self.variant(BalMode::Phi2)
    }
    #[doc = "enabled from start of both Phi1 and Phi2 and disabled by CSDCMP or at end of Phi1 or Phi2 (if non-overlap enabled). Intended usage: single IDAC CSX, or Full-Wave."]
    #[inline(always)]
    pub fn phi1_2(self) -> &'a mut crate::W<REG> {
        self.variant(BalMode::Phi1_2)
    }
}
#[doc = "Controls the usage mode of LEG1 and the Polarity bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Leg1Mode {
    #[doc = "0: General Purpose static mode: LEG1 and POLARITY are controlled by MMIO and optionally mixed with DSI (see DSI_CTRL_EN). No shunting is used, this saves power when off but also any on/off switching will take longer."]
    GpStatic = 0,
    #[doc = "1: General Purpose dynamic mode: LEG1 and POLARITY are controlled by MMIO and optionally mixed with DSI (see DSI_CTRL_EN). Shunting is used, so on/off switching is faster, but power is wasted when the leg is disabled."]
    Gp = 1,
    #[doc = "2: CSD static mode: LEG1 can only be on when the CSD Sequencer is in the Sample_init or Sample_norm state. In those states LEG1 is controlled by LEG1_EN, csd_sense and the CSD configuration. Polarity is controlled by the CSD configuration and operation. In addition leg1 enable and polarity can optionally be mixed with DSI (see DSI_CTRL_EN). No shunting is used, this saves power when off but also any on/off switching will take longer."]
    CsdStatic = 2,
    #[doc = "3: CSD dynamic mode: LEG1 can only be on when the CSD Sequencer is in the Sample_init or Sample_norm state. In thoses states LEG1 is controlled by LEG1_EN, the CSD configuration, csd_sense and the flopped CSDCMP output (CSDCMP_OUT_FF). Polarity is controlled by the CSD configuration and operation. In addition leg1 enable and polarity can optionally be mixed with DSI (see DSI_CTRL_EN). Shunting is used, so on/off switching is faster, but power is wasted when the leg is disabled."]
    Csd = 3,
}
impl From<Leg1Mode> for u8 {
    #[inline(always)]
    fn from(variant: Leg1Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Leg1Mode {
    type Ux = u8;
}
impl crate::IsEnum for Leg1Mode {}
#[doc = "Field `LEG1_MODE` reader - Controls the usage mode of LEG1 and the Polarity bit"]
pub type Leg1ModeR = crate::FieldReader<Leg1Mode>;
impl Leg1ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leg1Mode {
        match self.bits {
            0 => Leg1Mode::GpStatic,
            1 => Leg1Mode::Gp,
            2 => Leg1Mode::CsdStatic,
            3 => Leg1Mode::Csd,
            _ => unreachable!(),
        }
    }
    #[doc = "General Purpose static mode: LEG1 and POLARITY are controlled by MMIO and optionally mixed with DSI (see DSI_CTRL_EN). No shunting is used, this saves power when off but also any on/off switching will take longer."]
    #[inline(always)]
    pub fn is_gp_static(&self) -> bool {
        *self == Leg1Mode::GpStatic
    }
    #[doc = "General Purpose dynamic mode: LEG1 and POLARITY are controlled by MMIO and optionally mixed with DSI (see DSI_CTRL_EN). Shunting is used, so on/off switching is faster, but power is wasted when the leg is disabled."]
    #[inline(always)]
    pub fn is_gp(&self) -> bool {
        *self == Leg1Mode::Gp
    }
    #[doc = "CSD static mode: LEG1 can only be on when the CSD Sequencer is in the Sample_init or Sample_norm state. In those states LEG1 is controlled by LEG1_EN, csd_sense and the CSD configuration. Polarity is controlled by the CSD configuration and operation. In addition leg1 enable and polarity can optionally be mixed with DSI (see DSI_CTRL_EN). No shunting is used, this saves power when off but also any on/off switching will take longer."]
    #[inline(always)]
    pub fn is_csd_static(&self) -> bool {
        *self == Leg1Mode::CsdStatic
    }
    #[doc = "CSD dynamic mode: LEG1 can only be on when the CSD Sequencer is in the Sample_init or Sample_norm state. In thoses states LEG1 is controlled by LEG1_EN, the CSD configuration, csd_sense and the flopped CSDCMP output (CSDCMP_OUT_FF). Polarity is controlled by the CSD configuration and operation. In addition leg1 enable and polarity can optionally be mixed with DSI (see DSI_CTRL_EN). Shunting is used, so on/off switching is faster, but power is wasted when the leg is disabled."]
    #[inline(always)]
    pub fn is_csd(&self) -> bool {
        *self == Leg1Mode::Csd
    }
}
#[doc = "Field `LEG1_MODE` writer - Controls the usage mode of LEG1 and the Polarity bit"]
pub type Leg1ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Leg1Mode, crate::Safe>;
impl<'a, REG> Leg1ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "General Purpose static mode: LEG1 and POLARITY are controlled by MMIO and optionally mixed with DSI (see DSI_CTRL_EN). No shunting is used, this saves power when off but also any on/off switching will take longer."]
    #[inline(always)]
    pub fn gp_static(self) -> &'a mut crate::W<REG> {
        self.variant(Leg1Mode::GpStatic)
    }
    #[doc = "General Purpose dynamic mode: LEG1 and POLARITY are controlled by MMIO and optionally mixed with DSI (see DSI_CTRL_EN). Shunting is used, so on/off switching is faster, but power is wasted when the leg is disabled."]
    #[inline(always)]
    pub fn gp(self) -> &'a mut crate::W<REG> {
        self.variant(Leg1Mode::Gp)
    }
    #[doc = "CSD static mode: LEG1 can only be on when the CSD Sequencer is in the Sample_init or Sample_norm state. In those states LEG1 is controlled by LEG1_EN, csd_sense and the CSD configuration. Polarity is controlled by the CSD configuration and operation. In addition leg1 enable and polarity can optionally be mixed with DSI (see DSI_CTRL_EN). No shunting is used, this saves power when off but also any on/off switching will take longer."]
    #[inline(always)]
    pub fn csd_static(self) -> &'a mut crate::W<REG> {
        self.variant(Leg1Mode::CsdStatic)
    }
    #[doc = "CSD dynamic mode: LEG1 can only be on when the CSD Sequencer is in the Sample_init or Sample_norm state. In thoses states LEG1 is controlled by LEG1_EN, the CSD configuration, csd_sense and the flopped CSDCMP output (CSDCMP_OUT_FF). Polarity is controlled by the CSD configuration and operation. In addition leg1 enable and polarity can optionally be mixed with DSI (see DSI_CTRL_EN). Shunting is used, so on/off switching is faster, but power is wasted when the leg is disabled."]
    #[inline(always)]
    pub fn csd(self) -> &'a mut crate::W<REG> {
        self.variant(Leg1Mode::Csd)
    }
}
#[doc = "Controls the usage mode of LEG2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Leg2Mode {
    #[doc = "0: General Purpose static mode: LEG2 is controlled by MMIO and optionally mixed with DSI (see DSI_CTRL_EN). No shunting is used, this saves power when off but also any on/off switching will take longer."]
    GpStatic = 0,
    #[doc = "1: General Purpose dynamic mode: LEG2 is controlled by MMIO and optionally mixed with DSI (see DSI_CTRL_EN). Shunting is used, so on/off switching is faster, but power is wasted when the leg is disabled."]
    Gp = 1,
    #[doc = "2: CSD static mode: LEG2 can only be on when the CSD Sequencer is in the Sample_init or Sample_norm state. In those states LEG2 is controlled by LEG2_EN, csd_sense and the CSD configuration. Polarity is controlled by the CSD configuration and operation. In addition leg2 enable and polarity can optionally be mixed with DSI (see DSI_CTRL_EN). No shunting is used, this saves power when off but also any on/off switching will take longer."]
    CsdStatic = 2,
    #[doc = "3: CSD dynamic mode: LEG2 can only be on when the CSD Sequencer is in the Sample_init or Sample_norm state. In those states LEG2 is controlled by LEG2_EN, the CSD configuration, csd_sense and the flopped CSDCMP output (CSDCMP_OUT_FF). In addition leg2 enable can optionally be mixed with DSI (see DSI_CTRL_EN). Shunting is used, so on/off switching is faster, but power is wasted when the leg is disabled."]
    Csd = 3,
}
impl From<Leg2Mode> for u8 {
    #[inline(always)]
    fn from(variant: Leg2Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Leg2Mode {
    type Ux = u8;
}
impl crate::IsEnum for Leg2Mode {}
#[doc = "Field `LEG2_MODE` reader - Controls the usage mode of LEG2"]
pub type Leg2ModeR = crate::FieldReader<Leg2Mode>;
impl Leg2ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leg2Mode {
        match self.bits {
            0 => Leg2Mode::GpStatic,
            1 => Leg2Mode::Gp,
            2 => Leg2Mode::CsdStatic,
            3 => Leg2Mode::Csd,
            _ => unreachable!(),
        }
    }
    #[doc = "General Purpose static mode: LEG2 is controlled by MMIO and optionally mixed with DSI (see DSI_CTRL_EN). No shunting is used, this saves power when off but also any on/off switching will take longer."]
    #[inline(always)]
    pub fn is_gp_static(&self) -> bool {
        *self == Leg2Mode::GpStatic
    }
    #[doc = "General Purpose dynamic mode: LEG2 is controlled by MMIO and optionally mixed with DSI (see DSI_CTRL_EN). Shunting is used, so on/off switching is faster, but power is wasted when the leg is disabled."]
    #[inline(always)]
    pub fn is_gp(&self) -> bool {
        *self == Leg2Mode::Gp
    }
    #[doc = "CSD static mode: LEG2 can only be on when the CSD Sequencer is in the Sample_init or Sample_norm state. In those states LEG2 is controlled by LEG2_EN, csd_sense and the CSD configuration. Polarity is controlled by the CSD configuration and operation. In addition leg2 enable and polarity can optionally be mixed with DSI (see DSI_CTRL_EN). No shunting is used, this saves power when off but also any on/off switching will take longer."]
    #[inline(always)]
    pub fn is_csd_static(&self) -> bool {
        *self == Leg2Mode::CsdStatic
    }
    #[doc = "CSD dynamic mode: LEG2 can only be on when the CSD Sequencer is in the Sample_init or Sample_norm state. In those states LEG2 is controlled by LEG2_EN, the CSD configuration, csd_sense and the flopped CSDCMP output (CSDCMP_OUT_FF). In addition leg2 enable can optionally be mixed with DSI (see DSI_CTRL_EN). Shunting is used, so on/off switching is faster, but power is wasted when the leg is disabled."]
    #[inline(always)]
    pub fn is_csd(&self) -> bool {
        *self == Leg2Mode::Csd
    }
}
#[doc = "Field `LEG2_MODE` writer - Controls the usage mode of LEG2"]
pub type Leg2ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Leg2Mode, crate::Safe>;
impl<'a, REG> Leg2ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "General Purpose static mode: LEG2 is controlled by MMIO and optionally mixed with DSI (see DSI_CTRL_EN). No shunting is used, this saves power when off but also any on/off switching will take longer."]
    #[inline(always)]
    pub fn gp_static(self) -> &'a mut crate::W<REG> {
        self.variant(Leg2Mode::GpStatic)
    }
    #[doc = "General Purpose dynamic mode: LEG2 is controlled by MMIO and optionally mixed with DSI (see DSI_CTRL_EN). Shunting is used, so on/off switching is faster, but power is wasted when the leg is disabled."]
    #[inline(always)]
    pub fn gp(self) -> &'a mut crate::W<REG> {
        self.variant(Leg2Mode::Gp)
    }
    #[doc = "CSD static mode: LEG2 can only be on when the CSD Sequencer is in the Sample_init or Sample_norm state. In those states LEG2 is controlled by LEG2_EN, csd_sense and the CSD configuration. Polarity is controlled by the CSD configuration and operation. In addition leg2 enable and polarity can optionally be mixed with DSI (see DSI_CTRL_EN). No shunting is used, this saves power when off but also any on/off switching will take longer."]
    #[inline(always)]
    pub fn csd_static(self) -> &'a mut crate::W<REG> {
        self.variant(Leg2Mode::CsdStatic)
    }
    #[doc = "CSD dynamic mode: LEG2 can only be on when the CSD Sequencer is in the Sample_init or Sample_norm state. In those states LEG2 is controlled by LEG2_EN, the CSD configuration, csd_sense and the flopped CSDCMP output (CSDCMP_OUT_FF). In addition leg2 enable can optionally be mixed with DSI (see DSI_CTRL_EN). Shunting is used, so on/off switching is faster, but power is wasted when the leg is disabled."]
    #[inline(always)]
    pub fn csd(self) -> &'a mut crate::W<REG> {
        self.variant(Leg2Mode::Csd)
    }
}
#[doc = "Field `DSI_CTRL_EN` reader - Mix DSI inputs with MMIO controls or not (before getting mixed with CSD controls if enabled). 0: no DSI control IDACA_POLARITY = IDACA.POLARITY IDACA_LEG1_EN = IDACA.LEG1_EN IDACA_LEG2_EN = IDACA.LEG2_EN 1: Mix MMIO with DSI control IDACA_POLARITY = IDACA.POLARITY EXOR dsi_idaca_pol IDACA_LEG1_EN = IDACA.LEG1_EN AND dsi_idaca_leg1_en IDACA_LEG2_EN = IDACA.LEG2_EN AND dsi_idaca_leg2_en"]
pub type DsiCtrlEnR = crate::BitReader;
#[doc = "Field `DSI_CTRL_EN` writer - Mix DSI inputs with MMIO controls or not (before getting mixed with CSD controls if enabled). 0: no DSI control IDACA_POLARITY = IDACA.POLARITY IDACA_LEG1_EN = IDACA.LEG1_EN IDACA_LEG2_EN = IDACA.LEG2_EN 1: Mix MMIO with DSI control IDACA_POLARITY = IDACA.POLARITY EXOR dsi_idaca_pol IDACA_LEG1_EN = IDACA.LEG1_EN AND dsi_idaca_leg1_en IDACA_LEG2_EN = IDACA.LEG2_EN AND dsi_idaca_leg2_en"]
pub type DsiCtrlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "IDAC multiplier\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Range {
    #[doc = "0: 1 LSB = 37.5 nA"]
    IdacLo = 0,
    #[doc = "1: 1 LSB = 300 nA"]
    IdacMed = 1,
    #[doc = "2: 1 LSB = 2400 nA"]
    IdacHi = 2,
}
impl From<Range> for u8 {
    #[inline(always)]
    fn from(variant: Range) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Range {
    type Ux = u8;
}
impl crate::IsEnum for Range {}
#[doc = "Field `RANGE` reader - IDAC multiplier"]
pub type RangeR = crate::FieldReader<Range>;
impl RangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Range> {
        match self.bits {
            0 => Some(Range::IdacLo),
            1 => Some(Range::IdacMed),
            2 => Some(Range::IdacHi),
            _ => None,
        }
    }
    #[doc = "1 LSB = 37.5 nA"]
    #[inline(always)]
    pub fn is_idac_lo(&self) -> bool {
        *self == Range::IdacLo
    }
    #[doc = "1 LSB = 300 nA"]
    #[inline(always)]
    pub fn is_idac_med(&self) -> bool {
        *self == Range::IdacMed
    }
    #[doc = "1 LSB = 2400 nA"]
    #[inline(always)]
    pub fn is_idac_hi(&self) -> bool {
        *self == Range::IdacHi
    }
}
#[doc = "Field `RANGE` writer - IDAC multiplier"]
pub type RangeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Range>;
impl<'a, REG> RangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 LSB = 37.5 nA"]
    #[inline(always)]
    pub fn idac_lo(self) -> &'a mut crate::W<REG> {
        self.variant(Range::IdacLo)
    }
    #[doc = "1 LSB = 300 nA"]
    #[inline(always)]
    pub fn idac_med(self) -> &'a mut crate::W<REG> {
        self.variant(Range::IdacMed)
    }
    #[doc = "1 LSB = 2400 nA"]
    #[inline(always)]
    pub fn idac_hi(self) -> &'a mut crate::W<REG> {
        self.variant(Range::IdacHi)
    }
}
#[doc = "Field `LEG1_EN` reader - output enable for leg 1 to CSDBUSA"]
pub type Leg1EnR = crate::BitReader;
#[doc = "Field `LEG1_EN` writer - output enable for leg 1 to CSDBUSA"]
pub type Leg1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEG2_EN` reader - output enable for leg 2 to CSDBUSA"]
pub type Leg2EnR = crate::BitReader;
#[doc = "Field `LEG2_EN` writer - output enable for leg 2 to CSDBUSA"]
pub type Leg2EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Current value setting for this IDAC (7 bits)."]
    #[inline(always)]
    pub fn val(&self) -> ValR {
        ValR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Polarity is dynamic, this bit does not influence the logic in the SoftIP, it only goes to the HardIP."]
    #[inline(always)]
    pub fn pol_dyn(&self) -> PolDynR {
        PolDynR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Selects the polarity of the IDAC (sensing operation). Normally the actual polarity depends on this bit, optionally mixed with DSI (see DSI_CTRL_EN) and if LEG1_MODE==CSD also mixed with the CSD configuration and operation. However in mutual cap mode with one IDAC (config.mutual_cap=1 &amp; config.csx_dual_idac=0) the polarity of the IDAC is controlled by csd_sense."]
    #[inline(always)]
    pub fn polarity(&self) -> PolarityR {
        PolarityR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Balancing mode: only applies to legs configured as CSD."]
    #[inline(always)]
    pub fn bal_mode(&self) -> BalModeR {
        BalModeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Controls the usage mode of LEG1 and the Polarity bit"]
    #[inline(always)]
    pub fn leg1_mode(&self) -> Leg1ModeR {
        Leg1ModeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Controls the usage mode of LEG2"]
    #[inline(always)]
    pub fn leg2_mode(&self) -> Leg2ModeR {
        Leg2ModeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 21 - Mix DSI inputs with MMIO controls or not (before getting mixed with CSD controls if enabled). 0: no DSI control IDACA_POLARITY = IDACA.POLARITY IDACA_LEG1_EN = IDACA.LEG1_EN IDACA_LEG2_EN = IDACA.LEG2_EN 1: Mix MMIO with DSI control IDACA_POLARITY = IDACA.POLARITY EXOR dsi_idaca_pol IDACA_LEG1_EN = IDACA.LEG1_EN AND dsi_idaca_leg1_en IDACA_LEG2_EN = IDACA.LEG2_EN AND dsi_idaca_leg2_en"]
    #[inline(always)]
    pub fn dsi_ctrl_en(&self) -> DsiCtrlEnR {
        DsiCtrlEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - IDAC multiplier"]
    #[inline(always)]
    pub fn range(&self) -> RangeR {
        RangeR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - output enable for leg 1 to CSDBUSA"]
    #[inline(always)]
    pub fn leg1_en(&self) -> Leg1EnR {
        Leg1EnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - output enable for leg 2 to CSDBUSA"]
    #[inline(always)]
    pub fn leg2_en(&self) -> Leg2EnR {
        Leg2EnR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Current value setting for this IDAC (7 bits)."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> ValW<IdacaSpec> {
        ValW::new(self, 0)
    }
    #[doc = "Bit 7 - Polarity is dynamic, this bit does not influence the logic in the SoftIP, it only goes to the HardIP."]
    #[inline(always)]
    #[must_use]
    pub fn pol_dyn(&mut self) -> PolDynW<IdacaSpec> {
        PolDynW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Selects the polarity of the IDAC (sensing operation). Normally the actual polarity depends on this bit, optionally mixed with DSI (see DSI_CTRL_EN) and if LEG1_MODE==CSD also mixed with the CSD configuration and operation. However in mutual cap mode with one IDAC (config.mutual_cap=1 &amp; config.csx_dual_idac=0) the polarity of the IDAC is controlled by csd_sense."]
    #[inline(always)]
    #[must_use]
    pub fn polarity(&mut self) -> PolarityW<IdacaSpec> {
        PolarityW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Balancing mode: only applies to legs configured as CSD."]
    #[inline(always)]
    #[must_use]
    pub fn bal_mode(&mut self) -> BalModeW<IdacaSpec> {
        BalModeW::new(self, 10)
    }
    #[doc = "Bits 16:17 - Controls the usage mode of LEG1 and the Polarity bit"]
    #[inline(always)]
    #[must_use]
    pub fn leg1_mode(&mut self) -> Leg1ModeW<IdacaSpec> {
        Leg1ModeW::new(self, 16)
    }
    #[doc = "Bits 18:19 - Controls the usage mode of LEG2"]
    #[inline(always)]
    #[must_use]
    pub fn leg2_mode(&mut self) -> Leg2ModeW<IdacaSpec> {
        Leg2ModeW::new(self, 18)
    }
    #[doc = "Bit 21 - Mix DSI inputs with MMIO controls or not (before getting mixed with CSD controls if enabled). 0: no DSI control IDACA_POLARITY = IDACA.POLARITY IDACA_LEG1_EN = IDACA.LEG1_EN IDACA_LEG2_EN = IDACA.LEG2_EN 1: Mix MMIO with DSI control IDACA_POLARITY = IDACA.POLARITY EXOR dsi_idaca_pol IDACA_LEG1_EN = IDACA.LEG1_EN AND dsi_idaca_leg1_en IDACA_LEG2_EN = IDACA.LEG2_EN AND dsi_idaca_leg2_en"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_ctrl_en(&mut self) -> DsiCtrlEnW<IdacaSpec> {
        DsiCtrlEnW::new(self, 21)
    }
    #[doc = "Bits 22:23 - IDAC multiplier"]
    #[inline(always)]
    #[must_use]
    pub fn range(&mut self) -> RangeW<IdacaSpec> {
        RangeW::new(self, 22)
    }
    #[doc = "Bit 24 - output enable for leg 1 to CSDBUSA"]
    #[inline(always)]
    #[must_use]
    pub fn leg1_en(&mut self) -> Leg1EnW<IdacaSpec> {
        Leg1EnW::new(self, 24)
    }
    #[doc = "Bit 25 - output enable for leg 2 to CSDBUSA"]
    #[inline(always)]
    #[must_use]
    pub fn leg2_en(&mut self) -> Leg2EnW<IdacaSpec> {
        Leg2EnW::new(self, 25)
    }
}
#[doc = "IDACA Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`idaca::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idaca::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdacaSpec;
impl crate::RegisterSpec for IdacaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idaca::R`](R) reader structure"]
impl crate::Readable for IdacaSpec {}
#[doc = "`write(|w| ..)` method takes [`idaca::W`](W) writer structure"]
impl crate::Writable for IdacaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDACA to value 0"]
impl crate::Resettable for IdacaSpec {
    const RESET_VALUE: u32 = 0;
}
