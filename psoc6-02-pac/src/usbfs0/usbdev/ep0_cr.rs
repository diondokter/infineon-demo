#[doc = "Register `EP0_CR` reader"]
pub type R = crate::R<Ep0CrSpec>;
#[doc = "Register `EP0_CR` writer"]
pub type W = crate::W<Ep0CrSpec>;
#[doc = "The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Ignore all USB traffic to this endpoint"]
    Disable = 0,
    #[doc = "1: SETUP: Accept IN: NAK OUT: NAK"]
    NakInout = 1,
    #[doc = "2: SETUP: Accept IN: STALL OUT: ACK 0B tokens, NAK others"]
    StatusOutOnly = 2,
    #[doc = "3: SETUP: Accept IN: STALL OUT: STALL"]
    StallInout = 3,
    #[doc = "5: SETUP: Ignore IN: Ignore OUT: Accept Isochronous OUT token"]
    IsoOut = 5,
    #[doc = "6: SETUP: Accept IN: Respond with 0B data OUT: Stall"]
    StatusInOnly = 6,
    #[doc = "7: SETUP: Ignore IN: Accept Isochronous IN token OUT: Ignore"]
    IsoIn = 7,
    #[doc = "8: SETUP: Ignore IN: Ignore OUT: NAK"]
    NakOut = 8,
    #[doc = "9: SETUP: Ignore IN: Ignore OUT: Accept data and ACK if STALL=0, STALL otherwise. Change to MODE=8 after one succesfull OUT token."]
    AckOut = 9,
    #[doc = "11: SETUP: Accept IN: Respond with 0B data OUT: Accept data"]
    AckOutStatusIn = 11,
    #[doc = "12: SETUP: Ignore IN: NAK OUT: Ignore"]
    NakIn = 12,
    #[doc = "13: SETUP: Ignore IN: Respond to IN with data if STALL=0, STALL otherwise OUT: Ignore"]
    AckIn = 13,
    #[doc = "15: SETUP: Accept IN: Respond to IN with data OUT: ACK 0B tokens, NAK others"]
    AckInStatusOut = 15,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            0 => Some(Mode::Disable),
            1 => Some(Mode::NakInout),
            2 => Some(Mode::StatusOutOnly),
            3 => Some(Mode::StallInout),
            5 => Some(Mode::IsoOut),
            6 => Some(Mode::StatusInOnly),
            7 => Some(Mode::IsoIn),
            8 => Some(Mode::NakOut),
            9 => Some(Mode::AckOut),
            11 => Some(Mode::AckOutStatusIn),
            12 => Some(Mode::NakIn),
            13 => Some(Mode::AckIn),
            15 => Some(Mode::AckInStatusOut),
            _ => None,
        }
    }
    #[doc = "Ignore all USB traffic to this endpoint"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Mode::Disable
    }
    #[doc = "SETUP: Accept IN: NAK OUT: NAK"]
    #[inline(always)]
    pub fn is_nak_inout(&self) -> bool {
        *self == Mode::NakInout
    }
    #[doc = "SETUP: Accept IN: STALL OUT: ACK 0B tokens, NAK others"]
    #[inline(always)]
    pub fn is_status_out_only(&self) -> bool {
        *self == Mode::StatusOutOnly
    }
    #[doc = "SETUP: Accept IN: STALL OUT: STALL"]
    #[inline(always)]
    pub fn is_stall_inout(&self) -> bool {
        *self == Mode::StallInout
    }
    #[doc = "SETUP: Ignore IN: Ignore OUT: Accept Isochronous OUT token"]
    #[inline(always)]
    pub fn is_iso_out(&self) -> bool {
        *self == Mode::IsoOut
    }
    #[doc = "SETUP: Accept IN: Respond with 0B data OUT: Stall"]
    #[inline(always)]
    pub fn is_status_in_only(&self) -> bool {
        *self == Mode::StatusInOnly
    }
    #[doc = "SETUP: Ignore IN: Accept Isochronous IN token OUT: Ignore"]
    #[inline(always)]
    pub fn is_iso_in(&self) -> bool {
        *self == Mode::IsoIn
    }
    #[doc = "SETUP: Ignore IN: Ignore OUT: NAK"]
    #[inline(always)]
    pub fn is_nak_out(&self) -> bool {
        *self == Mode::NakOut
    }
    #[doc = "SETUP: Ignore IN: Ignore OUT: Accept data and ACK if STALL=0, STALL otherwise. Change to MODE=8 after one succesfull OUT token."]
    #[inline(always)]
    pub fn is_ack_out(&self) -> bool {
        *self == Mode::AckOut
    }
    #[doc = "SETUP: Accept IN: Respond with 0B data OUT: Accept data"]
    #[inline(always)]
    pub fn is_ack_out_status_in(&self) -> bool {
        *self == Mode::AckOutStatusIn
    }
    #[doc = "SETUP: Ignore IN: NAK OUT: Ignore"]
    #[inline(always)]
    pub fn is_nak_in(&self) -> bool {
        *self == Mode::NakIn
    }
    #[doc = "SETUP: Ignore IN: Respond to IN with data if STALL=0, STALL otherwise OUT: Ignore"]
    #[inline(always)]
    pub fn is_ack_in(&self) -> bool {
        *self == Mode::AckIn
    }
    #[doc = "SETUP: Accept IN: Respond to IN with data OUT: ACK 0B tokens, NAK others"]
    #[inline(always)]
    pub fn is_ack_in_status_out(&self) -> bool {
        *self == Mode::AckInStatusOut
    }
}
#[doc = "Field `MODE` writer - The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Ignore all USB traffic to this endpoint"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Disable)
    }
    #[doc = "SETUP: Accept IN: NAK OUT: NAK"]
    #[inline(always)]
    pub fn nak_inout(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::NakInout)
    }
    #[doc = "SETUP: Accept IN: STALL OUT: ACK 0B tokens, NAK others"]
    #[inline(always)]
    pub fn status_out_only(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::StatusOutOnly)
    }
    #[doc = "SETUP: Accept IN: STALL OUT: STALL"]
    #[inline(always)]
    pub fn stall_inout(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::StallInout)
    }
    #[doc = "SETUP: Ignore IN: Ignore OUT: Accept Isochronous OUT token"]
    #[inline(always)]
    pub fn iso_out(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::IsoOut)
    }
    #[doc = "SETUP: Accept IN: Respond with 0B data OUT: Stall"]
    #[inline(always)]
    pub fn status_in_only(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::StatusInOnly)
    }
    #[doc = "SETUP: Ignore IN: Accept Isochronous IN token OUT: Ignore"]
    #[inline(always)]
    pub fn iso_in(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::IsoIn)
    }
    #[doc = "SETUP: Ignore IN: Ignore OUT: NAK"]
    #[inline(always)]
    pub fn nak_out(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::NakOut)
    }
    #[doc = "SETUP: Ignore IN: Ignore OUT: Accept data and ACK if STALL=0, STALL otherwise. Change to MODE=8 after one succesfull OUT token."]
    #[inline(always)]
    pub fn ack_out(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::AckOut)
    }
    #[doc = "SETUP: Accept IN: Respond with 0B data OUT: Accept data"]
    #[inline(always)]
    pub fn ack_out_status_in(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::AckOutStatusIn)
    }
    #[doc = "SETUP: Ignore IN: NAK OUT: Ignore"]
    #[inline(always)]
    pub fn nak_in(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::NakIn)
    }
    #[doc = "SETUP: Ignore IN: Respond to IN with data if STALL=0, STALL otherwise OUT: Ignore"]
    #[inline(always)]
    pub fn ack_in(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::AckIn)
    }
    #[doc = "SETUP: Accept IN: Respond to IN with data OUT: ACK 0B tokens, NAK others"]
    #[inline(always)]
    pub fn ack_in_status_out(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::AckInStatusOut)
    }
}
#[doc = "The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AckedTxn {
    #[doc = "0: No ACK'd transactions since bit was last cleared."]
    AckedNo = 0,
    #[doc = "1: Indicates a transaction ended with an ACK."]
    AckedYes = 1,
}
impl From<AckedTxn> for bool {
    #[inline(always)]
    fn from(variant: AckedTxn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACKED_TXN` reader - The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
pub type AckedTxnR = crate::BitReader<AckedTxn>;
impl AckedTxnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AckedTxn {
        match self.bits {
            false => AckedTxn::AckedNo,
            true => AckedTxn::AckedYes,
        }
    }
    #[doc = "No ACK'd transactions since bit was last cleared."]
    #[inline(always)]
    pub fn is_acked_no(&self) -> bool {
        *self == AckedTxn::AckedNo
    }
    #[doc = "Indicates a transaction ended with an ACK."]
    #[inline(always)]
    pub fn is_acked_yes(&self) -> bool {
        *self == AckedTxn::AckedYes
    }
}
#[doc = "Field `ACKED_TXN` writer - The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
pub type AckedTxnW<'a, REG> = crate::BitWriter<'a, REG, AckedTxn>;
impl<'a, REG> AckedTxnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No ACK'd transactions since bit was last cleared."]
    #[inline(always)]
    pub fn acked_no(self) -> &'a mut crate::W<REG> {
        self.variant(AckedTxn::AckedNo)
    }
    #[doc = "Indicates a transaction ended with an ACK."]
    #[inline(always)]
    pub fn acked_yes(self) -> &'a mut crate::W<REG> {
        self.variant(AckedTxn::AckedYes)
    }
}
#[doc = "Field `OUT_RCVD` reader - When set this bit indicates a valid OUT packet has been received and ACKed. This bit is updated to '1' after the last received packet in an OUT transaction. When clear this bit indicates no OUT received. It is cleared by any writes to the register."]
pub type OutRcvdR = crate::BitReader;
#[doc = "Field `OUT_RCVD` writer - When set this bit indicates a valid OUT packet has been received and ACKed. This bit is updated to '1' after the last received packet in an OUT transaction. When clear this bit indicates no OUT received. It is cleared by any writes to the register."]
pub type OutRcvdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_RCVD` reader - When set this bit indicates a valid IN packet has been received. This bit is updated to '1' after the host acknowledges an IN data packet. When clear this bit indicates either no IN has been received or that the host did not acknowledge the IN data by sending ACK handshake. It is cleared by any writes to the register."]
pub type InRcvdR = crate::BitReader;
#[doc = "Field `IN_RCVD` writer - When set this bit indicates a valid IN packet has been received. This bit is updated to '1' after the host acknowledges an IN data packet. When clear this bit indicates either no IN has been received or that the host did not acknowledge the IN data by sending ACK handshake. It is cleared by any writes to the register."]
pub type InRcvdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SETUP_RCVD` reader - When set this bit indicates a valid SETUP packet was received and ACKed. This bit is forced HIGH from the start of the data packet phase of the SETUP transaction until the start of the ACK packet returned by the SIE. The CPU is prevented from clearing this bit during this interval. After this interval the bit will remain set until cleared by firmware. While this bit is set to '1' the CPU cannot write to the EP0_DRx registers. This prevents firmware from overwriting an incoming SETUP transaction before firmware has a chance to read the SETUP data. This bit is cleared by any non-locked writes to the register."]
pub type SetupRcvdR = crate::BitReader;
#[doc = "Field `SETUP_RCVD` writer - When set this bit indicates a valid SETUP packet was received and ACKed. This bit is forced HIGH from the start of the data packet phase of the SETUP transaction until the start of the ACK packet returned by the SIE. The CPU is prevented from clearing this bit during this interval. After this interval the bit will remain set until cleared by firmware. While this bit is set to '1' the CPU cannot write to the EP0_DRx registers. This prevents firmware from overwriting an incoming SETUP transaction before firmware has a chance to read the SETUP data. This bit is cleared by any non-locked writes to the register."]
pub type SetupRcvdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub fn acked_txn(&self) -> AckedTxnR {
        AckedTxnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When set this bit indicates a valid OUT packet has been received and ACKed. This bit is updated to '1' after the last received packet in an OUT transaction. When clear this bit indicates no OUT received. It is cleared by any writes to the register."]
    #[inline(always)]
    pub fn out_rcvd(&self) -> OutRcvdR {
        OutRcvdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When set this bit indicates a valid IN packet has been received. This bit is updated to '1' after the host acknowledges an IN data packet. When clear this bit indicates either no IN has been received or that the host did not acknowledge the IN data by sending ACK handshake. It is cleared by any writes to the register."]
    #[inline(always)]
    pub fn in_rcvd(&self) -> InRcvdR {
        InRcvdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When set this bit indicates a valid SETUP packet was received and ACKed. This bit is forced HIGH from the start of the data packet phase of the SETUP transaction until the start of the ACK packet returned by the SIE. The CPU is prevented from clearing this bit during this interval. After this interval the bit will remain set until cleared by firmware. While this bit is set to '1' the CPU cannot write to the EP0_DRx registers. This prevents firmware from overwriting an incoming SETUP transaction before firmware has a chance to read the SETUP data. This bit is cleared by any non-locked writes to the register."]
    #[inline(always)]
    pub fn setup_rcvd(&self) -> SetupRcvdR {
        SetupRcvdR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<Ep0CrSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 4 - The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
    #[inline(always)]
    #[must_use]
    pub fn acked_txn(&mut self) -> AckedTxnW<Ep0CrSpec> {
        AckedTxnW::new(self, 4)
    }
    #[doc = "Bit 5 - When set this bit indicates a valid OUT packet has been received and ACKed. This bit is updated to '1' after the last received packet in an OUT transaction. When clear this bit indicates no OUT received. It is cleared by any writes to the register."]
    #[inline(always)]
    #[must_use]
    pub fn out_rcvd(&mut self) -> OutRcvdW<Ep0CrSpec> {
        OutRcvdW::new(self, 5)
    }
    #[doc = "Bit 6 - When set this bit indicates a valid IN packet has been received. This bit is updated to '1' after the host acknowledges an IN data packet. When clear this bit indicates either no IN has been received or that the host did not acknowledge the IN data by sending ACK handshake. It is cleared by any writes to the register."]
    #[inline(always)]
    #[must_use]
    pub fn in_rcvd(&mut self) -> InRcvdW<Ep0CrSpec> {
        InRcvdW::new(self, 6)
    }
    #[doc = "Bit 7 - When set this bit indicates a valid SETUP packet was received and ACKed. This bit is forced HIGH from the start of the data packet phase of the SETUP transaction until the start of the ACK packet returned by the SIE. The CPU is prevented from clearing this bit during this interval. After this interval the bit will remain set until cleared by firmware. While this bit is set to '1' the CPU cannot write to the EP0_DRx registers. This prevents firmware from overwriting an incoming SETUP transaction before firmware has a chance to read the SETUP data. This bit is cleared by any non-locked writes to the register."]
    #[inline(always)]
    #[must_use]
    pub fn setup_rcvd(&mut self) -> SetupRcvdW<Ep0CrSpec> {
        SetupRcvdW::new(self, 7)
    }
}
#[doc = "Endpoint0 control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep0_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep0_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep0CrSpec;
impl crate::RegisterSpec for Ep0CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep0_cr::R`](R) reader structure"]
impl crate::Readable for Ep0CrSpec {}
#[doc = "`write(|w| ..)` method takes [`ep0_cr::W`](W) writer structure"]
impl crate::Writable for Ep0CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EP0_CR to value 0"]
impl crate::Resettable for Ep0CrSpec {
    const RESET_VALUE: u32 = 0;
}
