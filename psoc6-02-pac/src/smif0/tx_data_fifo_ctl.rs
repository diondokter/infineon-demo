#[doc = "Register `TX_DATA_FIFO_CTL` reader"]
pub type R = crate::R<TxDataFifoCtlSpec>;
#[doc = "Register `TX_DATA_FIFO_CTL` writer"]
pub type W = crate::W<TxDataFifoCtlSpec>;
#[doc = "Field `TRIGGER_LEVEL` reader - Determines when the TX data FIFO 'tr_tx_req' trigger is activated (trigger activation requires MMIO_MODE, the trigger is NOT activated in XIP_MODE): - Trigger is active when TX_DATA_FIFO_STATUS.USED &lt;= TRIGGER_LEVEL."]
pub type TriggerLevelR = crate::FieldReader;
#[doc = "Field `TRIGGER_LEVEL` writer - Determines when the TX data FIFO 'tr_tx_req' trigger is activated (trigger activation requires MMIO_MODE, the trigger is NOT activated in XIP_MODE): - Trigger is active when TX_DATA_FIFO_STATUS.USED &lt;= TRIGGER_LEVEL."]
pub type TriggerLevelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Determines when the TX data FIFO 'tr_tx_req' trigger is activated (trigger activation requires MMIO_MODE, the trigger is NOT activated in XIP_MODE): - Trigger is active when TX_DATA_FIFO_STATUS.USED &lt;= TRIGGER_LEVEL."]
    #[inline(always)]
    pub fn trigger_level(&self) -> TriggerLevelR {
        TriggerLevelR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Determines when the TX data FIFO 'tr_tx_req' trigger is activated (trigger activation requires MMIO_MODE, the trigger is NOT activated in XIP_MODE): - Trigger is active when TX_DATA_FIFO_STATUS.USED &lt;= TRIGGER_LEVEL."]
    #[inline(always)]
    #[must_use]
    pub fn trigger_level(&mut self) -> TriggerLevelW<TxDataFifoCtlSpec> {
        TriggerLevelW::new(self, 0)
    }
}
#[doc = "Transmitter data FIFO control\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_data_fifo_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_data_fifo_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxDataFifoCtlSpec;
impl crate::RegisterSpec for TxDataFifoCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_data_fifo_ctl::R`](R) reader structure"]
impl crate::Readable for TxDataFifoCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_data_fifo_ctl::W`](W) writer structure"]
impl crate::Writable for TxDataFifoCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX_DATA_FIFO_CTL to value 0"]
impl crate::Resettable for TxDataFifoCtlSpec {
    const RESET_VALUE: u32 = 0;
}
