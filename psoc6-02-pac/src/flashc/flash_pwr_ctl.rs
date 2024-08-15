#[doc = "Register `FLASH_PWR_CTL` reader"]
pub type R = crate::R<FlashPwrCtlSpec>;
#[doc = "Register `FLASH_PWR_CTL` writer"]
pub type W = crate::W<FlashPwrCtlSpec>;
#[doc = "Field `ENABLE` reader - Controls 'enable' pin of the Flash memory."]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Controls 'enable' pin of the Flash memory."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_HV` reader - Controls 'enable_hv' pin of the Flash memory."]
pub type EnableHvR = crate::BitReader;
#[doc = "Field `ENABLE_HV` writer - Controls 'enable_hv' pin of the Flash memory."]
pub type EnableHvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Controls 'enable' pin of the Flash memory."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Controls 'enable_hv' pin of the Flash memory."]
    #[inline(always)]
    pub fn enable_hv(&self) -> EnableHvR {
        EnableHvR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls 'enable' pin of the Flash memory."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<FlashPwrCtlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Controls 'enable_hv' pin of the Flash memory."]
    #[inline(always)]
    #[must_use]
    pub fn enable_hv(&mut self) -> EnableHvW<FlashPwrCtlSpec> {
        EnableHvW::new(self, 1)
    }
}
#[doc = "Flash power control\n\nYou can [`read`](crate::Reg::read) this register and get [`flash_pwr_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_pwr_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashPwrCtlSpec;
impl crate::RegisterSpec for FlashPwrCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flash_pwr_ctl::R`](R) reader structure"]
impl crate::Readable for FlashPwrCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`flash_pwr_ctl::W`](W) writer structure"]
impl crate::Writable for FlashPwrCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FLASH_PWR_CTL to value 0x03"]
impl crate::Resettable for FlashPwrCtlSpec {
    const RESET_VALUE: u32 = 0x03;
}
