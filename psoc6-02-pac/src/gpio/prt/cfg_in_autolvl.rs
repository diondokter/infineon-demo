#[doc = "Register `CFG_IN_AUTOLVL` reader"]
pub type R = crate::R<CfgInAutolvlSpec>;
#[doc = "Register `CFG_IN_AUTOLVL` writer"]
pub type W = crate::W<CfgInAutolvlSpec>;
#[doc = "Configures the input buffer mode (trip points and hysteresis) for GPIO upper bit. Lower bit is still selected by CFG_IN.VTRIP_SEL0_0 field. This field is used along with CFG_IN.VTRIP_SEL0_0 field as below: {CFG_IN_AUTOLVL.VTRIP_SEL0_1,CFG_IN.VTRIP_SEL0_0}: 0,0: CMOS 0,1: TTL 1,0: input buffer is compatible with automotive. 1,1: input buffer is compatible with automotvie\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VtripSel0_1 {
    #[doc = "0: Input buffer compatible with CMOS/TTL interfaces as described in CFG_IN.VTRIP_SEL0_0."]
    CmosOrTtl = 0,
    #[doc = "1: Input buffer compatible with AUTO (elevated Vil) interfaces when used along with CFG_IN.VTRIP_SEL0_0."]
    Auto = 1,
}
impl From<VtripSel0_1> for bool {
    #[inline(always)]
    fn from(variant: VtripSel0_1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VTRIP_SEL0_1` reader - Configures the input buffer mode (trip points and hysteresis) for GPIO upper bit. Lower bit is still selected by CFG_IN.VTRIP_SEL0_0 field. This field is used along with CFG_IN.VTRIP_SEL0_0 field as below: {CFG_IN_AUTOLVL.VTRIP_SEL0_1,CFG_IN.VTRIP_SEL0_0}: 0,0: CMOS 0,1: TTL 1,0: input buffer is compatible with automotive. 1,1: input buffer is compatible with automotvie"]
pub type VtripSel0_1R = crate::BitReader<VtripSel0_1>;
impl VtripSel0_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VtripSel0_1 {
        match self.bits {
            false => VtripSel0_1::CmosOrTtl,
            true => VtripSel0_1::Auto,
        }
    }
    #[doc = "Input buffer compatible with CMOS/TTL interfaces as described in CFG_IN.VTRIP_SEL0_0."]
    #[inline(always)]
    pub fn is_cmos_or_ttl(&self) -> bool {
        *self == VtripSel0_1::CmosOrTtl
    }
    #[doc = "Input buffer compatible with AUTO (elevated Vil) interfaces when used along with CFG_IN.VTRIP_SEL0_0."]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == VtripSel0_1::Auto
    }
}
#[doc = "Field `VTRIP_SEL0_1` writer - Configures the input buffer mode (trip points and hysteresis) for GPIO upper bit. Lower bit is still selected by CFG_IN.VTRIP_SEL0_0 field. This field is used along with CFG_IN.VTRIP_SEL0_0 field as below: {CFG_IN_AUTOLVL.VTRIP_SEL0_1,CFG_IN.VTRIP_SEL0_0}: 0,0: CMOS 0,1: TTL 1,0: input buffer is compatible with automotive. 1,1: input buffer is compatible with automotvie"]
pub type VtripSel0_1W<'a, REG> = crate::BitWriter<'a, REG, VtripSel0_1>;
impl<'a, REG> VtripSel0_1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input buffer compatible with CMOS/TTL interfaces as described in CFG_IN.VTRIP_SEL0_0."]
    #[inline(always)]
    pub fn cmos_or_ttl(self) -> &'a mut crate::W<REG> {
        self.variant(VtripSel0_1::CmosOrTtl)
    }
    #[doc = "Input buffer compatible with AUTO (elevated Vil) interfaces when used along with CFG_IN.VTRIP_SEL0_0."]
    #[inline(always)]
    pub fn auto(self) -> &'a mut crate::W<REG> {
        self.variant(VtripSel0_1::Auto)
    }
}
#[doc = "Field `VTRIP_SEL1_1` reader - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub type VtripSel1_1R = crate::BitReader;
#[doc = "Field `VTRIP_SEL1_1` writer - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub type VtripSel1_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL2_1` reader - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub type VtripSel2_1R = crate::BitReader;
#[doc = "Field `VTRIP_SEL2_1` writer - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub type VtripSel2_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL3_1` reader - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub type VtripSel3_1R = crate::BitReader;
#[doc = "Field `VTRIP_SEL3_1` writer - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub type VtripSel3_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL4_1` reader - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub type VtripSel4_1R = crate::BitReader;
#[doc = "Field `VTRIP_SEL4_1` writer - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub type VtripSel4_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL5_1` reader - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub type VtripSel5_1R = crate::BitReader;
#[doc = "Field `VTRIP_SEL5_1` writer - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub type VtripSel5_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL6_1` reader - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub type VtripSel6_1R = crate::BitReader;
#[doc = "Field `VTRIP_SEL6_1` writer - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub type VtripSel6_1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL7_1` reader - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub type VtripSel7_1R = crate::BitReader;
#[doc = "Field `VTRIP_SEL7_1` writer - Input buffer compatible with automotive (elevated Vil) interfaces."]
pub type VtripSel7_1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures the input buffer mode (trip points and hysteresis) for GPIO upper bit. Lower bit is still selected by CFG_IN.VTRIP_SEL0_0 field. This field is used along with CFG_IN.VTRIP_SEL0_0 field as below: {CFG_IN_AUTOLVL.VTRIP_SEL0_1,CFG_IN.VTRIP_SEL0_0}: 0,0: CMOS 0,1: TTL 1,0: input buffer is compatible with automotive. 1,1: input buffer is compatible with automotvie"]
    #[inline(always)]
    pub fn vtrip_sel0_1(&self) -> VtripSel0_1R {
        VtripSel0_1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel1_1(&self) -> VtripSel1_1R {
        VtripSel1_1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel2_1(&self) -> VtripSel2_1R {
        VtripSel2_1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel3_1(&self) -> VtripSel3_1R {
        VtripSel3_1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel4_1(&self) -> VtripSel4_1R {
        VtripSel4_1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel5_1(&self) -> VtripSel5_1R {
        VtripSel5_1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel6_1(&self) -> VtripSel6_1R {
        VtripSel6_1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    pub fn vtrip_sel7_1(&self) -> VtripSel7_1R {
        VtripSel7_1R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configures the input buffer mode (trip points and hysteresis) for GPIO upper bit. Lower bit is still selected by CFG_IN.VTRIP_SEL0_0 field. This field is used along with CFG_IN.VTRIP_SEL0_0 field as below: {CFG_IN_AUTOLVL.VTRIP_SEL0_1,CFG_IN.VTRIP_SEL0_0}: 0,0: CMOS 0,1: TTL 1,0: input buffer is compatible with automotive. 1,1: input buffer is compatible with automotvie"]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel0_1(&mut self) -> VtripSel0_1W<CfgInAutolvlSpec> {
        VtripSel0_1W::new(self, 0)
    }
    #[doc = "Bit 1 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel1_1(&mut self) -> VtripSel1_1W<CfgInAutolvlSpec> {
        VtripSel1_1W::new(self, 1)
    }
    #[doc = "Bit 2 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel2_1(&mut self) -> VtripSel2_1W<CfgInAutolvlSpec> {
        VtripSel2_1W::new(self, 2)
    }
    #[doc = "Bit 3 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel3_1(&mut self) -> VtripSel3_1W<CfgInAutolvlSpec> {
        VtripSel3_1W::new(self, 3)
    }
    #[doc = "Bit 4 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel4_1(&mut self) -> VtripSel4_1W<CfgInAutolvlSpec> {
        VtripSel4_1W::new(self, 4)
    }
    #[doc = "Bit 5 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel5_1(&mut self) -> VtripSel5_1W<CfgInAutolvlSpec> {
        VtripSel5_1W::new(self, 5)
    }
    #[doc = "Bit 6 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel6_1(&mut self) -> VtripSel6_1W<CfgInAutolvlSpec> {
        VtripSel6_1W::new(self, 6)
    }
    #[doc = "Bit 7 - Input buffer compatible with automotive (elevated Vil) interfaces."]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel7_1(&mut self) -> VtripSel7_1W<CfgInAutolvlSpec> {
        VtripSel7_1W::new(self, 7)
    }
}
#[doc = "Port input buffer AUTOLVL configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_in_autolvl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_in_autolvl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgInAutolvlSpec;
impl crate::RegisterSpec for CfgInAutolvlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_in_autolvl::R`](R) reader structure"]
impl crate::Readable for CfgInAutolvlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_in_autolvl::W`](W) writer structure"]
impl crate::Writable for CfgInAutolvlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_IN_AUTOLVL to value 0"]
impl crate::Resettable for CfgInAutolvlSpec {
    const RESET_VALUE: u32 = 0;
}
