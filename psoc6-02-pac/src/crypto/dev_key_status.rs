#[doc = "Register `DEV_KEY_STATUS` reader"]
pub type R = crate::R<DevKeyStatusSpec>;
#[doc = "Field `LOADED` reader - Specifies if a device key is present in the IP register buffer blocks 4 and 5. HW sets this field to '1' on successful completion of a LOAD_DEV_KEY instruction. HW clears this field to '0' when a CLEAR instruction is executed (the CLEAR instruction also sets the IP register buffer to '0')."]
pub type LoadedR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Specifies if a device key is present in the IP register buffer blocks 4 and 5. HW sets this field to '1' on successful completion of a LOAD_DEV_KEY instruction. HW clears this field to '0' when a CLEAR instruction is executed (the CLEAR instruction also sets the IP register buffer to '0')."]
    #[inline(always)]
    pub fn loaded(&self) -> LoadedR {
        LoadedR::new((self.bits & 1) != 0)
    }
}
#[doc = "Device key status\n\nYou can [`read`](crate::Reg::read) this register and get [`dev_key_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevKeyStatusSpec;
impl crate::RegisterSpec for DevKeyStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dev_key_status::R`](R) reader structure"]
impl crate::Readable for DevKeyStatusSpec {}
#[doc = "`reset()` method sets DEV_KEY_STATUS to value 0"]
impl crate::Resettable for DevKeyStatusSpec {
    const RESET_VALUE: u32 = 0;
}
