#[doc = "Register `TR_CTL` reader"]
pub type R = crate::R<TrCtlSpec>;
#[doc = "Register `TR_CTL` writer"]
pub type W = crate::W<TrCtlSpec>;
#[doc = "Field `RX_REQ_EN` reader - Trigger output ('tr_pdm_rx_req') enable for requests of DMA transfer '0': Disabled. '1': Enabled."]
pub type RxReqEnR = crate::BitReader;
#[doc = "Field `RX_REQ_EN` writer - Trigger output ('tr_pdm_rx_req') enable for requests of DMA transfer '0': Disabled. '1': Enabled."]
pub type RxReqEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - Trigger output ('tr_pdm_rx_req') enable for requests of DMA transfer '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn rx_req_en(&self) -> RxReqEnR {
        RxReqEnR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Trigger output ('tr_pdm_rx_req') enable for requests of DMA transfer '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn rx_req_en(&mut self) -> RxReqEnW<TrCtlSpec> {
        RxReqEnW::new(self, 16)
    }
}
#[doc = "Trigger control\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrCtlSpec;
impl crate::RegisterSpec for TrCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr_ctl::R`](R) reader structure"]
impl crate::Readable for TrCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`tr_ctl::W`](W) writer structure"]
impl crate::Writable for TrCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TR_CTL to value 0"]
impl crate::Resettable for TrCtlSpec {
    const RESET_VALUE: u32 = 0;
}
