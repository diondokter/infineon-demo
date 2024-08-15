#[doc = "Register `SL_SIZE` reader"]
pub type R = crate::R<SlSizeSpec>;
#[doc = "Register `SL_SIZE` writer"]
pub type W = crate::W<SlSizeSpec>;
#[doc = "Field `REGION_SIZE` reader - This field specifies the size of the slave region: '0': Undefined. '1': 4 B region (this is the smallest region size). '2': 8 B region '3': 16 B region '4': 32 B region '5': 64 B region '6': 128 B region '7': 256 B region '8': 512 B region '9': 1 KB region '10': 2 KB region '11': 4 KB region '12': 8 KB region '13': 16 KB region '14': 32 KB region '15': 64 KB region '16': 128 KB region '17': 256 KB region '18': 512 KB region '19': 1 MB region '20': 2 MB region '21': 4 MB region '22': 8 MB region '23': 16 MB region '24': 32 MB region '25': 64 MB region '26': 128 MB region '27': 256 MB region '28': 512 MB region '29': 1 GB region '30': 2 GB region '31': 4 GB region"]
pub type RegionSizeR = crate::FieldReader;
#[doc = "Field `REGION_SIZE` writer - This field specifies the size of the slave region: '0': Undefined. '1': 4 B region (this is the smallest region size). '2': 8 B region '3': 16 B region '4': 32 B region '5': 64 B region '6': 128 B region '7': 256 B region '8': 512 B region '9': 1 KB region '10': 2 KB region '11': 4 KB region '12': 8 KB region '13': 16 KB region '14': 32 KB region '15': 64 KB region '16': 128 KB region '17': 256 KB region '18': 512 KB region '19': 1 MB region '20': 2 MB region '21': 4 MB region '22': 8 MB region '23': 16 MB region '24': 32 MB region '25': 64 MB region '26': 128 MB region '27': 256 MB region '28': 512 MB region '29': 1 GB region '30': 2 GB region '31': 4 GB region"]
pub type RegionSizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `VALID` reader - Slave region enable: '0': Disabled. A disabled region will never result in a match on the transfer address. '1': Enabled."]
pub type ValidR = crate::BitReader;
#[doc = "Field `VALID` writer - Slave region enable: '0': Disabled. A disabled region will never result in a match on the transfer address. '1': Enabled."]
pub type ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 24:28 - This field specifies the size of the slave region: '0': Undefined. '1': 4 B region (this is the smallest region size). '2': 8 B region '3': 16 B region '4': 32 B region '5': 64 B region '6': 128 B region '7': 256 B region '8': 512 B region '9': 1 KB region '10': 2 KB region '11': 4 KB region '12': 8 KB region '13': 16 KB region '14': 32 KB region '15': 64 KB region '16': 128 KB region '17': 256 KB region '18': 512 KB region '19': 1 MB region '20': 2 MB region '21': 4 MB region '22': 8 MB region '23': 16 MB region '24': 32 MB region '25': 64 MB region '26': 128 MB region '27': 256 MB region '28': 512 MB region '29': 1 GB region '30': 2 GB region '31': 4 GB region"]
    #[inline(always)]
    pub fn region_size(&self) -> RegionSizeR {
        RegionSizeR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Slave region enable: '0': Disabled. A disabled region will never result in a match on the transfer address. '1': Enabled."]
    #[inline(always)]
    pub fn valid(&self) -> ValidR {
        ValidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 24:28 - This field specifies the size of the slave region: '0': Undefined. '1': 4 B region (this is the smallest region size). '2': 8 B region '3': 16 B region '4': 32 B region '5': 64 B region '6': 128 B region '7': 256 B region '8': 512 B region '9': 1 KB region '10': 2 KB region '11': 4 KB region '12': 8 KB region '13': 16 KB region '14': 32 KB region '15': 64 KB region '16': 128 KB region '17': 256 KB region '18': 512 KB region '19': 1 MB region '20': 2 MB region '21': 4 MB region '22': 8 MB region '23': 16 MB region '24': 32 MB region '25': 64 MB region '26': 128 MB region '27': 256 MB region '28': 512 MB region '29': 1 GB region '30': 2 GB region '31': 4 GB region"]
    #[inline(always)]
    #[must_use]
    pub fn region_size(&mut self) -> RegionSizeW<SlSizeSpec> {
        RegionSizeW::new(self, 24)
    }
    #[doc = "Bit 31 - Slave region enable: '0': Disabled. A disabled region will never result in a match on the transfer address. '1': Enabled."]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> ValidW<SlSizeSpec> {
        ValidW::new(self, 31)
    }
}
#[doc = "Slave region, size\n\nYou can [`read`](crate::Reg::read) this register and get [`sl_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sl_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlSizeSpec;
impl crate::RegisterSpec for SlSizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sl_size::R`](R) reader structure"]
impl crate::Readable for SlSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`sl_size::W`](W) writer structure"]
impl crate::Writable for SlSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SL_SIZE to value 0"]
impl crate::Resettable for SlSizeSpec {
    const RESET_VALUE: u32 = 0;
}
