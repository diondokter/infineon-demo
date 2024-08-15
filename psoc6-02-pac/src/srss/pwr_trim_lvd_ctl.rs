#[doc = "Register `PWR_TRIM_LVD_CTL` reader"]
pub type R = crate::R<PwrTrimLvdCtlSpec>;
#[doc = "Register `PWR_TRIM_LVD_CTL` writer"]
pub type W = crate::W<PwrTrimLvdCtlSpec>;
#[doc = "Field `HVLVD1_OFSTRIM` reader - HVLVD1 offset trim"]
pub type Hvlvd1OfstrimR = crate::FieldReader;
#[doc = "Field `HVLVD1_OFSTRIM` writer - HVLVD1 offset trim"]
pub type Hvlvd1OfstrimW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HVLVD1_ITRIM` reader - HVLVD1 current trim"]
pub type Hvlvd1ItrimR = crate::FieldReader;
#[doc = "Field `HVLVD1_ITRIM` writer - HVLVD1 current trim"]
pub type Hvlvd1ItrimW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - HVLVD1 offset trim"]
    #[inline(always)]
    pub fn hvlvd1_ofstrim(&self) -> Hvlvd1OfstrimR {
        Hvlvd1OfstrimR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - HVLVD1 current trim"]
    #[inline(always)]
    pub fn hvlvd1_itrim(&self) -> Hvlvd1ItrimR {
        Hvlvd1ItrimR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - HVLVD1 offset trim"]
    #[inline(always)]
    #[must_use]
    pub fn hvlvd1_ofstrim(&mut self) -> Hvlvd1OfstrimW<PwrTrimLvdCtlSpec> {
        Hvlvd1OfstrimW::new(self, 0)
    }
    #[doc = "Bits 4:6 - HVLVD1 current trim"]
    #[inline(always)]
    #[must_use]
    pub fn hvlvd1_itrim(&mut self) -> Hvlvd1ItrimW<PwrTrimLvdCtlSpec> {
        Hvlvd1ItrimW::new(self, 4)
    }
}
#[doc = "LVD Trim Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_trim_lvd_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_trim_lvd_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrTrimLvdCtlSpec;
impl crate::RegisterSpec for PwrTrimLvdCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_trim_lvd_ctl::R`](R) reader structure"]
impl crate::Readable for PwrTrimLvdCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`pwr_trim_lvd_ctl::W`](W) writer structure"]
impl crate::Writable for PwrTrimLvdCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_TRIM_LVD_CTL to value 0x20"]
impl crate::Resettable for PwrTrimLvdCtlSpec {
    const RESET_VALUE: u32 = 0x20;
}
