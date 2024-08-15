#[doc = "Register `RGRANT_DELAY_PRG` reader"]
pub type R = crate::R<RgrantDelayPrgSpec>;
#[doc = "Register `RGRANT_DELAY_PRG` writer"]
pub type W = crate::W<RgrantDelayPrgSpec>;
#[doc = "Field `RGRANT_DELAY_PRG_SEQ12` reader - PROG&amp;PRE_PROG: R-grant blocking delay on seq1-seq2 transition. Scale = ANA_CTL0.SCALE_SEQ12 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RgrantDelayPrgSeq12R = crate::FieldReader;
#[doc = "Field `RGRANT_DELAY_PRG_SEQ12` writer - PROG&amp;PRE_PROG: R-grant blocking delay on seq1-seq2 transition. Scale = ANA_CTL0.SCALE_SEQ12 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RgrantDelayPrgSeq12W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RGRANT_DELAY_PRG_SEQ23` reader - PROG&amp;PRE_PROG: R-grant blocking delay on seq2-seq3 transition. Scale = ANA_CTL0.SCALE_SEQ23 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RgrantDelayPrgSeq23R = crate::FieldReader;
#[doc = "Field `RGRANT_DELAY_PRG_SEQ23` writer - PROG&amp;PRE_PROG: R-grant blocking delay on seq2-seq3 transition. Scale = ANA_CTL0.SCALE_SEQ23 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RgrantDelayPrgSeq23W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RGRANT_DELAY_SEQ30` reader - PROG&amp;PRE_PROG &amp; ERASE: R-grant blocking delay on seq3-seq0 transition. Scale = ANA_CTL0.SCALE_SEQ30 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RgrantDelaySeq30R = crate::FieldReader;
#[doc = "Field `RGRANT_DELAY_SEQ30` writer - PROG&amp;PRE_PROG &amp; ERASE: R-grant blocking delay on seq3-seq0 transition. Scale = ANA_CTL0.SCALE_SEQ30 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
pub type RgrantDelaySeq30W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RGRANT_DELAY_CLK` reader - Frequency divider from clk_t to create the 8MHz reference clock for R_grant delay The value of this field is the integer result of 'clk_t frequency / 8'. Example: for clk_t=100 this field is INT(100/8) =12. This field is updated at runtime with the 'SW_RGRANT_DELAY_CLK ' value from the HV parameters table"]
pub type RgrantDelayClkR = crate::FieldReader;
#[doc = "Field `RGRANT_DELAY_CLK` writer - Frequency divider from clk_t to create the 8MHz reference clock for R_grant delay The value of this field is the integer result of 'clk_t frequency / 8'. Example: for clk_t=100 this field is INT(100/8) =12. This field is updated at runtime with the 'SW_RGRANT_DELAY_CLK ' value from the HV parameters table"]
pub type RgrantDelayClkW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HV_PARAMS_LOADED` reader - 0: HV Pulse common params not loaded 1: HV Pulse common params loaded: r-grant delays, r-grant scale, prescaler, timer values for seq1,seq2_pre, seq2_post, seq3"]
pub type HvParamsLoadedR = crate::BitReader;
#[doc = "Field `HV_PARAMS_LOADED` writer - 0: HV Pulse common params not loaded 1: HV Pulse common params loaded: r-grant delays, r-grant scale, prescaler, timer values for seq1,seq2_pre, seq2_post, seq3"]
pub type HvParamsLoadedW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PROG&amp;PRE_PROG: R-grant blocking delay on seq1-seq2 transition. Scale = ANA_CTL0.SCALE_SEQ12 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_prg_seq12(&self) -> RgrantDelayPrgSeq12R {
        RgrantDelayPrgSeq12R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - PROG&amp;PRE_PROG: R-grant blocking delay on seq2-seq3 transition. Scale = ANA_CTL0.SCALE_SEQ23 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_prg_seq23(&self) -> RgrantDelayPrgSeq23R {
        RgrantDelayPrgSeq23R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - PROG&amp;PRE_PROG &amp; ERASE: R-grant blocking delay on seq3-seq0 transition. Scale = ANA_CTL0.SCALE_SEQ30 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    pub fn rgrant_delay_seq30(&self) -> RgrantDelaySeq30R {
        RgrantDelaySeq30R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Frequency divider from clk_t to create the 8MHz reference clock for R_grant delay The value of this field is the integer result of 'clk_t frequency / 8'. Example: for clk_t=100 this field is INT(100/8) =12. This field is updated at runtime with the 'SW_RGRANT_DELAY_CLK ' value from the HV parameters table"]
    #[inline(always)]
    pub fn rgrant_delay_clk(&self) -> RgrantDelayClkR {
        RgrantDelayClkR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - 0: HV Pulse common params not loaded 1: HV Pulse common params loaded: r-grant delays, r-grant scale, prescaler, timer values for seq1,seq2_pre, seq2_post, seq3"]
    #[inline(always)]
    pub fn hv_params_loaded(&self) -> HvParamsLoadedR {
        HvParamsLoadedR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PROG&amp;PRE_PROG: R-grant blocking delay on seq1-seq2 transition. Scale = ANA_CTL0.SCALE_SEQ12 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn rgrant_delay_prg_seq12(&mut self) -> RgrantDelayPrgSeq12W<RgrantDelayPrgSpec> {
        RgrantDelayPrgSeq12W::new(self, 0)
    }
    #[doc = "Bits 8:15 - PROG&amp;PRE_PROG: R-grant blocking delay on seq2-seq3 transition. Scale = ANA_CTL0.SCALE_SEQ23 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn rgrant_delay_prg_seq23(&mut self) -> RgrantDelayPrgSeq23W<RgrantDelayPrgSpec> {
        RgrantDelayPrgSeq23W::new(self, 8)
    }
    #[doc = "Bits 16:23 - PROG&amp;PRE_PROG &amp; ERASE: R-grant blocking delay on seq3-seq0 transition. Scale = ANA_CTL0.SCALE_SEQ30 When = 0 R_GRANT_DELAY control is disabled when IF_SEL=1 R_GRANT_DELAY control is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn rgrant_delay_seq30(&mut self) -> RgrantDelaySeq30W<RgrantDelayPrgSpec> {
        RgrantDelaySeq30W::new(self, 16)
    }
    #[doc = "Bits 24:27 - Frequency divider from clk_t to create the 8MHz reference clock for R_grant delay The value of this field is the integer result of 'clk_t frequency / 8'. Example: for clk_t=100 this field is INT(100/8) =12. This field is updated at runtime with the 'SW_RGRANT_DELAY_CLK ' value from the HV parameters table"]
    #[inline(always)]
    #[must_use]
    pub fn rgrant_delay_clk(&mut self) -> RgrantDelayClkW<RgrantDelayPrgSpec> {
        RgrantDelayClkW::new(self, 24)
    }
    #[doc = "Bit 31 - 0: HV Pulse common params not loaded 1: HV Pulse common params loaded: r-grant delays, r-grant scale, prescaler, timer values for seq1,seq2_pre, seq2_post, seq3"]
    #[inline(always)]
    #[must_use]
    pub fn hv_params_loaded(&mut self) -> HvParamsLoadedW<RgrantDelayPrgSpec> {
        HvParamsLoadedW::new(self, 31)
    }
}
#[doc = "R-grant delay for program\n\nYou can [`read`](crate::Reg::read) this register and get [`rgrant_delay_prg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgrant_delay_prg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RgrantDelayPrgSpec;
impl crate::RegisterSpec for RgrantDelayPrgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rgrant_delay_prg::R`](R) reader structure"]
impl crate::Readable for RgrantDelayPrgSpec {}
#[doc = "`write(|w| ..)` method takes [`rgrant_delay_prg::W`](W) writer structure"]
impl crate::Writable for RgrantDelayPrgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RGRANT_DELAY_PRG to value 0x0100_0000"]
impl crate::Resettable for RgrantDelayPrgSpec {
    const RESET_VALUE: u32 = 0x0100_0000;
}
