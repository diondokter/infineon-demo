#[doc = "Register `CQISE` reader"]
pub type R = crate::R<CqiseSpec>;
#[doc = "Register `CQISE` writer"]
pub type W = crate::W<CqiseSpec>;
#[doc = "Field `HAC_STE` reader - Halt complete interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.HAC is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.HAC is disabled"]
pub type HacSteR = crate::BitReader;
#[doc = "Field `HAC_STE` writer - Halt complete interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.HAC is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.HAC is disabled"]
pub type HacSteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCC_STE` reader - Task complete interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.TCC is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.TCC is disabled"]
pub type TccSteR = crate::BitReader;
#[doc = "Field `TCC_STE` writer - Task complete interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.TCC is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.TCC is disabled"]
pub type TccSteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RED_STE` reader - Response error detected interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.RED is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.RED is disabled"]
pub type RedSteR = crate::BitReader;
#[doc = "Field `RED_STE` writer - Response error detected interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.RED is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.RED is disabled"]
pub type RedSteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCL_STE` reader - Task cleared interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.TCL is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.TCL is disabled"]
pub type TclSteR = crate::BitReader;
#[doc = "Field `TCL_STE` writer - Task cleared interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.TCL is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.TCL is disabled"]
pub type TclSteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCE_STE` reader - General Crypto Error interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.GCE is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.GCE is disabled"]
pub type GceSteR = crate::BitReader;
#[doc = "Field `GCE_STE` writer - General Crypto Error interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.GCE is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.GCE is disabled"]
pub type GceSteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICCE_STE` reader - Invalid Crypto Configuration Error interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.ICCE is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.ICCE is disabled"]
pub type IcceSteR = crate::BitReader;
#[doc = "Field `ICCE_STE` writer - Invalid Crypto Configuration Error interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.ICCE is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.ICCE is disabled"]
pub type IcceSteW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Halt complete interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.HAC is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.HAC is disabled"]
    #[inline(always)]
    pub fn hac_ste(&self) -> HacSteR {
        HacSteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Task complete interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.TCC is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.TCC is disabled"]
    #[inline(always)]
    pub fn tcc_ste(&self) -> TccSteR {
        TccSteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Response error detected interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.RED is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.RED is disabled"]
    #[inline(always)]
    pub fn red_ste(&self) -> RedSteR {
        RedSteR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Task cleared interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.TCL is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.TCL is disabled"]
    #[inline(always)]
    pub fn tcl_ste(&self) -> TclSteR {
        TclSteR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - General Crypto Error interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.GCE is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.GCE is disabled"]
    #[inline(always)]
    pub fn gce_ste(&self) -> GceSteR {
        GceSteR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Invalid Crypto Configuration Error interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.ICCE is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.ICCE is disabled"]
    #[inline(always)]
    pub fn icce_ste(&self) -> IcceSteR {
        IcceSteR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Halt complete interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.HAC is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.HAC is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn hac_ste(&mut self) -> HacSteW<CqiseSpec> {
        HacSteW::new(self, 0)
    }
    #[doc = "Bit 1 - Task complete interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.TCC is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.TCC is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn tcc_ste(&mut self) -> TccSteW<CqiseSpec> {
        TccSteW::new(self, 1)
    }
    #[doc = "Bit 2 - Response error detected interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.RED is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.RED is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn red_ste(&mut self) -> RedSteW<CqiseSpec> {
        RedSteW::new(self, 2)
    }
    #[doc = "Bit 3 - Task cleared interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.TCL is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.TCL is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn tcl_ste(&mut self) -> TclSteW<CqiseSpec> {
        TclSteW::new(self, 3)
    }
    #[doc = "Bit 4 - General Crypto Error interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.GCE is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.GCE is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn gce_ste(&mut self) -> GceSteW<CqiseSpec> {
        GceSteW::new(self, 4)
    }
    #[doc = "Bit 5 - Invalid Crypto Configuration Error interrupt status enable Values: - 0x1 (INT_STS_ENABLE): CQIS.ICCE is set when its interrupt condition is active - 0x0 (INT_STS_DISABLE): CQIS.ICCE is disabled"]
    #[inline(always)]
    #[must_use]
    pub fn icce_ste(&mut self) -> IcceSteW<CqiseSpec> {
        IcceSteW::new(self, 5)
    }
}
#[doc = "Command Queuing Interrupt Status Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqise::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqise::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CqiseSpec;
impl crate::RegisterSpec for CqiseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqise::R`](R) reader structure"]
impl crate::Readable for CqiseSpec {}
#[doc = "`write(|w| ..)` method takes [`cqise::W`](W) writer structure"]
impl crate::Writable for CqiseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CQISE to value 0"]
impl crate::Resettable for CqiseSpec {
    const RESET_VALUE: u32 = 0;
}
