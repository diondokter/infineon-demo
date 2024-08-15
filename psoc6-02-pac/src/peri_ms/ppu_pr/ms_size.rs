#[doc = "Register `MS_SIZE` reader"]
pub type R = crate::R<MsSizeSpec>;
#[doc = "Field `REGION_SIZE` reader - This field specifies the size of the master region: '5': 64 B region The master region includes the SL_ADDR, SL_SIZE, SL_ATT0, ..., SL_ATT3, MS_ADDR, MS_SIZE, MS_ATT0, ..., MS_ATT3 registers. Therefore, the access privileges for all these registers is determined by MS_ATT0, ..., MS_ATT3."]
pub type RegionSizeR = crate::FieldReader;
#[doc = "Field `VALID` reader - Master region enable: '1': Enabled."]
pub type ValidR = crate::BitReader;
impl R {
    #[doc = "Bits 24:28 - This field specifies the size of the master region: '5': 64 B region The master region includes the SL_ADDR, SL_SIZE, SL_ATT0, ..., SL_ATT3, MS_ADDR, MS_SIZE, MS_ATT0, ..., MS_ATT3 registers. Therefore, the access privileges for all these registers is determined by MS_ATT0, ..., MS_ATT3."]
    #[inline(always)]
    pub fn region_size(&self) -> RegionSizeR {
        RegionSizeR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Master region enable: '1': Enabled."]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Master region, size\n\nYou can [`read`](crate::Reg::read) this register and get [`ms_size::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsSizeSpec;
impl crate::RegisterSpec for MsSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ms_size::R`](R) reader structure"]
impl crate::Readable for MsSizeSpec {}
#[doc = "`reset()` method sets MS_SIZE to value 0x8500_0000"]
impl crate::Resettable for MsSizeSpec {
    const RESET_VALUE: u32 = 0x8500_0000;
}
