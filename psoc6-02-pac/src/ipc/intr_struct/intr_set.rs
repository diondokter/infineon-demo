#[doc = "Register `INTR_SET` reader"]
pub type R = crate::R<IntrSetSpec>;
#[doc = "Register `INTR_SET` writer"]
pub type W = crate::W<IntrSetSpec>;
#[doc = "Field `RELEASE` reader - SW writes a '1' to this field to set the corresponding field in the INTR register."]
pub type ReleaseR = crate::FieldReader<u16>;
#[doc = "Field `RELEASE` writer - SW writes a '1' to this field to set the corresponding field in the INTR register."]
pub type ReleaseW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NOTIFY` reader - SW writes a '1' to this field to set the corresponding field in the INTR register."]
pub type NotifyR = crate::FieldReader<u16>;
#[doc = "Field `NOTIFY` writer - SW writes a '1' to this field to set the corresponding field in the INTR register."]
pub type NotifyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - SW writes a '1' to this field to set the corresponding field in the INTR register."]
    #[inline(always)]
    pub fn release(&self) -> ReleaseR {
        ReleaseR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SW writes a '1' to this field to set the corresponding field in the INTR register."]
    #[inline(always)]
    pub fn notify(&self) -> NotifyR {
        NotifyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SW writes a '1' to this field to set the corresponding field in the INTR register."]
    #[inline(always)]
    #[must_use]
    pub fn release(&mut self) -> ReleaseW<IntrSetSpec> {
        ReleaseW::new(self, 0)
    }
    #[doc = "Bits 16:31 - SW writes a '1' to this field to set the corresponding field in the INTR register."]
    #[inline(always)]
    #[must_use]
    pub fn notify(&mut self) -> NotifyW<IntrSetSpec> {
        NotifyW::new(self, 16)
    }
}
#[doc = "Interrupt set\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrSetSpec;
impl crate::RegisterSpec for IntrSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_set::R`](R) reader structure"]
impl crate::Readable for IntrSetSpec {}
#[doc = "`write(|w| ..)` method takes [`intr_set::W`](W) writer structure"]
impl crate::Writable for IntrSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTR_SET to value 0"]
impl crate::Resettable for IntrSetSpec {
    const RESET_VALUE: u32 = 0;
}
