#[doc = "Register `RGRANT_SCALE_ERS` reader"]
pub type R = crate::R<RgrantScaleErsSpec>;
#[doc = "Register `RGRANT_SCALE_ERS` writer"]
pub type W = crate::W<RgrantScaleErsSpec>;
#[doc = "Field `SCALE_ERS_SEQ01` reader - ERASE: Scale for R_GRANT_DELAY on seq0-seq1 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type ScaleErsSeq01R = crate::FieldReader;
#[doc = "Field `SCALE_ERS_SEQ01` writer - ERASE: Scale for R_GRANT_DELAY on seq0-seq1 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type ScaleErsSeq01W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCALE_ERS_SEQ12` reader - ERASE: Scale for R_GRANT_DELAY on seq1-seq2 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type ScaleErsSeq12R = crate::FieldReader;
#[doc = "Field `SCALE_ERS_SEQ12` writer - ERASE: Scale for R_GRANT_DELAY on seq1-seq2 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type ScaleErsSeq12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCALE_ERS_SEQ23` reader - ERASE: Scale for R_GRANT_DELAY on seq2-seq3 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type ScaleErsSeq23R = crate::FieldReader;
#[doc = "Field `SCALE_ERS_SEQ23` writer - ERASE: Scale for R_GRANT_DELAY on seq2-seq3 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type ScaleErsSeq23W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCALE_ERS_PEON` reader - ERASE: Scale for R_GRANT_DELAY on PE On transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type ScaleErsPeonR = crate::FieldReader;
#[doc = "Field `SCALE_ERS_PEON` writer - ERASE: Scale for R_GRANT_DELAY on PE On transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type ScaleErsPeonW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCALE_ERS_PEOFF` reader - ERASE: Scale for R_GRANT_DELAY on PE OFF transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type ScaleErsPeoffR = crate::FieldReader;
#[doc = "Field `SCALE_ERS_PEOFF` writer - ERASE: Scale for R_GRANT_DELAY on PE OFF transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type ScaleErsPeoffW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RGRANT_DELAY_ERS_PEON` reader - ERASE: R-grant blocking delay on PE ON. Scale = ANA_CTL0.SCALE_PEON When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RgrantDelayErsPeonR = crate::FieldReader;
#[doc = "Field `RGRANT_DELAY_ERS_PEON` writer - ERASE: R-grant blocking delay on PE ON. Scale = ANA_CTL0.SCALE_PEON When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RgrantDelayErsPeonW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RGRANT_DELAY_ERS_PEOFF` reader - ERASE: R-grant blocking delay on PE OFF. Scale = ANA_CTL0.SCALE_PEOFF When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RgrantDelayErsPeoffR = crate::FieldReader;
#[doc = "Field `RGRANT_DELAY_ERS_PEOFF` writer - ERASE: R-grant blocking delay on PE OFF. Scale = ANA_CTL0.SCALE_PEOFF When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RgrantDelayErsPeoffW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - ERASE: Scale for R_GRANT_DELAY on seq0-seq1 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_ers_seq01(&self) -> ScaleErsSeq01R {
        ScaleErsSeq01R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - ERASE: Scale for R_GRANT_DELAY on seq1-seq2 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_ers_seq12(&self) -> ScaleErsSeq12R {
        ScaleErsSeq12R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - ERASE: Scale for R_GRANT_DELAY on seq2-seq3 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_ers_seq23(&self) -> ScaleErsSeq23R {
        ScaleErsSeq23R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - ERASE: Scale for R_GRANT_DELAY on PE On transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_ers_peon(&self) -> ScaleErsPeonR {
        ScaleErsPeonR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - ERASE: Scale for R_GRANT_DELAY on PE OFF transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_ers_peoff(&self) -> ScaleErsPeoffR {
        ScaleErsPeoffR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:23 - ERASE: R-grant blocking delay on PE ON. Scale = ANA_CTL0.SCALE_PEON When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_ers_peon(&self) -> RgrantDelayErsPeonR {
        RgrantDelayErsPeonR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - ERASE: R-grant blocking delay on PE OFF. Scale = ANA_CTL0.SCALE_PEOFF When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_ers_peoff(&self) -> RgrantDelayErsPeoffR {
        RgrantDelayErsPeoffR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ERASE: Scale for R_GRANT_DELAY on seq0-seq1 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    #[must_use]
    pub fn scale_ers_seq01(&mut self) -> ScaleErsSeq01W<RgrantScaleErsSpec> {
        ScaleErsSeq01W::new(self, 0)
    }
    #[doc = "Bits 2:3 - ERASE: Scale for R_GRANT_DELAY on seq1-seq2 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    #[must_use]
    pub fn scale_ers_seq12(&mut self) -> ScaleErsSeq12W<RgrantScaleErsSpec> {
        ScaleErsSeq12W::new(self, 2)
    }
    #[doc = "Bits 4:5 - ERASE: Scale for R_GRANT_DELAY on seq2-seq3 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    #[must_use]
    pub fn scale_ers_seq23(&mut self) -> ScaleErsSeq23W<RgrantScaleErsSpec> {
        ScaleErsSeq23W::new(self, 4)
    }
    #[doc = "Bits 6:7 - ERASE: Scale for R_GRANT_DELAY on PE On transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    #[must_use]
    pub fn scale_ers_peon(&mut self) -> ScaleErsPeonW<RgrantScaleErsSpec> {
        ScaleErsPeonW::new(self, 6)
    }
    #[doc = "Bits 8:9 - ERASE: Scale for R_GRANT_DELAY on PE OFF transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    #[must_use]
    pub fn scale_ers_peoff(&mut self) -> ScaleErsPeoffW<RgrantScaleErsSpec> {
        ScaleErsPeoffW::new(self, 8)
    }
    #[doc = "Bits 16:23 - ERASE: R-grant blocking delay on PE ON. Scale = ANA_CTL0.SCALE_PEON When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn rgrant_delay_ers_peon(&mut self) -> RgrantDelayErsPeonW<RgrantScaleErsSpec> {
        RgrantDelayErsPeonW::new(self, 16)
    }
    #[doc = "Bits 24:31 - ERASE: R-grant blocking delay on PE OFF. Scale = ANA_CTL0.SCALE_PEOFF When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn rgrant_delay_ers_peoff(&mut self) -> RgrantDelayErsPeoffW<RgrantScaleErsSpec> {
        RgrantDelayErsPeoffW::new(self, 24)
    }
}
#[doc = "R-grant delay scale for erase\n\nYou can [`read`](crate::Reg::read) this register and get [`rgrant_scale_ers::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgrant_scale_ers::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RgrantScaleErsSpec;
impl crate::RegisterSpec for RgrantScaleErsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rgrant_scale_ers::R`](R) reader structure"]
impl crate::Readable for RgrantScaleErsSpec {}
#[doc = "`write(|w| ..)` method takes [`rgrant_scale_ers::W`](W) writer structure"]
impl crate::Writable for RgrantScaleErsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RGRANT_SCALE_ERS to value 0"]
impl crate::Resettable for RgrantScaleErsSpec {
    const RESET_VALUE: u32 = 0;
}
