#[doc = "Register `SW_HS_N_SEL` reader"]
pub type R = crate::R<SwHsNSelSpec>;
#[doc = "Register `SW_HS_N_SEL` writer"]
pub type W = crate::W<SwHsNSelSpec>;
#[doc = "Field `SW_HCCC` reader - Set corresponding switch"]
pub type SwHcccR = crate::BitReader;
#[doc = "Field `SW_HCCC` writer - Set corresponding switch"]
pub type SwHcccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_HCCD` reader - Set corresponding switch"]
pub type SwHccdR = crate::BitReader;
#[doc = "Field `SW_HCCD` writer - Set corresponding switch"]
pub type SwHccdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_HCRH` reader - Select waveform for corresponding switch"]
pub type SwHcrhR = crate::FieldReader;
#[doc = "Field `SW_HCRH` writer - Select waveform for corresponding switch"]
pub type SwHcrhW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_HCRL` reader - Select waveform for corresponding switch"]
pub type SwHcrlR = crate::FieldReader;
#[doc = "Field `SW_HCRL` writer - Select waveform for corresponding switch"]
pub type SwHcrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hccc(&self) -> SwHcccR {
        SwHcccR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hccd(&self) -> SwHccdR {
        SwHccdR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_hcrh(&self) -> SwHcrhR {
        SwHcrhR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_hcrl(&self) -> SwHcrlR {
        SwHcrlR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hccc(&mut self) -> SwHcccW<SwHsNSelSpec> {
        SwHcccW::new(self, 16)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hccd(&mut self) -> SwHccdW<SwHsNSelSpec> {
        SwHccdW::new(self, 20)
    }
    #[doc = "Bits 24:26 - Select waveform for corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hcrh(&mut self) -> SwHcrhW<SwHsNSelSpec> {
        SwHcrhW::new(self, 24)
    }
    #[doc = "Bits 28:30 - Select waveform for corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hcrl(&mut self) -> SwHcrlW<SwHsNSelSpec> {
        SwHcrlW::new(self, 28)
    }
}
#[doc = "HSCMP Neg input switch Waveform selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_hs_n_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_hs_n_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwHsNSelSpec;
impl crate::RegisterSpec for SwHsNSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_hs_n_sel::R`](R) reader structure"]
impl crate::Readable for SwHsNSelSpec {}
#[doc = "`write(|w| ..)` method takes [`sw_hs_n_sel::W`](W) writer structure"]
impl crate::Writable for SwHsNSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SW_HS_N_SEL to value 0"]
impl crate::Resettable for SwHsNSelSpec {
    const RESET_VALUE: u32 = 0;
}
