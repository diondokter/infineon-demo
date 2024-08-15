#[doc = "Register `MODE_CTL` reader"]
pub type R = crate::R<ModeCtlSpec>;
#[doc = "Register `MODE_CTL` writer"]
pub type W = crate::W<ModeCtlSpec>;
#[doc = "Specifies PCM output channels as mono or stereo: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PCM_CHSET)\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PcmChSet {
    #[doc = "0: Channel disabled"]
    Disabled = 0,
    #[doc = "1: Mono left channel enable"]
    MonoL = 1,
    #[doc = "2: Mono right channel enable"]
    MonoR = 2,
    #[doc = "3: Stereo channel enable"]
    Stereo = 3,
}
impl From<PcmChSet> for u8 {
    #[inline(always)]
    fn from(variant: PcmChSet) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PcmChSet {
    type Ux = u8;
}
impl crate::IsEnum for PcmChSet {}
#[doc = "Field `PCM_CH_SET` reader - Specifies PCM output channels as mono or stereo: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PCM_CHSET)"]
pub type PcmChSetR = crate::FieldReader<PcmChSet>;
impl PcmChSetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PcmChSet {
        match self.bits {
            0 => PcmChSet::Disabled,
            1 => PcmChSet::MonoL,
            2 => PcmChSet::MonoR,
            3 => PcmChSet::Stereo,
            _ => unreachable!(),
        }
    }
    #[doc = "Channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PcmChSet::Disabled
    }
    #[doc = "Mono left channel enable"]
    #[inline(always)]
    pub fn is_mono_l(&self) -> bool {
        *self == PcmChSet::MonoL
    }
    #[doc = "Mono right channel enable"]
    #[inline(always)]
    pub fn is_mono_r(&self) -> bool {
        *self == PcmChSet::MonoR
    }
    #[doc = "Stereo channel enable"]
    #[inline(always)]
    pub fn is_stereo(&self) -> bool {
        *self == PcmChSet::Stereo
    }
}
#[doc = "Field `PCM_CH_SET` writer - Specifies PCM output channels as mono or stereo: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PCM_CHSET)"]
pub type PcmChSetW<'a, REG> = crate::FieldWriter<'a, REG, 2, PcmChSet, crate::Safe>;
impl<'a, REG> PcmChSetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PcmChSet::Disabled)
    }
    #[doc = "Mono left channel enable"]
    #[inline(always)]
    pub fn mono_l(self) -> &'a mut crate::W<REG> {
        self.variant(PcmChSet::MonoL)
    }
    #[doc = "Mono right channel enable"]
    #[inline(always)]
    pub fn mono_r(self) -> &'a mut crate::W<REG> {
        self.variant(PcmChSet::MonoR)
    }
    #[doc = "Stereo channel enable"]
    #[inline(always)]
    pub fn stereo(self) -> &'a mut crate::W<REG> {
        self.variant(PcmChSet::Stereo)
    }
}
#[doc = "Field `SWAP_LR` reader - Input data L/R channel swap: '1': Right/Left channel recording swap '0': No Swap (Note: This bit is connected to AR36U12.PDM_CORE_CFG.LRSWAP)"]
pub type SwapLrR = crate::BitReader;
#[doc = "Field `SWAP_LR` writer - Input data L/R channel swap: '1': Right/Left channel recording swap '0': No Swap (Note: This bit is connected to AR36U12.PDM_CORE_CFG.LRSWAP)"]
pub type SwapLrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Set time step for gain change during PGA or soft mute operation in number of 1/a sampling rate. (Note: These bits are connected to AR36U12.PDM_CORE_CFG.S_CYCLES)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCycles {
    #[doc = "0: 64steps"]
    StepNum64 = 0,
    #[doc = "1: 96steps"]
    StepNum96 = 1,
    #[doc = "2: 128steps"]
    StepNum128 = 2,
    #[doc = "3: 160steps"]
    StepNum160 = 3,
    #[doc = "4: 192steps"]
    StepNum192 = 4,
    #[doc = "5: 256steps"]
    StepNum256 = 5,
    #[doc = "6: 384steps"]
    StepNum384 = 6,
    #[doc = "7: 512steps"]
    StepNum512 = 7,
}
impl From<SCycles> for u8 {
    #[inline(always)]
    fn from(variant: SCycles) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SCycles {
    type Ux = u8;
}
impl crate::IsEnum for SCycles {}
#[doc = "Field `S_CYCLES` reader - Set time step for gain change during PGA or soft mute operation in number of 1/a sampling rate. (Note: These bits are connected to AR36U12.PDM_CORE_CFG.S_CYCLES)"]
pub type SCyclesR = crate::FieldReader<SCycles>;
impl SCyclesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SCycles {
        match self.bits {
            0 => SCycles::StepNum64,
            1 => SCycles::StepNum96,
            2 => SCycles::StepNum128,
            3 => SCycles::StepNum160,
            4 => SCycles::StepNum192,
            5 => SCycles::StepNum256,
            6 => SCycles::StepNum384,
            7 => SCycles::StepNum512,
            _ => unreachable!(),
        }
    }
    #[doc = "64steps"]
    #[inline(always)]
    pub fn is_step_num64(&self) -> bool {
        *self == SCycles::StepNum64
    }
    #[doc = "96steps"]
    #[inline(always)]
    pub fn is_step_num96(&self) -> bool {
        *self == SCycles::StepNum96
    }
    #[doc = "128steps"]
    #[inline(always)]
    pub fn is_step_num128(&self) -> bool {
        *self == SCycles::StepNum128
    }
    #[doc = "160steps"]
    #[inline(always)]
    pub fn is_step_num160(&self) -> bool {
        *self == SCycles::StepNum160
    }
    #[doc = "192steps"]
    #[inline(always)]
    pub fn is_step_num192(&self) -> bool {
        *self == SCycles::StepNum192
    }
    #[doc = "256steps"]
    #[inline(always)]
    pub fn is_step_num256(&self) -> bool {
        *self == SCycles::StepNum256
    }
    #[doc = "384steps"]
    #[inline(always)]
    pub fn is_step_num384(&self) -> bool {
        *self == SCycles::StepNum384
    }
    #[doc = "512steps"]
    #[inline(always)]
    pub fn is_step_num512(&self) -> bool {
        *self == SCycles::StepNum512
    }
}
#[doc = "Field `S_CYCLES` writer - Set time step for gain change during PGA or soft mute operation in number of 1/a sampling rate. (Note: These bits are connected to AR36U12.PDM_CORE_CFG.S_CYCLES)"]
pub type SCyclesW<'a, REG> = crate::FieldWriter<'a, REG, 3, SCycles, crate::Safe>;
impl<'a, REG> SCyclesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "64steps"]
    #[inline(always)]
    pub fn step_num64(self) -> &'a mut crate::W<REG> {
        self.variant(SCycles::StepNum64)
    }
    #[doc = "96steps"]
    #[inline(always)]
    pub fn step_num96(self) -> &'a mut crate::W<REG> {
        self.variant(SCycles::StepNum96)
    }
    #[doc = "128steps"]
    #[inline(always)]
    pub fn step_num128(self) -> &'a mut crate::W<REG> {
        self.variant(SCycles::StepNum128)
    }
    #[doc = "160steps"]
    #[inline(always)]
    pub fn step_num160(self) -> &'a mut crate::W<REG> {
        self.variant(SCycles::StepNum160)
    }
    #[doc = "192steps"]
    #[inline(always)]
    pub fn step_num192(self) -> &'a mut crate::W<REG> {
        self.variant(SCycles::StepNum192)
    }
    #[doc = "256steps"]
    #[inline(always)]
    pub fn step_num256(self) -> &'a mut crate::W<REG> {
        self.variant(SCycles::StepNum256)
    }
    #[doc = "384steps"]
    #[inline(always)]
    pub fn step_num384(self) -> &'a mut crate::W<REG> {
        self.variant(SCycles::StepNum384)
    }
    #[doc = "512steps"]
    #[inline(always)]
    pub fn step_num512(self) -> &'a mut crate::W<REG> {
        self.variant(SCycles::StepNum512)
    }
}
#[doc = "Phase difference from the rising edge of internal sampler clock (CLK_IS) to that of PDM_CKO clock: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PDMCKO_DLY)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CkoDelay {
    #[doc = "0: CLK_IS is 3*PDM_CLK period early"]
    Adv3 = 0,
    #[doc = "1: CLK_IS is 2*PDM_CLK period early"]
    Adv2 = 1,
    #[doc = "2: CLK_IS is 1*PDM_CLK period early"]
    Adv1 = 2,
    #[doc = "3: CLK_IS is the same as PDM_CKO"]
    NoDelay = 3,
    #[doc = "4: CLK_IS is 1*PDM_CLK period late"]
    Dly1 = 4,
    #[doc = "5: CLK_IS is 2*PDM_CLK period late"]
    Dly2 = 5,
    #[doc = "6: CLK_IS is 3*PDM_CLK period late"]
    Dly3 = 6,
    #[doc = "7: CLK_IS is 4*PDM_CLK period late"]
    Dly4 = 7,
}
impl From<CkoDelay> for u8 {
    #[inline(always)]
    fn from(variant: CkoDelay) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CkoDelay {
    type Ux = u8;
}
impl crate::IsEnum for CkoDelay {}
#[doc = "Field `CKO_DELAY` reader - Phase difference from the rising edge of internal sampler clock (CLK_IS) to that of PDM_CKO clock: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PDMCKO_DLY)"]
pub type CkoDelayR = crate::FieldReader<CkoDelay>;
impl CkoDelayR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CkoDelay {
        match self.bits {
            0 => CkoDelay::Adv3,
            1 => CkoDelay::Adv2,
            2 => CkoDelay::Adv1,
            3 => CkoDelay::NoDelay,
            4 => CkoDelay::Dly1,
            5 => CkoDelay::Dly2,
            6 => CkoDelay::Dly3,
            7 => CkoDelay::Dly4,
            _ => unreachable!(),
        }
    }
    #[doc = "CLK_IS is 3*PDM_CLK period early"]
    #[inline(always)]
    pub fn is_adv3(&self) -> bool {
        *self == CkoDelay::Adv3
    }
    #[doc = "CLK_IS is 2*PDM_CLK period early"]
    #[inline(always)]
    pub fn is_adv2(&self) -> bool {
        *self == CkoDelay::Adv2
    }
    #[doc = "CLK_IS is 1*PDM_CLK period early"]
    #[inline(always)]
    pub fn is_adv1(&self) -> bool {
        *self == CkoDelay::Adv1
    }
    #[doc = "CLK_IS is the same as PDM_CKO"]
    #[inline(always)]
    pub fn is_no_delay(&self) -> bool {
        *self == CkoDelay::NoDelay
    }
    #[doc = "CLK_IS is 1*PDM_CLK period late"]
    #[inline(always)]
    pub fn is_dly1(&self) -> bool {
        *self == CkoDelay::Dly1
    }
    #[doc = "CLK_IS is 2*PDM_CLK period late"]
    #[inline(always)]
    pub fn is_dly2(&self) -> bool {
        *self == CkoDelay::Dly2
    }
    #[doc = "CLK_IS is 3*PDM_CLK period late"]
    #[inline(always)]
    pub fn is_dly3(&self) -> bool {
        *self == CkoDelay::Dly3
    }
    #[doc = "CLK_IS is 4*PDM_CLK period late"]
    #[inline(always)]
    pub fn is_dly4(&self) -> bool {
        *self == CkoDelay::Dly4
    }
}
#[doc = "Field `CKO_DELAY` writer - Phase difference from the rising edge of internal sampler clock (CLK_IS) to that of PDM_CKO clock: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PDMCKO_DLY)"]
pub type CkoDelayW<'a, REG> = crate::FieldWriter<'a, REG, 3, CkoDelay, crate::Safe>;
impl<'a, REG> CkoDelayW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLK_IS is 3*PDM_CLK period early"]
    #[inline(always)]
    pub fn adv3(self) -> &'a mut crate::W<REG> {
        self.variant(CkoDelay::Adv3)
    }
    #[doc = "CLK_IS is 2*PDM_CLK period early"]
    #[inline(always)]
    pub fn adv2(self) -> &'a mut crate::W<REG> {
        self.variant(CkoDelay::Adv2)
    }
    #[doc = "CLK_IS is 1*PDM_CLK period early"]
    #[inline(always)]
    pub fn adv1(self) -> &'a mut crate::W<REG> {
        self.variant(CkoDelay::Adv1)
    }
    #[doc = "CLK_IS is the same as PDM_CKO"]
    #[inline(always)]
    pub fn no_delay(self) -> &'a mut crate::W<REG> {
        self.variant(CkoDelay::NoDelay)
    }
    #[doc = "CLK_IS is 1*PDM_CLK period late"]
    #[inline(always)]
    pub fn dly1(self) -> &'a mut crate::W<REG> {
        self.variant(CkoDelay::Dly1)
    }
    #[doc = "CLK_IS is 2*PDM_CLK period late"]
    #[inline(always)]
    pub fn dly2(self) -> &'a mut crate::W<REG> {
        self.variant(CkoDelay::Dly2)
    }
    #[doc = "CLK_IS is 3*PDM_CLK period late"]
    #[inline(always)]
    pub fn dly3(self) -> &'a mut crate::W<REG> {
        self.variant(CkoDelay::Dly3)
    }
    #[doc = "CLK_IS is 4*PDM_CLK period late"]
    #[inline(always)]
    pub fn dly4(self) -> &'a mut crate::W<REG> {
        self.variant(CkoDelay::Dly4)
    }
}
#[doc = "Field `HPF_GAIN` reader - Adjust high pass filter coefficients. H(Z) = (1 - Z-1 ) / \\[1 - (1- 2 -HPF_GAIN) Z-1 \\]
(Note: These bits are connected to AR36U12.PDM_CORE_CFG.HPGAIN)"]
pub type HpfGainR = crate::FieldReader;
#[doc = "Field `HPF_GAIN` writer - Adjust high pass filter coefficients. H(Z) = (1 - Z-1 ) / \\[1 - (1- 2 -HPF_GAIN) Z-1 \\]
(Note: These bits are connected to AR36U12.PDM_CORE_CFG.HPGAIN)"]
pub type HpfGainW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HPF_EN_N` reader - Enable high pass filter (active low) '1': Disabled. '0': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.ADCHPD)"]
pub type HpfEnNR = crate::BitReader;
#[doc = "Field `HPF_EN_N` writer - Enable high pass filter (active low) '1': Disabled. '0': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.ADCHPD)"]
pub type HpfEnNW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Specifies PCM output channels as mono or stereo: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PCM_CHSET)"]
    #[inline(always)]
    pub fn pcm_ch_set(&self) -> PcmChSetR {
        PcmChSetR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Input data L/R channel swap: '1': Right/Left channel recording swap '0': No Swap (Note: This bit is connected to AR36U12.PDM_CORE_CFG.LRSWAP)"]
    #[inline(always)]
    pub fn swap_lr(&self) -> SwapLrR {
        SwapLrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Set time step for gain change during PGA or soft mute operation in number of 1/a sampling rate. (Note: These bits are connected to AR36U12.PDM_CORE_CFG.S_CYCLES)"]
    #[inline(always)]
    pub fn s_cycles(&self) -> SCyclesR {
        SCyclesR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Phase difference from the rising edge of internal sampler clock (CLK_IS) to that of PDM_CKO clock: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PDMCKO_DLY)"]
    #[inline(always)]
    pub fn cko_delay(&self) -> CkoDelayR {
        CkoDelayR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:27 - Adjust high pass filter coefficients. H(Z) = (1 - Z-1 ) / \\[1 - (1- 2 -HPF_GAIN) Z-1 \\]
(Note: These bits are connected to AR36U12.PDM_CORE_CFG.HPGAIN)"]
    #[inline(always)]
    pub fn hpf_gain(&self) -> HpfGainR {
        HpfGainR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Enable high pass filter (active low) '1': Disabled. '0': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.ADCHPD)"]
    #[inline(always)]
    pub fn hpf_en_n(&self) -> HpfEnNR {
        HpfEnNR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Specifies PCM output channels as mono or stereo: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PCM_CHSET)"]
    #[inline(always)]
    #[must_use]
    pub fn pcm_ch_set(&mut self) -> PcmChSetW<ModeCtlSpec> {
        PcmChSetW::new(self, 0)
    }
    #[doc = "Bit 2 - Input data L/R channel swap: '1': Right/Left channel recording swap '0': No Swap (Note: This bit is connected to AR36U12.PDM_CORE_CFG.LRSWAP)"]
    #[inline(always)]
    #[must_use]
    pub fn swap_lr(&mut self) -> SwapLrW<ModeCtlSpec> {
        SwapLrW::new(self, 2)
    }
    #[doc = "Bits 8:10 - Set time step for gain change during PGA or soft mute operation in number of 1/a sampling rate. (Note: These bits are connected to AR36U12.PDM_CORE_CFG.S_CYCLES)"]
    #[inline(always)]
    #[must_use]
    pub fn s_cycles(&mut self) -> SCyclesW<ModeCtlSpec> {
        SCyclesW::new(self, 8)
    }
    #[doc = "Bits 16:18 - Phase difference from the rising edge of internal sampler clock (CLK_IS) to that of PDM_CKO clock: (Note: These bits are connected to AR36U12.PDM_CORE2_CFG.PDMCKO_DLY)"]
    #[inline(always)]
    #[must_use]
    pub fn cko_delay(&mut self) -> CkoDelayW<ModeCtlSpec> {
        CkoDelayW::new(self, 16)
    }
    #[doc = "Bits 24:27 - Adjust high pass filter coefficients. H(Z) = (1 - Z-1 ) / \\[1 - (1- 2 -HPF_GAIN) Z-1 \\]
(Note: These bits are connected to AR36U12.PDM_CORE_CFG.HPGAIN)"]
    #[inline(always)]
    #[must_use]
    pub fn hpf_gain(&mut self) -> HpfGainW<ModeCtlSpec> {
        HpfGainW::new(self, 24)
    }
    #[doc = "Bit 28 - Enable high pass filter (active low) '1': Disabled. '0': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.ADCHPD)"]
    #[inline(always)]
    #[must_use]
    pub fn hpf_en_n(&mut self) -> HpfEnNW<ModeCtlSpec> {
        HpfEnNW::new(self, 28)
    }
}
#[doc = "Mode control\n\nYou can [`read`](crate::Reg::read) this register and get [`mode_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModeCtlSpec;
impl crate::RegisterSpec for ModeCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode_ctl::R`](R) reader structure"]
impl crate::Readable for ModeCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`mode_ctl::W`](W) writer structure"]
impl crate::Writable for ModeCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODE_CTL to value 0x1b00_0103"]
impl crate::Resettable for ModeCtlSpec {
    const RESET_VALUE: u32 = 0x1b00_0103;
}
