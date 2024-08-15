#[doc = "Register `HOST_ERR` reader"]
pub type R = crate::R<HostErrSpec>;
#[doc = "Register `HOST_ERR` writer"]
pub type W = crate::W<HostErrSpec>;
#[doc = "These flags indicate the status of a handshake packet to be sent or received. These flags are set to 'NULL' when no handshake occurs due to an error or when a SOF token has been ended with the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). These bits are updated when sending or receiving has been ended. Write '11' to set the status back to 'NULL', all other write values are ignored. Note: This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hs {
    #[doc = "0: Acknowledge Packet"]
    Ack = 0,
    #[doc = "1: Non-Acknowledge Packet"]
    Nak = 1,
    #[doc = "2: Stall Packet"]
    Stall = 2,
    #[doc = "3: Null Packet"]
    Null = 3,
}
impl From<Hs> for u8 {
    #[inline(always)]
    fn from(variant: Hs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hs {
    type Ux = u8;
}
impl crate::IsEnum for Hs {}
#[doc = "Field `HS` reader - These flags indicate the status of a handshake packet to be sent or received. These flags are set to 'NULL' when no handshake occurs due to an error or when a SOF token has been ended with the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). These bits are updated when sending or receiving has been ended. Write '11' to set the status back to 'NULL', all other write values are ignored. Note: This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type HsR = crate::FieldReader<Hs>;
impl HsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hs {
        match self.bits {
            0 => Hs::Ack,
            1 => Hs::Nak,
            2 => Hs::Stall,
            3 => Hs::Null,
            _ => unreachable!(),
        }
    }
    #[doc = "Acknowledge Packet"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == Hs::Ack
    }
    #[doc = "Non-Acknowledge Packet"]
    #[inline(always)]
    pub fn is_nak(&self) -> bool {
        *self == Hs::Nak
    }
    #[doc = "Stall Packet"]
    #[inline(always)]
    pub fn is_stall(&self) -> bool {
        *self == Hs::Stall
    }
    #[doc = "Null Packet"]
    #[inline(always)]
    pub fn is_null(&self) -> bool {
        *self == Hs::Null
    }
}
#[doc = "Field `HS` writer - These flags indicate the status of a handshake packet to be sent or received. These flags are set to 'NULL' when no handshake occurs due to an error or when a SOF token has been ended with the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). These bits are updated when sending or receiving has been ended. Write '11' to set the status back to 'NULL', all other write values are ignored. Note: This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type HsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Hs, crate::Safe>;
impl<'a, REG> HsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Acknowledge Packet"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut crate::W<REG> {
        self.variant(Hs::Ack)
    }
    #[doc = "Non-Acknowledge Packet"]
    #[inline(always)]
    pub fn nak(self) -> &'a mut crate::W<REG> {
        self.variant(Hs::Nak)
    }
    #[doc = "Stall Packet"]
    #[inline(always)]
    pub fn stall(self) -> &'a mut crate::W<REG> {
        self.variant(Hs::Stall)
    }
    #[doc = "Null Packet"]
    #[inline(always)]
    pub fn null(self) -> &'a mut crate::W<REG> {
        self.variant(Hs::Null)
    }
}
#[doc = "Field `STUFF` reader - If this bit is set to '1', it means that a bit stuffing error has been detected. When this bit is '0', it means that no error is detected. If a stuffing error is detected, bit5 (TOUT) of this register is also set to '1'. Write '1' to clear, a write of '0' is ignored. '0' : No stuffing error. '1' : Stuffing error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type StuffR = crate::BitReader;
#[doc = "Field `STUFF` writer - If this bit is set to '1', it means that a bit stuffing error has been detected. When this bit is '0', it means that no error is detected. If a stuffing error is detected, bit5 (TOUT) of this register is also set to '1'. Write '1' to clear, a write of '0' is ignored. '0' : No stuffing error. '1' : Stuffing error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type StuffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TGERR` reader - If this bit is set to '1', it means that the data does not match the TGGL data. When this bit is '0', it means that no error is detected. Write '1' to clear, a write of '0' is ignored. '0' : No toggle error. '1' : Toggle error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type TgerrR = crate::BitReader;
#[doc = "Field `TGERR` writer - If this bit is set to '1', it means that the data does not match the TGGL data. When this bit is '0', it means that no error is detected. Write '1' to clear, a write of '0' is ignored. '0' : No toggle error. '1' : Toggle error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type TgerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC` reader - If this bit is set to '1', it means that a CRC error is detected in the USB Host. When this bit is '0', it means that no error is detected. If a CRC error is detected, bit5 (TOUT) of this register is also set to '1'. Write '1' to clear, a write of '0' is ignored. '0' : No CRC error. '1' : CRC error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type CrcR = crate::BitReader;
#[doc = "Field `CRC` writer - If this bit is set to '1', it means that a CRC error is detected in the USB Host. When this bit is '0', it means that no error is detected. If a CRC error is detected, bit5 (TOUT) of this register is also set to '1'. Write '1' to clear, a write of '0' is ignored. '0' : No CRC error. '1' : CRC error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type CrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUT` reader - If this bit is set to '1', it means that no response is returned from the device within the specified time after a token has been sent in the USB Host. When this bit is '0', it means that no timeout is detected. Write '1' to clear, a write of '0' is ignored. '0' : No timeout. '1' : Timeout has detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type ToutR = crate::BitReader;
#[doc = "Field `TOUT` writer - If this bit is set to '1', it means that no response is returned from the device within the specified time after a token has been sent in the USB Host. When this bit is '0', it means that no timeout is detected. Write '1' to clear, a write of '0' is ignored. '0' : No timeout. '1' : Timeout has detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type ToutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RERR` reader - When this bit is set to '1', it means that the received data exceeds the specified maximum number of packets in the USB Host. If a receive error is detected, bit5 (TOUT) of this register is also set to '1'. When this bit is '0', it means that no error is detected. Write '1' to clear, a write of '0' is ignored. '0' : No receive error. '1' : Maximum packet receive error detected. - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type RerrR = crate::BitReader;
#[doc = "Field `RERR` writer - When this bit is set to '1', it means that the received data exceeds the specified maximum number of packets in the USB Host. If a receive error is detected, bit5 (TOUT) of this register is also set to '1'. When this bit is '0', it means that no error is detected. Write '1' to clear, a write of '0' is ignored. '0' : No receive error. '1' : Maximum packet receive error detected. - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type RerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSTSOF` reader - If this bit is set to '1', it means that the SOF token can't be sent in the USB Host because other token is in process. When this bit is '0', it means that SOF token was sent with no error. Write '1' to clear, a write of '0' is ignored. '0' : SOF sent without error. '1' : SOF error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type LstsofR = crate::BitReader;
#[doc = "Field `LSTSOF` writer - If this bit is set to '1', it means that the SOF token can't be sent in the USB Host because other token is in process. When this bit is '0', it means that SOF token was sent with no error. Write '1' to clear, a write of '0' is ignored. '0' : SOF sent without error. '1' : SOF error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
pub type LstsofW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - These flags indicate the status of a handshake packet to be sent or received. These flags are set to 'NULL' when no handshake occurs due to an error or when a SOF token has been ended with the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). These bits are updated when sending or receiving has been ended. Write '11' to set the status back to 'NULL', all other write values are ignored. Note: This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn hs(&self) -> HsR {
        HsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - If this bit is set to '1', it means that a bit stuffing error has been detected. When this bit is '0', it means that no error is detected. If a stuffing error is detected, bit5 (TOUT) of this register is also set to '1'. Write '1' to clear, a write of '0' is ignored. '0' : No stuffing error. '1' : Stuffing error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn stuff(&self) -> StuffR {
        StuffR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - If this bit is set to '1', it means that the data does not match the TGGL data. When this bit is '0', it means that no error is detected. Write '1' to clear, a write of '0' is ignored. '0' : No toggle error. '1' : Toggle error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn tgerr(&self) -> TgerrR {
        TgerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - If this bit is set to '1', it means that a CRC error is detected in the USB Host. When this bit is '0', it means that no error is detected. If a CRC error is detected, bit5 (TOUT) of this register is also set to '1'. Write '1' to clear, a write of '0' is ignored. '0' : No CRC error. '1' : CRC error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - If this bit is set to '1', it means that no response is returned from the device within the specified time after a token has been sent in the USB Host. When this bit is '0', it means that no timeout is detected. Write '1' to clear, a write of '0' is ignored. '0' : No timeout. '1' : Timeout has detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn tout(&self) -> ToutR {
        ToutR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When this bit is set to '1', it means that the received data exceeds the specified maximum number of packets in the USB Host. If a receive error is detected, bit5 (TOUT) of this register is also set to '1'. When this bit is '0', it means that no error is detected. Write '1' to clear, a write of '0' is ignored. '0' : No receive error. '1' : Maximum packet receive error detected. - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn rerr(&self) -> RerrR {
        RerrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - If this bit is set to '1', it means that the SOF token can't be sent in the USB Host because other token is in process. When this bit is '0', it means that SOF token was sent with no error. Write '1' to clear, a write of '0' is ignored. '0' : SOF sent without error. '1' : SOF error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    pub fn lstsof(&self) -> LstsofR {
        LstsofR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - These flags indicate the status of a handshake packet to be sent or received. These flags are set to 'NULL' when no handshake occurs due to an error or when a SOF token has been ended with the TKNEN bit of the Host Token Endpoint Register (HOST_TOKEN). These bits are updated when sending or receiving has been ended. Write '11' to set the status back to 'NULL', all other write values are ignored. Note: This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn hs(&mut self) -> HsW<HostErrSpec> {
        HsW::new(self, 0)
    }
    #[doc = "Bit 2 - If this bit is set to '1', it means that a bit stuffing error has been detected. When this bit is '0', it means that no error is detected. If a stuffing error is detected, bit5 (TOUT) of this register is also set to '1'. Write '1' to clear, a write of '0' is ignored. '0' : No stuffing error. '1' : Stuffing error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn stuff(&mut self) -> StuffW<HostErrSpec> {
        StuffW::new(self, 2)
    }
    #[doc = "Bit 3 - If this bit is set to '1', it means that the data does not match the TGGL data. When this bit is '0', it means that no error is detected. Write '1' to clear, a write of '0' is ignored. '0' : No toggle error. '1' : Toggle error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn tgerr(&mut self) -> TgerrW<HostErrSpec> {
        TgerrW::new(self, 3)
    }
    #[doc = "Bit 4 - If this bit is set to '1', it means that a CRC error is detected in the USB Host. When this bit is '0', it means that no error is detected. If a CRC error is detected, bit5 (TOUT) of this register is also set to '1'. Write '1' to clear, a write of '0' is ignored. '0' : No CRC error. '1' : CRC error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CrcW<HostErrSpec> {
        CrcW::new(self, 4)
    }
    #[doc = "Bit 5 - If this bit is set to '1', it means that no response is returned from the device within the specified time after a token has been sent in the USB Host. When this bit is '0', it means that no timeout is detected. Write '1' to clear, a write of '0' is ignored. '0' : No timeout. '1' : Timeout has detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn tout(&mut self) -> ToutW<HostErrSpec> {
        ToutW::new(self, 5)
    }
    #[doc = "Bit 6 - When this bit is set to '1', it means that the received data exceeds the specified maximum number of packets in the USB Host. If a receive error is detected, bit5 (TOUT) of this register is also set to '1'. When this bit is '0', it means that no error is detected. Write '1' to clear, a write of '0' is ignored. '0' : No receive error. '1' : Maximum packet receive error detected. - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn rerr(&mut self) -> RerrW<HostErrSpec> {
        RerrW::new(self, 6)
    }
    #[doc = "Bit 7 - If this bit is set to '1', it means that the SOF token can't be sent in the USB Host because other token is in process. When this bit is '0', it means that SOF token was sent with no error. Write '1' to clear, a write of '0' is ignored. '0' : SOF sent without error. '1' : SOF error detected. Note: - This bit is set to the default value when the RST bit of the Host Control 1 Register (HOST_CTL1) is set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn lstsof(&mut self) -> LstsofW<HostErrSpec> {
        LstsofW::new(self, 7)
    }
}
#[doc = "Host Error Status Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`host_err::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_err::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HostErrSpec;
impl crate::RegisterSpec for HostErrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_err::R`](R) reader structure"]
impl crate::Readable for HostErrSpec {}
#[doc = "`write(|w| ..)` method takes [`host_err::W`](W) writer structure"]
impl crate::Writable for HostErrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_ERR to value 0x03"]
impl crate::Resettable for HostErrSpec {
    const RESET_VALUE: u32 = 0x03;
}
