#[doc = "Register `CQSSC2` reader"]
pub type R = crate::R<Cqssc2Spec>;
#[doc = "Register `CQSSC2` writer"]
pub type W = crate::W<Cqssc2Spec>;
#[doc = "Field `SQSCMD_RCA` reader - This field provides CQE with the contents of the 16-bit RCA field in SEND_QUEUE_STATUS (CMD13) command argument. CQE copies this field to bits 31:16 of the argument when transmitting SEND_ QUEUE_STATUS (CMD13) command."]
pub type SqscmdRcaR = crate::FieldReader<u16>;
#[doc = "Field `SQSCMD_RCA` writer - This field provides CQE with the contents of the 16-bit RCA field in SEND_QUEUE_STATUS (CMD13) command argument. CQE copies this field to bits 31:16 of the argument when transmitting SEND_ QUEUE_STATUS (CMD13) command."]
pub type SqscmdRcaW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This field provides CQE with the contents of the 16-bit RCA field in SEND_QUEUE_STATUS (CMD13) command argument. CQE copies this field to bits 31:16 of the argument when transmitting SEND_ QUEUE_STATUS (CMD13) command."]
    #[inline(always)]
    pub fn sqscmd_rca(&self) -> SqscmdRcaR {
        SqscmdRcaR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field provides CQE with the contents of the 16-bit RCA field in SEND_QUEUE_STATUS (CMD13) command argument. CQE copies this field to bits 31:16 of the argument when transmitting SEND_ QUEUE_STATUS (CMD13) command."]
    #[inline(always)]
    #[must_use]
    pub fn sqscmd_rca(&mut self) -> SqscmdRcaW<Cqssc2Spec> {
        SqscmdRcaW::new(self, 0)
    }
}
#[doc = "CQ Send Status Configuration 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqssc2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqssc2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cqssc2Spec;
impl crate::RegisterSpec for Cqssc2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqssc2::R`](R) reader structure"]
impl crate::Readable for Cqssc2Spec {}
#[doc = "`write(|w| ..)` method takes [`cqssc2::W`](W) writer structure"]
impl crate::Writable for Cqssc2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CQSSC2 to value 0"]
impl crate::Resettable for Cqssc2Spec {
    const RESET_VALUE: u32 = 0;
}
