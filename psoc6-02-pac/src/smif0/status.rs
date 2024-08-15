#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `BUSY` reader - Cache, cryptography, XIP, device interface or any other logic busy in the IP: '0': not busy '1': busy When BUSY is '0', the IP can be safely disabled without: - the potential loss of transient write data. - the potential risk of aborting an inflight SPI device interface transfer. When BUSY is '0', the mode of operation (XIP_MODE or MMIO_MODE) can be safely changed."]
pub type BusyR = crate::BitReader;
impl R {
    #[doc = "Bit 31 - Cache, cryptography, XIP, device interface or any other logic busy in the IP: '0': not busy '1': busy When BUSY is '0', the IP can be safely disabled without: - the potential loss of transient write data. - the potential risk of aborting an inflight SPI device interface transfer. When BUSY is '0', the mode of operation (XIP_MODE or MMIO_MODE) can be safely changed."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
