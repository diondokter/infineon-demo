#[doc = "Register `CMD` reader"]
pub type R = crate::R<CmdSpec>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `START_TR` reader - Software start trigger for the profiling time window. When written with '1', the profiling time window is started. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
pub type StartTrR = crate::BitReader;
#[doc = "Field `START_TR` writer - Software start trigger for the profiling time window. When written with '1', the profiling time window is started. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
pub type StartTrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP_TR` reader - Software stop trigger for the profiling time window. When written with '1', the profiling time window is stopped. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
pub type StopTrR = crate::BitReader;
#[doc = "Field `STOP_TR` writer - Software stop trigger for the profiling time window. When written with '1', the profiling time window is stopped. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
pub type StopTrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_ALL_CNT` reader - Counter clear. When written with '1', all profiling counter registers are cleared to 0x00."]
pub type ClrAllCntR = crate::BitReader;
#[doc = "Field `CLR_ALL_CNT` writer - Counter clear. When written with '1', all profiling counter registers are cleared to 0x00."]
pub type ClrAllCntW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software start trigger for the profiling time window. When written with '1', the profiling time window is started. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
    #[inline(always)]
    pub fn start_tr(&self) -> StartTrR {
        StartTrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software stop trigger for the profiling time window. When written with '1', the profiling time window is stopped. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
    #[inline(always)]
    pub fn stop_tr(&self) -> StopTrR {
        StopTrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Counter clear. When written with '1', all profiling counter registers are cleared to 0x00."]
    #[inline(always)]
    pub fn clr_all_cnt(&self) -> ClrAllCntR {
        ClrAllCntR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software start trigger for the profiling time window. When written with '1', the profiling time window is started. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
    #[inline(always)]
    #[must_use]
    pub fn start_tr(&mut self) -> StartTrW<CmdSpec> {
        StartTrW::new(self, 0)
    }
    #[doc = "Bit 1 - Software stop trigger for the profiling time window. When written with '1', the profiling time window is stopped. Can only be used in start / stop mode (PROFILE_WIN_MODE=0). Has no effect in enable mode (PROFILE_WIN_MODE=1)."]
    #[inline(always)]
    #[must_use]
    pub fn stop_tr(&mut self) -> StopTrW<CmdSpec> {
        StopTrW::new(self, 1)
    }
    #[doc = "Bit 8 - Counter clear. When written with '1', all profiling counter registers are cleared to 0x00."]
    #[inline(always)]
    #[must_use]
    pub fn clr_all_cnt(&mut self) -> ClrAllCntW<CmdSpec> {
        ClrAllCntW::new(self, 8)
    }
}
#[doc = "Profile command\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdSpec;
impl crate::RegisterSpec for CmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CmdSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CmdSpec {
    const RESET_VALUE: u32 = 0;
}
