#[doc = "Register `DEV_KEY_ADDR1_CTL` reader"]
pub type R = crate::R<DevKeyAddr1CtlSpec>;
#[doc = "Register `DEV_KEY_ADDR1_CTL` writer"]
pub type W = crate::W<DevKeyAddr1CtlSpec>;
#[doc = "Field `VALID` reader - See DEV_KEY_ADDR0_CTL."]
pub type ValidR = crate::BitReader;
#[doc = "Field `VALID` writer - See DEV_KEY_ADDR0_CTL."]
pub type ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - See DEV_KEY_ADDR0_CTL."]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - See DEV_KEY_ADDR0_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> ValidW<DevKeyAddr1CtlSpec> {
        ValidW::new(self, 31)
    }
}
#[doc = "Device key address 1 control\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_key_addr1_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_key_addr1_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevKeyAddr1CtlSpec;
impl crate::RegisterSpec for DevKeyAddr1CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_key_addr1_ctl::R`](R) reader structure"]
impl crate::Readable for DevKeyAddr1CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`dev_key_addr1_ctl::W`](W) writer structure"]
impl crate::Writable for DevKeyAddr1CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEV_KEY_ADDR1_CTL to value 0"]
impl crate::Resettable for DevKeyAddr1CtlSpec {
    const RESET_VALUE: u32 = 0;
}
