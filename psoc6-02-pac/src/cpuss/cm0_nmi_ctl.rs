#[doc = "Register `CM0_NMI_CTL[%s]` reader"]
pub type R = crate::R<Cm0NmiCtlSpec>;
#[doc = "Register `CM0_NMI_CTL[%s]` writer"]
pub type W = crate::W<Cm0NmiCtlSpec>;
#[doc = "Field `SYSTEM_INT_IDX` reader - System interrupt select for CPU NMI. The reset value ('1023') ensures that the CPU NMI is NOT connected to any system interrupt after DeepSleep reset."]
pub type SystemIntIdxR = crate::FieldReader<u16>;
#[doc = "Field `SYSTEM_INT_IDX` writer - System interrupt select for CPU NMI. The reset value ('1023') ensures that the CPU NMI is NOT connected to any system interrupt after DeepSleep reset."]
pub type SystemIntIdxW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - System interrupt select for CPU NMI. The reset value ('1023') ensures that the CPU NMI is NOT connected to any system interrupt after DeepSleep reset."]
    #[inline(always)]
    pub fn system_int_idx(&self) -> SystemIntIdxR {
        SystemIntIdxR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - System interrupt select for CPU NMI. The reset value ('1023') ensures that the CPU NMI is NOT connected to any system interrupt after DeepSleep reset."]
    #[inline(always)]
    #[must_use]
    pub fn system_int_idx(&mut self) -> SystemIntIdxW<Cm0NmiCtlSpec> {
        SystemIntIdxW::new(self, 0)
    }
}
#[doc = "CM0+ NMI control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm0_nmi_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm0_nmi_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm0NmiCtlSpec;
impl crate::RegisterSpec for Cm0NmiCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm0_nmi_ctl::R`](R) reader structure"]
impl crate::Readable for Cm0NmiCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`cm0_nmi_ctl::W`](W) writer structure"]
impl crate::Writable for Cm0NmiCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CM0_NMI_CTL[%s]
to value 0x03ff"]
impl crate::Resettable for Cm0NmiCtlSpec {
    const RESET_VALUE: u32 = 0x03ff;
}
