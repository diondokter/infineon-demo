#[doc = "Register `VDD_INTR` reader"]
pub type R = crate::R<VddIntrSpec>;
#[doc = "Register `VDD_INTR` writer"]
pub type W = crate::W<VddIntrSpec>;
#[doc = "Field `VDDIO_ACTIVE` reader - Supply state change detected. '0': No change to supply detected '1': Change to supply detected"]
pub type VddioActiveR = crate::FieldReader<u16>;
#[doc = "Field `VDDIO_ACTIVE` writer - Supply state change detected. '0': No change to supply detected '1': Change to supply detected"]
pub type VddioActiveW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `VDDA_ACTIVE` reader - Same as VDDIO_ACTIVE for the analog supply VDDA."]
pub type VddaActiveR = crate::BitReader;
#[doc = "Field `VDDA_ACTIVE` writer - Same as VDDIO_ACTIVE for the analog supply VDDA."]
pub type VddaActiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDD_ACTIVE` reader - The VDDD supply is always present during operation so a supply transition can not occur. This bit will always read back '1'."]
pub type VdddActiveR = crate::BitReader;
#[doc = "Field `VDDD_ACTIVE` writer - The VDDD supply is always present during operation so a supply transition can not occur. This bit will always read back '1'."]
pub type VdddActiveW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Supply state change detected. '0': No change to supply detected '1': Change to supply detected"]
    #[inline(always)]
    pub fn vddio_active(&self) -> VddioActiveR {
        VddioActiveR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 30 - Same as VDDIO_ACTIVE for the analog supply VDDA."]
    #[inline(always)]
    pub fn vdda_active(&self) -> VddaActiveR {
        VddaActiveR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - The VDDD supply is always present during operation so a supply transition can not occur. This bit will always read back '1'."]
    #[inline(always)]
    pub fn vddd_active(&self) -> VdddActiveR {
        VdddActiveR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Supply state change detected. '0': No change to supply detected '1': Change to supply detected"]
    #[inline(always)]
    #[must_use]
    pub fn vddio_active(&mut self) -> VddioActiveW<VddIntrSpec> {
        VddioActiveW::new(self, 0)
    }
    #[doc = "Bit 30 - Same as VDDIO_ACTIVE for the analog supply VDDA."]
    #[inline(always)]
    #[must_use]
    pub fn vdda_active(&mut self) -> VddaActiveW<VddIntrSpec> {
        VddaActiveW::new(self, 30)
    }
    #[doc = "Bit 31 - The VDDD supply is always present during operation so a supply transition can not occur. This bit will always read back '1'."]
    #[inline(always)]
    #[must_use]
    pub fn vddd_active(&mut self) -> VdddActiveW<VddIntrSpec> {
        VdddActiveW::new(self, 31)
    }
}
#[doc = "Supply detection interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`vdd_intr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdd_intr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VddIntrSpec;
impl crate::RegisterSpec for VddIntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdd_intr::R`](R) reader structure"]
impl crate::Readable for VddIntrSpec {}
#[doc = "`write(|w| ..)` method takes [`vdd_intr::W`](W) writer structure"]
impl crate::Writable for VddIntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VDD_INTR to value 0"]
impl crate::Resettable for VddIntrSpec {
    const RESET_VALUE: u32 = 0;
}
