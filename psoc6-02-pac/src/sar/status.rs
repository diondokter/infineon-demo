#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `CUR_CHAN` reader - current channel being sampled (channel 16 indicates the injection channel), only valid if BUSY."]
pub type CurChanR = crate::FieldReader;
#[doc = "Field `SW_VREF_NEG` reader - the current switch status, including DSI and sequencer controls, of the switch in the SARADC that shorts NEG with VREF input (see NEG_SEL)."]
pub type SwVrefNegR = crate::BitReader;
#[doc = "Field `BUSY` reader - If high then the SAR is busy with a conversion. This bit is always high when CONTINUOUS is set. Firmware should wait for this bit to be low before putting the SAR in power down."]
pub type BusyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - current channel being sampled (channel 16 indicates the injection channel), only valid if BUSY."]
    #[inline(always)]
    pub fn cur_chan(&self) -> CurChanR {
        CurChanR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 30 - the current switch status, including DSI and sequencer controls, of the switch in the SARADC that shorts NEG with VREF input (see NEG_SEL)."]
    #[inline(always)]
    pub fn sw_vref_neg(&self) -> SwVrefNegR {
        SwVrefNegR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - If high then the SAR is busy with a conversion. This bit is always high when CONTINUOUS is set. Firmware should wait for this bit to be low before putting the SAR in power down."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Current status of internal SAR registers (mostly for debug)\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
