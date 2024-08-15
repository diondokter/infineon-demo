#[doc = "Register `MCWDT_INTR` reader"]
pub type R = crate::R<McwdtIntrSpec>;
#[doc = "Register `MCWDT_INTR` writer"]
pub type W = crate::W<McwdtIntrSpec>;
#[doc = "Field `MCWDT_INT0` reader - MCWDT Interrupt Request for sub-counter 0. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE0=3."]
pub type McwdtInt0R = crate::BitReader;
#[doc = "Field `MCWDT_INT0` writer - MCWDT Interrupt Request for sub-counter 0. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE0=3."]
pub type McwdtInt0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCWDT_INT1` reader - MCWDT Interrupt Request for sub-counter 1. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE1=3."]
pub type McwdtInt1R = crate::BitReader;
#[doc = "Field `MCWDT_INT1` writer - MCWDT Interrupt Request for sub-counter 1. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE1=3."]
pub type McwdtInt1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCWDT_INT2` reader - MCWDT Interrupt Request for sub-counter 2. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE2=3."]
pub type McwdtInt2R = crate::BitReader;
#[doc = "Field `MCWDT_INT2` writer - MCWDT Interrupt Request for sub-counter 2. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE2=3."]
pub type McwdtInt2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MCWDT Interrupt Request for sub-counter 0. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE0=3."]
    #[inline(always)]
    pub fn mcwdt_int0(&self) -> McwdtInt0R {
        McwdtInt0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MCWDT Interrupt Request for sub-counter 1. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE1=3."]
    #[inline(always)]
    pub fn mcwdt_int1(&self) -> McwdtInt1R {
        McwdtInt1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MCWDT Interrupt Request for sub-counter 2. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE2=3."]
    #[inline(always)]
    pub fn mcwdt_int2(&self) -> McwdtInt2R {
        McwdtInt2R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCWDT Interrupt Request for sub-counter 0. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE0=3."]
    #[inline(always)]
    #[must_use]
    pub fn mcwdt_int0(&mut self) -> McwdtInt0W<McwdtIntrSpec> {
        McwdtInt0W::new(self, 0)
    }
    #[doc = "Bit 1 - MCWDT Interrupt Request for sub-counter 1. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE1=3."]
    #[inline(always)]
    #[must_use]
    pub fn mcwdt_int1(&mut self) -> McwdtInt1W<McwdtIntrSpec> {
        McwdtInt1W::new(self, 1)
    }
    #[doc = "Bit 2 - MCWDT Interrupt Request for sub-counter 2. This bit is set by hardware as configured by this registers. This bit must be cleared by firmware. Clearing this bit also prevents Reset from happening when WDT_MODE2=3."]
    #[inline(always)]
    #[must_use]
    pub fn mcwdt_int2(&mut self) -> McwdtInt2W<McwdtIntrSpec> {
        McwdtInt2W::new(self, 2)
    }
}
#[doc = "Multi-Counter Watchdog Counter Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcwdt_intr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcwdt_intr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McwdtIntrSpec;
impl crate::RegisterSpec for McwdtIntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcwdt_intr::R`](R) reader structure"]
impl crate::Readable for McwdtIntrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcwdt_intr::W`](W) writer structure"]
impl crate::Writable for McwdtIntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCWDT_INTR to value 0"]
impl crate::Resettable for McwdtIntrSpec {
    const RESET_VALUE: u32 = 0;
}
