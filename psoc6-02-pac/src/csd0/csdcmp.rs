#[doc = "Register `CSDCMP` reader"]
pub type R = crate::R<CsdcmpSpec>;
#[doc = "Register `CSDCMP` writer"]
pub type W = crate::W<CsdcmpSpec>;
#[doc = "CSD Comparator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CsdcmpEn {
    #[doc = "0: Disable comparator, output is zero"]
    Off = 0,
    #[doc = "1: On, regular operation. Note that CONFIG.LP_MODE determines the power mode level"]
    On = 1,
}
impl From<CsdcmpEn> for bool {
    #[inline(always)]
    fn from(variant: CsdcmpEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSDCMP_EN` reader - CSD Comparator Enable"]
pub type CsdcmpEnR = crate::BitReader<CsdcmpEn>;
impl CsdcmpEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CsdcmpEn {
        match self.bits {
            false => CsdcmpEn::Off,
            true => CsdcmpEn::On,
        }
    }
    #[doc = "Disable comparator, output is zero"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == CsdcmpEn::Off
    }
    #[doc = "On, regular operation. Note that CONFIG.LP_MODE determines the power mode level"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == CsdcmpEn::On
    }
}
#[doc = "Field `CSDCMP_EN` writer - CSD Comparator Enable"]
pub type CsdcmpEnW<'a, REG> = crate::BitWriter<'a, REG, CsdcmpEn>;
impl<'a, REG> CsdcmpEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable comparator, output is zero"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(CsdcmpEn::Off)
    }
    #[doc = "On, regular operation. Note that CONFIG.LP_MODE determines the power mode level"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(CsdcmpEn::On)
    }
}
#[doc = "Select which IDAC polarity to use to detect CSDCMP triggering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PolaritySel {
    #[doc = "0: Use idaca_pol (firmware setting with CSX and optionally DSI mixed in) to determine the direction, this is the most common use-case, used for normal CSD and normal CSX"]
    IdacaPol = 0,
    #[doc = "1: Use idacb_pol (firmware setting with optional DSI mixed in) to determine the direction, this is only used for normal CSD if IDACB is used i.s.o. IDACA (not common)"]
    IdacbPol = 1,
    #[doc = "2: Use the expression (csd_sense ? idaca_pol : idacb_pol) to determine the direction, this is only useful for the CSX with DUAL_IDAC use-case"]
    DualPol = 2,
}
impl From<PolaritySel> for u8 {
    #[inline(always)]
    fn from(variant: PolaritySel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PolaritySel {
    type Ux = u8;
}
impl crate::IsEnum for PolaritySel {}
#[doc = "Field `POLARITY_SEL` reader - Select which IDAC polarity to use to detect CSDCMP triggering"]
pub type PolaritySelR = crate::FieldReader<PolaritySel>;
impl PolaritySelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PolaritySel> {
        match self.bits {
            0 => Some(PolaritySel::IdacaPol),
            1 => Some(PolaritySel::IdacbPol),
            2 => Some(PolaritySel::DualPol),
            _ => None,
        }
    }
    #[doc = "Use idaca_pol (firmware setting with CSX and optionally DSI mixed in) to determine the direction, this is the most common use-case, used for normal CSD and normal CSX"]
    #[inline(always)]
    pub fn is_idaca_pol(&self) -> bool {
        *self == PolaritySel::IdacaPol
    }
    #[doc = "Use idacb_pol (firmware setting with optional DSI mixed in) to determine the direction, this is only used for normal CSD if IDACB is used i.s.o. IDACA (not common)"]
    #[inline(always)]
    pub fn is_idacb_pol(&self) -> bool {
        *self == PolaritySel::IdacbPol
    }
    #[doc = "Use the expression (csd_sense ? idaca_pol : idacb_pol) to determine the direction, this is only useful for the CSX with DUAL_IDAC use-case"]
    #[inline(always)]
    pub fn is_dual_pol(&self) -> bool {
        *self == PolaritySel::DualPol
    }
}
#[doc = "Field `POLARITY_SEL` writer - Select which IDAC polarity to use to detect CSDCMP triggering"]
pub type PolaritySelW<'a, REG> = crate::FieldWriter<'a, REG, 2, PolaritySel>;
impl<'a, REG> PolaritySelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use idaca_pol (firmware setting with CSX and optionally DSI mixed in) to determine the direction, this is the most common use-case, used for normal CSD and normal CSX"]
    #[inline(always)]
    pub fn idaca_pol(self) -> &'a mut crate::W<REG> {
        self.variant(PolaritySel::IdacaPol)
    }
    #[doc = "Use idacb_pol (firmware setting with optional DSI mixed in) to determine the direction, this is only used for normal CSD if IDACB is used i.s.o. IDACA (not common)"]
    #[inline(always)]
    pub fn idacb_pol(self) -> &'a mut crate::W<REG> {
        self.variant(PolaritySel::IdacbPol)
    }
    #[doc = "Use the expression (csd_sense ? idaca_pol : idacb_pol) to determine the direction, this is only useful for the CSX with DUAL_IDAC use-case"]
    #[inline(always)]
    pub fn dual_pol(self) -> &'a mut crate::W<REG> {
        self.variant(PolaritySel::DualPol)
    }
}
#[doc = "Select in what phase(s) the comparator is active, typically set to match the BAL_MODE of the used IDAC. Note, this also determines when a bad conversion is detected, namely at the beginning and end of the comparator active phase (also taking into account FILTER_DELAY and non-overlap).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CmpPhase {
    #[doc = "0: Comparator is active from start of Phi2 and kept active into Phi1. Intended usage: legacy CSD for balancing over a full csd_sense period (non-overlap should be turned off)"]
    Full = 0,
    #[doc = "1: Comparator is active during Phi1 only. Currently no known use-case."]
    Phi1 = 1,
    #[doc = "2: Comparator is active during Phi2 only. Intended usage: CSD Low EMI."]
    Phi2 = 2,
    #[doc = "3: Comparator is activated at the start of both Phi1 and Phi2 (non-overlap should be enabled). Intended usage: CSX, or Full-Wave."]
    Phi1_2 = 3,
}
impl From<CmpPhase> for u8 {
    #[inline(always)]
    fn from(variant: CmpPhase) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CmpPhase {
    type Ux = u8;
}
impl crate::IsEnum for CmpPhase {}
#[doc = "Field `CMP_PHASE` reader - Select in what phase(s) the comparator is active, typically set to match the BAL_MODE of the used IDAC. Note, this also determines when a bad conversion is detected, namely at the beginning and end of the comparator active phase (also taking into account FILTER_DELAY and non-overlap)."]
pub type CmpPhaseR = crate::FieldReader<CmpPhase>;
impl CmpPhaseR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CmpPhase {
        match self.bits {
            0 => CmpPhase::Full,
            1 => CmpPhase::Phi1,
            2 => CmpPhase::Phi2,
            3 => CmpPhase::Phi1_2,
            _ => unreachable!(),
        }
    }
    #[doc = "Comparator is active from start of Phi2 and kept active into Phi1. Intended usage: legacy CSD for balancing over a full csd_sense period (non-overlap should be turned off)"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == CmpPhase::Full
    }
    #[doc = "Comparator is active during Phi1 only. Currently no known use-case."]
    #[inline(always)]
    pub fn is_phi1(&self) -> bool {
        *self == CmpPhase::Phi1
    }
    #[doc = "Comparator is active during Phi2 only. Intended usage: CSD Low EMI."]
    #[inline(always)]
    pub fn is_phi2(&self) -> bool {
        *self == CmpPhase::Phi2
    }
    #[doc = "Comparator is activated at the start of both Phi1 and Phi2 (non-overlap should be enabled). Intended usage: CSX, or Full-Wave."]
    #[inline(always)]
    pub fn is_phi1_2(&self) -> bool {
        *self == CmpPhase::Phi1_2
    }
}
#[doc = "Field `CMP_PHASE` writer - Select in what phase(s) the comparator is active, typically set to match the BAL_MODE of the used IDAC. Note, this also determines when a bad conversion is detected, namely at the beginning and end of the comparator active phase (also taking into account FILTER_DELAY and non-overlap)."]
pub type CmpPhaseW<'a, REG> = crate::FieldWriter<'a, REG, 2, CmpPhase, crate::Safe>;
impl<'a, REG> CmpPhaseW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Comparator is active from start of Phi2 and kept active into Phi1. Intended usage: legacy CSD for balancing over a full csd_sense period (non-overlap should be turned off)"]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(CmpPhase::Full)
    }
    #[doc = "Comparator is active during Phi1 only. Currently no known use-case."]
    #[inline(always)]
    pub fn phi1(self) -> &'a mut crate::W<REG> {
        self.variant(CmpPhase::Phi1)
    }
    #[doc = "Comparator is active during Phi2 only. Intended usage: CSD Low EMI."]
    #[inline(always)]
    pub fn phi2(self) -> &'a mut crate::W<REG> {
        self.variant(CmpPhase::Phi2)
    }
    #[doc = "Comparator is activated at the start of both Phi1 and Phi2 (non-overlap should be enabled). Intended usage: CSX, or Full-Wave."]
    #[inline(always)]
    pub fn phi1_2(self) -> &'a mut crate::W<REG> {
        self.variant(CmpPhase::Phi1_2)
    }
}
#[doc = "Select which signal to output on dsi_sample_out.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmpMode {
    #[doc = "0: CSD mode: output the filtered sample signal on dsi_sample_out"]
    Csd = 0,
    #[doc = "1: General Purpose mode: output the unfiltered sample unfiltered comparator output, either asynchronous or flopped."]
    Gp = 1,
}
impl From<CmpMode> for bool {
    #[inline(always)]
    fn from(variant: CmpMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP_MODE` reader - Select which signal to output on dsi_sample_out."]
pub type CmpModeR = crate::BitReader<CmpMode>;
impl CmpModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CmpMode {
        match self.bits {
            false => CmpMode::Csd,
            true => CmpMode::Gp,
        }
    }
    #[doc = "CSD mode: output the filtered sample signal on dsi_sample_out"]
    #[inline(always)]
    pub fn is_csd(&self) -> bool {
        *self == CmpMode::Csd
    }
    #[doc = "General Purpose mode: output the unfiltered sample unfiltered comparator output, either asynchronous or flopped."]
    #[inline(always)]
    pub fn is_gp(&self) -> bool {
        *self == CmpMode::Gp
    }
}
#[doc = "Field `CMP_MODE` writer - Select which signal to output on dsi_sample_out."]
pub type CmpModeW<'a, REG> = crate::BitWriter<'a, REG, CmpMode>;
impl<'a, REG> CmpModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CSD mode: output the filtered sample signal on dsi_sample_out"]
    #[inline(always)]
    pub fn csd(self) -> &'a mut crate::W<REG> {
        self.variant(CmpMode::Csd)
    }
    #[doc = "General Purpose mode: output the unfiltered sample unfiltered comparator output, either asynchronous or flopped."]
    #[inline(always)]
    pub fn gp(self) -> &'a mut crate::W<REG> {
        self.variant(CmpMode::Gp)
    }
}
#[doc = "This bit controls whether the output directly from the comparator (csdcmp_out) or the flopped version (csdcmp_out_ff) is used. For CSD operation, the selected signal controls the IDAC(s), in GP mode the signal goes out on dsi_sample_out.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FeedbackMode {
    #[doc = "0: Use feedback from sampling flip-flop (used in most modes)."]
    Flop = 0,
    #[doc = "1: Use feedback from comparator directly (used in single Cmod mutual cap sensing only)"]
    Comp = 1,
}
impl From<FeedbackMode> for bool {
    #[inline(always)]
    fn from(variant: FeedbackMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FEEDBACK_MODE` reader - This bit controls whether the output directly from the comparator (csdcmp_out) or the flopped version (csdcmp_out_ff) is used. For CSD operation, the selected signal controls the IDAC(s), in GP mode the signal goes out on dsi_sample_out."]
pub type FeedbackModeR = crate::BitReader<FeedbackMode>;
impl FeedbackModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FeedbackMode {
        match self.bits {
            false => FeedbackMode::Flop,
            true => FeedbackMode::Comp,
        }
    }
    #[doc = "Use feedback from sampling flip-flop (used in most modes)."]
    #[inline(always)]
    pub fn is_flop(&self) -> bool {
        *self == FeedbackMode::Flop
    }
    #[doc = "Use feedback from comparator directly (used in single Cmod mutual cap sensing only)"]
    #[inline(always)]
    pub fn is_comp(&self) -> bool {
        *self == FeedbackMode::Comp
    }
}
#[doc = "Field `FEEDBACK_MODE` writer - This bit controls whether the output directly from the comparator (csdcmp_out) or the flopped version (csdcmp_out_ff) is used. For CSD operation, the selected signal controls the IDAC(s), in GP mode the signal goes out on dsi_sample_out."]
pub type FeedbackModeW<'a, REG> = crate::BitWriter<'a, REG, FeedbackMode>;
impl<'a, REG> FeedbackModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use feedback from sampling flip-flop (used in most modes)."]
    #[inline(always)]
    pub fn flop(self) -> &'a mut crate::W<REG> {
        self.variant(FeedbackMode::Flop)
    }
    #[doc = "Use feedback from comparator directly (used in single Cmod mutual cap sensing only)"]
    #[inline(always)]
    pub fn comp(self) -> &'a mut crate::W<REG> {
        self.variant(FeedbackMode::Comp)
    }
}
#[doc = "Field `AZ_EN` reader - Auto-Zero enable, allow the Sequencer to Auto-Zero this component"]
pub type AzEnR = crate::BitReader;
#[doc = "Field `AZ_EN` writer - Auto-Zero enable, allow the Sequencer to Auto-Zero this component"]
pub type AzEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CSD Comparator Enable"]
    #[inline(always)]
    pub fn csdcmp_en(&self) -> CsdcmpEnR {
        CsdcmpEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Select which IDAC polarity to use to detect CSDCMP triggering"]
    #[inline(always)]
    pub fn polarity_sel(&self) -> PolaritySelR {
        PolaritySelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Select in what phase(s) the comparator is active, typically set to match the BAL_MODE of the used IDAC. Note, this also determines when a bad conversion is detected, namely at the beginning and end of the comparator active phase (also taking into account FILTER_DELAY and non-overlap)."]
    #[inline(always)]
    pub fn cmp_phase(&self) -> CmpPhaseR {
        CmpPhaseR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 28 - Select which signal to output on dsi_sample_out."]
    #[inline(always)]
    pub fn cmp_mode(&self) -> CmpModeR {
        CmpModeR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit controls whether the output directly from the comparator (csdcmp_out) or the flopped version (csdcmp_out_ff) is used. For CSD operation, the selected signal controls the IDAC(s), in GP mode the signal goes out on dsi_sample_out."]
    #[inline(always)]
    pub fn feedback_mode(&self) -> FeedbackModeR {
        FeedbackModeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Auto-Zero enable, allow the Sequencer to Auto-Zero this component"]
    #[inline(always)]
    pub fn az_en(&self) -> AzEnR {
        AzEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CSD Comparator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csdcmp_en(&mut self) -> CsdcmpEnW<CsdcmpSpec> {
        CsdcmpEnW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Select which IDAC polarity to use to detect CSDCMP triggering"]
    #[inline(always)]
    #[must_use]
    pub fn polarity_sel(&mut self) -> PolaritySelW<CsdcmpSpec> {
        PolaritySelW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Select in what phase(s) the comparator is active, typically set to match the BAL_MODE of the used IDAC. Note, this also determines when a bad conversion is detected, namely at the beginning and end of the comparator active phase (also taking into account FILTER_DELAY and non-overlap)."]
    #[inline(always)]
    #[must_use]
    pub fn cmp_phase(&mut self) -> CmpPhaseW<CsdcmpSpec> {
        CmpPhaseW::new(self, 8)
    }
    #[doc = "Bit 28 - Select which signal to output on dsi_sample_out."]
    #[inline(always)]
    #[must_use]
    pub fn cmp_mode(&mut self) -> CmpModeW<CsdcmpSpec> {
        CmpModeW::new(self, 28)
    }
    #[doc = "Bit 29 - This bit controls whether the output directly from the comparator (csdcmp_out) or the flopped version (csdcmp_out_ff) is used. For CSD operation, the selected signal controls the IDAC(s), in GP mode the signal goes out on dsi_sample_out."]
    #[inline(always)]
    #[must_use]
    pub fn feedback_mode(&mut self) -> FeedbackModeW<CsdcmpSpec> {
        FeedbackModeW::new(self, 29)
    }
    #[doc = "Bit 31 - Auto-Zero enable, allow the Sequencer to Auto-Zero this component"]
    #[inline(always)]
    #[must_use]
    pub fn az_en(&mut self) -> AzEnW<CsdcmpSpec> {
        AzEnW::new(self, 31)
    }
}
#[doc = "CSD Comparator configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`csdcmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csdcmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CsdcmpSpec;
impl crate::RegisterSpec for CsdcmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csdcmp::R`](R) reader structure"]
impl crate::Readable for CsdcmpSpec {}
#[doc = "`write(|w| ..)` method takes [`csdcmp::W`](W) writer structure"]
impl crate::Writable for CsdcmpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSDCMP to value 0"]
impl crate::Resettable for CsdcmpSpec {
    const RESET_VALUE: u32 = 0;
}
