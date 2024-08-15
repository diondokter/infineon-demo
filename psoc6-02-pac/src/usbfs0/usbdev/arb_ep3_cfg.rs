#[doc = "Register `ARB_EP3_CFG` reader"]
pub type R = crate::R<ArbEp3CfgSpec>;
#[doc = "Register `ARB_EP3_CFG` writer"]
pub type W = crate::W<ArbEp3CfgSpec>;
#[doc = "Field `IN_DATA_RDY` reader - Indication that Endpoint Packet Data is Ready in Main memory"]
pub type InDataRdyR = crate::BitReader;
#[doc = "Field `IN_DATA_RDY` writer - Indication that Endpoint Packet Data is Ready in Main memory"]
pub type InDataRdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_REQ` reader - Manual DMA Request for a particular (1 to 8) endpoint; changing this field from 0 to 1 causes a DMA request to be generated."]
pub type DmaReqR = crate::BitReader;
#[doc = "Field `DMA_REQ` writer - Manual DMA Request for a particular (1 to 8) endpoint; changing this field from 0 to 1 causes a DMA request to be generated."]
pub type DmaReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrcBypass {
    #[doc = "0: No CRC bypass; CRC bytes will be written to memory and Termin will be generated for the CRC byte/s"]
    CrcNormal = 0,
    #[doc = "1: CRC Bypass Set; CRC bytes will not be written into memory and Termin will be generated for the last data byte/s"]
    CrcBypass = 1,
}
impl From<CrcBypass> for bool {
    #[inline(always)]
    fn from(variant: CrcBypass) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRC_BYPASS` reader - Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware"]
pub type CrcBypassR = crate::BitReader<CrcBypass>;
impl CrcBypassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CrcBypass {
        match self.bits {
            false => CrcBypass::CrcNormal,
            true => CrcBypass::CrcBypass,
        }
    }
    #[doc = "No CRC bypass; CRC bytes will be written to memory and Termin will be generated for the CRC byte/s"]
    #[inline(always)]
    pub fn is_crc_normal(&self) -> bool {
        *self == CrcBypass::CrcNormal
    }
    #[doc = "CRC Bypass Set; CRC bytes will not be written into memory and Termin will be generated for the last data byte/s"]
    #[inline(always)]
    pub fn is_crc_bypass(&self) -> bool {
        *self == CrcBypass::CrcBypass
    }
}
#[doc = "Field `CRC_BYPASS` writer - Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware"]
pub type CrcBypassW<'a, REG> = crate::BitWriter<'a, REG, CrcBypass>;
impl<'a, REG> CrcBypassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No CRC bypass; CRC bytes will be written to memory and Termin will be generated for the CRC byte/s"]
    #[inline(always)]
    pub fn crc_normal(self) -> &'a mut crate::W<REG> {
        self.variant(CrcBypass::CrcNormal)
    }
    #[doc = "CRC Bypass Set; CRC bytes will not be written into memory and Termin will be generated for the last data byte/s"]
    #[inline(always)]
    pub fn crc_bypass(self) -> &'a mut crate::W<REG> {
        self.variant(CrcBypass::CrcBypass)
    }
}
#[doc = "Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResetPtr {
    #[doc = "0: Do not Reset Pointer; Krypton Backward compatibility mode"]
    ResetKrypton = 0,
    #[doc = "1: Reset Pointer; recommended value for reduction of CPU Configuration Writes."]
    ResetNormal = 1,
}
impl From<ResetPtr> for bool {
    #[inline(always)]
    fn from(variant: ResetPtr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESET_PTR` reader - Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction."]
pub type ResetPtrR = crate::BitReader<ResetPtr>;
impl ResetPtrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ResetPtr {
        match self.bits {
            false => ResetPtr::ResetKrypton,
            true => ResetPtr::ResetNormal,
        }
    }
    #[doc = "Do not Reset Pointer; Krypton Backward compatibility mode"]
    #[inline(always)]
    pub fn is_reset_krypton(&self) -> bool {
        *self == ResetPtr::ResetKrypton
    }
    #[doc = "Reset Pointer; recommended value for reduction of CPU Configuration Writes."]
    #[inline(always)]
    pub fn is_reset_normal(&self) -> bool {
        *self == ResetPtr::ResetNormal
    }
}
#[doc = "Field `RESET_PTR` writer - Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction."]
pub type ResetPtrW<'a, REG> = crate::BitWriter<'a, REG, ResetPtr>;
impl<'a, REG> ResetPtrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Do not Reset Pointer; Krypton Backward compatibility mode"]
    #[inline(always)]
    pub fn reset_krypton(self) -> &'a mut crate::W<REG> {
        self.variant(ResetPtr::ResetKrypton)
    }
    #[doc = "Reset Pointer; recommended value for reduction of CPU Configuration Writes."]
    #[inline(always)]
    pub fn reset_normal(self) -> &'a mut crate::W<REG> {
        self.variant(ResetPtr::ResetNormal)
    }
}
impl R {
    #[doc = "Bit 0 - Indication that Endpoint Packet Data is Ready in Main memory"]
    #[inline(always)]
    pub fn in_data_rdy(&self) -> InDataRdyR {
        InDataRdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Manual DMA Request for a particular (1 to 8) endpoint; changing this field from 0 to 1 causes a DMA request to be generated."]
    #[inline(always)]
    pub fn dma_req(&self) -> DmaReqR {
        DmaReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware"]
    #[inline(always)]
    pub fn crc_bypass(&self) -> CrcBypassR {
        CrcBypassR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction."]
    #[inline(always)]
    pub fn reset_ptr(&self) -> ResetPtrR {
        ResetPtrR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indication that Endpoint Packet Data is Ready in Main memory"]
    #[inline(always)]
    #[must_use]
    pub fn in_data_rdy(&mut self) -> InDataRdyW<ArbEp3CfgSpec> {
        InDataRdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Manual DMA Request for a particular (1 to 8) endpoint; changing this field from 0 to 1 causes a DMA request to be generated."]
    #[inline(always)]
    #[must_use]
    pub fn dma_req(&mut self) -> DmaReqW<ArbEp3CfgSpec> {
        DmaReqW::new(self, 1)
    }
    #[doc = "Bit 2 - Configuration Setting to prevent CRC bytes from being written to memory and being read by firmware"]
    #[inline(always)]
    #[must_use]
    pub fn crc_bypass(&mut self) -> CrcBypassW<ArbEp3CfgSpec> {
        CrcBypassW::new(self, 2)
    }
    #[doc = "Bit 3 - Configuration Setting to Reset the RA and WA Pointers to their start values at the End of Packet transaction."]
    #[inline(always)]
    #[must_use]
    pub fn reset_ptr(&mut self) -> ResetPtrW<ArbEp3CfgSpec> {
        ResetPtrW::new(self, 3)
    }
}
#[doc = "Endpoint Configuration Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_ep3_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_ep3_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArbEp3CfgSpec;
impl crate::RegisterSpec for ArbEp3CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_ep3_cfg::R`](R) reader structure"]
impl crate::Readable for ArbEp3CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`arb_ep3_cfg::W`](W) writer structure"]
impl crate::Writable for ArbEp3CfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARB_EP3_CFG to value 0"]
impl crate::Resettable for ArbEp3CfgSpec {
    const RESET_VALUE: u32 = 0;
}
