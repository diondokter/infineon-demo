#[doc = "Register `DEV_KEY_CTL1` reader"]
pub type R = crate::R<DevKeyCtl1Spec>;
#[doc = "Register `DEV_KEY_CTL1` writer"]
pub type W = crate::W<DevKeyCtl1Spec>;
#[doc = "Field `ALLOWED` reader - See DEV_KEY_CTL0."]
pub type AllowedR = crate::BitReader;
#[doc = "Field `ALLOWED` writer - See DEV_KEY_CTL0."]
pub type AllowedW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - See DEV_KEY_CTL0."]
    #[inline(always)]
    pub fn allowed(&self) -> AllowedR {
        AllowedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - See DEV_KEY_CTL0."]
    #[inline(always)]
    #[must_use]
    pub fn allowed(&mut self) -> AllowedW<DevKeyCtl1Spec> {
        AllowedW::new(self, 0)
    }
}
#[doc = "Device key control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_key_ctl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_key_ctl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevKeyCtl1Spec;
impl crate::RegisterSpec for DevKeyCtl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_key_ctl1::R`](R) reader structure"]
impl crate::Readable for DevKeyCtl1Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_key_ctl1::W`](W) writer structure"]
impl crate::Writable for DevKeyCtl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEV_KEY_CTL1 to value 0"]
impl crate::Resettable for DevKeyCtl1Spec {
    const RESET_VALUE: u32 = 0;
}
