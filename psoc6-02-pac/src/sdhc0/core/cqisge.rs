#[doc = "Register `CQISGE` reader"]
pub type R = crate::R<CqisgeSpec>;
#[doc = "Register `CQISGE` writer"]
pub type W = crate::W<CqisgeSpec>;
#[doc = "Field `HAC_SGE` reader - Halt complete interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.HAC interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.HAC interrupt signal generation is disabled"]
pub type HacSgeR = crate::BitReader;
#[doc = "Field `HAC_SGE` writer - Halt complete interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.HAC interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.HAC interrupt signal generation is disabled"]
pub type HacSgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCC_SGE` reader - Task complete interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.TCC interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.TCC interrupt signal generation is disabled"]
pub type TccSgeR = crate::BitReader;
#[doc = "Field `TCC_SGE` writer - Task complete interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.TCC interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.TCC interrupt signal generation is disabled"]
pub type TccSgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RED_SGE` reader - Response error detected interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.RED interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.RED interrupt signal generation is disabled"]
pub type RedSgeR = crate::BitReader;
#[doc = "Field `RED_SGE` writer - Response error detected interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.RED interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.RED interrupt signal generation is disabled"]
pub type RedSgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCL_SGE` reader - Task cleared interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.TCL interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.TCL interrupt signal generation is disabled"]
pub type TclSgeR = crate::BitReader;
#[doc = "Field `TCL_SGE` writer - Task cleared interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.TCL interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.TCL interrupt signal generation is disabled"]
pub type TclSgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCE_SGE` reader - General Crypto Error interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.GCE interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.GCE interrupt signal generation is disabled"]
pub type GceSgeR = crate::BitReader;
#[doc = "Field `GCE_SGE` writer - General Crypto Error interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.GCE interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.GCE interrupt signal generation is disabled"]
pub type GceSgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICCE_SGE` reader - Invalid Crypto Configuration Error interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.ICCE interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.ICCE interrupt signal generation is disabled"]
pub type IcceSgeR = crate::BitReader;
#[doc = "Field `ICCE_SGE` writer - Invalid Crypto Configuration Error interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.ICCE interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.ICCE interrupt signal generation is disabled"]
pub type IcceSgeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Halt complete interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.HAC interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.HAC interrupt signal generation is disabled"]
    #[inline(always)]
    pub fn hac_sge(&self) -> HacSgeR {
        HacSgeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Task complete interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.TCC interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.TCC interrupt signal generation is disabled"]
    #[inline(always)]
    pub fn tcc_sge(&self) -> TccSgeR {
        TccSgeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Response error detected interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.RED interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.RED interrupt signal generation is disabled"]
    #[inline(always)]
    pub fn red_sge(&self) -> RedSgeR {
        RedSgeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Task cleared interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.TCL interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.TCL interrupt signal generation is disabled"]
    #[inline(always)]
    pub fn tcl_sge(&self) -> TclSgeR {
        TclSgeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - General Crypto Error interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.GCE interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.GCE interrupt signal generation is disabled"]
    #[inline(always)]
    pub fn gce_sge(&self) -> GceSgeR {
        GceSgeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Invalid Crypto Configuration Error interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.ICCE interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.ICCE interrupt signal generation is disabled"]
    #[inline(always)]
    pub fn icce_sge(&self) -> IcceSgeR {
        IcceSgeR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Halt complete interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.HAC interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.HAC interrupt signal generation is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn hac_sge(&mut self) -> HacSgeW<CqisgeSpec> {
        HacSgeW::new(self, 0)
    }
    #[doc = "Bit 1 - Task complete interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.TCC interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.TCC interrupt signal generation is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn tcc_sge(&mut self) -> TccSgeW<CqisgeSpec> {
        TccSgeW::new(self, 1)
    }
    #[doc = "Bit 2 - Response error detected interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.RED interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.RED interrupt signal generation is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn red_sge(&mut self) -> RedSgeW<CqisgeSpec> {
        RedSgeW::new(self, 2)
    }
    #[doc = "Bit 3 - Task cleared interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.TCL interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.TCL interrupt signal generation is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn tcl_sge(&mut self) -> TclSgeW<CqisgeSpec> {
        TclSgeW::new(self, 3)
    }
    #[doc = "Bit 4 - General Crypto Error interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.GCE interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.GCE interrupt signal generation is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn gce_sge(&mut self) -> GceSgeW<CqisgeSpec> {
        GceSgeW::new(self, 4)
    }
    #[doc = "Bit 5 - Invalid Crypto Configuration Error interrupt signal enable Values: - 0x1 (INT_SIG_ENABLE): CQIS.ICCE interrupt signal generation is active - 0x0 (INT_SIG_DISABLE): CQIS.ICCE interrupt signal generation is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn icce_sge(&mut self) -> IcceSgeW<CqisgeSpec> {
        IcceSgeW::new(self, 5)
    }
}
#[doc = "Command Queuing Interrupt signal enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqisge::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqisge::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqisgeSpec;
impl crate::RegisterSpec for CqisgeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqisge::R`](R) reader structure"]
impl crate::Readable for CqisgeSpec {}
#[doc = "`write(|w| ..)` method takes [`cqisge::W`](W) writer structure"]
impl crate::Writable for CqisgeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CQISGE to value 0"]
impl crate::Resettable for CqisgeSpec {
    const RESET_VALUE: u32 = 0;
}
