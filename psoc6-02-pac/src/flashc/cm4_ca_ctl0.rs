#[doc = "Register `CM4_CA_CTL0` reader"]
pub type R = crate::R<Cm4CaCtl0Spec>;
#[doc = "Register `CM4_CA_CTL0` writer"]
pub type W = crate::W<Cm4CaCtl0Spec>;
#[doc = "Field `RAM_ECC_EN` reader - See CM0_CA_CTL."]
pub type RamEccEnR = crate::BitReader;
#[doc = "Field `RAM_ECC_EN` writer - See CM0_CA_CTL."]
pub type RamEccEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM_ECC_INJ_EN` reader - See CM0_CA_CTL."]
pub type RamEccInjEnR = crate::BitReader;
#[doc = "Field `RAM_ECC_INJ_EN` writer - See CM0_CA_CTL."]
pub type RamEccInjEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAY` reader - See CM0_CA_CTL."]
pub type WayR = crate::FieldReader;
#[doc = "Field `WAY` writer - See CM0_CA_CTL."]
pub type WayW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SET_ADDR` reader - See CM0_CA_CTL."]
pub type SetAddrR = crate::FieldReader;
#[doc = "Field `SET_ADDR` writer - See CM0_CA_CTL."]
pub type SetAddrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PREF_EN` reader - See CM0_CA_CTL."]
pub type PrefEnR = crate::BitReader;
#[doc = "Field `PREF_EN` writer - See CM0_CA_CTL."]
pub type PrefEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CA_EN` reader - See CM0_CA_CTL."]
pub type CaEnR = crate::BitReader;
#[doc = "Field `CA_EN` writer - See CM0_CA_CTL."]
pub type CaEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - See CM0_CA_CTL."]
    #[inline(always)]
    pub fn ram_ecc_en(&self) -> RamEccEnR {
        RamEccEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - See CM0_CA_CTL."]
    #[inline(always)]
    pub fn ram_ecc_inj_en(&self) -> RamEccInjEnR {
        RamEccInjEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:17 - See CM0_CA_CTL."]
    #[inline(always)]
    pub fn way(&self) -> WayR {
        WayR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:26 - See CM0_CA_CTL."]
    #[inline(always)]
    pub fn set_addr(&self) -> SetAddrR {
        SetAddrR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 30 - See CM0_CA_CTL."]
    #[inline(always)]
    pub fn pref_en(&self) -> PrefEnR {
        PrefEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - See CM0_CA_CTL."]
    #[inline(always)]
    pub fn ca_en(&self) -> CaEnR {
        CaEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - See CM0_CA_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn ram_ecc_en(&mut self) -> RamEccEnW<Cm4CaCtl0Spec> {
        RamEccEnW::new(self, 0)
    }
    #[doc = "Bit 1 - See CM0_CA_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn ram_ecc_inj_en(&mut self) -> RamEccInjEnW<Cm4CaCtl0Spec> {
        RamEccInjEnW::new(self, 1)
    }
    #[doc = "Bits 16:17 - See CM0_CA_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn way(&mut self) -> WayW<Cm4CaCtl0Spec> {
        WayW::new(self, 16)
    }
    #[doc = "Bits 24:26 - See CM0_CA_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn set_addr(&mut self) -> SetAddrW<Cm4CaCtl0Spec> {
        SetAddrW::new(self, 24)
    }
    #[doc = "Bit 30 - See CM0_CA_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn pref_en(&mut self) -> PrefEnW<Cm4CaCtl0Spec> {
        PrefEnW::new(self, 30)
    }
    #[doc = "Bit 31 - See CM0_CA_CTL."]
    #[inline(always)]
    #[must_use]
    pub fn ca_en(&mut self) -> CaEnW<Cm4CaCtl0Spec> {
        CaEnW::new(self, 31)
    }
}
#[doc = "CM4 cache control\n\nYou can [`read`](crate::Reg::read) this register and get [`cm4_ca_ctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cm4_ca_ctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cm4CaCtl0Spec;
impl crate::RegisterSpec for Cm4CaCtl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cm4_ca_ctl0::R`](R) reader structure"]
impl crate::Readable for Cm4CaCtl0Spec {}
#[doc = "`write(|w| ..)` method takes [`cm4_ca_ctl0::W`](W) writer structure"]
impl crate::Writable for Cm4CaCtl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CM4_CA_CTL0 to value 0xc000_0001"]
impl crate::Resettable for Cm4CaCtl0Spec {
    const RESET_VALUE: u32 = 0xc000_0001;
}
