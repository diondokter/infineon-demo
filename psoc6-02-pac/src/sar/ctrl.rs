#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "VREF buffer low power mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PwrCtrlVref {
    #[doc = "0: full power (100 percent) (default), bypass cap, max clk_sar is 18MHz."]
    Pwr100 = 0,
    #[doc = "1: 80 percent power"]
    Pwr80 = 1,
    #[doc = "2: 60 percent power"]
    Pwr60 = 2,
    #[doc = "3: 50 percent power"]
    Pwr50 = 3,
    #[doc = "4: 40 percent power"]
    Pwr40 = 4,
    #[doc = "5: 30 percent power"]
    Pwr30 = 5,
    #[doc = "6: 20 percent power"]
    Pwr20 = 6,
    #[doc = "7: 10 percent power"]
    Pwr10 = 7,
}
impl From<PwrCtrlVref> for u8 {
    #[inline(always)]
    fn from(variant: PwrCtrlVref) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PwrCtrlVref {
    type Ux = u8;
}
impl crate::IsEnum for PwrCtrlVref {}
#[doc = "Field `PWR_CTRL_VREF` reader - VREF buffer low power mode."]
pub type PwrCtrlVrefR = crate::FieldReader<PwrCtrlVref>;
impl PwrCtrlVrefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PwrCtrlVref {
        match self.bits {
            0 => PwrCtrlVref::Pwr100,
            1 => PwrCtrlVref::Pwr80,
            2 => PwrCtrlVref::Pwr60,
            3 => PwrCtrlVref::Pwr50,
            4 => PwrCtrlVref::Pwr40,
            5 => PwrCtrlVref::Pwr30,
            6 => PwrCtrlVref::Pwr20,
            7 => PwrCtrlVref::Pwr10,
            _ => unreachable!(),
        }
    }
    #[doc = "full power (100 percent) (default), bypass cap, max clk_sar is 18MHz."]
    #[inline(always)]
    pub fn is_pwr_100(&self) -> bool {
        *self == PwrCtrlVref::Pwr100
    }
    #[doc = "80 percent power"]
    #[inline(always)]
    pub fn is_pwr_80(&self) -> bool {
        *self == PwrCtrlVref::Pwr80
    }
    #[doc = "60 percent power"]
    #[inline(always)]
    pub fn is_pwr_60(&self) -> bool {
        *self == PwrCtrlVref::Pwr60
    }
    #[doc = "50 percent power"]
    #[inline(always)]
    pub fn is_pwr_50(&self) -> bool {
        *self == PwrCtrlVref::Pwr50
    }
    #[doc = "40 percent power"]
    #[inline(always)]
    pub fn is_pwr_40(&self) -> bool {
        *self == PwrCtrlVref::Pwr40
    }
    #[doc = "30 percent power"]
    #[inline(always)]
    pub fn is_pwr_30(&self) -> bool {
        *self == PwrCtrlVref::Pwr30
    }
    #[doc = "20 percent power"]
    #[inline(always)]
    pub fn is_pwr_20(&self) -> bool {
        *self == PwrCtrlVref::Pwr20
    }
    #[doc = "10 percent power"]
    #[inline(always)]
    pub fn is_pwr_10(&self) -> bool {
        *self == PwrCtrlVref::Pwr10
    }
}
#[doc = "Field `PWR_CTRL_VREF` writer - VREF buffer low power mode."]
pub type PwrCtrlVrefW<'a, REG> = crate::FieldWriter<'a, REG, 3, PwrCtrlVref, crate::Safe>;
impl<'a, REG> PwrCtrlVrefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "full power (100 percent) (default), bypass cap, max clk_sar is 18MHz."]
    #[inline(always)]
    pub fn pwr_100(self) -> &'a mut crate::W<REG> {
        self.variant(PwrCtrlVref::Pwr100)
    }
    #[doc = "80 percent power"]
    #[inline(always)]
    pub fn pwr_80(self) -> &'a mut crate::W<REG> {
        self.variant(PwrCtrlVref::Pwr80)
    }
    #[doc = "60 percent power"]
    #[inline(always)]
    pub fn pwr_60(self) -> &'a mut crate::W<REG> {
        self.variant(PwrCtrlVref::Pwr60)
    }
    #[doc = "50 percent power"]
    #[inline(always)]
    pub fn pwr_50(self) -> &'a mut crate::W<REG> {
        self.variant(PwrCtrlVref::Pwr50)
    }
    #[doc = "40 percent power"]
    #[inline(always)]
    pub fn pwr_40(self) -> &'a mut crate::W<REG> {
        self.variant(PwrCtrlVref::Pwr40)
    }
    #[doc = "30 percent power"]
    #[inline(always)]
    pub fn pwr_30(self) -> &'a mut crate::W<REG> {
        self.variant(PwrCtrlVref::Pwr30)
    }
    #[doc = "20 percent power"]
    #[inline(always)]
    pub fn pwr_20(self) -> &'a mut crate::W<REG> {
        self.variant(PwrCtrlVref::Pwr20)
    }
    #[doc = "10 percent power"]
    #[inline(always)]
    pub fn pwr_10(self) -> &'a mut crate::W<REG> {
        self.variant(PwrCtrlVref::Pwr10)
    }
}
#[doc = "SARADC internal VREF selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VrefSel {
    #[doc = "0: VREF0 from PRB (VREF buffer on)"]
    Vref0 = 0,
    #[doc = "1: VREF1 from PRB (VREF buffer on)"]
    Vref1 = 1,
    #[doc = "2: VREF2 from PRB (VREF buffer on)"]
    Vref2 = 2,
    #[doc = "3: VREF from AROUTE (VREF buffer on)"]
    VrefAroute = 3,
    #[doc = "4: 1.024V from BandGap (VREF buffer on)"]
    Vbgr = 4,
    #[doc = "5: External precision Vref direct from a pin (low impedance path)."]
    VrefExt = 5,
    #[doc = "6: Vdda/2 (VREF buffer on)"]
    VddaDiv2 = 6,
    #[doc = "7: Vdda."]
    Vdda = 7,
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
#[doc = "Field `VREF_SEL` reader - SARADC internal VREF selection."]
pub type VrefSelR = crate::FieldReader<VrefSel>;
impl VrefSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VrefSel {
        match self.bits {
            0 => VrefSel::Vref0,
            1 => VrefSel::Vref1,
            2 => VrefSel::Vref2,
            3 => VrefSel::VrefAroute,
            4 => VrefSel::Vbgr,
            5 => VrefSel::VrefExt,
            6 => VrefSel::VddaDiv2,
            7 => VrefSel::Vdda,
            _ => unreachable!(),
        }
    }
    #[doc = "VREF0 from PRB (VREF buffer on)"]
    #[inline(always)]
    pub fn is_vref0(&self) -> bool {
        *self == VrefSel::Vref0
    }
    #[doc = "VREF1 from PRB (VREF buffer on)"]
    #[inline(always)]
    pub fn is_vref1(&self) -> bool {
        *self == VrefSel::Vref1
    }
    #[doc = "VREF2 from PRB (VREF buffer on)"]
    #[inline(always)]
    pub fn is_vref2(&self) -> bool {
        *self == VrefSel::Vref2
    }
    #[doc = "VREF from AROUTE (VREF buffer on)"]
    #[inline(always)]
    pub fn is_vref_aroute(&self) -> bool {
        *self == VrefSel::VrefAroute
    }
    #[doc = "1.024V from BandGap (VREF buffer on)"]
    #[inline(always)]
    pub fn is_vbgr(&self) -> bool {
        *self == VrefSel::Vbgr
    }
    #[doc = "External precision Vref direct from a pin (low impedance path)."]
    #[inline(always)]
    pub fn is_vref_ext(&self) -> bool {
        *self == VrefSel::VrefExt
    }
    #[doc = "Vdda/2 (VREF buffer on)"]
    #[inline(always)]
    pub fn is_vdda_div_2(&self) -> bool {
        *self == VrefSel::VddaDiv2
    }
    #[doc = "Vdda."]
    #[inline(always)]
    pub fn is_vdda(&self) -> bool {
        *self == VrefSel::Vdda
    }
}
#[doc = "Field `VREF_SEL` writer - SARADC internal VREF selection."]
pub type VrefSelW<'a, REG> = crate::FieldWriter<'a, REG, 3, VrefSel, crate::Safe>;
impl<'a, REG> VrefSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VREF0 from PRB (VREF buffer on)"]
    #[inline(always)]
    pub fn vref0(self) -> &'a mut crate::W<REG> {
        self.variant(VrefSel::Vref0)
    }
    #[doc = "VREF1 from PRB (VREF buffer on)"]
    #[inline(always)]
    pub fn vref1(self) -> &'a mut crate::W<REG> {
        self.variant(VrefSel::Vref1)
    }
    #[doc = "VREF2 from PRB (VREF buffer on)"]
    #[inline(always)]
    pub fn vref2(self) -> &'a mut crate::W<REG> {
        self.variant(VrefSel::Vref2)
    }
    #[doc = "VREF from AROUTE (VREF buffer on)"]
    #[inline(always)]
    pub fn vref_aroute(self) -> &'a mut crate::W<REG> {
        self.variant(VrefSel::VrefAroute)
    }
    #[doc = "1.024V from BandGap (VREF buffer on)"]
    #[inline(always)]
    pub fn vbgr(self) -> &'a mut crate::W<REG> {
        self.variant(VrefSel::Vbgr)
    }
    #[doc = "External precision Vref direct from a pin (low impedance path)."]
    #[inline(always)]
    pub fn vref_ext(self) -> &'a mut crate::W<REG> {
        self.variant(VrefSel::VrefExt)
    }
    #[doc = "Vdda/2 (VREF buffer on)"]
    #[inline(always)]
    pub fn vdda_div_2(self) -> &'a mut crate::W<REG> {
        self.variant(VrefSel::VddaDiv2)
    }
    #[doc = "Vdda."]
    #[inline(always)]
    pub fn vdda(self) -> &'a mut crate::W<REG> {
        self.variant(VrefSel::Vdda)
    }
}
#[doc = "Field `VREF_BYP_CAP_EN` reader - VREF bypass cap enable for when VREF buffer is on"]
pub type VrefBypCapEnR = crate::BitReader;
#[doc = "Field `VREF_BYP_CAP_EN` writer - VREF bypass cap enable for when VREF buffer is on"]
pub type VrefBypCapEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "SARADC internal NEG selection for Single ended conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NegSel {
    #[doc = "0: NEG input of SARADC is connected to 'vssa_kelvin', gives more precision around zero. Note this opens both SARADC internal switches, therefore use this value to insert a break-before-make cycle on those switches when SWITCH_DISABLE is high."]
    VssaKelvin = 0,
    #[doc = "1: NEG input of SARADC is connected to VSSA in AROUTE close to the SARADC"]
    ArtVssa = 1,
    #[doc = "2: NEG input of SARADC is connected to P1 pin of SARMUX"]
    P1 = 2,
    #[doc = "3: NEG input of SARADC is connected to P3 pin of SARMUX"]
    P3 = 3,
    #[doc = "4: NEG input of SARADC is connected to P5 pin of SARMUX"]
    P5 = 4,
    #[doc = "5: NEG input of SARADC is connected to P7 pin of SARMUX"]
    P7 = 5,
    #[doc = "6: NEG input of SARADC is connected to an ACORE in AROUTE"]
    Acore = 6,
    #[doc = "7: NEG input of SARADC is shorted with VREF input of SARADC."]
    Vref = 7,
}
impl From<NegSel> for u8 {
    #[inline(always)]
    fn from(variant: NegSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NegSel {
    type Ux = u8;
}
impl crate::IsEnum for NegSel {}
#[doc = "Field `NEG_SEL` reader - SARADC internal NEG selection for Single ended conversion"]
pub type NegSelR = crate::FieldReader<NegSel>;
impl NegSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NegSel {
        match self.bits {
            0 => NegSel::VssaKelvin,
            1 => NegSel::ArtVssa,
            2 => NegSel::P1,
            3 => NegSel::P3,
            4 => NegSel::P5,
            5 => NegSel::P7,
            6 => NegSel::Acore,
            7 => NegSel::Vref,
            _ => unreachable!(),
        }
    }
    #[doc = "NEG input of SARADC is connected to 'vssa_kelvin', gives more precision around zero. Note this opens both SARADC internal switches, therefore use this value to insert a break-before-make cycle on those switches when SWITCH_DISABLE is high."]
    #[inline(always)]
    pub fn is_vssa_kelvin(&self) -> bool {
        *self == NegSel::VssaKelvin
    }
    #[doc = "NEG input of SARADC is connected to VSSA in AROUTE close to the SARADC"]
    #[inline(always)]
    pub fn is_art_vssa(&self) -> bool {
        *self == NegSel::ArtVssa
    }
    #[doc = "NEG input of SARADC is connected to P1 pin of SARMUX"]
    #[inline(always)]
    pub fn is_p1(&self) -> bool {
        *self == NegSel::P1
    }
    #[doc = "NEG input of SARADC is connected to P3 pin of SARMUX"]
    #[inline(always)]
    pub fn is_p3(&self) -> bool {
        *self == NegSel::P3
    }
    #[doc = "NEG input of SARADC is connected to P5 pin of SARMUX"]
    #[inline(always)]
    pub fn is_p5(&self) -> bool {
        *self == NegSel::P5
    }
    #[doc = "NEG input of SARADC is connected to P7 pin of SARMUX"]
    #[inline(always)]
    pub fn is_p7(&self) -> bool {
        *self == NegSel::P7
    }
    #[doc = "NEG input of SARADC is connected to an ACORE in AROUTE"]
    #[inline(always)]
    pub fn is_acore(&self) -> bool {
        *self == NegSel::Acore
    }
    #[doc = "NEG input of SARADC is shorted with VREF input of SARADC."]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == NegSel::Vref
    }
}
#[doc = "Field `NEG_SEL` writer - SARADC internal NEG selection for Single ended conversion"]
pub type NegSelW<'a, REG> = crate::FieldWriter<'a, REG, 3, NegSel, crate::Safe>;
impl<'a, REG> NegSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NEG input of SARADC is connected to 'vssa_kelvin', gives more precision around zero. Note this opens both SARADC internal switches, therefore use this value to insert a break-before-make cycle on those switches when SWITCH_DISABLE is high."]
    #[inline(always)]
    pub fn vssa_kelvin(self) -> &'a mut crate::W<REG> {
        self.variant(NegSel::VssaKelvin)
    }
    #[doc = "NEG input of SARADC is connected to VSSA in AROUTE close to the SARADC"]
    #[inline(always)]
    pub fn art_vssa(self) -> &'a mut crate::W<REG> {
        self.variant(NegSel::ArtVssa)
    }
    #[doc = "NEG input of SARADC is connected to P1 pin of SARMUX"]
    #[inline(always)]
    pub fn p1(self) -> &'a mut crate::W<REG> {
        self.variant(NegSel::P1)
    }
    #[doc = "NEG input of SARADC is connected to P3 pin of SARMUX"]
    #[inline(always)]
    pub fn p3(self) -> &'a mut crate::W<REG> {
        self.variant(NegSel::P3)
    }
    #[doc = "NEG input of SARADC is connected to P5 pin of SARMUX"]
    #[inline(always)]
    pub fn p5(self) -> &'a mut crate::W<REG> {
        self.variant(NegSel::P5)
    }
    #[doc = "NEG input of SARADC is connected to P7 pin of SARMUX"]
    #[inline(always)]
    pub fn p7(self) -> &'a mut crate::W<REG> {
        self.variant(NegSel::P7)
    }
    #[doc = "NEG input of SARADC is connected to an ACORE in AROUTE"]
    #[inline(always)]
    pub fn acore(self) -> &'a mut crate::W<REG> {
        self.variant(NegSel::Acore)
    }
    #[doc = "NEG input of SARADC is shorted with VREF input of SARADC."]
    #[inline(always)]
    pub fn vref(self) -> &'a mut crate::W<REG> {
        self.variant(NegSel::Vref)
    }
}
#[doc = "Field `SAR_HW_CTRL_NEGVREF` reader - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for VREF to NEG switch."]
pub type SarHwCtrlNegvrefR = crate::BitReader;
#[doc = "Field `SAR_HW_CTRL_NEGVREF` writer - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for VREF to NEG switch."]
pub type SarHwCtrlNegvrefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Set the comparator latch delay in accordance with SAR conversion rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CompDly {
    #[doc = "0: 2.5ns delay, use this for 2.5Msps"]
    D2p5 = 0,
    #[doc = "1: 4.0ns delay, use this for 2.0Msps"]
    D4 = 1,
    #[doc = "2: 10ns delay, use this for 1.5Msps"]
    D10 = 2,
    #[doc = "3: 12ns delay, use this for 1.0Msps or less"]
    D12 = 3,
}
impl From<CompDly> for u8 {
    #[inline(always)]
    fn from(variant: CompDly) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CompDly {
    type Ux = u8;
}
impl crate::IsEnum for CompDly {}
#[doc = "Field `COMP_DLY` reader - Set the comparator latch delay in accordance with SAR conversion rate"]
pub type CompDlyR = crate::FieldReader<CompDly>;
impl CompDlyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CompDly {
        match self.bits {
            0 => CompDly::D2p5,
            1 => CompDly::D4,
            2 => CompDly::D10,
            3 => CompDly::D12,
            _ => unreachable!(),
        }
    }
    #[doc = "2.5ns delay, use this for 2.5Msps"]
    #[inline(always)]
    pub fn is_d2p5(&self) -> bool {
        *self == CompDly::D2p5
    }
    #[doc = "4.0ns delay, use this for 2.0Msps"]
    #[inline(always)]
    pub fn is_d4(&self) -> bool {
        *self == CompDly::D4
    }
    #[doc = "10ns delay, use this for 1.5Msps"]
    #[inline(always)]
    pub fn is_d10(&self) -> bool {
        *self == CompDly::D10
    }
    #[doc = "12ns delay, use this for 1.0Msps or less"]
    #[inline(always)]
    pub fn is_d12(&self) -> bool {
        *self == CompDly::D12
    }
}
#[doc = "Field `COMP_DLY` writer - Set the comparator latch delay in accordance with SAR conversion rate"]
pub type CompDlyW<'a, REG> = crate::FieldWriter<'a, REG, 2, CompDly, crate::Safe>;
impl<'a, REG> CompDlyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2.5ns delay, use this for 2.5Msps"]
    #[inline(always)]
    pub fn d2p5(self) -> &'a mut crate::W<REG> {
        self.variant(CompDly::D2p5)
    }
    #[doc = "4.0ns delay, use this for 2.0Msps"]
    #[inline(always)]
    pub fn d4(self) -> &'a mut crate::W<REG> {
        self.variant(CompDly::D4)
    }
    #[doc = "10ns delay, use this for 1.5Msps"]
    #[inline(always)]
    pub fn d10(self) -> &'a mut crate::W<REG> {
        self.variant(CompDly::D10)
    }
    #[doc = "12ns delay, use this for 1.0Msps or less"]
    #[inline(always)]
    pub fn d12(self) -> &'a mut crate::W<REG> {
        self.variant(CompDly::D12)
    }
}
#[doc = "Field `SPARE` reader - Spare controls, not yet designated, for late changes done with an ECO"]
pub type SpareR = crate::FieldReader;
#[doc = "Field `SPARE` writer - Spare controls, not yet designated, for late changes done with an ECO"]
pub type SpareW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BOOSTPUMP_EN` reader - deprecated"]
pub type BoostpumpEnR = crate::BitReader;
#[doc = "Field `BOOSTPUMP_EN` writer - deprecated"]
pub type BoostpumpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFBUF_EN` reader - For normal ADC operation this bit must be set, for all reference choices - internal, external or vdda based reference. Setting this bit is critical to proper function of switches inside SARREF block."]
pub type RefbufEnR = crate::BitReader;
#[doc = "Field `REFBUF_EN` writer - For normal ADC operation this bit must be set, for all reference choices - internal, external or vdda based reference. Setting this bit is critical to proper function of switches inside SARREF block."]
pub type RefbufEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Comparator power mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CompPwr {
    #[doc = "0: Power = 100 percent, Use this for SAR Clock Frequency greater than 18MHz"]
    P100 = 0,
    #[doc = "1: N/A"]
    P80 = 1,
    #[doc = "2: Power = 60 percent, Use this for SAR Clock Frequency greater than 1.8MHz up to 18MHz."]
    P60 = 2,
    #[doc = "3: N/A"]
    P50 = 3,
    #[doc = "4: N/A"]
    P40 = 4,
    #[doc = "5: N/A"]
    P30 = 5,
    #[doc = "6: Power = 20 percent, Use this for SAR Clock Frequency less than or equal to 1.8MHz"]
    P20 = 6,
    #[doc = "7: N/A"]
    P10 = 7,
}
impl From<CompPwr> for u8 {
    #[inline(always)]
    fn from(variant: CompPwr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CompPwr {
    type Ux = u8;
}
impl crate::IsEnum for CompPwr {}
#[doc = "Field `COMP_PWR` reader - Comparator power mode."]
pub type CompPwrR = crate::FieldReader<CompPwr>;
impl CompPwrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CompPwr {
        match self.bits {
            0 => CompPwr::P100,
            1 => CompPwr::P80,
            2 => CompPwr::P60,
            3 => CompPwr::P50,
            4 => CompPwr::P40,
            5 => CompPwr::P30,
            6 => CompPwr::P20,
            7 => CompPwr::P10,
            _ => unreachable!(),
        }
    }
    #[doc = "Power = 100 percent, Use this for SAR Clock Frequency greater than 18MHz"]
    #[inline(always)]
    pub fn is_p100(&self) -> bool {
        *self == CompPwr::P100
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_p80(&self) -> bool {
        *self == CompPwr::P80
    }
    #[doc = "Power = 60 percent, Use this for SAR Clock Frequency greater than 1.8MHz up to 18MHz."]
    #[inline(always)]
    pub fn is_p60(&self) -> bool {
        *self == CompPwr::P60
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_p50(&self) -> bool {
        *self == CompPwr::P50
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_p40(&self) -> bool {
        *self == CompPwr::P40
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_p30(&self) -> bool {
        *self == CompPwr::P30
    }
    #[doc = "Power = 20 percent, Use this for SAR Clock Frequency less than or equal to 1.8MHz"]
    #[inline(always)]
    pub fn is_p20(&self) -> bool {
        *self == CompPwr::P20
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_p10(&self) -> bool {
        *self == CompPwr::P10
    }
}
#[doc = "Field `COMP_PWR` writer - Comparator power mode."]
pub type CompPwrW<'a, REG> = crate::FieldWriter<'a, REG, 3, CompPwr, crate::Safe>;
impl<'a, REG> CompPwrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Power = 100 percent, Use this for SAR Clock Frequency greater than 18MHz"]
    #[inline(always)]
    pub fn p100(self) -> &'a mut crate::W<REG> {
        self.variant(CompPwr::P100)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn p80(self) -> &'a mut crate::W<REG> {
        self.variant(CompPwr::P80)
    }
    #[doc = "Power = 60 percent, Use this for SAR Clock Frequency greater than 1.8MHz up to 18MHz."]
    #[inline(always)]
    pub fn p60(self) -> &'a mut crate::W<REG> {
        self.variant(CompPwr::P60)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn p50(self) -> &'a mut crate::W<REG> {
        self.variant(CompPwr::P50)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn p40(self) -> &'a mut crate::W<REG> {
        self.variant(CompPwr::P40)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn p30(self) -> &'a mut crate::W<REG> {
        self.variant(CompPwr::P30)
    }
    #[doc = "Power = 20 percent, Use this for SAR Clock Frequency less than or equal to 1.8MHz"]
    #[inline(always)]
    pub fn p20(self) -> &'a mut crate::W<REG> {
        self.variant(CompPwr::P20)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn p10(self) -> &'a mut crate::W<REG> {
        self.variant(CompPwr::P10)
    }
}
#[doc = "Field `DEEPSLEEP_ON` reader - - 0: SARMUX IP disabled off during DeepSleep power mode - 1: SARMUX IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
pub type DeepsleepOnR = crate::BitReader;
#[doc = "Field `DEEPSLEEP_ON` writer - - 0: SARMUX IP disabled off during DeepSleep power mode - 1: SARMUX IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
pub type DeepsleepOnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSI_SYNC_CONFIG` reader - - 0: bypass clock domain synchronization of the DSI config signals. - 1: synchronize the DSI config signals to peripheral clock domain."]
pub type DsiSyncConfigR = crate::BitReader;
#[doc = "Field `DSI_SYNC_CONFIG` writer - - 0: bypass clock domain synchronization of the DSI config signals. - 1: synchronize the DSI config signals to peripheral clock domain."]
pub type DsiSyncConfigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSI_MODE` reader - SAR sequencer takes configuration from DSI signals (note this also has the same effect as SWITCH_DISABLE==1) - 0: Normal mode, SAR sequencer operates according to CHAN_EN enables and CHAN_CONFIG channel configurations - 1: CHAN_EN, INJ_START_EN and channel configurations in CHAN_CONFIG and INJ_CHAN_CONFIG are ignored"]
pub type DsiModeR = crate::BitReader;
#[doc = "Field `DSI_MODE` writer - SAR sequencer takes configuration from DSI signals (note this also has the same effect as SWITCH_DISABLE==1) - 0: Normal mode, SAR sequencer operates according to CHAN_EN enables and CHAN_CONFIG channel configurations - 1: CHAN_EN, INJ_START_EN and channel configurations in CHAN_CONFIG and INJ_CHAN_CONFIG are ignored"]
pub type DsiModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWITCH_DISABLE` reader - Disable SAR sequencer from enabling routing switches (note DSI and firmware can always close switches independent of this control) - 0: Normal mode, SAR sequencer changes switches according to pin address in channel configurations - 1: Switches disabled, SAR sequencer does not enable any switches, it is the responsibility of the firmware or UDBs (through DSI) to set the switches to route the signal to be converted through the SARMUX"]
pub type SwitchDisableR = crate::BitReader;
#[doc = "Field `SWITCH_DISABLE` writer - Disable SAR sequencer from enabling routing switches (note DSI and firmware can always close switches independent of this control) - 0: Normal mode, SAR sequencer changes switches according to pin address in channel configurations - 1: Switches disabled, SAR sequencer does not enable any switches, it is the responsibility of the firmware or UDBs (through DSI) to set the switches to route the signal to be converted through the SARMUX"]
pub type SwitchDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED` reader - - 0: SAR IP disabled (put analog in power down and stop clocks), also can clear FW_TRIGGER and INJ_START_EN (if not tailgating) on write. - 1: SAR IP enabled."]
pub type EnabledR = crate::BitReader;
#[doc = "Field `ENABLED` writer - - 0: SAR IP disabled (put analog in power down and stop clocks), also can clear FW_TRIGGER and INJ_START_EN (if not tailgating) on write. - 1: SAR IP enabled."]
pub type EnabledW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - VREF buffer low power mode."]
    #[inline(always)]
    pub fn pwr_ctrl_vref(&self) -> PwrCtrlVrefR {
        PwrCtrlVrefR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - SARADC internal VREF selection."]
    #[inline(always)]
    pub fn vref_sel(&self) -> VrefSelR {
        VrefSelR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - VREF bypass cap enable for when VREF buffer is on"]
    #[inline(always)]
    pub fn vref_byp_cap_en(&self) -> VrefBypCapEnR {
        VrefBypCapEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 9:11 - SARADC internal NEG selection for Single ended conversion"]
    #[inline(always)]
    pub fn neg_sel(&self) -> NegSelR {
        NegSelR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 13 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for VREF to NEG switch."]
    #[inline(always)]
    pub fn sar_hw_ctrl_negvref(&self) -> SarHwCtrlNegvrefR {
        SarHwCtrlNegvrefR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Set the comparator latch delay in accordance with SAR conversion rate"]
    #[inline(always)]
    pub fn comp_dly(&self) -> CompDlyR {
        CompDlyR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Spare controls, not yet designated, for late changes done with an ECO"]
    #[inline(always)]
    pub fn spare(&self) -> SpareR {
        SpareR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - deprecated"]
    #[inline(always)]
    pub fn boostpump_en(&self) -> BoostpumpEnR {
        BoostpumpEnR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - For normal ADC operation this bit must be set, for all reference choices - internal, external or vdda based reference. Setting this bit is critical to proper function of switches inside SARREF block."]
    #[inline(always)]
    pub fn refbuf_en(&self) -> RefbufEnR {
        RefbufEnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Comparator power mode."]
    #[inline(always)]
    pub fn comp_pwr(&self) -> CompPwrR {
        CompPwrR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - - 0: SARMUX IP disabled off during DeepSleep power mode - 1: SARMUX IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub fn deepsleep_on(&self) -> DeepsleepOnR {
        DeepsleepOnR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - - 0: bypass clock domain synchronization of the DSI config signals. - 1: synchronize the DSI config signals to peripheral clock domain."]
    #[inline(always)]
    pub fn dsi_sync_config(&self) -> DsiSyncConfigR {
        DsiSyncConfigR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SAR sequencer takes configuration from DSI signals (note this also has the same effect as SWITCH_DISABLE==1) - 0: Normal mode, SAR sequencer operates according to CHAN_EN enables and CHAN_CONFIG channel configurations - 1: CHAN_EN, INJ_START_EN and channel configurations in CHAN_CONFIG and INJ_CHAN_CONFIG are ignored"]
    #[inline(always)]
    pub fn dsi_mode(&self) -> DsiModeR {
        DsiModeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Disable SAR sequencer from enabling routing switches (note DSI and firmware can always close switches independent of this control) - 0: Normal mode, SAR sequencer changes switches according to pin address in channel configurations - 1: Switches disabled, SAR sequencer does not enable any switches, it is the responsibility of the firmware or UDBs (through DSI) to set the switches to route the signal to be converted through the SARMUX"]
    #[inline(always)]
    pub fn switch_disable(&self) -> SwitchDisableR {
        SwitchDisableR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - - 0: SAR IP disabled (put analog in power down and stop clocks), also can clear FW_TRIGGER and INJ_START_EN (if not tailgating) on write. - 1: SAR IP enabled."]
    #[inline(always)]
    pub fn enabled(&self) -> EnabledR {
        EnabledR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - VREF buffer low power mode."]
    #[inline(always)]
    #[must_use]
    pub fn pwr_ctrl_vref(&mut self) -> PwrCtrlVrefW<CtrlSpec> {
        PwrCtrlVrefW::new(self, 0)
    }
    #[doc = "Bits 4:6 - SARADC internal VREF selection."]
    #[inline(always)]
    #[must_use]
    pub fn vref_sel(&mut self) -> VrefSelW<CtrlSpec> {
        VrefSelW::new(self, 4)
    }
    #[doc = "Bit 7 - VREF bypass cap enable for when VREF buffer is on"]
    #[inline(always)]
    #[must_use]
    pub fn vref_byp_cap_en(&mut self) -> VrefBypCapEnW<CtrlSpec> {
        VrefBypCapEnW::new(self, 7)
    }
    #[doc = "Bits 9:11 - SARADC internal NEG selection for Single ended conversion"]
    #[inline(always)]
    #[must_use]
    pub fn neg_sel(&mut self) -> NegSelW<CtrlSpec> {
        NegSelW::new(self, 9)
    }
    #[doc = "Bit 13 - Hardware control: 0=only firmware control, 1=hardware control masked by firmware setting for VREF to NEG switch."]
    #[inline(always)]
    #[must_use]
    pub fn sar_hw_ctrl_negvref(&mut self) -> SarHwCtrlNegvrefW<CtrlSpec> {
        SarHwCtrlNegvrefW::new(self, 13)
    }
    #[doc = "Bits 14:15 - Set the comparator latch delay in accordance with SAR conversion rate"]
    #[inline(always)]
    #[must_use]
    pub fn comp_dly(&mut self) -> CompDlyW<CtrlSpec> {
        CompDlyW::new(self, 14)
    }
    #[doc = "Bits 16:19 - Spare controls, not yet designated, for late changes done with an ECO"]
    #[inline(always)]
    #[must_use]
    pub fn spare(&mut self) -> SpareW<CtrlSpec> {
        SpareW::new(self, 16)
    }
    #[doc = "Bit 20 - deprecated"]
    #[inline(always)]
    #[must_use]
    pub fn boostpump_en(&mut self) -> BoostpumpEnW<CtrlSpec> {
        BoostpumpEnW::new(self, 20)
    }
    #[doc = "Bit 21 - For normal ADC operation this bit must be set, for all reference choices - internal, external or vdda based reference. Setting this bit is critical to proper function of switches inside SARREF block."]
    #[inline(always)]
    #[must_use]
    pub fn refbuf_en(&mut self) -> RefbufEnW<CtrlSpec> {
        RefbufEnW::new(self, 21)
    }
    #[doc = "Bits 24:26 - Comparator power mode."]
    #[inline(always)]
    #[must_use]
    pub fn comp_pwr(&mut self) -> CompPwrW<CtrlSpec> {
        CompPwrW::new(self, 24)
    }
    #[doc = "Bit 27 - - 0: SARMUX IP disabled off during DeepSleep power mode - 1: SARMUX IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    #[must_use]
    pub fn deepsleep_on(&mut self) -> DeepsleepOnW<CtrlSpec> {
        DeepsleepOnW::new(self, 27)
    }
    #[doc = "Bit 28 - - 0: bypass clock domain synchronization of the DSI config signals. - 1: synchronize the DSI config signals to peripheral clock domain."]
    #[inline(always)]
    #[must_use]
    pub fn dsi_sync_config(&mut self) -> DsiSyncConfigW<CtrlSpec> {
        DsiSyncConfigW::new(self, 28)
    }
    #[doc = "Bit 29 - SAR sequencer takes configuration from DSI signals (note this also has the same effect as SWITCH_DISABLE==1) - 0: Normal mode, SAR sequencer operates according to CHAN_EN enables and CHAN_CONFIG channel configurations - 1: CHAN_EN, INJ_START_EN and channel configurations in CHAN_CONFIG and INJ_CHAN_CONFIG are ignored"]
    #[inline(always)]
    #[must_use]
    pub fn dsi_mode(&mut self) -> DsiModeW<CtrlSpec> {
        DsiModeW::new(self, 29)
    }
    #[doc = "Bit 30 - Disable SAR sequencer from enabling routing switches (note DSI and firmware can always close switches independent of this control) - 0: Normal mode, SAR sequencer changes switches according to pin address in channel configurations - 1: Switches disabled, SAR sequencer does not enable any switches, it is the responsibility of the firmware or UDBs (through DSI) to set the switches to route the signal to be converted through the SARMUX"]
    #[inline(always)]
    #[must_use]
    pub fn switch_disable(&mut self) -> SwitchDisableW<CtrlSpec> {
        SwitchDisableW::new(self, 30)
    }
    #[doc = "Bit 31 - - 0: SAR IP disabled (put analog in power down and stop clocks), also can clear FW_TRIGGER and INJ_START_EN (if not tailgating) on write. - 1: SAR IP enabled."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> EnabledW<CtrlSpec> {
        EnabledW::new(self, 31)
    }
}
#[doc = "Analog control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x1000_0000"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x1000_0000;
}
