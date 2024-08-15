#[doc = "Register `CQSSC1` reader"]
pub type R = crate::R<Cqssc1Spec>;
#[doc = "Register `CQSSC1` writer"]
pub type W = crate::W<Cqssc1Spec>;
#[doc = "Field `SQSCMD_IDLE_TMR` reader - This field configures the polling period to be used when using periodic SEND_QUEUE_STATUS (CMD13) polling. Periodic polling is used when tasks are pending in the device, but no data transfer is in progress. When a SEND_QUEUE_STATUS response indicates that no task is ready for execution, CQE counts the configured time until it issues the next SEND_QUEUE_STATUS. Timer units are clock periods of the clock whose frequency is specified in the Internal Timer Clock Frequency field CQCAP register. The minimum value is 0001h (1 clock period) and the maximum value is FFFFh (65535 clock periods). For example, a CQCAP field value of 0 indicates a 19.2 MHz clock frequency (period = 52.08 ns). If the setting in CQSSC1.CIT is 1000h, the calculated polling period is 4096*52.08 ns= 213.33 ns. Should be programmed only when CQCFG.CQ_EN is '0'."]
pub type SqscmdIdleTmrR = crate::FieldReader<u16>;
#[doc = "Field `SQSCMD_IDLE_TMR` writer - This field configures the polling period to be used when using periodic SEND_QUEUE_STATUS (CMD13) polling. Periodic polling is used when tasks are pending in the device, but no data transfer is in progress. When a SEND_QUEUE_STATUS response indicates that no task is ready for execution, CQE counts the configured time until it issues the next SEND_QUEUE_STATUS. Timer units are clock periods of the clock whose frequency is specified in the Internal Timer Clock Frequency field CQCAP register. The minimum value is 0001h (1 clock period) and the maximum value is FFFFh (65535 clock periods). For example, a CQCAP field value of 0 indicates a 19.2 MHz clock frequency (period = 52.08 ns). If the setting in CQSSC1.CIT is 1000h, the calculated polling period is 4096*52.08 ns= 213.33 ns. Should be programmed only when CQCFG.CQ_EN is '0'."]
pub type SqscmdIdleTmrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SQSCMD_BLK_CNT` reader - This field indicates when SQS CMD is sent while data transfer is in progress. A value of 'n' indicates that CQE sends status command on the CMD line, during the transfer of data block BLOCK_CNTn, on the data lines, where BLOCK_CNT is the number of blocks in the current transaction. - 0x0: SEND_QUEUE_STATUS (CMD13) command is not sent during the transaction. Instead, it is sent only when the data lines are idle. - 0x1: SEND_QUEUE_STATUS command is to be sent during the last block of the transaction. - 0x2: SEND_QUEUE_STATUS command when last 2 blocks are pending. - 0x3: SEND_QUEUE_STATUS command when last 3 blocks are pending. - ........ - 0xf: SEND_QUEUE_STATUS command when last 15 blocks are pending. Should be programmed only when CQCFG.CQ_EN is '0'"]
pub type SqscmdBlkCntR = crate::FieldReader;
#[doc = "Field `SQSCMD_BLK_CNT` writer - This field indicates when SQS CMD is sent while data transfer is in progress. A value of 'n' indicates that CQE sends status command on the CMD line, during the transfer of data block BLOCK_CNTn, on the data lines, where BLOCK_CNT is the number of blocks in the current transaction. - 0x0: SEND_QUEUE_STATUS (CMD13) command is not sent during the transaction. Instead, it is sent only when the data lines are idle. - 0x1: SEND_QUEUE_STATUS command is to be sent during the last block of the transaction. - 0x2: SEND_QUEUE_STATUS command when last 2 blocks are pending. - 0x3: SEND_QUEUE_STATUS command when last 3 blocks are pending. - ........ - 0xf: SEND_QUEUE_STATUS command when last 15 blocks are pending. Should be programmed only when CQCFG.CQ_EN is '0'"]
pub type SqscmdBlkCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - This field configures the polling period to be used when using periodic SEND_QUEUE_STATUS (CMD13) polling. Periodic polling is used when tasks are pending in the device, but no data transfer is in progress. When a SEND_QUEUE_STATUS response indicates that no task is ready for execution, CQE counts the configured time until it issues the next SEND_QUEUE_STATUS. Timer units are clock periods of the clock whose frequency is specified in the Internal Timer Clock Frequency field CQCAP register. The minimum value is 0001h (1 clock period) and the maximum value is FFFFh (65535 clock periods). For example, a CQCAP field value of 0 indicates a 19.2 MHz clock frequency (period = 52.08 ns). If the setting in CQSSC1.CIT is 1000h, the calculated polling period is 4096*52.08 ns= 213.33 ns. Should be programmed only when CQCFG.CQ_EN is '0'."]
    #[inline(always)]
    pub fn sqscmd_idle_tmr(&self) -> SqscmdIdleTmrR {
        SqscmdIdleTmrR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - This field indicates when SQS CMD is sent while data transfer is in progress. A value of 'n' indicates that CQE sends status command on the CMD line, during the transfer of data block BLOCK_CNTn, on the data lines, where BLOCK_CNT is the number of blocks in the current transaction. - 0x0: SEND_QUEUE_STATUS (CMD13) command is not sent during the transaction. Instead, it is sent only when the data lines are idle. - 0x1: SEND_QUEUE_STATUS command is to be sent during the last block of the transaction. - 0x2: SEND_QUEUE_STATUS command when last 2 blocks are pending. - 0x3: SEND_QUEUE_STATUS command when last 3 blocks are pending. - ........ - 0xf: SEND_QUEUE_STATUS command when last 15 blocks are pending. Should be programmed only when CQCFG.CQ_EN is '0'"]
    #[inline(always)]
    pub fn sqscmd_blk_cnt(&self) -> SqscmdBlkCntR {
        SqscmdBlkCntR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - This field configures the polling period to be used when using periodic SEND_QUEUE_STATUS (CMD13) polling. Periodic polling is used when tasks are pending in the device, but no data transfer is in progress. When a SEND_QUEUE_STATUS response indicates that no task is ready for execution, CQE counts the configured time until it issues the next SEND_QUEUE_STATUS. Timer units are clock periods of the clock whose frequency is specified in the Internal Timer Clock Frequency field CQCAP register. The minimum value is 0001h (1 clock period) and the maximum value is FFFFh (65535 clock periods). For example, a CQCAP field value of 0 indicates a 19.2 MHz clock frequency (period = 52.08 ns). If the setting in CQSSC1.CIT is 1000h, the calculated polling period is 4096*52.08 ns= 213.33 ns. Should be programmed only when CQCFG.CQ_EN is '0'."]
    #[inline(always)]
    #[must_use]
    pub fn sqscmd_idle_tmr(&mut self) -> SqscmdIdleTmrW<Cqssc1Spec> {
        SqscmdIdleTmrW::new(self, 0)
    }
    #[doc = "Bits 16:19 - This field indicates when SQS CMD is sent while data transfer is in progress. A value of 'n' indicates that CQE sends status command on the CMD line, during the transfer of data block BLOCK_CNTn, on the data lines, where BLOCK_CNT is the number of blocks in the current transaction. - 0x0: SEND_QUEUE_STATUS (CMD13) command is not sent during the transaction. Instead, it is sent only when the data lines are idle. - 0x1: SEND_QUEUE_STATUS command is to be sent during the last block of the transaction. - 0x2: SEND_QUEUE_STATUS command when last 2 blocks are pending. - 0x3: SEND_QUEUE_STATUS command when last 3 blocks are pending. - ........ - 0xf: SEND_QUEUE_STATUS command when last 15 blocks are pending. Should be programmed only when CQCFG.CQ_EN is '0'"]
    #[inline(always)]
    #[must_use]
    pub fn sqscmd_blk_cnt(&mut self) -> SqscmdBlkCntW<Cqssc1Spec> {
        SqscmdBlkCntW::new(self, 16)
    }
}
#[doc = "CQ Send Status Configuration 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`cqssc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cqssc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cqssc1Spec;
impl crate::RegisterSpec for Cqssc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cqssc1::R`](R) reader structure"]
impl crate::Readable for Cqssc1Spec {}
#[doc = "`write(|w| ..)` method takes [`cqssc1::W`](W) writer structure"]
impl crate::Writable for Cqssc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CQSSC1 to value 0x0001_1000"]
impl crate::Resettable for Cqssc1Spec {
    const RESET_VALUE: u32 = 0x0001_1000;
}
