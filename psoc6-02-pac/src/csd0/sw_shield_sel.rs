#[doc = "Register `SW_SHIELD_SEL` reader"]
pub type R = crate::R<SwShieldSelSpec>;
#[doc = "Register `SW_SHIELD_SEL` writer"]
pub type W = crate::W<SwShieldSelSpec>;
#[doc = "Field `SW_HCAV` reader - N/A"]
pub type SwHcavR = crate::FieldReader;
#[doc = "Field `SW_HCAV` writer - N/A"]
pub type SwHcavW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_HCAG` reader - Select waveform for corresponding switch"]
pub type SwHcagR = crate::FieldReader;
#[doc = "Field `SW_HCAG` writer - Select waveform for corresponding switch"]
pub type SwHcagW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_HCBV` reader - N/A"]
pub type SwHcbvR = crate::FieldReader;
#[doc = "Field `SW_HCBV` writer - N/A"]
pub type SwHcbvW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_HCBG` reader - Select waveform for corresponding switch, using csd_shield as base"]
pub type SwHcbgR = crate::FieldReader;
#[doc = "Field `SW_HCBG` writer - Select waveform for corresponding switch, using csd_shield as base"]
pub type SwHcbgW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_HCCV` reader - Set corresponding switch"]
pub type SwHccvR = crate::BitReader;
#[doc = "Field `SW_HCCV` writer - Set corresponding switch"]
pub type SwHccvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_HCCG` reader - Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
pub type SwHccgR = crate::BitReader;
#[doc = "Field `SW_HCCG` writer - Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
pub type SwHccgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - N/A"]
    #[inline(always)]
    pub fn sw_hcav(&self) -> SwHcavR {
        SwHcavR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_hcag(&self) -> SwHcagR {
        SwHcagR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - N/A"]
    #[inline(always)]
    pub fn sw_hcbv(&self) -> SwHcbvR {
        SwHcbvR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Select waveform for corresponding switch, using csd_shield as base"]
    #[inline(always)]
    pub fn sw_hcbg(&self) -> SwHcbgR {
        SwHcbgR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hccv(&self) -> SwHccvR {
        SwHccvR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
    #[inline(always)]
    pub fn sw_hccg(&self) -> SwHccgR {
        SwHccgR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hcav(&mut self) -> SwHcavW<SwShieldSelSpec> {
        SwHcavW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Select waveform for corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hcag(&mut self) -> SwHcagW<SwShieldSelSpec> {
        SwHcagW::new(self, 4)
    }
    #[doc = "Bits 8:10 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hcbv(&mut self) -> SwHcbvW<SwShieldSelSpec> {
        SwHcbvW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Select waveform for corresponding switch, using csd_shield as base"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hcbg(&mut self) -> SwHcbgW<SwShieldSelSpec> {
        SwHcbgW::new(self, 12)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hccv(&mut self) -> SwHccvW<SwShieldSelSpec> {
        SwHccvW::new(self, 16)
    }
    #[doc = "Bit 20 - Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
    #[inline(always)]
    #[must_use]
    pub fn sw_hccg(&mut self) -> SwHccgW<SwShieldSelSpec> {
        SwHccgW::new(self, 20)
    }
}
#[doc = "Shielding switches Waveform selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_shield_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_shield_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwShieldSelSpec;
impl crate::RegisterSpec for SwShieldSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_shield_sel::R`](R) reader structure"]
impl crate::Readable for SwShieldSelSpec {}
#[doc = "`write(|w| ..)` method takes [`sw_shield_sel::W`](W) writer structure"]
impl crate::Writable for SwShieldSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SW_SHIELD_SEL to value 0"]
impl crate::Resettable for SwShieldSelSpec {
    const RESET_VALUE: u32 = 0;
}
