#[doc = "Register `RESET` reader"]
pub type R = crate::R<ResetSpec>;
#[doc = "Register `RESET` writer"]
pub type W = crate::W<ResetSpec>;
#[doc = "Field `RESET` reader - Writing 1 to this register resets the backup logic. Hardware clears it when the reset is complete. After setting this register, firmware should confirm it reads as 0 before attempting to write other backup registers."]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - Writing 1 to this register resets the backup logic. Hardware clears it when the reset is complete. After setting this register, firmware should confirm it reads as 0 before attempting to write other backup registers."]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - Writing 1 to this register resets the backup logic. Hardware clears it when the reset is complete. After setting this register, firmware should confirm it reads as 0 before attempting to write other backup registers."]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Writing 1 to this register resets the backup logic. Hardware clears it when the reset is complete. After setting this register, firmware should confirm it reads as 0 before attempting to write other backup registers."]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> ResetW<ResetSpec> {
        ResetW::new(self, 31)
    }
}
#[doc = "Backup reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`reset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetSpec;
impl crate::RegisterSpec for ResetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset::R`](R) reader structure"]
impl crate::Readable for ResetSpec {}
#[doc = "`write(|w| ..)` method takes [`reset::W`](W) writer structure"]
impl crate::Writable for ResetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESET to value 0"]
impl crate::Resettable for ResetSpec {
    const RESET_VALUE: u32 = 0;
}
