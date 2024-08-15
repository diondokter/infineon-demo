#[doc = "Register `SW_CMP_N_SEL` reader"]
pub type R = crate::R<SwCmpNSelSpec>;
#[doc = "Register `SW_CMP_N_SEL` writer"]
pub type W = crate::W<SwCmpNSelSpec>;
#[doc = "Field `SW_SCRH` reader - Select waveform for corresponding switch"]
pub type SwScrhR = crate::FieldReader;
#[doc = "Field `SW_SCRH` writer - Select waveform for corresponding switch"]
pub type SwScrhW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_SCRL` reader - Select waveform for corresponding switch"]
pub type SwScrlR = crate::FieldReader;
#[doc = "Field `SW_SCRL` writer - Select waveform for corresponding switch"]
pub type SwScrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 24:26 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_scrh(&self) -> SwScrhR {
        SwScrhR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_scrl(&self) -> SwScrlR {
        SwScrlR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 24:26 - Select waveform for corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_scrh(&mut self) -> SwScrhW<SwCmpNSelSpec> {
        SwScrhW::new(self, 24)
    }
    #[doc = "Bits 28:30 - Select waveform for corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_scrl(&mut self) -> SwScrlW<SwCmpNSelSpec> {
        SwScrlW::new(self, 28)
    }
}
#[doc = "CSDCMP Neg Switch Waveform selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_cmp_n_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_cmp_n_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwCmpNSelSpec;
impl crate::RegisterSpec for SwCmpNSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_cmp_n_sel::R`](R) reader structure"]
impl crate::Readable for SwCmpNSelSpec {}
#[doc = "`write(|w| ..)` method takes [`sw_cmp_n_sel::W`](W) writer structure"]
impl crate::Writable for SwCmpNSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SW_CMP_N_SEL to value 0"]
impl crate::Resettable for SwCmpNSelSpec {
    const RESET_VALUE: u32 = 0;
}
