#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `BUSY` reader - Reflects the state of the IP: '0': Idle/no busy. '1': Busy: - Instruction is pending in the instruction FIFO. - Instruction is busy in a IP component (e.g. SHA1, SHA2, SHA3, DES, TDES, AES, CHACHA, ...). - Store FIFO is busy. - TR or PR command is busy."]
pub type BusyR = crate::BitReader;
impl R {
    #[doc = "Bit 31 - Reflects the state of the IP: '0': Idle/no busy. '1': Busy: - Instruction is pending in the instruction FIFO. - Instruction is busy in a IP component (e.g. SHA1, SHA2, SHA3, DES, TDES, AES, CHACHA, ...). - Store FIFO is busy. - TR or PR command is busy."]
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
