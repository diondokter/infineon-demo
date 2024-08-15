#[doc = "Register `RGRANT_DELAY_ERS` reader"]
pub type R = crate::R<RgrantDelayErsSpec>;
#[doc = "Register `RGRANT_DELAY_ERS` writer"]
pub type W = crate::W<RgrantDelayErsSpec>;
#[doc = "Field `RGRANT_DELAY_ERS_SEQ01` reader - ERASE: R-grant blocking delay on seq0-seq1 transition. Scale = ANA_CTL0.SCALE_SEQ01 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RgrantDelayErsSeq01R = crate::FieldReader;
#[doc = "Field `RGRANT_DELAY_ERS_SEQ01` writer - ERASE: R-grant blocking delay on seq0-seq1 transition. Scale = ANA_CTL0.SCALE_SEQ01 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RgrantDelayErsSeq01W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RGRANT_DELAY_ERS_SEQ12` reader - ERASE: R-grant blocking delay on seq1-seq2 transition. Scale = ANA_CTL0.SCALE_SEQ12 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RgrantDelayErsSeq12R = crate::FieldReader;
#[doc = "Field `RGRANT_DELAY_ERS_SEQ12` writer - ERASE: R-grant blocking delay on seq1-seq2 transition. Scale = ANA_CTL0.SCALE_SEQ12 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RgrantDelayErsSeq12W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RGRANT_DELAY_ERS_SEQ23` reader - ERASE: R-grant blocking delay on seq2-seq3 transition. Scale = ANA_CTL0.SCALE_SEQ23 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RgrantDelayErsSeq23R = crate::FieldReader;
#[doc = "Field `RGRANT_DELAY_ERS_SEQ23` writer - ERASE: R-grant blocking delay on seq2-seq3 transition. Scale = ANA_CTL0.SCALE_SEQ23 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RgrantDelayErsSeq23W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - ERASE: R-grant blocking delay on seq0-seq1 transition. Scale = ANA_CTL0.SCALE_SEQ01 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_ers_seq01(&self) -> RgrantDelayErsSeq01R {
        RgrantDelayErsSeq01R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - ERASE: R-grant blocking delay on seq1-seq2 transition. Scale = ANA_CTL0.SCALE_SEQ12 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_ers_seq12(&self) -> RgrantDelayErsSeq12R {
        RgrantDelayErsSeq12R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ERASE: R-grant blocking delay on seq2-seq3 transition. Scale = ANA_CTL0.SCALE_SEQ23 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_ers_seq23(&self) -> RgrantDelayErsSeq23R {
        RgrantDelayErsSeq23R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ERASE: R-grant blocking delay on seq0-seq1 transition. Scale = ANA_CTL0.SCALE_SEQ01 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn rgrant_delay_ers_seq01(&mut self) -> RgrantDelayErsSeq01W<RgrantDelayErsSpec> {
        RgrantDelayErsSeq01W::new(self, 0)
    }
    #[doc = "Bits 8:15 - ERASE: R-grant blocking delay on seq1-seq2 transition. Scale = ANA_CTL0.SCALE_SEQ12 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn rgrant_delay_ers_seq12(&mut self) -> RgrantDelayErsSeq12W<RgrantDelayErsSpec> {
        RgrantDelayErsSeq12W::new(self, 8)
    }
    #[doc = "Bits 16:23 - ERASE: R-grant blocking delay on seq2-seq3 transition. Scale = ANA_CTL0.SCALE_SEQ23 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn rgrant_delay_ers_seq23(&mut self) -> RgrantDelayErsSeq23W<RgrantDelayErsSpec> {
        RgrantDelayErsSeq23W::new(self, 16)
    }
}
#[doc = "R-grant delay for erase\n\nYou can [`read`](crate::Reg::read) this register and get [`rgrant_delay_ers::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgrant_delay_ers::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RgrantDelayErsSpec;
impl crate::RegisterSpec for RgrantDelayErsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rgrant_delay_ers::R`](R) reader structure"]
impl crate::Readable for RgrantDelayErsSpec {}
#[doc = "`write(|w| ..)` method takes [`rgrant_delay_ers::W`](W) writer structure"]
impl crate::Writable for RgrantDelayErsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RGRANT_DELAY_ERS to value 0"]
impl crate::Resettable for RgrantDelayErsSpec {
    const RESET_VALUE: u32 = 0;
}
