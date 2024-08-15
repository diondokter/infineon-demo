#[doc = "Register `SEQ_TIME` reader"]
pub type R = crate::R<SeqTimeSpec>;
#[doc = "Register `SEQ_TIME` writer"]
pub type W = crate::W<SeqTimeSpec>;
#[doc = "Field `AZ_TIME` reader - Define Auto-Zero time in csd_sense cycles -1."]
pub type AzTimeR = crate::FieldReader;
#[doc = "Field `AZ_TIME` writer - Define Auto-Zero time in csd_sense cycles -1."]
pub type AzTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Define Auto-Zero time in csd_sense cycles -1."]
    #[inline(always)]
    pub fn az_time(&self) -> AzTimeR {
        AzTimeR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Define Auto-Zero time in csd_sense cycles -1."]
    #[inline(always)]
    #[must_use]
    pub fn az_time(&mut self) -> AzTimeW<SeqTimeSpec> {
        AzTimeW::new(self, 0)
    }
}
#[doc = "Sequencer Timing\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeqTimeSpec;
impl crate::RegisterSpec for SeqTimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq_time::R`](R) reader structure"]
impl crate::Readable for SeqTimeSpec {}
#[doc = "`write(|w| ..)` method takes [`seq_time::W`](W) writer structure"]
impl crate::Writable for SeqTimeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEQ_TIME to value 0"]
impl crate::Resettable for SeqTimeSpec {
    const RESET_VALUE: u32 = 0;
}
