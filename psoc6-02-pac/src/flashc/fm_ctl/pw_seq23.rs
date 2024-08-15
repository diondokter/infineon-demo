#[doc = "Register `PW_SEQ23` reader"]
pub type R = crate::R<PwSeq23Spec>;
#[doc = "Register `PW_SEQ23` writer"]
pub type W = crate::W<PwSeq23Spec>;
#[doc = "Field `PW_SEQ2_POST` reader - Seq2 post delay"]
pub type PwSeq2PostR = crate::FieldReader<u16>;
#[doc = "Field `PW_SEQ2_POST` writer - Seq2 post delay"]
pub type PwSeq2PostW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PW_SEQ3` reader - Seq3 delay"]
pub type PwSeq3R = crate::FieldReader<u16>;
#[doc = "Field `PW_SEQ3` writer - Seq3 delay"]
pub type PwSeq3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Seq2 post delay"]
    #[inline(always)]
    pub fn pw_seq2_post(&self) -> PwSeq2PostR {
        PwSeq2PostR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Seq3 delay"]
    #[inline(always)]
    pub fn pw_seq3(&self) -> PwSeq3R {
        PwSeq3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Seq2 post delay"]
    #[inline(always)]
    #[must_use]
    pub fn pw_seq2_post(&mut self) -> PwSeq2PostW<PwSeq23Spec> {
        PwSeq2PostW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Seq3 delay"]
    #[inline(always)]
    #[must_use]
    pub fn pw_seq3(&mut self) -> PwSeq3W<PwSeq23Spec> {
        PwSeq3W::new(self, 16)
    }
}
#[doc = "HV Pulse Delay for seq2 post &amp; seq3\n\nYou can [`read`](crate::Reg::read) this register and get [`pw_seq23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pw_seq23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwSeq23Spec;
impl crate::RegisterSpec for PwSeq23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pw_seq23::R`](R) reader structure"]
impl crate::Readable for PwSeq23Spec {}
#[doc = "`write(|w| ..)` method takes [`pw_seq23::W`](W) writer structure"]
impl crate::Writable for PwSeq23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PW_SEQ23 to value 0"]
impl crate::Resettable for PwSeq23Spec {
    const RESET_VALUE: u32 = 0;
}
