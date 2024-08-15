#[doc = "Register `DEV_KEY_ADDR0` reader"]
pub type R = crate::R<DevKeyAddr0Spec>;
#[doc = "Register `DEV_KEY_ADDR0` writer"]
pub type W = crate::W<DevKeyAddr0Spec>;
#[doc = "Field `ADDR32` reader - Specifies the memory address of the device key in memory. A LOAD_DEV_KEY instruction uses this address to load a device key from memory into the IP register buffer blocks 4 and 5."]
pub type Addr32R = crate::FieldReader<u32>;
#[doc = "Field `ADDR32` writer - Specifies the memory address of the device key in memory. A LOAD_DEV_KEY instruction uses this address to load a device key from memory into the IP register buffer blocks 4 and 5."]
pub type Addr32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Specifies the memory address of the device key in memory. A LOAD_DEV_KEY instruction uses this address to load a device key from memory into the IP register buffer blocks 4 and 5."]
    #[inline(always)]
    pub fn addr32(&self) -> Addr32R {
        Addr32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Specifies the memory address of the device key in memory. A LOAD_DEV_KEY instruction uses this address to load a device key from memory into the IP register buffer blocks 4 and 5."]
    #[inline(always)]
    #[must_use]
    pub fn addr32(&mut self) -> Addr32W<DevKeyAddr0Spec> {
        Addr32W::new(self, 0)
    }
}
#[doc = "Device key address 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_key_addr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_key_addr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevKeyAddr0Spec;
impl crate::RegisterSpec for DevKeyAddr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_key_addr0::R`](R) reader structure"]
impl crate::Readable for DevKeyAddr0Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_key_addr0::W`](W) writer structure"]
impl crate::Writable for DevKeyAddr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEV_KEY_ADDR0 to value 0"]
impl crate::Resettable for DevKeyAddr0Spec {
    const RESET_VALUE: u32 = 0;
}
