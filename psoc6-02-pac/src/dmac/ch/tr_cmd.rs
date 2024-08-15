#[doc = "Register `TR_CMD` reader"]
pub type R = crate::R<TrCmdSpec>;
#[doc = "Register `TR_CMD` writer"]
pub type W = crate::W<TrCmdSpec>;
#[doc = "Field `ACTIVATE` reader - Software trigger. When written with '1', a trigger is generated which sets 'trigger pending' (only if the channel is enabled). A read always returns a 0."]
pub type ActivateR = crate::BitReader;
#[doc = "Field `ACTIVATE` writer - Software trigger. When written with '1', a trigger is generated which sets 'trigger pending' (only if the channel is enabled). A read always returns a 0."]
pub type ActivateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software trigger. When written with '1', a trigger is generated which sets 'trigger pending' (only if the channel is enabled). A read always returns a 0."]
    #[inline(always)]
    pub fn activate(&self) -> ActivateR {
        ActivateR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software trigger. When written with '1', a trigger is generated which sets 'trigger pending' (only if the channel is enabled). A read always returns a 0."]
    #[inline(always)]
    #[must_use]
    pub fn activate(&mut self) -> ActivateW<TrCmdSpec> {
        ActivateW::new(self, 0)
    }
}
#[doc = "Channle software trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrCmdSpec;
impl crate::RegisterSpec for TrCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr_cmd::R`](R) reader structure"]
impl crate::Readable for TrCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`tr_cmd::W`](W) writer structure"]
impl crate::Writable for TrCmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TR_CMD to value 0"]
impl crate::Resettable for TrCmdSpec {
    const RESET_VALUE: u32 = 0;
}
