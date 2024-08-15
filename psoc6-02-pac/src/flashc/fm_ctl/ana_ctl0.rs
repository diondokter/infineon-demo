#[doc = "Register `ANA_CTL0` reader"]
pub type R = crate::R<AnaCtl0Spec>;
#[doc = "Register `ANA_CTL0` writer"]
pub type W = crate::W<AnaCtl0Spec>;
#[doc = "Field `MDAC` reader - Trimming of the output margin Voltage as a function of Vpos and Vneg."]
pub type MdacR = crate::FieldReader;
#[doc = "Field `MDAC` writer - Trimming of the output margin Voltage as a function of Vpos and Vneg."]
pub type MdacW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CSLDAC` reader - Trimming of common source line DAC."]
pub type CsldacR = crate::FieldReader;
#[doc = "Field `CSLDAC` writer - Trimming of common source line DAC."]
pub type CsldacW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FLIP_AMUXBUS_AB` reader - Flips amuxbusa and amuxbusb 0: amuxbusa, amuxbusb 1: amuxbusb, amuxbusb"]
pub type FlipAmuxbusAbR = crate::BitReader;
#[doc = "Field `FLIP_AMUXBUS_AB` writer - Flips amuxbusa and amuxbusb 0: amuxbusa, amuxbusb 1: amuxbusb, amuxbusb"]
pub type FlipAmuxbusAbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NDAC_MIN` reader - NDAC staircase min value"]
pub type NdacMinR = crate::FieldReader;
#[doc = "Field `NDAC_MIN` writer - NDAC staircase min value"]
pub type NdacMinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PDAC_MIN` reader - PDAC staircase min value"]
pub type PdacMinR = crate::FieldReader;
#[doc = "Field `PDAC_MIN` writer - PDAC staircase min value"]
pub type PdacMinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCALE_PRG_SEQ01` reader - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on seq0-seq1 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type ScalePrgSeq01R = crate::FieldReader;
#[doc = "Field `SCALE_PRG_SEQ01` writer - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on seq0-seq1 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type ScalePrgSeq01W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCALE_PRG_SEQ12` reader - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on seq1-seq2 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type ScalePrgSeq12R = crate::FieldReader;
#[doc = "Field `SCALE_PRG_SEQ12` writer - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on seq1-seq2 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type ScalePrgSeq12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCALE_PRG_SEQ23` reader - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on seq2-seq3 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type ScalePrgSeq23R = crate::FieldReader;
#[doc = "Field `SCALE_PRG_SEQ23` writer - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on seq2-seq3 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type ScalePrgSeq23W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCALE_SEQ30` reader - PROG&amp;PRE_PROG&amp; ERASE: Scale for R_GRANT_DELAY on seq3-seq0 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type ScaleSeq30R = crate::FieldReader;
#[doc = "Field `SCALE_SEQ30` writer - PROG&amp;PRE_PROG&amp; ERASE: Scale for R_GRANT_DELAY on seq3-seq0 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type ScaleSeq30W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCALE_PRG_PEON` reader - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on PE On transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type ScalePrgPeonR = crate::FieldReader;
#[doc = "Field `SCALE_PRG_PEON` writer - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on PE On transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type ScalePrgPeonW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCALE_PRG_PEOFF` reader - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on PE OFF transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type ScalePrgPeoffR = crate::FieldReader;
#[doc = "Field `SCALE_PRG_PEOFF` writer - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on PE OFF transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
pub type ScalePrgPeoffW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - Trimming of the output margin Voltage as a function of Vpos and Vneg."]
    #[inline(always)]
    pub fn mdac(&self) -> MdacR {
        MdacR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Trimming of common source line DAC."]
    #[inline(always)]
    pub fn csldac(&self) -> CsldacR {
        CsldacR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Flips amuxbusa and amuxbusb 0: amuxbusa, amuxbusb 1: amuxbusb, amuxbusb"]
    #[inline(always)]
    pub fn flip_amuxbus_ab(&self) -> FlipAmuxbusAbR {
        FlipAmuxbusAbR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - NDAC staircase min value"]
    #[inline(always)]
    pub fn ndac_min(&self) -> NdacMinR {
        NdacMinR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - PDAC staircase min value"]
    #[inline(always)]
    pub fn pdac_min(&self) -> PdacMinR {
        PdacMinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on seq0-seq1 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_prg_seq01(&self) -> ScalePrgSeq01R {
        ScalePrgSeq01R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on seq1-seq2 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_prg_seq12(&self) -> ScalePrgSeq12R {
        ScalePrgSeq12R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on seq2-seq3 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_prg_seq23(&self) -> ScalePrgSeq23R {
        ScalePrgSeq23R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - PROG&amp;PRE_PROG&amp; ERASE: Scale for R_GRANT_DELAY on seq3-seq0 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_seq30(&self) -> ScaleSeq30R {
        ScaleSeq30R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on PE On transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_prg_peon(&self) -> ScalePrgPeonR {
        ScalePrgPeonR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on PE OFF transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    pub fn scale_prg_peoff(&self) -> ScalePrgPeoffR {
        ScalePrgPeoffR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Trimming of the output margin Voltage as a function of Vpos and Vneg."]
    #[inline(always)]
    #[must_use]
    pub fn mdac(&mut self) -> MdacW<AnaCtl0Spec> {
        MdacW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Trimming of common source line DAC."]
    #[inline(always)]
    #[must_use]
    pub fn csldac(&mut self) -> CsldacW<AnaCtl0Spec> {
        CsldacW::new(self, 8)
    }
    #[doc = "Bit 11 - Flips amuxbusa and amuxbusb 0: amuxbusa, amuxbusb 1: amuxbusb, amuxbusb"]
    #[inline(always)]
    #[must_use]
    pub fn flip_amuxbus_ab(&mut self) -> FlipAmuxbusAbW<AnaCtl0Spec> {
        FlipAmuxbusAbW::new(self, 11)
    }
    #[doc = "Bits 12:15 - NDAC staircase min value"]
    #[inline(always)]
    #[must_use]
    pub fn ndac_min(&mut self) -> NdacMinW<AnaCtl0Spec> {
        NdacMinW::new(self, 12)
    }
    #[doc = "Bits 16:19 - PDAC staircase min value"]
    #[inline(always)]
    #[must_use]
    pub fn pdac_min(&mut self) -> PdacMinW<AnaCtl0Spec> {
        PdacMinW::new(self, 16)
    }
    #[doc = "Bits 20:21 - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on seq0-seq1 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    #[must_use]
    pub fn scale_prg_seq01(&mut self) -> ScalePrgSeq01W<AnaCtl0Spec> {
        ScalePrgSeq01W::new(self, 20)
    }
    #[doc = "Bits 22:23 - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on seq1-seq2 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    #[must_use]
    pub fn scale_prg_seq12(&mut self) -> ScalePrgSeq12W<AnaCtl0Spec> {
        ScalePrgSeq12W::new(self, 22)
    }
    #[doc = "Bits 24:25 - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on seq2-seq3 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    #[must_use]
    pub fn scale_prg_seq23(&mut self) -> ScalePrgSeq23W<AnaCtl0Spec> {
        ScalePrgSeq23W::new(self, 24)
    }
    #[doc = "Bits 26:27 - PROG&amp;PRE_PROG&amp; ERASE: Scale for R_GRANT_DELAY on seq3-seq0 transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    #[must_use]
    pub fn scale_seq30(&mut self) -> ScaleSeq30W<AnaCtl0Spec> {
        ScaleSeq30W::new(self, 26)
    }
    #[doc = "Bits 28:29 - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on PE On transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    #[must_use]
    pub fn scale_prg_peon(&mut self) -> ScalePrgPeonW<AnaCtl0Spec> {
        ScalePrgPeonW::new(self, 28)
    }
    #[doc = "Bits 30:31 - PROG&amp;PRE_PROG: Scale for R_GRANT_DELAY on PE OFF transition: 00: 0.125uS 01: 1uS 10: 10uS 11: 100uS"]
    #[inline(always)]
    #[must_use]
    pub fn scale_prg_peoff(&mut self) -> ScalePrgPeoffW<AnaCtl0Spec> {
        ScalePrgPeoffW::new(self, 30)
    }
}
#[doc = "Analog control 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ana_ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ana_ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnaCtl0Spec;
impl crate::RegisterSpec for AnaCtl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_ctl0::R`](R) reader structure"]
impl crate::Readable for AnaCtl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ana_ctl0::W`](W) writer structure"]
impl crate::Writable for AnaCtl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANA_CTL0 to value 0x0400"]
impl crate::Resettable for AnaCtl0Spec {
    const RESET_VALUE: u32 = 0x0400;
}
