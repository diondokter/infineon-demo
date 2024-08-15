#[doc = "Register `ATT0` reader"]
pub type R = crate::R<Att0Spec>;
#[doc = "Register `ATT0` writer"]
pub type W = crate::W<Att0Spec>;
#[doc = "Field `UR` reader - User read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed)."]
pub type UrR = crate::BitReader;
#[doc = "Field `UR` writer - User read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed)."]
pub type UrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UW` reader - User write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
pub type UwR = crate::BitReader;
#[doc = "Field `UW` writer - User write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
pub type UwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UX` reader - User execute enable: '0': Disabled (user, execute accesses are NOT allowed). '1': Enabled (user, execute accesses are allowed)."]
pub type UxR = crate::BitReader;
#[doc = "Field `UX` writer - User execute enable: '0': Disabled (user, execute accesses are NOT allowed). '1': Enabled (user, execute accesses are allowed)."]
pub type UxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR` reader - Privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed)."]
pub type PrR = crate::BitReader;
#[doc = "Field `PR` writer - Privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed)."]
pub type PrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PW` reader - Privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
pub type PwR = crate::BitReader;
#[doc = "Field `PW` writer - Privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
pub type PwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PX` reader - Privileged execute enable: '0': Disabled (privileged, execute accesses are NOT allowed). '1': Enabled (privileged, execute accesses are allowed)."]
pub type PxR = crate::BitReader;
#[doc = "Field `PX` writer - Privileged execute enable: '0': Disabled (privileged, execute accesses are NOT allowed). '1': Enabled (privileged, execute accesses are allowed)."]
pub type PxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NS` reader - Non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
pub type NsR = crate::BitReader;
#[doc = "Field `NS` writer - Non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
pub type NsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC_MASK_0` reader - This field specifies protection context identifier based access control for protection context '0'."]
pub type PcMask0R = crate::BitReader;
#[doc = "Field `PC_MASK_15_TO_1` reader - This field specifies protection context identifier based access control. Bit i: protection context i+1 enable. If '0', protection context i+1 access is disabled; i.e. not allowed. If '1', protection context i+1 access is enabled; i.e. allowed."]
pub type PcMask15To1R = crate::FieldReader<u16>;
#[doc = "Field `PC_MASK_15_TO_1` writer - This field specifies protection context identifier based access control. Bit i: protection context i+1 enable. If '0', protection context i+1 access is disabled; i.e. not allowed. If '1', protection context i+1 access is enabled; i.e. allowed."]
pub type PcMask15To1W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `REGION_SIZE` reader - This field specifies the region size: '0'-'6': Undefined. '7': 256 B region '8': 512 B region '9': 1 KB region '10': 2 KB region '11': 4 KB region '12': 8 KB region '13': 16 KB region '14': 32 KB region '15': 64 KB region '16': 128 KB region '17': 256 KB region '18': 512 KB region '19': 1 MB region '20': 2 MB region '21': 4 MB region '22': 8 MB region '23': 16 MB region '24': 32 MB region '25': 64 MB region '26': 128 MB region '27': 256 MB region '28': 512 MB region '39': 1 GB region '30': 2 GB region '31': 4 GB region"]
pub type RegionSizeR = crate::FieldReader;
#[doc = "Field `REGION_SIZE` writer - This field specifies the region size: '0'-'6': Undefined. '7': 256 B region '8': 512 B region '9': 1 KB region '10': 2 KB region '11': 4 KB region '12': 8 KB region '13': 16 KB region '14': 32 KB region '15': 64 KB region '16': 128 KB region '17': 256 KB region '18': 512 KB region '19': 1 MB region '20': 2 MB region '21': 4 MB region '22': 8 MB region '23': 16 MB region '24': 32 MB region '25': 64 MB region '26': 128 MB region '27': 256 MB region '28': 512 MB region '39': 1 GB region '30': 2 GB region '31': 4 GB region"]
pub type RegionSizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PC_MATCH` reader - This field specifies if the PC field participates in the 'matching' process or the 'access evaluation' process: '0': PC field participates in 'access evaluation'. '1': PC field participates in 'matching'. 'Matching' process. For each protection structure, the process identifies if a transfer address is contained within the address range. This identifies the 'matching' regions. 'Access evaluation' process. For each protection structure, the process evaluates the bus transfer access attributes against the access control attributes. Note that it is possible to define different access control for multiple protection contexts by using multiple protection structures with the same address region and PC_MATCH set to '1'."]
pub type PcMatchR = crate::BitReader;
#[doc = "Field `PC_MATCH` writer - This field specifies if the PC field participates in the 'matching' process or the 'access evaluation' process: '0': PC field participates in 'access evaluation'. '1': PC field participates in 'matching'. 'Matching' process. For each protection structure, the process identifies if a transfer address is contained within the address range. This identifies the 'matching' regions. 'Access evaluation' process. For each protection structure, the process evaluates the bus transfer access attributes against the access control attributes. Note that it is possible to define different access control for multiple protection contexts by using multiple protection structures with the same address region and PC_MATCH set to '1'."]
pub type PcMatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLED` reader - Region enable: '0': Disabled. A disabled region will never result in a match on the bus transfer address. '1': Enabled. Note: a disabled address region performs logic gating to reduce dynamic power consumption."]
pub type EnabledR = crate::BitReader;
#[doc = "Field `ENABLED` writer - Region enable: '0': Disabled. A disabled region will never result in a match on the bus transfer address. '1': Enabled. Note: a disabled address region performs logic gating to reduce dynamic power consumption."]
pub type EnabledW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - User read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed)."]
    #[inline(always)]
    pub fn ur(&self) -> UrR {
        UrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - User write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
    #[inline(always)]
    pub fn uw(&self) -> UwR {
        UwR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - User execute enable: '0': Disabled (user, execute accesses are NOT allowed). '1': Enabled (user, execute accesses are allowed)."]
    #[inline(always)]
    pub fn ux(&self) -> UxR {
        UxR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed)."]
    #[inline(always)]
    pub fn pr(&self) -> PrR {
        PrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
    #[inline(always)]
    pub fn pw(&self) -> PwR {
        PwR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Privileged execute enable: '0': Disabled (privileged, execute accesses are NOT allowed). '1': Enabled (privileged, execute accesses are allowed)."]
    #[inline(always)]
    pub fn px(&self) -> PxR {
        PxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
    #[inline(always)]
    pub fn ns(&self) -> NsR {
        NsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - This field specifies protection context identifier based access control for protection context '0'."]
    #[inline(always)]
    pub fn pc_mask_0(&self) -> PcMask0R {
        PcMask0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:23 - This field specifies protection context identifier based access control. Bit i: protection context i+1 enable. If '0', protection context i+1 access is disabled; i.e. not allowed. If '1', protection context i+1 access is enabled; i.e. allowed."]
    #[inline(always)]
    pub fn pc_mask_15_to_1(&self) -> PcMask15To1R {
        PcMask15To1R::new(((self.bits >> 9) & 0x7fff) as u16)
    }
    #[doc = "Bits 24:28 - This field specifies the region size: '0'-'6': Undefined. '7': 256 B region '8': 512 B region '9': 1 KB region '10': 2 KB region '11': 4 KB region '12': 8 KB region '13': 16 KB region '14': 32 KB region '15': 64 KB region '16': 128 KB region '17': 256 KB region '18': 512 KB region '19': 1 MB region '20': 2 MB region '21': 4 MB region '22': 8 MB region '23': 16 MB region '24': 32 MB region '25': 64 MB region '26': 128 MB region '27': 256 MB region '28': 512 MB region '39': 1 GB region '30': 2 GB region '31': 4 GB region"]
    #[inline(always)]
    pub fn region_size(&self) -> RegionSizeR {
        RegionSizeR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 30 - This field specifies if the PC field participates in the 'matching' process or the 'access evaluation' process: '0': PC field participates in 'access evaluation'. '1': PC field participates in 'matching'. 'Matching' process. For each protection structure, the process identifies if a transfer address is contained within the address range. This identifies the 'matching' regions. 'Access evaluation' process. For each protection structure, the process evaluates the bus transfer access attributes against the access control attributes. Note that it is possible to define different access control for multiple protection contexts by using multiple protection structures with the same address region and PC_MATCH set to '1'."]
    #[inline(always)]
    pub fn pc_match(&self) -> PcMatchR {
        PcMatchR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Region enable: '0': Disabled. A disabled region will never result in a match on the bus transfer address. '1': Enabled. Note: a disabled address region performs logic gating to reduce dynamic power consumption."]
    #[inline(always)]
    pub fn enabled(&self) -> EnabledR {
        EnabledR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - User read enable: '0': Disabled (user, read accesses are NOT allowed). '1': Enabled (user, read accesses are allowed)."]
    #[inline(always)]
    #[must_use]
    pub fn ur(&mut self) -> UrW<Att0Spec> {
        UrW::new(self, 0)
    }
    #[doc = "Bit 1 - User write enable: '0': Disabled (user, write accesses are NOT allowed). '1': Enabled (user, write accesses are allowed)."]
    #[inline(always)]
    #[must_use]
    pub fn uw(&mut self) -> UwW<Att0Spec> {
        UwW::new(self, 1)
    }
    #[doc = "Bit 2 - User execute enable: '0': Disabled (user, execute accesses are NOT allowed). '1': Enabled (user, execute accesses are allowed)."]
    #[inline(always)]
    #[must_use]
    pub fn ux(&mut self) -> UxW<Att0Spec> {
        UxW::new(self, 2)
    }
    #[doc = "Bit 3 - Privileged read enable: '0': Disabled (privileged, read accesses are NOT allowed). '1': Enabled (privileged, read accesses are allowed)."]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PrW<Att0Spec> {
        PrW::new(self, 3)
    }
    #[doc = "Bit 4 - Privileged write enable: '0': Disabled (privileged, write accesses are NOT allowed). '1': Enabled (privileged, write accesses are allowed)."]
    #[inline(always)]
    #[must_use]
    pub fn pw(&mut self) -> PwW<Att0Spec> {
        PwW::new(self, 4)
    }
    #[doc = "Bit 5 - Privileged execute enable: '0': Disabled (privileged, execute accesses are NOT allowed). '1': Enabled (privileged, execute accesses are allowed)."]
    #[inline(always)]
    #[must_use]
    pub fn px(&mut self) -> PxW<Att0Spec> {
        PxW::new(self, 5)
    }
    #[doc = "Bit 6 - Non-secure: '0': Secure (secure accesses allowed, non-secure access NOT allowed). '1': Non-secure (both secure and non-secure accesses allowed)."]
    #[inline(always)]
    #[must_use]
    pub fn ns(&mut self) -> NsW<Att0Spec> {
        NsW::new(self, 6)
    }
    #[doc = "Bits 9:23 - This field specifies protection context identifier based access control. Bit i: protection context i+1 enable. If '0', protection context i+1 access is disabled; i.e. not allowed. If '1', protection context i+1 access is enabled; i.e. allowed."]
    #[inline(always)]
    #[must_use]
    pub fn pc_mask_15_to_1(&mut self) -> PcMask15To1W<Att0Spec> {
        PcMask15To1W::new(self, 9)
    }
    #[doc = "Bits 24:28 - This field specifies the region size: '0'-'6': Undefined. '7': 256 B region '8': 512 B region '9': 1 KB region '10': 2 KB region '11': 4 KB region '12': 8 KB region '13': 16 KB region '14': 32 KB region '15': 64 KB region '16': 128 KB region '17': 256 KB region '18': 512 KB region '19': 1 MB region '20': 2 MB region '21': 4 MB region '22': 8 MB region '23': 16 MB region '24': 32 MB region '25': 64 MB region '26': 128 MB region '27': 256 MB region '28': 512 MB region '39': 1 GB region '30': 2 GB region '31': 4 GB region"]
    #[inline(always)]
    #[must_use]
    pub fn region_size(&mut self) -> RegionSizeW<Att0Spec> {
        RegionSizeW::new(self, 24)
    }
    #[doc = "Bit 30 - This field specifies if the PC field participates in the 'matching' process or the 'access evaluation' process: '0': PC field participates in 'access evaluation'. '1': PC field participates in 'matching'. 'Matching' process. For each protection structure, the process identifies if a transfer address is contained within the address range. This identifies the 'matching' regions. 'Access evaluation' process. For each protection structure, the process evaluates the bus transfer access attributes against the access control attributes. Note that it is possible to define different access control for multiple protection contexts by using multiple protection structures with the same address region and PC_MATCH set to '1'."]
    #[inline(always)]
    #[must_use]
    pub fn pc_match(&mut self) -> PcMatchW<Att0Spec> {
        PcMatchW::new(self, 30)
    }
    #[doc = "Bit 31 - Region enable: '0': Disabled. A disabled region will never result in a match on the bus transfer address. '1': Enabled. Note: a disabled address region performs logic gating to reduce dynamic power consumption."]
    #[inline(always)]
    #[must_use]
    pub fn enabled(&mut self) -> EnabledW<Att0Spec> {
        EnabledW::new(self, 31)
    }
}
#[doc = "SMPU region attributes 0 (slave structure)\n\nYou can [`read`](crate::Reg::read) this register and get [`att0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`att0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Att0Spec;
impl crate::RegisterSpec for Att0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`att0::R`](R) reader structure"]
impl crate::Readable for Att0Spec {}
#[doc = "`write(|w| ..)` method takes [`att0::W`](W) writer structure"]
impl crate::Writable for Att0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATT0 to value 0x0100"]
impl crate::Resettable for Att0Spec {
    const RESET_VALUE: u32 = 0x0100;
}
