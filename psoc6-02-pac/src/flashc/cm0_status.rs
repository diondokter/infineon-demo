#[doc = "Register `CM0_STATUS` reader"]
pub type R = crate::R<Cm0StatusSpec>;
#[doc = "Register `CM0_STATUS` writer"]
pub type W = crate::W<Cm0StatusSpec>;
#[doc = "Field `MAIN_INTERNAL_ERR` reader - Specifies/registers the occurrence of a FLASH macro main interface internal error (typically the result of a read access while a program erase operation is ongoing) as a result of a CM0+ access (or debug access via SYS_AP/CM0_AP). SW clears this field to '0'. HW sets this field to '1' on a FLASH macro main interface internal error. Typically, SW reads this field after a code section to detect the occurrence of an error. Note: this field is independent of FLASH_CTL.MAIN_ERR_SILENT."]
pub type MainInternalErrR = crate::BitReader;
#[doc = "Field `MAIN_INTERNAL_ERR` writer - Specifies/registers the occurrence of a FLASH macro main interface internal error (typically the result of a read access while a program erase operation is ongoing) as a result of a CM0+ access (or debug access via SYS_AP/CM0_AP). SW clears this field to '0'. HW sets this field to '1' on a FLASH macro main interface internal error. Typically, SW reads this field after a code section to detect the occurrence of an error. Note: this field is independent of FLASH_CTL.MAIN_ERR_SILENT."]
pub type MainInternalErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WORK_INTERNAL_ERR` reader - See CM0_STATUS.MAIN_INTERNAL_ERROR."]
pub type WorkInternalErrR = crate::BitReader;
#[doc = "Field `WORK_INTERNAL_ERR` writer - See CM0_STATUS.MAIN_INTERNAL_ERROR."]
pub type WorkInternalErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Specifies/registers the occurrence of a FLASH macro main interface internal error (typically the result of a read access while a program erase operation is ongoing) as a result of a CM0+ access (or debug access via SYS_AP/CM0_AP). SW clears this field to '0'. HW sets this field to '1' on a FLASH macro main interface internal error. Typically, SW reads this field after a code section to detect the occurrence of an error. Note: this field is independent of FLASH_CTL.MAIN_ERR_SILENT."]
    #[inline(always)]
    pub fn main_internal_err(&self) -> MainInternalErrR {
        MainInternalErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - See CM0_STATUS.MAIN_INTERNAL_ERROR."]
    #[inline(always)]
    pub fn work_internal_err(&self) -> WorkInternalErrR {
        WorkInternalErrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies/registers the occurrence of a FLASH macro main interface internal error (typically the result of a read access while a program erase operation is ongoing) as a result of a CM0+ access (or debug access via SYS_AP/CM0_AP). SW clears this field to '0'. HW sets this field to '1' on a FLASH macro main interface internal error. Typically, SW reads this field after a code section to detect the occurrence of an error. Note: this field is independent of FLASH_CTL.MAIN_ERR_SILENT."]
    #[inline(always)]
    #[must_use]
    pub fn main_internal_err(&mut self) -> MainInternalErrW<Cm0StatusSpec> {
        MainInternalErrW::new(self, 0)
    }
    #[doc = "Bit 1 - See CM0_STATUS.MAIN_INTERNAL_ERROR."]
    #[inline(always)]
    #[must_use]
    pub fn work_internal_err(&mut self) -> WorkInternalErrW<Cm0StatusSpec> {
        WorkInternalErrW::new(self, 1)
    }
}
#[doc = "CM0+ interface status\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm0StatusSpec;
impl crate::RegisterSpec for Cm0StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm0_status::R`](R) reader structure"]
impl crate::Readable for Cm0StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`cm0_status::W`](W) writer structure"]
impl crate::Writable for Cm0StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CM0_STATUS to value 0"]
impl crate::Resettable for Cm0StatusSpec {
    const RESET_VALUE: u32 = 0;
}
