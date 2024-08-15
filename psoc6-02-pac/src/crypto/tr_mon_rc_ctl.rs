#[doc = "Register `TR_MON_RC_CTL` reader"]
pub type R = crate::R<TrMonRcCtlSpec>;
#[doc = "Register `TR_MON_RC_CTL` writer"]
pub type W = crate::W<TrMonRcCtlSpec>;
#[doc = "Field `CUTOFF_COUNT8` reader - Cutoff count (legal range is \\[1, 255\\]): '0': Illegal. '1': 1 repetition. ... '255': 255 repetitions."]
pub type CutoffCount8R = crate::FieldReader;
#[doc = "Field `CUTOFF_COUNT8` writer - Cutoff count (legal range is \\[1, 255\\]): '0': Illegal. '1': 1 repetition. ... '255': 255 repetitions."]
pub type CutoffCount8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Cutoff count (legal range is \\[1, 255\\]): '0': Illegal. '1': 1 repetition. ... '255': 255 repetitions."]
    #[inline(always)]
    pub fn cutoff_count8(&self) -> CutoffCount8R {
        CutoffCount8R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Cutoff count (legal range is \\[1, 255\\]): '0': Illegal. '1': 1 repetition. ... '255': 255 repetitions."]
    #[inline(always)]
    #[must_use]
    pub fn cutoff_count8(&mut self) -> CutoffCount8W<TrMonRcCtlSpec> {
        CutoffCount8W::new(self, 0)
    }
}
#[doc = "True random monitor RC control\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_mon_rc_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_mon_rc_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrMonRcCtlSpec;
impl crate::RegisterSpec for TrMonRcCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr_mon_rc_ctl::R`](R) reader structure"]
impl crate::Readable for TrMonRcCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`tr_mon_rc_ctl::W`](W) writer structure"]
impl crate::Writable for TrMonRcCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TR_MON_RC_CTL to value 0xff"]
impl crate::Resettable for TrMonRcCtlSpec {
    const RESET_VALUE: u32 = 0xff;
}
