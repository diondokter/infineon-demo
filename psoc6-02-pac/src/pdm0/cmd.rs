#[doc = "Register `CMD` reader"]
pub type R = crate::R<CmdSpec>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `STREAM_EN` reader - Enable data streaming flow: '0': Disabled. '1': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.PDMA_EN)"]
pub type StreamEnR = crate::BitReader;
#[doc = "Field `STREAM_EN` writer - Enable data streaming flow: '0': Disabled. '1': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.PDMA_EN)"]
pub type StreamEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable data streaming flow: '0': Disabled. '1': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.PDMA_EN)"]
    #[inline(always)]
    pub fn stream_en(&self) -> StreamEnR {
        StreamEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable data streaming flow: '0': Disabled. '1': Enabled. (Note: This bit is connected to AR36U12.PDM_CORE_CFG.PDMA_EN)"]
    #[inline(always)]
    #[must_use]
    pub fn stream_en(&mut self) -> StreamEnW<CmdSpec> {
        StreamEnW::new(self, 0)
    }
}
#[doc = "Command\n\nYou can [`read`](crate::Reg::read) this register and get [`cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
