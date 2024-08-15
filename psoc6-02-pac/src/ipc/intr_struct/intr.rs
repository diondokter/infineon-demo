#[doc = "Register `INTR` reader"]
pub type R = crate::R<IntrSpec>;
#[doc = "Register `INTR` writer"]
pub type W = crate::W<IntrSpec>;
#[doc = "Field `RELEASE` reader - These interrupt cause fields are activated (HW sets the field to '1') when a IPC release event is detected. One bit field for each master. SW writes a '1' to these field to clear the interrupt cause."]
pub type ReleaseR = crate::FieldReader<u16>;
#[doc = "Field `RELEASE` writer - These interrupt cause fields are activated (HW sets the field to '1') when a IPC release event is detected. One bit field for each master. SW writes a '1' to these field to clear the interrupt cause."]
pub type ReleaseW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NOTIFY` reader - These interrupt cause fields are activated (HW sets the field to '1') when a IPC notification event is detected. One bit field for each master. SW writes a '1' to these field to clear the interrupt cause."]
pub type NotifyR = crate::FieldReader<u16>;
#[doc = "Field `NOTIFY` writer - These interrupt cause fields are activated (HW sets the field to '1') when a IPC notification event is detected. One bit field for each master. SW writes a '1' to these field to clear the interrupt cause."]
pub type NotifyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - These interrupt cause fields are activated (HW sets the field to '1') when a IPC release event is detected. One bit field for each master. SW writes a '1' to these field to clear the interrupt cause."]
    #[inline(always)]
    pub fn release(&self) -> ReleaseR {
        ReleaseR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - These interrupt cause fields are activated (HW sets the field to '1') when a IPC notification event is detected. One bit field for each master. SW writes a '1' to these field to clear the interrupt cause."]
    #[inline(always)]
    pub fn notify(&self) -> NotifyR {
        NotifyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - These interrupt cause fields are activated (HW sets the field to '1') when a IPC release event is detected. One bit field for each master. SW writes a '1' to these field to clear the interrupt cause."]
    #[inline(always)]
    #[must_use]
    pub fn release(&mut self) -> ReleaseW<IntrSpec> {
        ReleaseW::new(self, 0)
    }
    #[doc = "Bits 16:31 - These interrupt cause fields are activated (HW sets the field to '1') when a IPC notification event is detected. One bit field for each master. SW writes a '1' to these field to clear the interrupt cause."]
    #[inline(always)]
    #[must_use]
    pub fn notify(&mut self) -> NotifyW<IntrSpec> {
        NotifyW::new(self, 16)
    }
}
#[doc = "Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSpec;
impl crate::RegisterSpec for IntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr::R`](R) reader structure"]
impl crate::Readable for IntrSpec {}
#[doc = "`write(|w| ..)` method takes [`intr::W`](W) writer structure"]
impl crate::Writable for IntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for IntrSpec {
    const RESET_VALUE: u32 = 0;
}
