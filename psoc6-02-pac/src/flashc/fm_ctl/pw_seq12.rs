#[doc = "Register `PW_SEQ12` reader"]
pub type R = crate::R<PwSeq12Spec>;
#[doc = "Register `PW_SEQ12` writer"]
pub type W = crate::W<PwSeq12Spec>;
#[doc = "Field `PW_SEQ1` reader - Seq1 delay"]
pub type PwSeq1R = crate::FieldReader<u16>;
#[doc = "Field `PW_SEQ1` writer - Seq1 delay"]
pub type PwSeq1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PW_SEQ2_PRE` reader - Seq2 pre delay"]
pub type PwSeq2PreR = crate::FieldReader<u16>;
#[doc = "Field `PW_SEQ2_PRE` writer - Seq2 pre delay"]
pub type PwSeq2PreW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Seq1 delay"]
    #[inline(always)]
    pub fn pw_seq1(&self) -> PwSeq1R {
        PwSeq1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Seq2 pre delay"]
    #[inline(always)]
    pub fn pw_seq2_pre(&self) -> PwSeq2PreR {
        PwSeq2PreR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Seq1 delay"]
    #[inline(always)]
    #[must_use]
    pub fn pw_seq1(&mut self) -> PwSeq1W<PwSeq12Spec> {
        PwSeq1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Seq2 pre delay"]
    #[inline(always)]
    #[must_use]
    pub fn pw_seq2_pre(&mut self) -> PwSeq2PreW<PwSeq12Spec> {
        PwSeq2PreW::new(self, 16)
    }
}
#[doc = "HV Pulse Delay for seq 1&amp;2 pre\n\nYou can [`read`](crate::Reg::read) this register and get [`pw_seq12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pw_seq12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwSeq12Spec;
impl crate::RegisterSpec for PwSeq12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pw_seq12::R`](R) reader structure"]
impl crate::Readable for PwSeq12Spec {}
#[doc = "`write(|w| ..)` method takes [`pw_seq12::W`](W) writer structure"]
impl crate::Writable for PwSeq12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PW_SEQ12 to value 0"]
impl crate::Resettable for PwSeq12Spec {
    const RESET_VALUE: u32 = 0;
}
