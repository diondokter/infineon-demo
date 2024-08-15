#[doc = "Register `VDD_INTR_MASKED` reader"]
pub type R = crate::R<VddIntrMaskedSpec>;
#[doc = "Field `VDDIO_ACTIVE` reader - Supply transition detected AND masked '0': Interrupt was not forwarded to CPU '1': Interrupt occurred and was forwarded to CPU"]
pub type VddioActiveR = crate::FieldReader<u16>;
#[doc = "Field `VDDA_ACTIVE` reader - Same as VDDIO_ACTIVE for the analog supply VDDA."]
pub type VddaActiveR = crate::BitReader;
#[doc = "Field `VDDD_ACTIVE` reader - Same as VDDIO_ACTIVE for the digital supply VDDD."]
pub type VdddActiveR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Supply transition detected AND masked '0': Interrupt was not forwarded to CPU '1': Interrupt occurred and was forwarded to CPU"]
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
#[doc = "Supply detection interrupt masked register\n\nYou can [`read`](crate::Reg::read) this register and get [`vdd_intr_masked::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VddIntrMaskedSpec;
impl crate::RegisterSpec for VddIntrMaskedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vdd_intr_masked::R`](R) reader structure"]
impl crate::Readable for VddIntrMaskedSpec {}
#[doc = "`reset()` method sets VDD_INTR_MASKED to value 0"]
impl crate::Resettable for VddIntrMaskedSpec {
    const RESET_VALUE: u32 = 0;
}
