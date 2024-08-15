#[doc = "Register `CM4_SYSTEM_INT_CTL[%s]` reader"]
pub type R = crate::R<Cm4SystemIntCtlSpec>;
#[doc = "Register `CM4_SYSTEM_INT_CTL[%s]` writer"]
pub type W = crate::W<Cm4SystemIntCtlSpec>;
#[doc = "Field `CPU_INT_IDX` reader - N/A"]
pub type CpuIntIdxR = crate::FieldReader;
#[doc = "Field `CPU_INT_IDX` writer - N/A"]
pub type CpuIntIdxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CPU_INT_VALID` reader - N/A"]
pub type CpuIntValidR = crate::BitReader;
#[doc = "Field `CPU_INT_VALID` writer - N/A"]
pub type CpuIntValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - N/A"]
    #[inline(always)]
    pub fn cpu_int_idx(&self) -> CpuIntIdxR {
        CpuIntIdxR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    pub fn cpu_int_valid(&self) -> CpuIntValidR {
        CpuIntValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_int_idx(&mut self) -> CpuIntIdxW<Cm4SystemIntCtlSpec> {
        CpuIntIdxW::new(self, 0)
    }
    #[doc = "Bit 31 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_int_valid(&mut self) -> CpuIntValidW<Cm4SystemIntCtlSpec> {
        CpuIntValidW::new(self, 31)
    }
}
#[doc = "CM4 system interrupt control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_system_int_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm4_system_int_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm4SystemIntCtlSpec;
impl crate::RegisterSpec for Cm4SystemIntCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm4_system_int_ctl::R`](R) reader structure"]
impl crate::Readable for Cm4SystemIntCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`cm4_system_int_ctl::W`](W) writer structure"]
impl crate::Writable for Cm4SystemIntCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CM4_SYSTEM_INT_CTL[%s]
to value 0"]
impl crate::Resettable for Cm4SystemIntCtlSpec {
    const RESET_VALUE: u32 = 0;
}
