#[doc = "Register `ARB_CFG` reader"]
pub type R = crate::R<ArbCfgSpec>;
#[doc = "Register `ARB_CFG` writer"]
pub type W = crate::W<ArbCfgSpec>;
#[doc = "Field `AUTO_MEM` reader - Enables Auto Memory Configuration. Manual memory configuration by default."]
pub type AutoMemR = crate::BitReader;
#[doc = "Field `AUTO_MEM` writer - Enables Auto Memory Configuration. Manual memory configuration by default."]
pub type AutoMemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DMA Access Configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DmaCfg {
    #[doc = "0: No DMA"]
    DmaNone = 0,
    #[doc = "1: Manual DMA"]
    DmaManual = 1,
    #[doc = "2: Auto DMA"]
    DmaAuto = 2,
}
impl From<DmaCfg> for u8 {
    #[inline(always)]
    fn from(variant: DmaCfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DmaCfg {
    type Ux = u8;
}
impl crate::IsEnum for DmaCfg {}
#[doc = "Field `DMA_CFG` reader - DMA Access Configuration."]
pub type DmaCfgR = crate::FieldReader<DmaCfg>;
impl DmaCfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DmaCfg> {
        match self.bits {
            0 => Some(DmaCfg::DmaNone),
            1 => Some(DmaCfg::DmaManual),
            2 => Some(DmaCfg::DmaAuto),
            _ => None,
        }
    }
    #[doc = "No DMA"]
    #[inline(always)]
    pub fn is_dma_none(&self) -> bool {
        *self == DmaCfg::DmaNone
    }
    #[doc = "Manual DMA"]
    #[inline(always)]
    pub fn is_dma_manual(&self) -> bool {
        *self == DmaCfg::DmaManual
    }
    #[doc = "Auto DMA"]
    #[inline(always)]
    pub fn is_dma_auto(&self) -> bool {
        *self == DmaCfg::DmaAuto
    }
}
#[doc = "Field `DMA_CFG` writer - DMA Access Configuration."]
pub type DmaCfgW<'a, REG> = crate::FieldWriter<'a, REG, 2, DmaCfg>;
impl<'a, REG> DmaCfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No DMA"]
    #[inline(always)]
    pub fn dma_none(self) -> &'a mut crate::W<REG> {
        self.variant(DmaCfg::DmaNone)
    }
    #[doc = "Manual DMA"]
    #[inline(always)]
    pub fn dma_manual(self) -> &'a mut crate::W<REG> {
        self.variant(DmaCfg::DmaManual)
    }
    #[doc = "Auto DMA"]
    #[inline(always)]
    pub fn dma_auto(self) -> &'a mut crate::W<REG> {
        self.variant(DmaCfg::DmaAuto)
    }
}
#[doc = "Field `CFG_CMP` reader - Register Configuration Complete Indication. Posedge is detected on this bit. Hence a 0 to 1 transition is required."]
pub type CfgCmpR = crate::BitReader;
#[doc = "Field `CFG_CMP` writer - Register Configuration Complete Indication. Posedge is detected on this bit. Hence a 0 to 1 transition is required."]
pub type CfgCmpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Enables Auto Memory Configuration. Manual memory configuration by default."]
    #[inline(always)]
    pub fn auto_mem(&self) -> AutoMemR {
        AutoMemR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - DMA Access Configuration."]
    #[inline(always)]
    pub fn dma_cfg(&self) -> DmaCfgR {
        DmaCfgR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Register Configuration Complete Indication. Posedge is detected on this bit. Hence a 0 to 1 transition is required."]
    #[inline(always)]
    pub fn cfg_cmp(&self) -> CfgCmpR {
        CfgCmpR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Enables Auto Memory Configuration. Manual memory configuration by default."]
    #[inline(always)]
    #[must_use]
    pub fn auto_mem(&mut self) -> AutoMemW<ArbCfgSpec> {
        AutoMemW::new(self, 4)
    }
    #[doc = "Bits 5:6 - DMA Access Configuration."]
    #[inline(always)]
    #[must_use]
    pub fn dma_cfg(&mut self) -> DmaCfgW<ArbCfgSpec> {
        DmaCfgW::new(self, 5)
    }
    #[doc = "Bit 7 - Register Configuration Complete Indication. Posedge is detected on this bit. Hence a 0 to 1 transition is required."]
    #[inline(always)]
    #[must_use]
    pub fn cfg_cmp(&mut self) -> CfgCmpW<ArbCfgSpec> {
        CfgCmpW::new(self, 7)
    }
}
#[doc = "Arbiter Configuration Register *1\n\nYou can [`read`](crate::Reg::read) this register and get [`arb_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arb_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ArbCfgSpec;
impl crate::RegisterSpec for ArbCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arb_cfg::R`](R) reader structure"]
impl crate::Readable for ArbCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`arb_cfg::W`](W) writer structure"]
impl crate::Writable for ArbCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ARB_CFG to value 0"]
impl crate::Resettable for ArbCfgSpec {
    const RESET_VALUE: u32 = 0;
}
