#[doc = "Register `BGAP_CTRL_R` reader"]
pub type R = crate::R<BgapCtrlRSpec>;
#[doc = "Register `BGAP_CTRL_R` writer"]
pub type W = crate::W<BgapCtrlRSpec>;
#[doc = "Field `STOP_BG_REQ` reader - Stop At Block Gap Request This bit is used to stop executing read and write transactions at the next block gap for non-DMA, SDMA, and ADMA transfers. Values: - 0x0 (XFER): Transfer - 0x1 (STOP): Stop"]
pub type StopBgReqR = crate::BitReader;
#[doc = "Field `STOP_BG_REQ` writer - Stop At Block Gap Request This bit is used to stop executing read and write transactions at the next block gap for non-DMA, SDMA, and ADMA transfers. Values: - 0x0 (XFER): Transfer - 0x1 (STOP): Stop"]
pub type StopBgReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONTINUE_REQ` reader - Continue Request This bit is used to restart the transaction, which was stopped using the Stop At Block Gap Request. The Host Controller automatically clears this bit when the transaction restarts. If stop at block gap request is set to 1, any write to this bit is ignored. Values: - 0x0 (NO_AFFECT): No Affect - 0x1 (RESTART): Restart"]
pub type ContinueReqR = crate::BitReader;
#[doc = "Field `CONTINUE_REQ` writer - Continue Request This bit is used to restart the transaction, which was stopped using the Stop At Block Gap Request. The Host Controller automatically clears this bit when the transaction restarts. If stop at block gap request is set to 1, any write to this bit is ignored. Values: - 0x0 (NO_AFFECT): No Affect - 0x1 (RESTART): Restart"]
pub type ContinueReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_WAIT_CTRL` reader - N/A"]
pub type RdWaitCtrlR = crate::BitReader;
#[doc = "Field `RD_WAIT_CTRL` writer - N/A"]
pub type RdWaitCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_AT_BGAP` reader - Interrupt At Block Gap This bit is valid only in the 4-bit mode of an SDIO card and is used to select a sample point in the interrupt cycle. Setting to 1 enables interrupt detection at the block gap for a multiple block transfer. Values: - 0x0 (DISABLE): Disabled - 0x1 (ENABLE): Enabled"]
pub type IntAtBgapR = crate::BitReader;
#[doc = "Field `INT_AT_BGAP` writer - Interrupt At Block Gap This bit is valid only in the 4-bit mode of an SDIO card and is used to select a sample point in the interrupt cycle. Setting to 1 enables interrupt detection at the block gap for a multiple block transfer. Values: - 0x0 (DISABLE): Disabled - 0x1 (ENABLE): Enabled"]
pub type IntAtBgapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stop At Block Gap Request This bit is used to stop executing read and write transactions at the next block gap for non-DMA, SDMA, and ADMA transfers. Values: - 0x0 (XFER): Transfer - 0x1 (STOP): Stop"]
    #[inline(always)]
    pub fn stop_bg_req(&self) -> StopBgReqR {
        StopBgReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Continue Request This bit is used to restart the transaction, which was stopped using the Stop At Block Gap Request. The Host Controller automatically clears this bit when the transaction restarts. If stop at block gap request is set to 1, any write to this bit is ignored. Values: - 0x0 (NO_AFFECT): No Affect - 0x1 (RESTART): Restart"]
    #[inline(always)]
    pub fn continue_req(&self) -> ContinueReqR {
        ContinueReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn rd_wait_ctrl(&self) -> RdWaitCtrlR {
        RdWaitCtrlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt At Block Gap This bit is valid only in the 4-bit mode of an SDIO card and is used to select a sample point in the interrupt cycle. Setting to 1 enables interrupt detection at the block gap for a multiple block transfer. Values: - 0x0 (DISABLE): Disabled - 0x1 (ENABLE): Enabled"]
    #[inline(always)]
    pub fn int_at_bgap(&self) -> IntAtBgapR {
        IntAtBgapR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop At Block Gap Request This bit is used to stop executing read and write transactions at the next block gap for non-DMA, SDMA, and ADMA transfers. Values: - 0x0 (XFER): Transfer - 0x1 (STOP): Stop"]
    #[inline(always)]
    #[must_use]
    pub fn stop_bg_req(&mut self) -> StopBgReqW<BgapCtrlRSpec> {
        StopBgReqW::new(self, 0)
    }
    #[doc = "Bit 1 - Continue Request This bit is used to restart the transaction, which was stopped using the Stop At Block Gap Request. The Host Controller automatically clears this bit when the transaction restarts. If stop at block gap request is set to 1, any write to this bit is ignored. Values: - 0x0 (NO_AFFECT): No Affect - 0x1 (RESTART): Restart"]
    #[inline(always)]
    #[must_use]
    pub fn continue_req(&mut self) -> ContinueReqW<BgapCtrlRSpec> {
        ContinueReqW::new(self, 1)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn rd_wait_ctrl(&mut self) -> RdWaitCtrlW<BgapCtrlRSpec> {
        RdWaitCtrlW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt At Block Gap This bit is valid only in the 4-bit mode of an SDIO card and is used to select a sample point in the interrupt cycle. Setting to 1 enables interrupt detection at the block gap for a multiple block transfer. Values: - 0x0 (DISABLE): Disabled - 0x1 (ENABLE): Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn int_at_bgap(&mut self) -> IntAtBgapW<BgapCtrlRSpec> {
        IntAtBgapW::new(self, 3)
    }
}
#[doc = "Block Gap Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bgap_ctrl_r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgap_ctrl_r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BgapCtrlRSpec;
impl crate::RegisterSpec for BgapCtrlRSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bgap_ctrl_r::R`](R) reader structure"]
impl crate::Readable for BgapCtrlRSpec {}
#[doc = "`write(|w| ..)` method takes [`bgap_ctrl_r::W`](W) writer structure"]
impl crate::Writable for BgapCtrlRSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets BGAP_CTRL_R to value 0"]
impl crate::Resettable for BgapCtrlRSpec {
    const RESET_VALUE: u8 = 0;
}
