#[doc = "Register `CFG_IN` reader"]
pub type R = crate::R<CfgInSpec>;
#[doc = "Register `CFG_IN` writer"]
pub type W = crate::W<CfgInSpec>;
#[doc = "Configures the pin 0 input buffer mode (trip points and hysteresis)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VtripSel0_0 {
    #[doc = "0: PSoC 6:: Input buffer compatible with CMOS and I2C interfaces Traveo II: Full encoding is shown in CFG_IN_AUTOLVL.VTRIP_SEL0_1"]
    Cmos = 0,
    #[doc = "1: PSoC 6:: Input buffer compatible with TTL and MediaLB interfaces Traveo II: full encoding is shown in CFG_IN_AUTOLVL.VTRIP_SEL0_1"]
    Ttl = 1,
}
impl From<VtripSel0_0> for bool {
    #[inline(always)]
    fn from(variant: VtripSel0_0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VTRIP_SEL0_0` reader - Configures the pin 0 input buffer mode (trip points and hysteresis)"]
pub type VtripSel0_0R = crate::BitReader<VtripSel0_0>;
impl VtripSel0_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VtripSel0_0 {
        match self.bits {
            false => VtripSel0_0::Cmos,
            true => VtripSel0_0::Ttl,
        }
    }
    #[doc = "PSoC 6:: Input buffer compatible with CMOS and I2C interfaces Traveo II: Full encoding is shown in CFG_IN_AUTOLVL.VTRIP_SEL0_1"]
    #[inline(always)]
    pub fn is_cmos(&self) -> bool {
        *self == VtripSel0_0::Cmos
    }
    #[doc = "PSoC 6:: Input buffer compatible with TTL and MediaLB interfaces Traveo II: full encoding is shown in CFG_IN_AUTOLVL.VTRIP_SEL0_1"]
    #[inline(always)]
    pub fn is_ttl(&self) -> bool {
        *self == VtripSel0_0::Ttl
    }
}
#[doc = "Field `VTRIP_SEL0_0` writer - Configures the pin 0 input buffer mode (trip points and hysteresis)"]
pub type VtripSel0_0W<'a, REG> = crate::BitWriter<'a, REG, VtripSel0_0>;
impl<'a, REG> VtripSel0_0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PSoC 6:: Input buffer compatible with CMOS and I2C interfaces Traveo II: Full encoding is shown in CFG_IN_AUTOLVL.VTRIP_SEL0_1"]
    #[inline(always)]
    pub fn cmos(self) -> &'a mut crate::W<REG> {
        self.variant(VtripSel0_0::Cmos)
    }
    #[doc = "PSoC 6:: Input buffer compatible with TTL and MediaLB interfaces Traveo II: full encoding is shown in CFG_IN_AUTOLVL.VTRIP_SEL0_1"]
    #[inline(always)]
    pub fn ttl(self) -> &'a mut crate::W<REG> {
        self.variant(VtripSel0_0::Ttl)
    }
}
#[doc = "Field `VTRIP_SEL1_0` reader - Configures the pin 1 input buffer mode (trip points and hysteresis)"]
pub type VtripSel1_0R = crate::BitReader;
#[doc = "Field `VTRIP_SEL1_0` writer - Configures the pin 1 input buffer mode (trip points and hysteresis)"]
pub type VtripSel1_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL2_0` reader - Configures the pin 2 input buffer mode (trip points and hysteresis)"]
pub type VtripSel2_0R = crate::BitReader;
#[doc = "Field `VTRIP_SEL2_0` writer - Configures the pin 2 input buffer mode (trip points and hysteresis)"]
pub type VtripSel2_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL3_0` reader - Configures the pin 3 input buffer mode (trip points and hysteresis)"]
pub type VtripSel3_0R = crate::BitReader;
#[doc = "Field `VTRIP_SEL3_0` writer - Configures the pin 3 input buffer mode (trip points and hysteresis)"]
pub type VtripSel3_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL4_0` reader - Configures the pin 4 input buffer mode (trip points and hysteresis)"]
pub type VtripSel4_0R = crate::BitReader;
#[doc = "Field `VTRIP_SEL4_0` writer - Configures the pin 4 input buffer mode (trip points and hysteresis)"]
pub type VtripSel4_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL5_0` reader - Configures the pin 5 input buffer mode (trip points and hysteresis)"]
pub type VtripSel5_0R = crate::BitReader;
#[doc = "Field `VTRIP_SEL5_0` writer - Configures the pin 5 input buffer mode (trip points and hysteresis)"]
pub type VtripSel5_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL6_0` reader - Configures the pin 6 input buffer mode (trip points and hysteresis)"]
pub type VtripSel6_0R = crate::BitReader;
#[doc = "Field `VTRIP_SEL6_0` writer - Configures the pin 6 input buffer mode (trip points and hysteresis)"]
pub type VtripSel6_0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTRIP_SEL7_0` reader - Configures the pin 7 input buffer mode (trip points and hysteresis)"]
pub type VtripSel7_0R = crate::BitReader;
#[doc = "Field `VTRIP_SEL7_0` writer - Configures the pin 7 input buffer mode (trip points and hysteresis)"]
pub type VtripSel7_0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures the pin 0 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel0_0(&self) -> VtripSel0_0R {
        VtripSel0_0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures the pin 1 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel1_0(&self) -> VtripSel1_0R {
        VtripSel1_0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures the pin 2 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel2_0(&self) -> VtripSel2_0R {
        VtripSel2_0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures the pin 3 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel3_0(&self) -> VtripSel3_0R {
        VtripSel3_0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures the pin 4 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel4_0(&self) -> VtripSel4_0R {
        VtripSel4_0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures the pin 5 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel5_0(&self) -> VtripSel5_0R {
        VtripSel5_0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configures the pin 6 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel6_0(&self) -> VtripSel6_0R {
        VtripSel6_0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Configures the pin 7 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    pub fn vtrip_sel7_0(&self) -> VtripSel7_0R {
        VtripSel7_0R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Configures the pin 0 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel0_0(&mut self) -> VtripSel0_0W<CfgInSpec> {
        VtripSel0_0W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures the pin 1 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel1_0(&mut self) -> VtripSel1_0W<CfgInSpec> {
        VtripSel1_0W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures the pin 2 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel2_0(&mut self) -> VtripSel2_0W<CfgInSpec> {
        VtripSel2_0W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures the pin 3 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel3_0(&mut self) -> VtripSel3_0W<CfgInSpec> {
        VtripSel3_0W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures the pin 4 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel4_0(&mut self) -> VtripSel4_0W<CfgInSpec> {
        VtripSel4_0W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures the pin 5 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel5_0(&mut self) -> VtripSel5_0W<CfgInSpec> {
        VtripSel5_0W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures the pin 6 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel6_0(&mut self) -> VtripSel6_0W<CfgInSpec> {
        VtripSel6_0W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures the pin 7 input buffer mode (trip points and hysteresis)"]
    #[inline(always)]
    #[must_use]
    pub fn vtrip_sel7_0(&mut self) -> VtripSel7_0W<CfgInSpec> {
        VtripSel7_0W::new(self, 7)
    }
}
#[doc = "Port input buffer configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg_in::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg_in::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgInSpec;
impl crate::RegisterSpec for CfgInSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_in::R`](R) reader structure"]
impl crate::Readable for CfgInSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_in::W`](W) writer structure"]
impl crate::Writable for CfgInSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_IN to value 0"]
impl crate::Resettable for CfgInSpec {
    const RESET_VALUE: u32 = 0;
}
