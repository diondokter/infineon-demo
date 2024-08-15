#[doc = "Register `PWR_LVD_CTL` reader"]
pub type R = crate::R<PwrLvdCtlSpec>;
#[doc = "Register `PWR_LVD_CTL` writer"]
pub type W = crate::W<PwrLvdCtlSpec>;
#[doc = "Field `HVLVD1_TRIPSEL` reader - Threshold selection for HVLVD1. Disable the LVD (HVLVD1_EN=0) before changing the threshold. 0: rise=1.225V (nom), fall=1.2V (nom) 1: rise=1.425V (nom), fall=1.4V (nom) 2: rise=1.625V (nom), fall=1.6V (nom) 3: rise=1.825V (nom), fall=1.8V (nom) 4: rise=2.025V (nom), fall=2V (nom) 5: rise=2.125V (nom), fall=2.1V (nom) 6: rise=2.225V (nom), fall=2.2V (nom) 7: rise=2.325V (nom), fall=2.3V (nom) 8: rise=2.425V (nom), fall=2.4V (nom) 9: rise=2.525V (nom), fall=2.5V (nom) 10: rise=2.625V (nom), fall=2.6V (nom) 11: rise=2.725V (nom), fall=2.7V (nom) 12: rise=2.825V (nom), fall=2.8V (nom) 13: rise=2.925V (nom), fall=2.9V (nom) 14: rise=3.025V (nom), fall=3.0V (nom) 15: rise=3.125V (nom), fall=3.1V (nom)"]
pub type Hvlvd1TripselR = crate::FieldReader;
#[doc = "Field `HVLVD1_TRIPSEL` writer - Threshold selection for HVLVD1. Disable the LVD (HVLVD1_EN=0) before changing the threshold. 0: rise=1.225V (nom), fall=1.2V (nom) 1: rise=1.425V (nom), fall=1.4V (nom) 2: rise=1.625V (nom), fall=1.6V (nom) 3: rise=1.825V (nom), fall=1.8V (nom) 4: rise=2.025V (nom), fall=2V (nom) 5: rise=2.125V (nom), fall=2.1V (nom) 6: rise=2.225V (nom), fall=2.2V (nom) 7: rise=2.325V (nom), fall=2.3V (nom) 8: rise=2.425V (nom), fall=2.4V (nom) 9: rise=2.525V (nom), fall=2.5V (nom) 10: rise=2.625V (nom), fall=2.6V (nom) 11: rise=2.725V (nom), fall=2.7V (nom) 12: rise=2.825V (nom), fall=2.8V (nom) 13: rise=2.925V (nom), fall=2.9V (nom) 14: rise=3.025V (nom), fall=3.0V (nom) 15: rise=3.125V (nom), fall=3.1V (nom)"]
pub type Hvlvd1TripselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Source selection for HVLVD1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hvlvd1Srcsel {
    #[doc = "0: Select VDDD"]
    Vddd = 0,
    #[doc = "1: Select AMUXBUSA (VDDD branch)"]
    Amuxbusa = 1,
    #[doc = "2: N/A"]
    Rsvd = 2,
    #[doc = "3: N/A"]
    Vddio = 3,
    #[doc = "4: Select AMUXBUSB (VDDD branch)"]
    Amuxbusb = 4,
}
impl From<Hvlvd1Srcsel> for u8 {
    #[inline(always)]
    fn from(variant: Hvlvd1Srcsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hvlvd1Srcsel {
    type Ux = u8;
}
impl crate::IsEnum for Hvlvd1Srcsel {}
#[doc = "Field `HVLVD1_SRCSEL` reader - Source selection for HVLVD1"]
pub type Hvlvd1SrcselR = crate::FieldReader<Hvlvd1Srcsel>;
impl Hvlvd1SrcselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hvlvd1Srcsel> {
        match self.bits {
            0 => Some(Hvlvd1Srcsel::Vddd),
            1 => Some(Hvlvd1Srcsel::Amuxbusa),
            2 => Some(Hvlvd1Srcsel::Rsvd),
            3 => Some(Hvlvd1Srcsel::Vddio),
            4 => Some(Hvlvd1Srcsel::Amuxbusb),
            _ => None,
        }
    }
    #[doc = "Select VDDD"]
    #[inline(always)]
    pub fn is_vddd(&self) -> bool {
        *self == Hvlvd1Srcsel::Vddd
    }
    #[doc = "Select AMUXBUSA (VDDD branch)"]
    #[inline(always)]
    pub fn is_amuxbusa(&self) -> bool {
        *self == Hvlvd1Srcsel::Amuxbusa
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_rsvd(&self) -> bool {
        *self == Hvlvd1Srcsel::Rsvd
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn is_vddio(&self) -> bool {
        *self == Hvlvd1Srcsel::Vddio
    }
    #[doc = "Select AMUXBUSB (VDDD branch)"]
    #[inline(always)]
    pub fn is_amuxbusb(&self) -> bool {
        *self == Hvlvd1Srcsel::Amuxbusb
    }
}
#[doc = "Field `HVLVD1_SRCSEL` writer - Source selection for HVLVD1"]
pub type Hvlvd1SrcselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Hvlvd1Srcsel>;
impl<'a, REG> Hvlvd1SrcselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select VDDD"]
    #[inline(always)]
    pub fn vddd(self) -> &'a mut crate::W<REG> {
        self.variant(Hvlvd1Srcsel::Vddd)
    }
    #[doc = "Select AMUXBUSA (VDDD branch)"]
    #[inline(always)]
    pub fn amuxbusa(self) -> &'a mut crate::W<REG> {
        self.variant(Hvlvd1Srcsel::Amuxbusa)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn rsvd(self) -> &'a mut crate::W<REG> {
        self.variant(Hvlvd1Srcsel::Rsvd)
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn vddio(self) -> &'a mut crate::W<REG> {
        self.variant(Hvlvd1Srcsel::Vddio)
    }
    #[doc = "Select AMUXBUSB (VDDD branch)"]
    #[inline(always)]
    pub fn amuxbusb(self) -> &'a mut crate::W<REG> {
        self.variant(Hvlvd1Srcsel::Amuxbusb)
    }
}
#[doc = "Field `HVLVD1_EN` reader - Enable HVLVD1 voltage monitor. When the LVD is enabled, it takes 20us for it to settle. There is no hardware stabilization counter, and it may falsely trigger during settling. It is recommended that firmware keep the interrupt masked for at least 8us, write a 1'b1 to the corresponding SRSS_INTR field to any falsely pended interrupt, and then optionally unmask the interrupt. After enabling, it is further recommended to read the related PWR_LVD_STATUS field, since the interrupt only triggers on edges. This bit is cleared (LVD is disabled) when entering DEEPSLEEP to prevent false interrupts during wakeup."]
pub type Hvlvd1EnR = crate::BitReader;
#[doc = "Field `HVLVD1_EN` writer - Enable HVLVD1 voltage monitor. When the LVD is enabled, it takes 20us for it to settle. There is no hardware stabilization counter, and it may falsely trigger during settling. It is recommended that firmware keep the interrupt masked for at least 8us, write a 1'b1 to the corresponding SRSS_INTR field to any falsely pended interrupt, and then optionally unmask the interrupt. After enabling, it is further recommended to read the related PWR_LVD_STATUS field, since the interrupt only triggers on edges. This bit is cleared (LVD is disabled) when entering DEEPSLEEP to prevent false interrupts during wakeup."]
pub type Hvlvd1EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Threshold selection for HVLVD1. Disable the LVD (HVLVD1_EN=0) before changing the threshold. 0: rise=1.225V (nom), fall=1.2V (nom) 1: rise=1.425V (nom), fall=1.4V (nom) 2: rise=1.625V (nom), fall=1.6V (nom) 3: rise=1.825V (nom), fall=1.8V (nom) 4: rise=2.025V (nom), fall=2V (nom) 5: rise=2.125V (nom), fall=2.1V (nom) 6: rise=2.225V (nom), fall=2.2V (nom) 7: rise=2.325V (nom), fall=2.3V (nom) 8: rise=2.425V (nom), fall=2.4V (nom) 9: rise=2.525V (nom), fall=2.5V (nom) 10: rise=2.625V (nom), fall=2.6V (nom) 11: rise=2.725V (nom), fall=2.7V (nom) 12: rise=2.825V (nom), fall=2.8V (nom) 13: rise=2.925V (nom), fall=2.9V (nom) 14: rise=3.025V (nom), fall=3.0V (nom) 15: rise=3.125V (nom), fall=3.1V (nom)"]
    #[inline(always)]
    pub fn hvlvd1_tripsel(&self) -> Hvlvd1TripselR {
        Hvlvd1TripselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Source selection for HVLVD1"]
    #[inline(always)]
    pub fn hvlvd1_srcsel(&self) -> Hvlvd1SrcselR {
        Hvlvd1SrcselR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Enable HVLVD1 voltage monitor. When the LVD is enabled, it takes 20us for it to settle. There is no hardware stabilization counter, and it may falsely trigger during settling. It is recommended that firmware keep the interrupt masked for at least 8us, write a 1'b1 to the corresponding SRSS_INTR field to any falsely pended interrupt, and then optionally unmask the interrupt. After enabling, it is further recommended to read the related PWR_LVD_STATUS field, since the interrupt only triggers on edges. This bit is cleared (LVD is disabled) when entering DEEPSLEEP to prevent false interrupts during wakeup."]
    #[inline(always)]
    pub fn hvlvd1_en(&self) -> Hvlvd1EnR {
        Hvlvd1EnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Threshold selection for HVLVD1. Disable the LVD (HVLVD1_EN=0) before changing the threshold. 0: rise=1.225V (nom), fall=1.2V (nom) 1: rise=1.425V (nom), fall=1.4V (nom) 2: rise=1.625V (nom), fall=1.6V (nom) 3: rise=1.825V (nom), fall=1.8V (nom) 4: rise=2.025V (nom), fall=2V (nom) 5: rise=2.125V (nom), fall=2.1V (nom) 6: rise=2.225V (nom), fall=2.2V (nom) 7: rise=2.325V (nom), fall=2.3V (nom) 8: rise=2.425V (nom), fall=2.4V (nom) 9: rise=2.525V (nom), fall=2.5V (nom) 10: rise=2.625V (nom), fall=2.6V (nom) 11: rise=2.725V (nom), fall=2.7V (nom) 12: rise=2.825V (nom), fall=2.8V (nom) 13: rise=2.925V (nom), fall=2.9V (nom) 14: rise=3.025V (nom), fall=3.0V (nom) 15: rise=3.125V (nom), fall=3.1V (nom)"]
    #[inline(always)]
    #[must_use]
    pub fn hvlvd1_tripsel(&mut self) -> Hvlvd1TripselW<PwrLvdCtlSpec> {
        Hvlvd1TripselW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Source selection for HVLVD1"]
    #[inline(always)]
    #[must_use]
    pub fn hvlvd1_srcsel(&mut self) -> Hvlvd1SrcselW<PwrLvdCtlSpec> {
        Hvlvd1SrcselW::new(self, 4)
    }
    #[doc = "Bit 7 - Enable HVLVD1 voltage monitor. When the LVD is enabled, it takes 20us for it to settle. There is no hardware stabilization counter, and it may falsely trigger during settling. It is recommended that firmware keep the interrupt masked for at least 8us, write a 1'b1 to the corresponding SRSS_INTR field to any falsely pended interrupt, and then optionally unmask the interrupt. After enabling, it is further recommended to read the related PWR_LVD_STATUS field, since the interrupt only triggers on edges. This bit is cleared (LVD is disabled) when entering DEEPSLEEP to prevent false interrupts during wakeup."]
    #[inline(always)]
    #[must_use]
    pub fn hvlvd1_en(&mut self) -> Hvlvd1EnW<PwrLvdCtlSpec> {
        Hvlvd1EnW::new(self, 7)
    }
}
#[doc = "Low Voltage Detector (LVD) Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_lvd_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_lvd_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrLvdCtlSpec;
impl crate::RegisterSpec for PwrLvdCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_lvd_ctl::R`](R) reader structure"]
impl crate::Readable for PwrLvdCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`pwr_lvd_ctl::W`](W) writer structure"]
impl crate::Writable for PwrLvdCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_LVD_CTL to value 0"]
impl crate::Resettable for PwrLvdCtlSpec {
    const RESET_VALUE: u32 = 0;
}
