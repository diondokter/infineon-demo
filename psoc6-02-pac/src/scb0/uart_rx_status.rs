#[doc = "Register `UART_RX_STATUS` reader"]
pub type R = crate::R<UartRxStatusSpec>;
#[doc = "Field `BR_COUNTER` reader - Amount of peripheral clock periods that constitute the transmission of a 0x55 data frame (sent least significant bit first) as determined by the receiver. BR_COUNTER / 8 is the amount of peripheral clock periods that constitute a bit period. This field has valid data when INTR_RX.BAUD_DETECT is set to '1'."]
pub type BrCounterR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Amount of peripheral clock periods that constitute the transmission of a 0x55 data frame (sent least significant bit first) as determined by the receiver. BR_COUNTER / 8 is the amount of peripheral clock periods that constitute a bit period. This field has valid data when INTR_RX.BAUD_DETECT is set to '1'."]
    #[inline(always)]
    pub fn br_counter(&self) -> BrCounterR {
        BrCounterR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "UART receiver status\n\nYou can [`read`](crate::Reg::read) this register and get [`uart_rx_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartRxStatusSpec;
impl crate::RegisterSpec for UartRxStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_rx_status::R`](R) reader structure"]
impl crate::Readable for UartRxStatusSpec {}
#[doc = "`reset()` method sets UART_RX_STATUS to value 0"]
impl crate::Resettable for UartRxStatusSpec {
    const RESET_VALUE: u32 = 0;
}
