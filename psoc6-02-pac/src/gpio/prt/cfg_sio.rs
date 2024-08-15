#[doc = "Register `CFG_SIO` reader"]
pub type R = crate::R<CfgSioSpec>;
#[doc = "Register `CFG_SIO` writer"]
pub type W = crate::W<CfgSioSpec>;
#[doc = "Field `VREG_EN01` reader - Selects the output buffer mode: '0': Unregulated output buffer '1': Regulated output buffer The regulated output mode is selected ONLY if the CFG.DRIVE_MODE bits are set to the strong pull up (Z_1 = '5') mode. If the CFG.DRIVE_MODE bits are set to any other mode the regulated output buffer will be disabled and the standard CMOS output buffer is used."]
pub type VregEn01R = crate::BitReader;
#[doc = "Field `VREG_EN01` writer - Selects the output buffer mode: '0': Unregulated output buffer '1': Regulated output buffer The regulated output mode is selected ONLY if the CFG.DRIVE_MODE bits are set to the strong pull up (Z_1 = '5') mode. If the CFG.DRIVE_MODE bits are set to any other mode the regulated output buffer will be disabled and the standard CMOS output buffer is used."]
pub type VregEn01W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBUF_SEL01` reader - Selects the input buffer mode: 0: Singled ended input buffer 1: Differential input buffer"]
pub type IbufSel01R = crate::BitReader;
#[doc = "Field `IBUF_SEL01` writer - Selects the input buffer mode: 0: Singled ended input buffer 1: Differential input buffer"]
pub type IbufSel01W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL01` reader - Selects the input buffer trip-point in single ended input buffer mode (IBUF_SEL = '0'): '0': Input buffer functions as a CMOS input buffer. '1': Input buffer functions as a TTL input buffer. In differential input buffer mode (IBUF_SEL = '1') '0': Trip-point is 0.5*Vddio or 0.5*Voh (depends on VREF_SEL/VOH_SEL) '1': Trip-point is 0.4*Vddio or 1.0*Vref (depends on VREF_SEL)"]
pub type VtripSel01R = crate::BitReader;
#[doc = "Field `VTRIP_SEL01` writer - Selects the input buffer trip-point in single ended input buffer mode (IBUF_SEL = '0'): '0': Input buffer functions as a CMOS input buffer. '1': Input buffer functions as a TTL input buffer. In differential input buffer mode (IBUF_SEL = '1') '0': Trip-point is 0.5*Vddio or 0.5*Voh (depends on VREF_SEL/VOH_SEL) '1': Trip-point is 0.4*Vddio or 1.0*Vref (depends on VREF_SEL)"]
pub type VtripSel01W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREF_SEL01` reader - Selects reference voltage (Vref) trip-point of the input buffer: '0': Trip-point reference from pin_ref '1': Trip-point reference of SRSS internal reference Vref (1.2 V) '2': Trip-point reference of AMUXBUS_A '3': Trip-point reference of AMUXBUS_B"]
pub type VrefSel01R = crate::FieldReader;
#[doc = "Field `VREF_SEL01` writer - Selects reference voltage (Vref) trip-point of the input buffer: '0': Trip-point reference from pin_ref '1': Trip-point reference of SRSS internal reference Vref (1.2 V) '2': Trip-point reference of AMUXBUS_A '3': Trip-point reference of AMUXBUS_B"]
pub type VrefSel01W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VOH_SEL01` reader - Selects the regulated Voh output level and trip point of the input buffer for a specific SIO pin pair. Voh depends on the selected reference voltage (VREF_SEL). '0': Voh = 1*reference; e.g. reference at 1.2V -> Voh = 1.2V '1': Voh = 1.25*reference; e.g. reference at 1.2V -> Voh = 1.5V '2': Voh = 1.49*reference; e.g. reference at 1.2V -> Voh = ~1.8V '3': Voh = 1.67*reference; e.g. reference at 1.2V -> Voh = 2V '4': Voh = 2.08*reference; e.g. reference at 1.2V -> Voh = 2.5V '5': Voh = 2.5*reference; e.g. reference at 1.2V -> Voh = 3V '6': Voh = 2.78*reference; e.g. reference at 1.2V -> Voh = ~3.3V '7': Voh = 4.16*reference; e.g. reference at 1.2V -> Voh = 5.0V Note: The upper value on Voh is limited to Vddio - 400mV"]
pub type VohSel01R = crate::FieldReader;
#[doc = "Field `VOH_SEL01` writer - Selects the regulated Voh output level and trip point of the input buffer for a specific SIO pin pair. Voh depends on the selected reference voltage (VREF_SEL). '0': Voh = 1*reference; e.g. reference at 1.2V -> Voh = 1.2V '1': Voh = 1.25*reference; e.g. reference at 1.2V -> Voh = 1.5V '2': Voh = 1.49*reference; e.g. reference at 1.2V -> Voh = ~1.8V '3': Voh = 1.67*reference; e.g. reference at 1.2V -> Voh = 2V '4': Voh = 2.08*reference; e.g. reference at 1.2V -> Voh = 2.5V '5': Voh = 2.5*reference; e.g. reference at 1.2V -> Voh = 3V '6': Voh = 2.78*reference; e.g. reference at 1.2V -> Voh = ~3.3V '7': Voh = 4.16*reference; e.g. reference at 1.2V -> Voh = 5.0V Note: The upper value on Voh is limited to Vddio - 400mV"]
pub type VohSel01W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VREG_EN23` reader - See corresponding definition for IO pins 0 and 1"]
pub type VregEn23R = crate::BitReader;
#[doc = "Field `VREG_EN23` writer - See corresponding definition for IO pins 0 and 1"]
pub type VregEn23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBUF_SEL23` reader - See corresponding definition for IO pins 0 and 1"]
pub type IbufSel23R = crate::BitReader;
#[doc = "Field `IBUF_SEL23` writer - See corresponding definition for IO pins 0 and 1"]
pub type IbufSel23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL23` reader - See corresponding definition for IO pins 0 and 1"]
pub type VtripSel23R = crate::BitReader;
#[doc = "Field `VTRIP_SEL23` writer - See corresponding definition for IO pins 0 and 1"]
pub type VtripSel23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREF_SEL23` reader - See corresponding definition for IO pins 0 and 1"]
pub type VrefSel23R = crate::FieldReader;
#[doc = "Field `VREF_SEL23` writer - See corresponding definition for IO pins 0 and 1"]
pub type VrefSel23W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VOH_SEL23` reader - See corresponding definition for IO pins 0 and 1"]
pub type VohSel23R = crate::FieldReader;
#[doc = "Field `VOH_SEL23` writer - See corresponding definition for IO pins 0 and 1"]
pub type VohSel23W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VREG_EN45` reader - See corresponding definition for IO pins 0 and 1"]
pub type VregEn45R = crate::BitReader;
#[doc = "Field `VREG_EN45` writer - See corresponding definition for IO pins 0 and 1"]
pub type VregEn45W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBUF_SEL45` reader - See corresponding definition for IO pins 0 and 1"]
pub type IbufSel45R = crate::BitReader;
#[doc = "Field `IBUF_SEL45` writer - See corresponding definition for IO pins 0 and 1"]
pub type IbufSel45W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL45` reader - See corresponding definition for IO pins 0 and 1"]
pub type VtripSel45R = crate::BitReader;
#[doc = "Field `VTRIP_SEL45` writer - See corresponding definition for IO pins 0 and 1"]
pub type VtripSel45W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREF_SEL45` reader - See corresponding definition for IO pins 0 and 1"]
pub type VrefSel45R = crate::FieldReader;
#[doc = "Field `VREF_SEL45` writer - See corresponding definition for IO pins 0 and 1"]
pub type VrefSel45W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VOH_SEL45` reader - See corresponding definition for IO pins 0 and 1"]
pub type VohSel45R = crate::FieldReader;
#[doc = "Field `VOH_SEL45` writer - See corresponding definition for IO pins 0 and 1"]
pub type VohSel45W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `VREG_EN67` reader - See corresponding definition for IO pins 0 and 1"]
pub type VregEn67R = crate::BitReader;
#[doc = "Field `VREG_EN67` writer - See corresponding definition for IO pins 0 and 1"]
pub type VregEn67W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBUF_SEL67` reader - See corresponding definition for IO pins 0 and 1"]
pub type IbufSel67R = crate::BitReader;
#[doc = "Field `IBUF_SEL67` writer - See corresponding definition for IO pins 0 and 1"]
pub type IbufSel67W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL67` reader - See corresponding definition for IO pins 0 and 1"]
pub type VtripSel67R = crate::BitReader;
#[doc = "Field `VTRIP_SEL67` writer - See corresponding definition for IO pins 0 and 1"]
pub type VtripSel67W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREF_SEL67` reader - See corresponding definition for IO pins 0 and 1"]
pub type VrefSel67R = crate::FieldReader;
#[doc = "Field `VREF_SEL67` writer - See corresponding definition for IO pins 0 and 1"]
pub type VrefSel67W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VOH_SEL67` reader - See corresponding definition for IO pins 0 and 1"]
pub type VohSel67R = crate::FieldReader;
#[doc = "Field `VOH_SEL67` writer - See corresponding definition for IO pins 0 and 1"]
pub type VohSel67W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Selects the output buffer mode: '0': Unregulated output buffer '1': Regulated output buffer The regulated output mode is selected ONLY if the CFG.DRIVE_MODE bits are set to the strong pull up (Z_1 = '5') mode. If the CFG.DRIVE_MODE bits are set to any other mode the regulated output buffer will be disabled and the standard CMOS output buffer is used."]
    #[inline(always)]
    pub fn vreg_en01(&self) -> VregEn01R {
        VregEn01R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects the input buffer mode: 0: Singled ended input buffer 1: Differential input buffer"]
    #[inline(always)]
    pub fn ibuf_sel01(&self) -> IbufSel01R {
        IbufSel01R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects the input buffer trip-point in single ended input buffer mode (IBUF_SEL = '0'): '0': Input buffer functions as a CMOS input buffer. '1': Input buffer functions as a TTL input buffer. In differential input buffer mode (IBUF_SEL = '1') '0': Trip-point is 0.5*Vddio or 0.5*Voh (depends on VREF_SEL/VOH_SEL) '1': Trip-point is 0.4*Vddio or 1.0*Vref (depends on VREF_SEL)"]
    #[inline(always)]
    pub fn vtrip_sel01(&self) -> VtripSel01R {
        VtripSel01R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Selects reference voltage (Vref) trip-point of the input buffer: '0': Trip-point reference from pin_ref '1': Trip-point reference of SRSS internal reference Vref (1.2 V) '2': Trip-point reference of AMUXBUS_A '3': Trip-point reference of AMUXBUS_B"]
    #[inline(always)]
    pub fn vref_sel01(&self) -> VrefSel01R {
        VrefSel01R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - Selects the regulated Voh output level and trip point of the input buffer for a specific SIO pin pair. Voh depends on the selected reference voltage (VREF_SEL). '0': Voh = 1*reference; e.g. reference at 1.2V -> Voh = 1.2V '1': Voh = 1.25*reference; e.g. reference at 1.2V -> Voh = 1.5V '2': Voh = 1.49*reference; e.g. reference at 1.2V -> Voh = ~1.8V '3': Voh = 1.67*reference; e.g. reference at 1.2V -> Voh = 2V '4': Voh = 2.08*reference; e.g. reference at 1.2V -> Voh = 2.5V '5': Voh = 2.5*reference; e.g. reference at 1.2V -> Voh = 3V '6': Voh = 2.78*reference; e.g. reference at 1.2V -> Voh = ~3.3V '7': Voh = 4.16*reference; e.g. reference at 1.2V -> Voh = 5.0V Note: The upper value on Voh is limited to Vddio - 400mV"]
    #[inline(always)]
    pub fn voh_sel01(&self) -> VohSel01R {
        VohSel01R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vreg_en23(&self) -> VregEn23R {
        VregEn23R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn ibuf_sel23(&self) -> IbufSel23R {
        IbufSel23R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vtrip_sel23(&self) -> VtripSel23R {
        VtripSel23R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vref_sel23(&self) -> VrefSel23R {
        VrefSel23R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:15 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn voh_sel23(&self) -> VohSel23R {
        VohSel23R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vreg_en45(&self) -> VregEn45R {
        VregEn45R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn ibuf_sel45(&self) -> IbufSel45R {
        IbufSel45R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vtrip_sel45(&self) -> VtripSel45R {
        VtripSel45R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vref_sel45(&self) -> VrefSel45R {
        VrefSel45R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bits 21:23 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn voh_sel45(&self) -> VohSel45R {
        VohSel45R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vreg_en67(&self) -> VregEn67R {
        VregEn67R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn ibuf_sel67(&self) -> IbufSel67R {
        IbufSel67R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vtrip_sel67(&self) -> VtripSel67R {
        VtripSel67R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn vref_sel67(&self) -> VrefSel67R {
        VrefSel67R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 29:31 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    pub fn voh_sel67(&self) -> VohSel67R {
        VohSel67R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Selects the output buffer mode: '0': Unregulated output buffer '1': Regulated output buffer The regulated output mode is selected ONLY if the CFG.DRIVE_MODE bits are set to the strong pull up (Z_1 = '5') mode. If the CFG.DRIVE_MODE bits are set to any other mode the regulated output buffer will be disabled and the standard CMOS output buffer is used."]
    #[inline(always)]
    #[must_use]
    pub fn vreg_en01(&mut self) -> VregEn01W<CfgSioSpec> {
        VregEn01W::new(self, 0)
    }
    #[doc = "Bit 1 - Selects the input buffer mode: 0: Singled ended input buffer 1: Differential input buffer"]
    #[inline(always)]
    #[must_use]
    pub fn ibuf_sel01(&mut self) -> IbufSel01W<CfgSioSpec> {
        IbufSel01W::new(self, 1)
    }
    #[doc = "Bit 2 - Selects the input buffer trip-point in single ended input buffer mode (IBUF_SEL = '0'): '0': Input buffer functions as a CMOS input buffer. '1': Input buffer functions as a TTL input buffer. In differential input buffer mode (IBUF_SEL = '1') '0': Trip-point is 0.5*Vddio or 0.5*Voh (depends on VREF_SEL/VOH_SEL) '1': Trip-point is 0.4*Vddio or 1.0*Vref (depends on VREF_SEL)"]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel01(&mut self) -> VtripSel01W<CfgSioSpec> {
        VtripSel01W::new(self, 2)
    }
    #[doc = "Bits 3:4 - Selects reference voltage (Vref) trip-point of the input buffer: '0': Trip-point reference from pin_ref '1': Trip-point reference of SRSS internal reference Vref (1.2 V) '2': Trip-point reference of AMUXBUS_A '3': Trip-point reference of AMUXBUS_B"]
    #[inline(always)]
    #[must_use]
    pub fn vref_sel01(&mut self) -> VrefSel01W<CfgSioSpec> {
        VrefSel01W::new(self, 3)
    }
    #[doc = "Bits 5:7 - Selects the regulated Voh output level and trip point of the input buffer for a specific SIO pin pair. Voh depends on the selected reference voltage (VREF_SEL). '0': Voh = 1*reference; e.g. reference at 1.2V -> Voh = 1.2V '1': Voh = 1.25*reference; e.g. reference at 1.2V -> Voh = 1.5V '2': Voh = 1.49*reference; e.g. reference at 1.2V -> Voh = ~1.8V '3': Voh = 1.67*reference; e.g. reference at 1.2V -> Voh = 2V '4': Voh = 2.08*reference; e.g. reference at 1.2V -> Voh = 2.5V '5': Voh = 2.5*reference; e.g. reference at 1.2V -> Voh = 3V '6': Voh = 2.78*reference; e.g. reference at 1.2V -> Voh = ~3.3V '7': Voh = 4.16*reference; e.g. reference at 1.2V -> Voh = 5.0V Note: The upper value on Voh is limited to Vddio - 400mV"]
    #[inline(always)]
    #[must_use]
    pub fn voh_sel01(&mut self) -> VohSel01W<CfgSioSpec> {
        VohSel01W::new(self, 5)
    }
    #[doc = "Bit 8 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn vreg_en23(&mut self) -> VregEn23W<CfgSioSpec> {
        VregEn23W::new(self, 8)
    }
    #[doc = "Bit 9 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn ibuf_sel23(&mut self) -> IbufSel23W<CfgSioSpec> {
        IbufSel23W::new(self, 9)
    }
    #[doc = "Bit 10 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel23(&mut self) -> VtripSel23W<CfgSioSpec> {
        VtripSel23W::new(self, 10)
    }
    #[doc = "Bits 11:12 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn vref_sel23(&mut self) -> VrefSel23W<CfgSioSpec> {
        VrefSel23W::new(self, 11)
    }
    #[doc = "Bits 13:15 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn voh_sel23(&mut self) -> VohSel23W<CfgSioSpec> {
        VohSel23W::new(self, 13)
    }
    #[doc = "Bit 16 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn vreg_en45(&mut self) -> VregEn45W<CfgSioSpec> {
        VregEn45W::new(self, 16)
    }
    #[doc = "Bit 17 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn ibuf_sel45(&mut self) -> IbufSel45W<CfgSioSpec> {
        IbufSel45W::new(self, 17)
    }
    #[doc = "Bit 18 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel45(&mut self) -> VtripSel45W<CfgSioSpec> {
        VtripSel45W::new(self, 18)
    }
    #[doc = "Bits 19:20 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn vref_sel45(&mut self) -> VrefSel45W<CfgSioSpec> {
        VrefSel45W::new(self, 19)
    }
    #[doc = "Bits 21:23 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn voh_sel45(&mut self) -> VohSel45W<CfgSioSpec> {
        VohSel45W::new(self, 21)
    }
    #[doc = "Bit 24 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn vreg_en67(&mut self) -> VregEn67W<CfgSioSpec> {
        VregEn67W::new(self, 24)
    }
    #[doc = "Bit 25 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn ibuf_sel67(&mut self) -> IbufSel67W<CfgSioSpec> {
        IbufSel67W::new(self, 25)
    }
    #[doc = "Bit 26 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel67(&mut self) -> VtripSel67W<CfgSioSpec> {
        VtripSel67W::new(self, 26)
    }
    #[doc = "Bits 27:28 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn vref_sel67(&mut self) -> VrefSel67W<CfgSioSpec> {
        VrefSel67W::new(self, 27)
    }
    #[doc = "Bits 29:31 - See corresponding definition for IO pins 0 and 1"]
    #[inline(always)]
    #[must_use]
    pub fn voh_sel67(&mut self) -> VohSel67W<CfgSioSpec> {
        VohSel67W::new(self, 29)
    }
}
#[doc = "Port SIO configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_sio::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_sio::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSioSpec;
impl crate::RegisterSpec for CfgSioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_sio::R`](R) reader structure"]
impl crate::Readable for CfgSioSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_sio::W`](W) writer structure"]
impl crate::Writable for CfgSioSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_SIO to value 0"]
impl crate::Resettable for CfgSioSpec {
    const RESET_VALUE: u32 = 0;
}
