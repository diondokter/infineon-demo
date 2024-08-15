#[doc = "Register `LPM_CTL` reader"]
pub type R = crate::R<LpmCtlSpec>;
#[doc = "Register `LPM_CTL` writer"]
pub type W = crate::W<LpmCtlSpec>;
#[doc = "Field `LPM_EN` reader - LPM enable 0: Disabled, LPM token will not get a response (backward compatibility mode) 1: Enable, LPM token will get a handshake response (ACK, STALL, NYET or NAK) A STALL will be sent if the bLinkState is not 0001b A NYET, NAK or ACK response will be sent depending on the NYET_EN and LPM_ACK_RESP bits below"]
pub type LpmEnR = crate::BitReader;
#[doc = "Field `LPM_EN` writer - LPM enable 0: Disabled, LPM token will not get a response (backward compatibility mode) 1: Enable, LPM token will get a handshake response (ACK, STALL, NYET or NAK) A STALL will be sent if the bLinkState is not 0001b A NYET, NAK or ACK response will be sent depending on the NYET_EN and LPM_ACK_RESP bits below"]
pub type LpmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPM_ACK_RESP` reader - LPM ACK response enable (if LPM_EN=1), to allow firmware to refuse a low power request 0: a LPM token will get a NYET or NAK (depending on NYET_EN bit below) response and the device will NOT go to a low power mode 1: a LPM token will get an ACK response and the device will go to the requested low power mode"]
pub type LpmAckRespR = crate::BitReader;
#[doc = "Field `LPM_ACK_RESP` writer - LPM ACK response enable (if LPM_EN=1), to allow firmware to refuse a low power request 0: a LPM token will get a NYET or NAK (depending on NYET_EN bit below) response and the device will NOT go to a low power mode 1: a LPM token will get an ACK response and the device will go to the requested low power mode"]
pub type LpmAckRespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYET_EN` reader - Allow firmware to choose which response to use for an LPM token (LPM_EN=1) when the device is NOT ready to go to the requested low power mode (LPM_ACK_RESP=0). 0: a LPM token will get an NAK response (indicating a CRC error), the host is expected to repeat the LPM token. 1: a LPM token will get a NYET response"]
pub type NyetEnR = crate::BitReader;
#[doc = "Field `NYET_EN` writer - Allow firmware to choose which response to use for an LPM token (LPM_EN=1) when the device is NOT ready to go to the requested low power mode (LPM_ACK_RESP=0). 0: a LPM token will get an NAK response (indicating a CRC error), the host is expected to repeat the LPM token. 1: a LPM token will get a NYET response"]
pub type NyetEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUB_RESP` reader - Enable a STALL response for all undefined SubPIDs, i.e. other than LPM (0011b). If not enabled then there will be no response (Error) for the undefined SubPIDs."]
pub type SubRespR = crate::BitReader;
#[doc = "Field `SUB_RESP` writer - Enable a STALL response for all undefined SubPIDs, i.e. other than LPM (0011b). If not enabled then there will be no response (Error) for the undefined SubPIDs."]
pub type SubRespW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LPM enable 0: Disabled, LPM token will not get a response (backward compatibility mode) 1: Enable, LPM token will get a handshake response (ACK, STALL, NYET or NAK) A STALL will be sent if the bLinkState is not 0001b A NYET, NAK or ACK response will be sent depending on the NYET_EN and LPM_ACK_RESP bits below"]
    #[inline(always)]
    pub fn lpm_en(&self) -> LpmEnR {
        LpmEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPM ACK response enable (if LPM_EN=1), to allow firmware to refuse a low power request 0: a LPM token will get a NYET or NAK (depending on NYET_EN bit below) response and the device will NOT go to a low power mode 1: a LPM token will get an ACK response and the device will go to the requested low power mode"]
    #[inline(always)]
    pub fn lpm_ack_resp(&self) -> LpmAckRespR {
        LpmAckRespR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Allow firmware to choose which response to use for an LPM token (LPM_EN=1) when the device is NOT ready to go to the requested low power mode (LPM_ACK_RESP=0). 0: a LPM token will get an NAK response (indicating a CRC error), the host is expected to repeat the LPM token. 1: a LPM token will get a NYET response"]
    #[inline(always)]
    pub fn nyet_en(&self) -> NyetEnR {
        NyetEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable a STALL response for all undefined SubPIDs, i.e. other than LPM (0011b). If not enabled then there will be no response (Error) for the undefined SubPIDs."]
    #[inline(always)]
    pub fn sub_resp(&self) -> SubRespR {
        SubRespR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPM enable 0: Disabled, LPM token will not get a response (backward compatibility mode) 1: Enable, LPM token will get a handshake response (ACK, STALL, NYET or NAK) A STALL will be sent if the bLinkState is not 0001b A NYET, NAK or ACK response will be sent depending on the NYET_EN and LPM_ACK_RESP bits below"]
    #[inline(always)]
    #[must_use]
    pub fn lpm_en(&mut self) -> LpmEnW<LpmCtlSpec> {
        LpmEnW::new(self, 0)
    }
    #[doc = "Bit 1 - LPM ACK response enable (if LPM_EN=1), to allow firmware to refuse a low power request 0: a LPM token will get a NYET or NAK (depending on NYET_EN bit below) response and the device will NOT go to a low power mode 1: a LPM token will get an ACK response and the device will go to the requested low power mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpm_ack_resp(&mut self) -> LpmAckRespW<LpmCtlSpec> {
        LpmAckRespW::new(self, 1)
    }
    #[doc = "Bit 2 - Allow firmware to choose which response to use for an LPM token (LPM_EN=1) when the device is NOT ready to go to the requested low power mode (LPM_ACK_RESP=0). 0: a LPM token will get an NAK response (indicating a CRC error), the host is expected to repeat the LPM token. 1: a LPM token will get a NYET response"]
    #[inline(always)]
    #[must_use]
    pub fn nyet_en(&mut self) -> NyetEnW<LpmCtlSpec> {
        NyetEnW::new(self, 2)
    }
    #[doc = "Bit 4 - Enable a STALL response for all undefined SubPIDs, i.e. other than LPM (0011b). If not enabled then there will be no response (Error) for the undefined SubPIDs."]
    #[inline(always)]
    #[must_use]
    pub fn sub_resp(&mut self) -> SubRespW<LpmCtlSpec> {
        SubRespW::new(self, 4)
    }
}
#[doc = "LPM Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpm_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpm_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpmCtlSpec;
impl crate::RegisterSpec for LpmCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpm_ctl::R`](R) reader structure"]
impl crate::Readable for LpmCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`lpm_ctl::W`](W) writer structure"]
impl crate::Writable for LpmCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPM_CTL to value 0"]
impl crate::Resettable for LpmCtlSpec {
    const RESET_VALUE: u32 = 0;
}
