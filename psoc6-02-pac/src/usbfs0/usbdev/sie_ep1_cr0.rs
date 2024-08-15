#[doc = "Register `SIE_EP1_CR0` reader"]
pub type R = crate::R<SieEp1Cr0Spec>;
#[doc = "Register `SIE_EP1_CR0` writer"]
pub type W = crate::W<SieEp1Cr0Spec>;
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
#[doc = "Field `NAK_INT_EN` reader - When set this bit causes an endpoint interrupt to be generated even when a transfer completes with a NAK."]
pub type NakIntEnR = crate::BitReader;
#[doc = "Field `NAK_INT_EN` writer - When set this bit causes an endpoint interrupt to be generated even when a transfer completes with a NAK."]
pub type NakIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_IN_TXN` reader - The Error in transaction bit is set whenever an error is detected. For an IN transaction, this indicates a no response from HOST scenario. For an OUT transaction, this represents an RxErr (PID error/ CRC error/ bit-stuff error scenario). This bit is cleared by any writes to the register."]
pub type ErrInTxnR = crate::BitReader;
#[doc = "Field `ERR_IN_TXN` writer - The Error in transaction bit is set whenever an error is detected. For an IN transaction, this indicates a no response from HOST scenario. For an OUT transaction, this represents an RxErr (PID error/ CRC error/ bit-stuff error scenario). This bit is cleared by any writes to the register."]
pub type ErrInTxnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - When this bit is set the SIE stalls an OUT packet if the Mode bits are set to ACK-OUT. The SIE stalls an IN packet if the mode bits are set to ACK-IN. This bit must be clear for all other modes."]
pub type StallR = crate::BitReader;
#[doc = "Field `STALL` writer - When this bit is set the SIE stalls an OUT packet if the Mode bits are set to ACK-OUT. The SIE stalls an IN packet if the mode bits are set to ACK-IN. This bit must be clear for all other modes."]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 5 - When set this bit causes an endpoint interrupt to be generated even when a transfer completes with a NAK."]
    #[inline(always)]
    pub fn nak_int_en(&self) -> NakIntEnR {
        NakIntEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The Error in transaction bit is set whenever an error is detected. For an IN transaction, this indicates a no response from HOST scenario. For an OUT transaction, this represents an RxErr (PID error/ CRC error/ bit-stuff error scenario). This bit is cleared by any writes to the register."]
    #[inline(always)]
    pub fn err_in_txn(&self) -> ErrInTxnR {
        ErrInTxnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When this bit is set the SIE stalls an OUT packet if the Mode bits are set to ACK-OUT. The SIE stalls an IN packet if the mode bits are set to ACK-IN. This bit must be clear for all other modes."]
    #[inline(always)]
    pub fn stall(&self) -> StallR {
        StallR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - The mode controls how the USB SIE responds to traffic and how the USB SIE changes the mode of that endpoint as a result of host packets to the endpoint."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<SieEp1Cr0Spec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 4 - The ACK'd transaction bit is set whenever the SIE engages in a transaction to the register's endpoint that completes with an ACK packet. This bit is cleared by any writes to the register."]
    #[inline(always)]
    #[must_use]
    pub fn acked_txn(&mut self) -> AckedTxnW<SieEp1Cr0Spec> {
        AckedTxnW::new(self, 4)
    }
    #[doc = "Bit 5 - When set this bit causes an endpoint interrupt to be generated even when a transfer completes with a NAK."]
    #[inline(always)]
    #[must_use]
    pub fn nak_int_en(&mut self) -> NakIntEnW<SieEp1Cr0Spec> {
        NakIntEnW::new(self, 5)
    }
    #[doc = "Bit 6 - The Error in transaction bit is set whenever an error is detected. For an IN transaction, this indicates a no response from HOST scenario. For an OUT transaction, this represents an RxErr (PID error/ CRC error/ bit-stuff error scenario). This bit is cleared by any writes to the register."]
    #[inline(always)]
    #[must_use]
    pub fn err_in_txn(&mut self) -> ErrInTxnW<SieEp1Cr0Spec> {
        ErrInTxnW::new(self, 6)
    }
    #[doc = "Bit 7 - When this bit is set the SIE stalls an OUT packet if the Mode bits are set to ACK-OUT. The SIE stalls an IN packet if the mode bits are set to ACK-IN. This bit must be clear for all other modes."]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> StallW<SieEp1Cr0Spec> {
        StallW::new(self, 7)
    }
}
#[doc = "Non-control endpoint's control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sie_ep1_cr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sie_ep1_cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SieEp1Cr0Spec;
impl crate::RegisterSpec for SieEp1Cr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sie_ep1_cr0::R`](R) reader structure"]
impl crate::Readable for SieEp1Cr0Spec {}
#[doc = "`write(|w| ..)` method takes [`sie_ep1_cr0::W`](W) writer structure"]
impl crate::Writable for SieEp1Cr0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SIE_EP1_CR0 to value 0"]
impl crate::Resettable for SieEp1Cr0Spec {
    const RESET_VALUE: u32 = 0;
}
