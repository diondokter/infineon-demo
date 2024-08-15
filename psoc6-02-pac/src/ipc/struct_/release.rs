#[doc = "Register `RELEASE` writer"]
pub type W = crate::W<ReleaseSpec>;
#[doc = "Field `INTR_RELEASE` writer - Writing this field releases a lock and allows for the generation of release events to the IPC interrupt structures, but only when the lock is acquired (LOCK_STATUS.ACQUIRED is '1'). The IPC release cause fields associated with this IPC structure are set to '1', but only for those IPC interrupt structures for which the corresponding bit field in INTR_RELEASE\\[\\]
is set to '1'. SW writes a '1' to the bit fields to generate a release event. Due to the transient nature of this event, SW always reads a '0' from this field."]
pub type IntrReleaseW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Writing this field releases a lock and allows for the generation of release events to the IPC interrupt structures, but only when the lock is acquired (LOCK_STATUS.ACQUIRED is '1'). The IPC release cause fields associated with this IPC structure are set to '1', but only for those IPC interrupt structures for which the corresponding bit field in INTR_RELEASE\\[\\]
is set to '1'. SW writes a '1' to the bit fields to generate a release event. Due to the transient nature of this event, SW always reads a '0' from this field."]
    #[inline(always)]
    #[must_use]
    pub fn intr_release(&mut self) -> IntrReleaseW<ReleaseSpec> {
        IntrReleaseW::new(self, 0)
    }
}
#[doc = "IPC release\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`release::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReleaseSpec;
impl crate::RegisterSpec for ReleaseSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`release::W`](W) writer structure"]
impl crate::Writable for ReleaseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RELEASE to value 0"]
impl crate::Resettable for ReleaseSpec {
    const RESET_VALUE: u32 = 0;
}
