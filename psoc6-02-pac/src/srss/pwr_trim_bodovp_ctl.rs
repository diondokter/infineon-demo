#[doc = "Register `PWR_TRIM_BODOVP_CTL` reader"]
pub type R = crate::R<PwrTrimBodovpCtlSpec>;
#[doc = "Register `PWR_TRIM_BODOVP_CTL` writer"]
pub type W = crate::W<PwrTrimBodovpCtlSpec>;
#[doc = "Field `HVPORBOD_TRIPSEL` reader - HVPORBOD trip point selection. Monitors vddd. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type HvporbodTripselR = crate::FieldReader;
#[doc = "Field `HVPORBOD_TRIPSEL` writer - HVPORBOD trip point selection. Monitors vddd. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type HvporbodTripselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HVPORBOD_OFSTRIM` reader - HVPORBOD offset trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type HvporbodOfstrimR = crate::FieldReader;
#[doc = "Field `HVPORBOD_OFSTRIM` writer - HVPORBOD offset trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type HvporbodOfstrimW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HVPORBOD_ITRIM` reader - HVPORBOD current trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type HvporbodItrimR = crate::FieldReader;
#[doc = "Field `HVPORBOD_ITRIM` writer - HVPORBOD current trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type HvporbodItrimW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LVPORBOD_TRIPSEL` reader - LVPORBOD trip point selection. Monitors vccd. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type LvporbodTripselR = crate::FieldReader;
#[doc = "Field `LVPORBOD_TRIPSEL` writer - LVPORBOD trip point selection. Monitors vccd. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type LvporbodTripselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LVPORBOD_OFSTRIM` reader - LVPORBOD offset trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type LvporbodOfstrimR = crate::FieldReader;
#[doc = "Field `LVPORBOD_OFSTRIM` writer - LVPORBOD offset trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type LvporbodOfstrimW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LVPORBOD_ITRIM` reader - LVPORBOD current trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type LvporbodItrimR = crate::FieldReader;
#[doc = "Field `LVPORBOD_ITRIM` writer - LVPORBOD current trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
pub type LvporbodItrimW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - HVPORBOD trip point selection. Monitors vddd. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn hvporbod_tripsel(&self) -> HvporbodTripselR {
        HvporbodTripselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - HVPORBOD offset trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn hvporbod_ofstrim(&self) -> HvporbodOfstrimR {
        HvporbodOfstrimR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9 - HVPORBOD current trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn hvporbod_itrim(&self) -> HvporbodItrimR {
        HvporbodItrimR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 10:12 - LVPORBOD trip point selection. Monitors vccd. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn lvporbod_tripsel(&self) -> LvporbodTripselR {
        LvporbodTripselR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 14:16 - LVPORBOD offset trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn lvporbod_ofstrim(&self) -> LvporbodOfstrimR {
        LvporbodOfstrimR::new(((self.bits >> 14) & 7) as u8)
    }
    #[doc = "Bits 17:19 - LVPORBOD current trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    pub fn lvporbod_itrim(&self) -> LvporbodItrimR {
        LvporbodItrimR::new(((self.bits >> 17) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - HVPORBOD trip point selection. Monitors vddd. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    #[must_use]
    pub fn hvporbod_tripsel(&mut self) -> HvporbodTripselW<PwrTrimBodovpCtlSpec> {
        HvporbodTripselW::new(self, 0)
    }
    #[doc = "Bits 4:6 - HVPORBOD offset trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    #[must_use]
    pub fn hvporbod_ofstrim(&mut self) -> HvporbodOfstrimW<PwrTrimBodovpCtlSpec> {
        HvporbodOfstrimW::new(self, 4)
    }
    #[doc = "Bits 7:9 - HVPORBOD current trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    #[must_use]
    pub fn hvporbod_itrim(&mut self) -> HvporbodItrimW<PwrTrimBodovpCtlSpec> {
        HvporbodItrimW::new(self, 7)
    }
    #[doc = "Bits 10:12 - LVPORBOD trip point selection. Monitors vccd. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    #[must_use]
    pub fn lvporbod_tripsel(&mut self) -> LvporbodTripselW<PwrTrimBodovpCtlSpec> {
        LvporbodTripselW::new(self, 10)
    }
    #[doc = "Bits 14:16 - LVPORBOD offset trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    #[must_use]
    pub fn lvporbod_ofstrim(&mut self) -> LvporbodOfstrimW<PwrTrimBodovpCtlSpec> {
        LvporbodOfstrimW::new(self, 14)
    }
    #[doc = "Bits 17:19 - LVPORBOD current trim. This register is only reset by XRES/POR/BOD/HIBERNATE."]
    #[inline(always)]
    #[must_use]
    pub fn lvporbod_itrim(&mut self) -> LvporbodItrimW<PwrTrimBodovpCtlSpec> {
        LvporbodItrimW::new(self, 17)
    }
}
#[doc = "BOD/OVP Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_trim_bodovp_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_trim_bodovp_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrTrimBodovpCtlSpec;
impl crate::RegisterSpec for PwrTrimBodovpCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_trim_bodovp_ctl::R`](R) reader structure"]
impl crate::Readable for PwrTrimBodovpCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`pwr_trim_bodovp_ctl::W`](W) writer structure"]
impl crate::Writable for PwrTrimBodovpCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_TRIM_BODOVP_CTL to value 0x0004_0d04"]
impl crate::Resettable for PwrTrimBodovpCtlSpec {
    const RESET_VALUE: u32 = 0x0004_0d04;
}
