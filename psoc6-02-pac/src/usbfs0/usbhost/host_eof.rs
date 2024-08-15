#[doc = "Register `HOST_EOF` reader"]
pub type R = crate::R<HostEofSpec>;
#[doc = "Register `HOST_EOF` writer"]
pub type W = crate::W<HostEofSpec>;
#[doc = "Field `EOF` reader - These bits are used to specify the time to disable token sending before transferring SOF. Specify the time with a margin, which is longer than the one-packet length. The time unit is the 1-bit transfer time. Setting example: MAXPKT = 64 bytes, full-speed mode (Token_length + packet_length + header + CRC)*7/6 + Turn_around_time =(34 bit + 546 bit)*7/6 + 36 bit = 712.7 bit Therefore, set 0x2C9. Note: - This bit is not reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type EofR = crate::FieldReader<u16>;
#[doc = "Field `EOF` writer - These bits are used to specify the time to disable token sending before transferring SOF. Specify the time with a margin, which is longer than the one-packet length. The time unit is the 1-bit transfer time. Setting example: MAXPKT = 64 bytes, full-speed mode (Token_length + packet_length + header + CRC)*7/6 + Turn_around_time =(34 bit + 546 bit)*7/6 + 36 bit = 712.7 bit Therefore, set 0x2C9. Note: - This bit is not reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type EofW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - These bits are used to specify the time to disable token sending before transferring SOF. Specify the time with a margin, which is longer than the one-packet length. The time unit is the 1-bit transfer time. Setting example: MAXPKT = 64 bytes, full-speed mode (Token_length + packet_length + header + CRC)*7/6 + Turn_around_time =(34 bit + 546 bit)*7/6 + 36 bit = 712.7 bit Therefore, set 0x2C9. Note: - This bit is not reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn eof(&self) -> EofR {
        EofR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - These bits are used to specify the time to disable token sending before transferring SOF. Specify the time with a margin, which is longer than the one-packet length. The time unit is the 1-bit transfer time. Setting example: MAXPKT = 64 bytes, full-speed mode (Token_length + packet_length + header + CRC)*7/6 + Turn_around_time =(34 bit + 546 bit)*7/6 + 36 bit = 712.7 bit Therefore, set 0x2C9. Note: - This bit is not reset to default even if the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn eof(&mut self) -> EofW<HostEofSpec> {
        EofW::new(self, 0)
    }
}
#[doc = "Host EOF Setup Register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_eof::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_eof::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostEofSpec;
impl crate::RegisterSpec for HostEofSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_eof::R`](R) reader structure"]
impl crate::Readable for HostEofSpec {}
#[doc = "`write(|w| ..)` method takes [`host_eof::W`](W) writer structure"]
impl crate::Writable for HostEofSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_EOF to value 0"]
impl crate::Resettable for HostEofSpec {
    const RESET_VALUE: u32 = 0;
}
