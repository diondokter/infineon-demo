#[doc = "Register `SW_BYP_SEL` reader"]
pub type R = crate::R<SwBypSelSpec>;
#[doc = "Register `SW_BYP_SEL` writer"]
pub type W = crate::W<SwBypSelSpec>;
#[doc = "Field `SW_BYA` reader - Set corresponding switch"]
pub type SwByaR = crate::BitReader;
#[doc = "Field `SW_BYA` writer - Set corresponding switch"]
pub type SwByaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_BYB` reader - Set corresponding switch"]
pub type SwBybR = crate::BitReader;
#[doc = "Field `SW_BYB` writer - Set corresponding switch"]
pub type SwBybW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_CBCC` reader - Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
pub type SwCbccR = crate::BitReader;
#[doc = "Field `SW_CBCC` writer - Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
pub type SwCbccW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_bya(&self) -> SwByaR {
        SwByaR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_byb(&self) -> SwBybR {
        SwBybR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
    #[inline(always)]
    pub fn sw_cbcc(&self) -> SwCbccR {
        SwCbccR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_bya(&mut self) -> SwByaW<SwBypSelSpec> {
        SwByaW::new(self, 12)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_byb(&mut self) -> SwBybW<SwBypSelSpec> {
        SwBybW::new(self, 16)
    }
    #[doc = "Bit 20 - Set corresponding switch If the ADC is enabled then this switch is directly controlled by the ADC sequencer."]
    #[inline(always)]
    #[must_use]
    pub fn sw_cbcc(&mut self) -> SwCbccW<SwBypSelSpec> {
        SwCbccW::new(self, 20)
    }
}
#[doc = "AMUXBUS bypass switches Waveform selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_byp_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_byp_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwBypSelSpec;
impl crate::RegisterSpec for SwBypSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_byp_sel::R`](R) reader structure"]
impl crate::Readable for SwBypSelSpec {}
#[doc = "`write(|w| ..)` method takes [`sw_byp_sel::W`](W) writer structure"]
impl crate::Writable for SwBypSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SW_BYP_SEL to value 0"]
impl crate::Resettable for SwBypSelSpec {
    const RESET_VALUE: u32 = 0;
}
