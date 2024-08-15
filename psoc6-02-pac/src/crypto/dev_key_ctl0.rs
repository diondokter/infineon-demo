#[doc = "Register `DEV_KEY_CTL0` reader"]
pub type R = crate::R<DevKeyCtl0Spec>;
#[doc = "Register `DEV_KEY_CTL0` writer"]
pub type W = crate::W<DevKeyCtl0Spec>;
#[doc = "Field `ALLOWED` reader - Specifies if a LOAD_DEV_KEY instruction is allowed to use the device key in memory: '0': Not allowed. '1': Allowed. Note: For successful completion of a LOAD_DEV_KEY instruction, both the associated DEV_KEY_ADDR_CTL.VALID and DEV_KEY_CTL.ALLOWED fields must be '1'. On successful instruction completion, DEV_KEY_STATUS.LOADED is set to '1'. On unsuccessful completion, the instruction FIFO is cleared and the IP is locked; an Active reset or an IP reset (CTL.ENABLED), which reinitializes the IP, is required. Note: A LOAD_DEV_KEY loads the device key from memory with protection context '0'."]
pub type AllowedR = crate::BitReader;
#[doc = "Field `ALLOWED` writer - Specifies if a LOAD_DEV_KEY instruction is allowed to use the device key in memory: '0': Not allowed. '1': Allowed. Note: For successful completion of a LOAD_DEV_KEY instruction, both the associated DEV_KEY_ADDR_CTL.VALID and DEV_KEY_CTL.ALLOWED fields must be '1'. On successful instruction completion, DEV_KEY_STATUS.LOADED is set to '1'. On unsuccessful completion, the instruction FIFO is cleared and the IP is locked; an Active reset or an IP reset (CTL.ENABLED), which reinitializes the IP, is required. Note: A LOAD_DEV_KEY loads the device key from memory with protection context '0'."]
pub type AllowedW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Specifies if a LOAD_DEV_KEY instruction is allowed to use the device key in memory: '0': Not allowed. '1': Allowed. Note: For successful completion of a LOAD_DEV_KEY instruction, both the associated DEV_KEY_ADDR_CTL.VALID and DEV_KEY_CTL.ALLOWED fields must be '1'. On successful instruction completion, DEV_KEY_STATUS.LOADED is set to '1'. On unsuccessful completion, the instruction FIFO is cleared and the IP is locked; an Active reset or an IP reset (CTL.ENABLED), which reinitializes the IP, is required. Note: A LOAD_DEV_KEY loads the device key from memory with protection context '0'."]
    #[inline(always)]
    pub fn allowed(&self) -> AllowedR {
        AllowedR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies if a LOAD_DEV_KEY instruction is allowed to use the device key in memory: '0': Not allowed. '1': Allowed. Note: For successful completion of a LOAD_DEV_KEY instruction, both the associated DEV_KEY_ADDR_CTL.VALID and DEV_KEY_CTL.ALLOWED fields must be '1'. On successful instruction completion, DEV_KEY_STATUS.LOADED is set to '1'. On unsuccessful completion, the instruction FIFO is cleared and the IP is locked; an Active reset or an IP reset (CTL.ENABLED), which reinitializes the IP, is required. Note: A LOAD_DEV_KEY loads the device key from memory with protection context '0'."]
    #[inline(always)]
    #[must_use]
    pub fn allowed(&mut self) -> AllowedW<DevKeyCtl0Spec> {
        AllowedW::new(self, 0)
    }
}
#[doc = "Device key control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_key_ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dev_key_ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevKeyCtl0Spec;
impl crate::RegisterSpec for DevKeyCtl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_key_ctl0::R`](R) reader structure"]
impl crate::Readable for DevKeyCtl0Spec {}
#[doc = "`write(|w| ..)` method takes [`dev_key_ctl0::W`](W) writer structure"]
impl crate::Writable for DevKeyCtl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEV_KEY_CTL0 to value 0"]
impl crate::Resettable for DevKeyCtl0Spec {
    const RESET_VALUE: u32 = 0;
}
