#[doc = "Register `TR_MON_AP_CTL` reader"]
pub type R = crate::R<TrMonApCtlSpec>;
#[doc = "Register `TR_MON_AP_CTL` writer"]
pub type W = crate::W<TrMonApCtlSpec>;
#[doc = "Field `CUTOFF_COUNT16` reader - Cutoff count (legal range is \\[1, 65535\\]). '0': Illegal. '1': 1 occurrence. ... '65535': 65535 occurrences."]
pub type CutoffCount16R = crate::FieldReader<u16>;
#[doc = "Field `CUTOFF_COUNT16` writer - Cutoff count (legal range is \\[1, 65535\\]). '0': Illegal. '1': 1 occurrence. ... '65535': 65535 occurrences."]
pub type CutoffCount16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WINDOW_SIZE` reader - Window size (minus 1) : '0': 1 bit. ... '65535': 65536 bits."]
pub type WindowSizeR = crate::FieldReader<u16>;
#[doc = "Field `WINDOW_SIZE` writer - Window size (minus 1) : '0': 1 bit. ... '65535': 65536 bits."]
pub type WindowSizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Cutoff count (legal range is \\[1, 65535\\]). '0': Illegal. '1': 1 occurrence. ... '65535': 65535 occurrences."]
    #[inline(always)]
    pub fn cutoff_count16(&self) -> CutoffCount16R {
        CutoffCount16R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Window size (minus 1) : '0': 1 bit. ... '65535': 65536 bits."]
    #[inline(always)]
    pub fn window_size(&self) -> WindowSizeR {
        WindowSizeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Cutoff count (legal range is \\[1, 65535\\]). '0': Illegal. '1': 1 occurrence. ... '65535': 65535 occurrences."]
    #[inline(always)]
    #[must_use]
    pub fn cutoff_count16(&mut self) -> CutoffCount16W<TrMonApCtlSpec> {
        CutoffCount16W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Window size (minus 1) : '0': 1 bit. ... '65535': 65536 bits."]
    #[inline(always)]
    #[must_use]
    pub fn window_size(&mut self) -> WindowSizeW<TrMonApCtlSpec> {
        WindowSizeW::new(self, 16)
    }
}
#[doc = "True random monitor AP control\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_mon_ap_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_mon_ap_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrMonApCtlSpec;
impl crate::RegisterSpec for TrMonApCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr_mon_ap_ctl::R`](R) reader structure"]
impl crate::Readable for TrMonApCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`tr_mon_ap_ctl::W`](W) writer structure"]
impl crate::Writable for TrMonApCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TR_MON_AP_CTL to value 0xffff_ffff"]
impl crate::Resettable for TrMonApCtlSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
