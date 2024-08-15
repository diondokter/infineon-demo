#[doc = "Register `TIMEOUT_CTL` reader"]
pub type R = crate::R<TimeoutCtlSpec>;
#[doc = "Register `TIMEOUT_CTL` writer"]
pub type W = crate::W<TimeoutCtlSpec>;
#[doc = "Field `TIMEOUT` reader - This field specifies a number of clock cycles (clk_slow). If an AHB-Lite bus transfer takes more than the specified number of cycles (timeout detection), the bus transfer is terminated with an AHB-Lite bus error and a fault is generated (and possibly recorded in the fault report structure(s)). '0x0000'-'0xfffe': Number of clock cycles. '0xffff': This value is the default/reset value and specifies that no timeout detection is performed: a bus transfer will never be terminated and a fault will never be generated."]
pub type TimeoutR = crate::FieldReader<u16>;
#[doc = "Field `TIMEOUT` writer - This field specifies a number of clock cycles (clk_slow). If an AHB-Lite bus transfer takes more than the specified number of cycles (timeout detection), the bus transfer is terminated with an AHB-Lite bus error and a fault is generated (and possibly recorded in the fault report structure(s)). '0x0000'-'0xfffe': Number of clock cycles. '0xffff': This value is the default/reset value and specifies that no timeout detection is performed: a bus transfer will never be terminated and a fault will never be generated."]
pub type TimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - This field specifies a number of clock cycles (clk_slow). If an AHB-Lite bus transfer takes more than the specified number of cycles (timeout detection), the bus transfer is terminated with an AHB-Lite bus error and a fault is generated (and possibly recorded in the fault report structure(s)). '0x0000'-'0xfffe': Number of clock cycles. '0xffff': This value is the default/reset value and specifies that no timeout detection is performed: a bus transfer will never be terminated and a fault will never be generated."]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field specifies a number of clock cycles (clk_slow). If an AHB-Lite bus transfer takes more than the specified number of cycles (timeout detection), the bus transfer is terminated with an AHB-Lite bus error and a fault is generated (and possibly recorded in the fault report structure(s)). '0x0000'-'0xfffe': Number of clock cycles. '0xffff': This value is the default/reset value and specifies that no timeout detection is performed: a bus transfer will never be terminated and a fault will never be generated."]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TimeoutW<TimeoutCtlSpec> {
        TimeoutW::new(self, 0)
    }
}
#[doc = "Timeout control\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimeoutCtlSpec;
impl crate::RegisterSpec for TimeoutCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timeout_ctl::R`](R) reader structure"]
impl crate::Readable for TimeoutCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`timeout_ctl::W`](W) writer structure"]
impl crate::Writable for TimeoutCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMEOUT_CTL to value 0xffff"]
impl crate::Resettable for TimeoutCtlSpec {
    const RESET_VALUE: u32 = 0xffff;
}
