#[doc = "Register `PRODUCT_ID` reader"]
pub type R = crate::R<ProductIdSpec>;
#[doc = "Field `FAMILY_ID` reader - Family ID. Common ID for a product family."]
pub type FamilyIdR = crate::FieldReader<u16>;
#[doc = "Field `MAJOR_REV` reader - Major Revision, starts with 1, increments with all layer tape-out (implemented with metal ECO-able tie-off)"]
pub type MajorRevR = crate::FieldReader;
#[doc = "Field `MINOR_REV` reader - Minor Revision, starts with 1, increments with metal layer only tape-out (implemented with metal ECO-able tie-off)"]
pub type MinorRevR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:11 - Family ID. Common ID for a product family."]
    #[inline(always)]
    pub fn family_id(&self) -> FamilyIdR {
        FamilyIdR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19 - Major Revision, starts with 1, increments with all layer tape-out (implemented with metal ECO-able tie-off)"]
    #[inline(always)]
    pub fn major_rev(&self) -> MajorRevR {
        MajorRevR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Minor Revision, starts with 1, increments with metal layer only tape-out (implemented with metal ECO-able tie-off)"]
    #[inline(always)]
    pub fn minor_rev(&self) -> MinorRevR {
        MinorRevR::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
#[doc = "Product identifier and version (same as CoreSight RomTables)\n\nYou can [`read`](crate::Reg::read) this register and get [`product_id::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProductIdSpec;
impl crate::RegisterSpec for ProductIdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`product_id::R`](R) reader structure"]
impl crate::Readable for ProductIdSpec {}
#[doc = "`reset()` method sets PRODUCT_ID to value 0"]
impl crate::Resettable for ProductIdSpec {
    const RESET_VALUE: u32 = 0;
}
