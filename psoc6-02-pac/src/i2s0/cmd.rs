#[doc = "Register `CMD` reader"]
pub type R = crate::R<CmdSpec>;
#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `TX_START` reader - Transmitter enable: '0': Disabled. '1': Enabled."]
pub type TxStartR = crate::BitReader;
#[doc = "Field `TX_START` writer - Transmitter enable: '0': Disabled. '1': Enabled."]
pub type TxStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_PAUSE` reader - Pause enable: '0': Disabled (TX FIFO data is sent over I2S). '1': Enabled ('0' data is sent over I2S, instead of TX FIFO data)."]
pub type TxPauseR = crate::BitReader;
#[doc = "Field `TX_PAUSE` writer - Pause enable: '0': Disabled (TX FIFO data is sent over I2S). '1': Enabled ('0' data is sent over I2S, instead of TX FIFO data)."]
pub type TxPauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_START` reader - Receiver enable: '0': Disabled. '1': Enabled."]
pub type RxStartR = crate::BitReader;
#[doc = "Field `RX_START` writer - Receiver enable: '0': Disabled. '1': Enabled."]
pub type RxStartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmitter enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn tx_start(&self) -> TxStartR {
        TxStartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Pause enable: '0': Disabled (TX FIFO data is sent over I2S). '1': Enabled ('0' data is sent over I2S, instead of TX FIFO data)."]
    #[inline(always)]
    pub fn tx_pause(&self) -> TxPauseR {
        TxPauseR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Receiver enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    pub fn rx_start(&self) -> RxStartR {
        RxStartR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmitter enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn tx_start(&mut self) -> TxStartW<CmdSpec> {
        TxStartW::new(self, 0)
    }
    #[doc = "Bit 8 - Pause enable: '0': Disabled (TX FIFO data is sent over I2S). '1': Enabled ('0' data is sent over I2S, instead of TX FIFO data)."]
    #[inline(always)]
    #[must_use]
    pub fn tx_pause(&mut self) -> TxPauseW<CmdSpec> {
        TxPauseW::new(self, 8)
    }
    #[doc = "Bit 16 - Receiver enable: '0': Disabled. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn rx_start(&mut self) -> RxStartW<CmdSpec> {
        RxStartW::new(self, 16)
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
