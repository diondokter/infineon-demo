#[doc = "Register `DIVIDER` reader"]
pub type R = crate::R<DividerSpec>;
#[doc = "Register `DIVIDER` writer"]
pub type W = crate::W<DividerSpec>;
#[doc = "Field `SUBFR_DIV` reader - Input clock frequency divide value, to generate the 1/4 sub-frame period. The sub-frame period is 4*(SUBFR_DIV+1) cycles long."]
pub type SubfrDivR = crate::FieldReader<u16>;
#[doc = "Field `SUBFR_DIV` writer - Input clock frequency divide value, to generate the 1/4 sub-frame period. The sub-frame period is 4*(SUBFR_DIV+1) cycles long."]
pub type SubfrDivW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DEAD_DIV` reader - Length of the dead time period in cycles. When set to zero, no dead time period exists."]
pub type DeadDivR = crate::FieldReader<u16>;
#[doc = "Field `DEAD_DIV` writer - Length of the dead time period in cycles. When set to zero, no dead time period exists."]
pub type DeadDivW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input clock frequency divide value, to generate the 1/4 sub-frame period. The sub-frame period is 4*(SUBFR_DIV+1) cycles long."]
    #[inline(always)]
    pub fn subfr_div(&self) -> SubfrDivR {
        SubfrDivR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Length of the dead time period in cycles. When set to zero, no dead time period exists."]
    #[inline(always)]
    pub fn dead_div(&self) -> DeadDivR {
        DeadDivR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Input clock frequency divide value, to generate the 1/4 sub-frame period. The sub-frame period is 4*(SUBFR_DIV+1) cycles long."]
    #[inline(always)]
    #[must_use]
    pub fn subfr_div(&mut self) -> SubfrDivW<DividerSpec> {
        SubfrDivW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Length of the dead time period in cycles. When set to zero, no dead time period exists."]
    #[inline(always)]
    #[must_use]
    pub fn dead_div(&mut self) -> DeadDivW<DividerSpec> {
        DeadDivW::new(self, 16)
    }
}
#[doc = "LCD Divider Register\n\nYou can [`read`](crate::Reg::read) this register and get [`divider::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divider::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DividerSpec;
impl crate::RegisterSpec for DividerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`divider::R`](R) reader structure"]
impl crate::Readable for DividerSpec {}
#[doc = "`write(|w| ..)` method takes [`divider::W`](W) writer structure"]
impl crate::Writable for DividerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIVIDER to value 0"]
impl crate::Resettable for DividerSpec {
    const RESET_VALUE: u32 = 0;
}
