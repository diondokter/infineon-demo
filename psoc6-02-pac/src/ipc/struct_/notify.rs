#[doc = "Register `NOTIFY` writer"]
pub type W = crate::W<NotifySpec>;
#[doc = "Field `INTR_NOTIFY` writer - This field allows for the generation of notification events to the IPC interrupt structures. The IPC notification cause fields associated with this IPC structure are set to '1', but only for those IPC interrupt structures for which the corresponding bit field in INTR_NOTIFY\\[\\]
is set to '1'. SW writes a '1' to the bit fields to generate a notify event. Due to the transient nature of this event, SW always reads a '0' from this field."]
pub type IntrNotifyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - This field allows for the generation of notification events to the IPC interrupt structures. The IPC notification cause fields associated with this IPC structure are set to '1', but only for those IPC interrupt structures for which the corresponding bit field in INTR_NOTIFY\\[\\]
is set to '1'. SW writes a '1' to the bit fields to generate a notify event. Due to the transient nature of this event, SW always reads a '0' from this field."]
    #[inline(always)]
    #[must_use]
    pub fn intr_notify(&mut self) -> IntrNotifyW<NotifySpec> {
        IntrNotifyW::new(self, 0)
    }
}
#[doc = "IPC notification\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`notify::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NotifySpec;
impl crate::RegisterSpec for NotifySpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`notify::W`](W) writer structure"]
impl crate::Writable for NotifySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NOTIFY to value 0"]
impl crate::Resettable for NotifySpec {
    const RESET_VALUE: u32 = 0;
}
