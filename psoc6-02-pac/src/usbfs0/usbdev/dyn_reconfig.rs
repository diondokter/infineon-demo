#[doc = "Register `DYN_RECONFIG` reader"]
pub type R = crate::R<DynReconfigSpec>;
#[doc = "Register `DYN_RECONFIG` writer"]
pub type W = crate::W<DynReconfigSpec>;
#[doc = "Field `DYN_CONFIG_EN` reader - This bit is used to enable the dynamic re-configuration for the selected EP. If set to 1, indicates the reconfiguration required for selected EP. Use 0 for EP1, 1 for EP2, etc."]
pub type DynConfigEnR = crate::BitReader;
#[doc = "Field `DYN_CONFIG_EN` writer - This bit is used to enable the dynamic re-configuration for the selected EP. If set to 1, indicates the reconfiguration required for selected EP. Use 0 for EP1, 1 for EP2, etc."]
pub type DynConfigEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DYN_RECONFIG_EPNO` reader - These bits indicates the EP number for which reconfiguration is required when dyn_config_en bit is set to 1."]
pub type DynReconfigEpnoR = crate::FieldReader;
#[doc = "Field `DYN_RECONFIG_EPNO` writer - These bits indicates the EP number for which reconfiguration is required when dyn_config_en bit is set to 1."]
pub type DynReconfigEpnoW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DYN_RECONFIG_RDY_STS` reader - This bit indicates the ready status for the dynamic reconfiguration, when set to 1, indicates the block is ready for reconfiguration."]
pub type DynReconfigRdyStsR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit is used to enable the dynamic re-configuration for the selected EP. If set to 1, indicates the reconfiguration required for selected EP. Use 0 for EP1, 1 for EP2, etc."]
    #[inline(always)]
    pub fn dyn_config_en(&self) -> DynConfigEnR {
        DynConfigEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - These bits indicates the EP number for which reconfiguration is required when dyn_config_en bit is set to 1."]
    #[inline(always)]
    pub fn dyn_reconfig_epno(&self) -> DynReconfigEpnoR {
        DynReconfigEpnoR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - This bit indicates the ready status for the dynamic reconfiguration, when set to 1, indicates the block is ready for reconfiguration."]
    #[inline(always)]
    pub fn dyn_reconfig_rdy_sts(&self) -> DynReconfigRdyStsR {
        DynReconfigRdyStsR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to enable the dynamic re-configuration for the selected EP. If set to 1, indicates the reconfiguration required for selected EP. Use 0 for EP1, 1 for EP2, etc."]
    #[inline(always)]
    #[must_use]
    pub fn dyn_config_en(&mut self) -> DynConfigEnW<DynReconfigSpec> {
        DynConfigEnW::new(self, 0)
    }
    #[doc = "Bits 1:3 - These bits indicates the EP number for which reconfiguration is required when dyn_config_en bit is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn dyn_reconfig_epno(&mut self) -> DynReconfigEpnoW<DynReconfigSpec> {
        DynReconfigEpnoW::new(self, 1)
    }
}
#[doc = "USB Dynamic reconfiguration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dyn_reconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dyn_reconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DynReconfigSpec;
impl crate::RegisterSpec for DynReconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dyn_reconfig::R`](R) reader structure"]
impl crate::Readable for DynReconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`dyn_reconfig::W`](W) writer structure"]
impl crate::Writable for DynReconfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DYN_RECONFIG to value 0"]
impl crate::Resettable for DynReconfigSpec {
    const RESET_VALUE: u32 = 0;
}
