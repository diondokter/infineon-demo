#[doc = "Register `ADDR1` reader"]
pub type R = crate::R<Addr1Spec>;
#[doc = "Field `SUBREGION_DISABLE` reader - This field is used to individually disabled the eight equally sized subregions in which a region is partitioned. Subregion disable: Bit 0: subregion 0 disable. Bit 1: subregion 1 disable. Bit 2: subregion 2 disable. Bit 3: subregion 3 disable. Bit 4: subregion 4 disable. Bit 5: subregion 5 disable. Bit 6: subregion 6 disable. Bit 7: subregion 7 disable. Two out of a total of eight 32 B subregions are enabled. These subregions includes region structures 0 and 1. Note: this field is read-only."]
pub type SubregionDisableR = crate::FieldReader;
#[doc = "Field `ADDR24` reader - This field specifies the most significant bits of the 32-bit address of an address region. 'ADDR_DEF1': base address of structure. Note: this field is read-only."]
pub type Addr24R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - This field is used to individually disabled the eight equally sized subregions in which a region is partitioned. Subregion disable: Bit 0: subregion 0 disable. Bit 1: subregion 1 disable. Bit 2: subregion 2 disable. Bit 3: subregion 3 disable. Bit 4: subregion 4 disable. Bit 5: subregion 5 disable. Bit 6: subregion 6 disable. Bit 7: subregion 7 disable. Two out of a total of eight 32 B subregions are enabled. These subregions includes region structures 0 and 1. Note: this field is read-only."]
    #[inline(always)]
    pub fn subregion_disable(&self) -> SubregionDisableR {
        SubregionDisableR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - This field specifies the most significant bits of the 32-bit address of an address region. 'ADDR_DEF1': base address of structure. Note: this field is read-only."]
    #[inline(always)]
    pub fn addr24(&self) -> Addr24R {
        Addr24R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "SMPU region address 1 (master structure)\n\nYou can [`read`](crate::Reg::read) this register and get [`addr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addr1Spec;
impl crate::RegisterSpec for Addr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr1::R`](R) reader structure"]
impl crate::Readable for Addr1Spec {}
#[doc = "`reset()` method sets ADDR1 to value 0"]
impl crate::Resettable for Addr1Spec {
    const RESET_VALUE: u32 = 0;
}
