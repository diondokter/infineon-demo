#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Select Iref supply.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IrefSel {
    #[doc = "0: select SRSS Iref (default)"]
    IrefSrss = 0,
    #[doc = "1: select PASS.AREF Iref, only available if PASS IP is on the chip."]
    IrefPass = 1,
}
impl From<IrefSel> for bool {
    #[inline(always)]
    fn from(variant: IrefSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IREF_SEL` reader - Select Iref supply."]
pub type IrefSelR = crate::BitReader<IrefSel>;
impl IrefSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IrefSel {
        match self.bits {
            false => IrefSel::IrefSrss,
            true => IrefSel::IrefPass,
        }
    }
    #[doc = "select SRSS Iref (default)"]
    #[inline(always)]
    pub fn is_iref_srss(&self) -> bool {
        *self == IrefSel::IrefSrss
    }
    #[doc = "select PASS.AREF Iref, only available if PASS IP is on the chip."]
    #[inline(always)]
    pub fn is_iref_pass(&self) -> bool {
        *self == IrefSel::IrefPass
    }
}
#[doc = "Field `IREF_SEL` writer - Select Iref supply."]
pub type IrefSelW<'a, REG> = crate::BitWriter<'a, REG, IrefSel>;
impl<'a, REG> IrefSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "select SRSS Iref (default)"]
    #[inline(always)]
    pub fn iref_srss(self) -> &'a mut crate::W<REG> {
        self.variant(IrefSel::IrefSrss)
    }
    #[doc = "select PASS.AREF Iref, only available if PASS IP is on the chip."]
    #[inline(always)]
    pub fn iref_pass(self) -> &'a mut crate::W<REG> {
        self.variant(IrefSel::IrefPass)
    }
}
#[doc = "Field `FILTER_DELAY` reader - This value determines the number of cycles that the digital filter makes the CSDCMP output ignored while the counter counts and IDAC is on. When set to 0 the digital filter is off. When set to any other value the ignoring will last for FILTER_DELAY clk_csd cycles after the start of each measurement and from the first comparator trip to the end of each measurement."]
pub type FilterDelayR = crate::FieldReader;
#[doc = "Field `FILTER_DELAY` writer - This value determines the number of cycles that the digital filter makes the CSDCMP output ignored while the counter counts and IDAC is on. When set to 0 the digital filter is off. When set to any other value the ignoring will last for FILTER_DELAY clk_csd cycles after the start of each measurement and from the first comparator trip to the end of each measurement."]
pub type FilterDelayW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Selects the delay by which csd_shield is delayed relative to csd_sense.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ShieldDelay {
    #[doc = "0: Delay line is off, csd_shield=csd_sense"]
    Off = 0,
    #[doc = "1: Introduces a 5ns delay (typ)"]
    D5ns = 1,
    #[doc = "2: Introduces a 10ns delay (typ)"]
    D10ns = 2,
    #[doc = "3: Introduces a 20ns delay (typ)"]
    D20ns = 3,
}
impl From<ShieldDelay> for u8 {
    #[inline(always)]
    fn from(variant: ShieldDelay) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ShieldDelay {
    type Ux = u8;
}
impl crate::IsEnum for ShieldDelay {}
#[doc = "Field `SHIELD_DELAY` reader - Selects the delay by which csd_shield is delayed relative to csd_sense."]
pub type ShieldDelayR = crate::FieldReader<ShieldDelay>;
impl ShieldDelayR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ShieldDelay {
        match self.bits {
            0 => ShieldDelay::Off,
            1 => ShieldDelay::D5ns,
            2 => ShieldDelay::D10ns,
            3 => ShieldDelay::D20ns,
            _ => unreachable!(),
        }
    }
    #[doc = "Delay line is off, csd_shield=csd_sense"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == ShieldDelay::Off
    }
    #[doc = "Introduces a 5ns delay (typ)"]
    #[inline(always)]
    pub fn is_d5ns(&self) -> bool {
        *self == ShieldDelay::D5ns
    }
    #[doc = "Introduces a 10ns delay (typ)"]
    #[inline(always)]
    pub fn is_d10ns(&self) -> bool {
        *self == ShieldDelay::D10ns
    }
    #[doc = "Introduces a 20ns delay (typ)"]
    #[inline(always)]
    pub fn is_d20ns(&self) -> bool {
        *self == ShieldDelay::D20ns
    }
}
#[doc = "Field `SHIELD_DELAY` writer - Selects the delay by which csd_shield is delayed relative to csd_sense."]
pub type ShieldDelayW<'a, REG> = crate::FieldWriter<'a, REG, 2, ShieldDelay, crate::Safe>;
impl<'a, REG> ShieldDelayW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Delay line is off, csd_shield=csd_sense"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(ShieldDelay::Off)
    }
    #[doc = "Introduces a 5ns delay (typ)"]
    #[inline(always)]
    pub fn d5ns(self) -> &'a mut crate::W<REG> {
        self.variant(ShieldDelay::D5ns)
    }
    #[doc = "Introduces a 10ns delay (typ)"]
    #[inline(always)]
    pub fn d10ns(self) -> &'a mut crate::W<REG> {
        self.variant(ShieldDelay::D10ns)
    }
    #[doc = "Introduces a 20ns delay (typ)"]
    #[inline(always)]
    pub fn d20ns(self) -> &'a mut crate::W<REG> {
        self.variant(ShieldDelay::D20ns)
    }
}
#[doc = "Field `SENSE_EN` reader - Enables the sense modulator output. 0: all switches, static or dynamic, are open and IDAC in CSD mode is off 1: switches and IDAC can be closed/on as per MMIO setting and CSD sequencer."]
pub type SenseEnR = crate::BitReader;
#[doc = "Field `SENSE_EN` writer - Enables the sense modulator output. 0: all switches, static or dynamic, are open and IDAC in CSD mode is off 1: switches and IDAC can be closed/on as per MMIO setting and CSD sequencer."]
pub type SenseEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Enables full wave cap sensing mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FullWave {
    #[doc = "0: Half Wave mode (normal). In this mode the comparator always trips in the same direction (positive or negative edge) and the same Vref, i.e. no polarity change."]
    Halfwave = 0,
    #[doc = "1: Full Wave mode. In this mode the comparator trips in opposite direction and with different Vref in each phase, i.e. the polarity flips."]
    Fullwave = 1,
}
impl From<FullWave> for bool {
    #[inline(always)]
    fn from(variant: FullWave) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FULL_WAVE` reader - Enables full wave cap sensing mode"]
pub type FullWaveR = crate::BitReader<FullWave>;
impl FullWaveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FullWave {
        match self.bits {
            false => FullWave::Halfwave,
            true => FullWave::Fullwave,
        }
    }
    #[doc = "Half Wave mode (normal). In this mode the comparator always trips in the same direction (positive or negative edge) and the same Vref, i.e. no polarity change."]
    #[inline(always)]
    pub fn is_halfwave(&self) -> bool {
        *self == FullWave::Halfwave
    }
    #[doc = "Full Wave mode. In this mode the comparator trips in opposite direction and with different Vref in each phase, i.e. the polarity flips."]
    #[inline(always)]
    pub fn is_fullwave(&self) -> bool {
        *self == FullWave::Fullwave
    }
}
#[doc = "Field `FULL_WAVE` writer - Enables full wave cap sensing mode"]
pub type FullWaveW<'a, REG> = crate::BitWriter<'a, REG, FullWave>;
impl<'a, REG> FullWaveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Half Wave mode (normal). In this mode the comparator always trips in the same direction (positive or negative edge) and the same Vref, i.e. no polarity change."]
    #[inline(always)]
    pub fn halfwave(self) -> &'a mut crate::W<REG> {
        self.variant(FullWave::Halfwave)
    }
    #[doc = "Full Wave mode. In this mode the comparator trips in opposite direction and with different Vref in each phase, i.e. the polarity flips."]
    #[inline(always)]
    pub fn fullwave(self) -> &'a mut crate::W<REG> {
        self.variant(FullWave::Fullwave)
    }
}
#[doc = "Enables mutual cap sensing mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MutualCap {
    #[doc = "0: Self-cap mode (configure sense line as CSD_SENSE)"]
    Selfcap = 0,
    #[doc = "1: Mutual-cap mode (configure Tx line as CSD_SENSE, inverted Tx line as CSD_SHIELD and Rx Line as AMUXA). In this mode the polarity bit of the IDAC is controlled by csd_sense."]
    Mutualcap = 1,
}
impl From<MutualCap> for bool {
    #[inline(always)]
    fn from(variant: MutualCap) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUTUAL_CAP` reader - Enables mutual cap sensing mode"]
pub type MutualCapR = crate::BitReader<MutualCap>;
impl MutualCapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MutualCap {
        match self.bits {
            false => MutualCap::Selfcap,
            true => MutualCap::Mutualcap,
        }
    }
    #[doc = "Self-cap mode (configure sense line as CSD_SENSE)"]
    #[inline(always)]
    pub fn is_selfcap(&self) -> bool {
        *self == MutualCap::Selfcap
    }
    #[doc = "Mutual-cap mode (configure Tx line as CSD_SENSE, inverted Tx line as CSD_SHIELD and Rx Line as AMUXA). In this mode the polarity bit of the IDAC is controlled by csd_sense."]
    #[inline(always)]
    pub fn is_mutualcap(&self) -> bool {
        *self == MutualCap::Mutualcap
    }
}
#[doc = "Field `MUTUAL_CAP` writer - Enables mutual cap sensing mode"]
pub type MutualCapW<'a, REG> = crate::BitWriter<'a, REG, MutualCap>;
impl<'a, REG> MutualCapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Self-cap mode (configure sense line as CSD_SENSE)"]
    #[inline(always)]
    pub fn selfcap(self) -> &'a mut crate::W<REG> {
        self.variant(MutualCap::Selfcap)
    }
    #[doc = "Mutual-cap mode (configure Tx line as CSD_SENSE, inverted Tx line as CSD_SHIELD and Rx Line as AMUXA). In this mode the polarity bit of the IDAC is controlled by csd_sense."]
    #[inline(always)]
    pub fn mutualcap(self) -> &'a mut crate::W<REG> {
        self.variant(MutualCap::Mutualcap)
    }
}
#[doc = "Enable the use of two counters for MUTUAL cap sensing mode (CSX), do not use when MUTUAL_CAP=0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CsxDualCnt {
    #[doc = "0: Use one counter for both phases (source and sink)."]
    One = 0,
    #[doc = "1: Use two counters, separate count for when csd_sense is high and when csd_sense is low."]
    Two = 1,
}
impl From<CsxDualCnt> for bool {
    #[inline(always)]
    fn from(variant: CsxDualCnt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSX_DUAL_CNT` reader - Enable the use of two counters for MUTUAL cap sensing mode (CSX), do not use when MUTUAL_CAP=0"]
pub type CsxDualCntR = crate::BitReader<CsxDualCnt>;
impl CsxDualCntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CsxDualCnt {
        match self.bits {
            false => CsxDualCnt::One,
            true => CsxDualCnt::Two,
        }
    }
    #[doc = "Use one counter for both phases (source and sink)."]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CsxDualCnt::One
    }
    #[doc = "Use two counters, separate count for when csd_sense is high and when csd_sense is low."]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == CsxDualCnt::Two
    }
}
#[doc = "Field `CSX_DUAL_CNT` writer - Enable the use of two counters for MUTUAL cap sensing mode (CSX), do not use when MUTUAL_CAP=0"]
pub type CsxDualCntW<'a, REG> = crate::BitWriter<'a, REG, CsxDualCnt>;
impl<'a, REG> CsxDualCntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use one counter for both phases (source and sink)."]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(CsxDualCnt::One)
    }
    #[doc = "Use two counters, separate count for when csd_sense is high and when csd_sense is low."]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(CsxDualCnt::Two)
    }
}
#[doc = "Select what to output on the dsi_count bus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DsiCountSel {
    #[doc = "0: depending on the dsi_count_val_sel input either output RESULT_VAL1.VALUE (0) or RESULT_VAL2.VALUE (1) on the dsi_count bus. Note that dsi_count_val_sel is not synchronized, i.e. it controls the mux combinatorially."]
    CsdResult = 0,
    #[doc = "1: output ADC_RES.VIN_CNT on the dsi_count bus"]
    AdcResult = 1,
}
impl From<DsiCountSel> for bool {
    #[inline(always)]
    fn from(variant: DsiCountSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSI_COUNT_SEL` reader - Select what to output on the dsi_count bus."]
pub type DsiCountSelR = crate::BitReader<DsiCountSel>;
impl DsiCountSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DsiCountSel {
        match self.bits {
            false => DsiCountSel::CsdResult,
            true => DsiCountSel::AdcResult,
        }
    }
    #[doc = "depending on the dsi_count_val_sel input either output RESULT_VAL1.VALUE (0) or RESULT_VAL2.VALUE (1) on the dsi_count bus. Note that dsi_count_val_sel is not synchronized, i.e. it controls the mux combinatorially."]
    #[inline(always)]
    pub fn is_csd_result(&self) -> bool {
        *self == DsiCountSel::CsdResult
    }
    #[doc = "output ADC_RES.VIN_CNT on the dsi_count bus"]
    #[inline(always)]
    pub fn is_adc_result(&self) -> bool {
        *self == DsiCountSel::AdcResult
    }
}
#[doc = "Field `DSI_COUNT_SEL` writer - Select what to output on the dsi_count bus."]
pub type DsiCountSelW<'a, REG> = crate::BitWriter<'a, REG, DsiCountSel>;
impl<'a, REG> DsiCountSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "depending on the dsi_count_val_sel input either output RESULT_VAL1.VALUE (0) or RESULT_VAL2.VALUE (1) on the dsi_count bus. Note that dsi_count_val_sel is not synchronized, i.e. it controls the mux combinatorially."]
    #[inline(always)]
    pub fn csd_result(self) -> &'a mut crate::W<REG> {
        self.variant(DsiCountSel::CsdResult)
    }
    #[doc = "output ADC_RES.VIN_CNT on the dsi_count bus"]
    #[inline(always)]
    pub fn adc_result(self) -> &'a mut crate::W<REG> {
        self.variant(DsiCountSel::AdcResult)
    }
}
#[doc = "Field `DSI_SAMPLE_EN` reader - Enables the use of the dsi_sample_in input instead of the comparator output to strobe COUNTER."]
pub type DsiSampleEnR = crate::BitReader;
#[doc = "Field `DSI_SAMPLE_EN` writer - Enables the use of the dsi_sample_in input instead of the comparator output to strobe COUNTER."]
pub type DsiSampleEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAMPLE_SYNC` reader - Enables double synchronizing of sample input from DSI (only relevant when DSI_SAMPLE_EN=1)."]
pub type SampleSyncR = crate::BitReader;
#[doc = "Field `SAMPLE_SYNC` writer - Enables double synchronizing of sample input from DSI (only relevant when DSI_SAMPLE_EN=1)."]
pub type SampleSyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSI_SENSE_EN` reader - Enables the use of the dsi_sense_in input instead of the internally generated modulation signal to drive csd_sense and csd_shield signals."]
pub type DsiSenseEnR = crate::BitReader;
#[doc = "Field `DSI_SENSE_EN` writer - Enables the use of the dsi_sense_in input instead of the internally generated modulation signal to drive csd_sense and csd_shield signals."]
pub type DsiSenseEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_MODE` reader - Select the power mode for the CSD components (REFGEN, AMBUF, CSDCMP, HSCMP): 0: High Power mode 1: Low Power mode"]
pub type LpModeR = crate::BitReader;
#[doc = "Field `LP_MODE` writer - Select the power mode for the CSD components (REFGEN, AMBUF, CSDCMP, HSCMP): 0: High Power mode 1: Low Power mode"]
pub type LpModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - Master enable of the CSDv2 IP. Must be set to 1 for any CSDv2, ADC or IDAC operation to function. When 0 all analog components will be off and all switches will be open."]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Master enable of the CSDv2 IP. Must be set to 1 for any CSDv2, ADC or IDAC operation to function. When 0 all analog components will be off and all switches will be open."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Select Iref supply."]
    #[inline(always)]
    pub fn iref_sel(&self) -> IrefSelR {
        IrefSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:8 - This value determines the number of cycles that the digital filter makes the CSDCMP output ignored while the counter counts and IDAC is on. When set to 0 the digital filter is off. When set to any other value the ignoring will last for FILTER_DELAY clk_csd cycles after the start of each measurement and from the first comparator trip to the end of each measurement."]
    #[inline(always)]
    pub fn filter_delay(&self) -> FilterDelayR {
        FilterDelayR::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 10:11 - Selects the delay by which csd_shield is delayed relative to csd_sense."]
    #[inline(always)]
    pub fn shield_delay(&self) -> ShieldDelayR {
        ShieldDelayR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Enables the sense modulator output. 0: all switches, static or dynamic, are open and IDAC in CSD mode is off 1: switches and IDAC can be closed/on as per MMIO setting and CSD sequencer."]
    #[inline(always)]
    pub fn sense_en(&self) -> SenseEnR {
        SenseEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 17 - Enables full wave cap sensing mode"]
    #[inline(always)]
    pub fn full_wave(&self) -> FullWaveR {
        FullWaveR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enables mutual cap sensing mode"]
    #[inline(always)]
    pub fn mutual_cap(&self) -> MutualCapR {
        MutualCapR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable the use of two counters for MUTUAL cap sensing mode (CSX), do not use when MUTUAL_CAP=0"]
    #[inline(always)]
    pub fn csx_dual_cnt(&self) -> CsxDualCntR {
        CsxDualCntR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Select what to output on the dsi_count bus."]
    #[inline(always)]
    pub fn dsi_count_sel(&self) -> DsiCountSelR {
        DsiCountSelR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enables the use of the dsi_sample_in input instead of the comparator output to strobe COUNTER."]
    #[inline(always)]
    pub fn dsi_sample_en(&self) -> DsiSampleEnR {
        DsiSampleEnR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enables double synchronizing of sample input from DSI (only relevant when DSI_SAMPLE_EN=1)."]
    #[inline(always)]
    pub fn sample_sync(&self) -> SampleSyncR {
        SampleSyncR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enables the use of the dsi_sense_in input instead of the internally generated modulation signal to drive csd_sense and csd_shield signals."]
    #[inline(always)]
    pub fn dsi_sense_en(&self) -> DsiSenseEnR {
        DsiSenseEnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - Select the power mode for the CSD components (REFGEN, AMBUF, CSDCMP, HSCMP): 0: High Power mode 1: Low Power mode"]
    #[inline(always)]
    pub fn lp_mode(&self) -> LpModeR {
        LpModeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Master enable of the CSDv2 IP. Must be set to 1 for any CSDv2, ADC or IDAC operation to function. When 0 all analog components will be off and all switches will be open."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select Iref supply."]
    #[inline(always)]
    #[must_use]
    pub fn iref_sel(&mut self) -> IrefSelW<ConfigSpec> {
        IrefSelW::new(self, 0)
    }
    #[doc = "Bits 4:8 - This value determines the number of cycles that the digital filter makes the CSDCMP output ignored while the counter counts and IDAC is on. When set to 0 the digital filter is off. When set to any other value the ignoring will last for FILTER_DELAY clk_csd cycles after the start of each measurement and from the first comparator trip to the end of each measurement."]
    #[inline(always)]
    #[must_use]
    pub fn filter_delay(&mut self) -> FilterDelayW<ConfigSpec> {
        FilterDelayW::new(self, 4)
    }
    #[doc = "Bits 10:11 - Selects the delay by which csd_shield is delayed relative to csd_sense."]
    #[inline(always)]
    #[must_use]
    pub fn shield_delay(&mut self) -> ShieldDelayW<ConfigSpec> {
        ShieldDelayW::new(self, 10)
    }
    #[doc = "Bit 12 - Enables the sense modulator output. 0: all switches, static or dynamic, are open and IDAC in CSD mode is off 1: switches and IDAC can be closed/on as per MMIO setting and CSD sequencer."]
    #[inline(always)]
    #[must_use]
    pub fn sense_en(&mut self) -> SenseEnW<ConfigSpec> {
        SenseEnW::new(self, 12)
    }
    #[doc = "Bit 17 - Enables full wave cap sensing mode"]
    #[inline(always)]
    #[must_use]
    pub fn full_wave(&mut self) -> FullWaveW<ConfigSpec> {
        FullWaveW::new(self, 17)
    }
    #[doc = "Bit 18 - Enables mutual cap sensing mode"]
    #[inline(always)]
    #[must_use]
    pub fn mutual_cap(&mut self) -> MutualCapW<ConfigSpec> {
        MutualCapW::new(self, 18)
    }
    #[doc = "Bit 19 - Enable the use of two counters for MUTUAL cap sensing mode (CSX), do not use when MUTUAL_CAP=0"]
    #[inline(always)]
    #[must_use]
    pub fn csx_dual_cnt(&mut self) -> CsxDualCntW<ConfigSpec> {
        CsxDualCntW::new(self, 19)
    }
    #[doc = "Bit 24 - Select what to output on the dsi_count bus."]
    #[inline(always)]
    #[must_use]
    pub fn dsi_count_sel(&mut self) -> DsiCountSelW<ConfigSpec> {
        DsiCountSelW::new(self, 24)
    }
    #[doc = "Bit 25 - Enables the use of the dsi_sample_in input instead of the comparator output to strobe COUNTER."]
    #[inline(always)]
    #[must_use]
    pub fn dsi_sample_en(&mut self) -> DsiSampleEnW<ConfigSpec> {
        DsiSampleEnW::new(self, 25)
    }
    #[doc = "Bit 26 - Enables double synchronizing of sample input from DSI (only relevant when DSI_SAMPLE_EN=1)."]
    #[inline(always)]
    #[must_use]
    pub fn sample_sync(&mut self) -> SampleSyncW<ConfigSpec> {
        SampleSyncW::new(self, 26)
    }
    #[doc = "Bit 27 - Enables the use of the dsi_sense_in input instead of the internally generated modulation signal to drive csd_sense and csd_shield signals."]
    #[inline(always)]
    #[must_use]
    pub fn dsi_sense_en(&mut self) -> DsiSenseEnW<ConfigSpec> {
        DsiSenseEnW::new(self, 27)
    }
    #[doc = "Bit 30 - Select the power mode for the CSD components (REFGEN, AMBUF, CSDCMP, HSCMP): 0: High Power mode 1: Low Power mode"]
    #[inline(always)]
    #[must_use]
    pub fn lp_mode(&mut self) -> LpModeW<ConfigSpec> {
        LpModeW::new(self, 30)
    }
    #[doc = "Bit 31 - Master enable of the CSDv2 IP. Must be set to 1 for any CSDv2, ADC or IDAC operation to function. When 0 all analog components will be off and all switches will be open."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<ConfigSpec> {
        EnableW::new(self, 31)
    }
}
#[doc = "Configuration and Control\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0x0400_0000"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0x0400_0000;
}
