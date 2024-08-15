#[doc = "Register `VDD_INTR_MASK` reader"]
pub type R = crate::R<VddIntrMaskSpec>;
#[doc = "Register `VDD_INTR_MASK` writer"]
pub type W = crate::W<VddIntrMaskSpec>;
#[doc = "Field `VDDIO_ACTIVE` reader - Masks supply interrupt on VDDIO. '0': VDDIO interrupt forwarding disabled '1': VDDIO interrupt forwarding enabled"]
pub type VddioActiveR = crate::FieldReader<u16>;
#[doc = "Field `VDDIO_ACTIVE` writer - Masks supply interrupt on VDDIO. '0': VDDIO interrupt forwarding disabled '1': VDDIO interrupt forwarding enabled"]
pub type VddioActiveW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `VDDA_ACTIVE` reader - Same as VDDIO_ACTIVE for the analog supply VDDA."]
pub type VddaActiveR = crate::BitReader;
#[doc = "Field `VDDA_ACTIVE` writer - Same as VDDIO_ACTIVE for the analog supply VDDA."]
pub type VddaActiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDD_ACTIVE` reader - Same as VDDIO_ACTIVE for the digital supply VDDD."]
pub type VdddActiveR = crate::BitReader;
#[doc = "Field `VDDD_ACTIVE` writer - Same as VDDIO_ACTIVE for the digital supply VDDD."]
pub type VdddActiveW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Masks supply interrupt on VDDIO. '0': VDDIO interrupt forwarding disabled '1': VDDIO interrupt forwarding enabled"]
    #[inline(always)]
    pub fn vddio_active(&self) -> VddioActiveR {
        VddioActiveR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 30 - Same as VDDIO_ACTIVE for the analog supply VDDA."]
    #[inline(always)]
    pub fn vdda_active(&self) -> VddaActiveR {
        VddaActiveR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Same as VDDIO_ACTIVE for the digital supply VDDD."]
    #[inline(always)]
    pub fn vddd_active(&self) -> VdddActiveR {
        VdddActiveR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Masks supply interrupt on VDDIO. '0': VDDIO interrupt forwarding disabled '1': VDDIO interrupt forwarding enabled"]
    #[inline(always)]
    #[must_use]
    pub fn vddio_active(&mut self) -> VddioActiveW<VddIntrMaskSpec> {
        VddioActiveW::new(self, 0)
    }
    #[doc = "Bit 30 - Same as VDDIO_ACTIVE for the analog supply VDDA."]
    #[inline(always)]
    #[must_use]
    pub fn vdda_active(&mut self) -> VddaActiveW<VddIntrMaskSpec> {
        VddaActiveW::new(self, 30)
    }
    #[doc = "Bit 31 - Same as VDDIO_ACTIVE for the digital supply VDDD."]
    #[inline(always)]
    #[must_use]
    pub fn vddd_active(&mut self) -> VdddActiveW<VddIntrMaskSpec> {
        VdddActiveW::new(self, 31)
    }
}
#[doc = "Supply detection interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`vdd_intr_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdd_intr_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VddIntrMaskSpec;
impl crate::RegisterSpec for VddIntrMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdd_intr_mask::R`](R) reader structure"]
impl crate::Readable for VddIntrMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`vdd_intr_mask::W`](W) writer structure"]
impl crate::Writable for VddIntrMaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VDD_INTR_MASK to value 0"]
impl crate::Resettable for VddIntrMaskSpec {
    const RESET_VALUE: u32 = 0;
}
