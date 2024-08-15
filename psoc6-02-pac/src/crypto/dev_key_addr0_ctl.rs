#[doc = "Register `DEV_KEY_ADDR0_CTL` reader"]
pub type R = crate::R<DevKeyAddr0CtlSpec>;
#[doc = "Register `DEV_KEY_ADDR0_CTL` writer"]
pub type W = crate::W<DevKeyAddr0CtlSpec>;
#[doc = "Field `VALID` reader - Specifies if the address in the associated DEV_KEY_ADDR0 is valid: '0': Address not valid; i.e. no device key specified. '1': Address valid; i.e. device key specified. Note: A LOAD_DEV_KEY instruction requires that the device key's valid field is '1'."]
pub type ValidR = crate::BitReader;
#[doc = "Field `VALID` writer - Specifies if the address in the associated DEV_KEY_ADDR0 is valid: '0': Address not valid; i.e. no device key specified. '1': Address valid; i.e. device key specified. Note: A LOAD_DEV_KEY instruction requires that the device key's valid field is '1'."]
pub type ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - Specifies if the address in the associated DEV_KEY_ADDR0 is valid: '0': Address not valid; i.e. no device key specified. '1': Address valid; i.e. device key specified. Note: A LOAD_DEV_KEY instruction requires that the device key's valid field is '1'."]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Specifies if the address in the associated DEV_KEY_ADDR0 is valid: '0': Address not valid; i.e. no device key specified. '1': Address valid; i.e. device key specified. Note: A LOAD_DEV_KEY instruction requires that the device key's valid field is '1'."]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> ValidW<DevKeyAddr0CtlSpec> {
        ValidW::new(self, 31)
    }
}
#[doc = "Device key address 0 control\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_key_addr0_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_key_addr0_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevKeyAddr0CtlSpec;
impl crate::RegisterSpec for DevKeyAddr0CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_key_addr0_ctl::R`](R) reader structure"]
impl crate::Readable for DevKeyAddr0CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`dev_key_addr0_ctl::W`](W) writer structure"]
impl crate::Writable for DevKeyAddr0CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEV_KEY_ADDR0_CTL to value 0"]
impl crate::Resettable for DevKeyAddr0CtlSpec {
    const RESET_VALUE: u32 = 0;
}
