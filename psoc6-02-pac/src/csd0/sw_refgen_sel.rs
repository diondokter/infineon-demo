#[doc = "Register `SW_REFGEN_SEL` reader"]
pub type R = crate::R<SwRefgenSelSpec>;
#[doc = "Register `SW_REFGEN_SEL` writer"]
pub type W = crate::W<SwRefgenSelSpec>;
#[doc = "Field `SW_IAIB` reader - Set corresponding switch"]
pub type SwIaibR = crate::BitReader;
#[doc = "Field `SW_IAIB` writer - Set corresponding switch"]
pub type SwIaibW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_IBCB` reader - Set corresponding switch"]
pub type SwIbcbR = crate::BitReader;
#[doc = "Field `SW_IBCB` writer - Set corresponding switch"]
pub type SwIbcbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_SGMB` reader - Set corresponding switch"]
pub type SwSgmbR = crate::BitReader;
#[doc = "Field `SW_SGMB` writer - Set corresponding switch"]
pub type SwSgmbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_SGRP` reader - Set corresponding switch"]
pub type SwSgrpR = crate::BitReader;
#[doc = "Field `SW_SGRP` writer - Set corresponding switch"]
pub type SwSgrpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_SGRE` reader - Set corresponding switch"]
pub type SwSgreR = crate::BitReader;
#[doc = "Field `SW_SGRE` writer - Set corresponding switch"]
pub type SwSgreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_SGR` reader - Set corresponding switch"]
pub type SwSgrR = crate::BitReader;
#[doc = "Field `SW_SGR` writer - Set corresponding switch"]
pub type SwSgrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_iaib(&self) -> SwIaibR {
        SwIaibR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_ibcb(&self) -> SwIbcbR {
        SwIbcbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgmb(&self) -> SwSgmbR {
        SwSgmbR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgrp(&self) -> SwSgrpR {
        SwSgrpR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgre(&self) -> SwSgreR {
        SwSgreR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sgr(&self) -> SwSgrR {
        SwSgrR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_iaib(&mut self) -> SwIaibW<SwRefgenSelSpec> {
        SwIaibW::new(self, 0)
    }
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ibcb(&mut self) -> SwIbcbW<SwRefgenSelSpec> {
        SwIbcbW::new(self, 4)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_sgmb(&mut self) -> SwSgmbW<SwRefgenSelSpec> {
        SwSgmbW::new(self, 16)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_sgrp(&mut self) -> SwSgrpW<SwRefgenSelSpec> {
        SwSgrpW::new(self, 20)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_sgre(&mut self) -> SwSgreW<SwRefgenSelSpec> {
        SwSgreW::new(self, 24)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_sgr(&mut self) -> SwSgrW<SwRefgenSelSpec> {
        SwSgrW::new(self, 28)
    }
}
#[doc = "Reference Generator Switch Waveform selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_refgen_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_refgen_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwRefgenSelSpec;
impl crate::RegisterSpec for SwRefgenSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_refgen_sel::R`](R) reader structure"]
impl crate::Readable for SwRefgenSelSpec {}
#[doc = "`write(|w| ..)` method takes [`sw_refgen_sel::W`](W) writer structure"]
impl crate::Writable for SwRefgenSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SW_REFGEN_SEL to value 0"]
impl crate::Resettable for SwRefgenSelSpec {
    const RESET_VALUE: u32 = 0;
}
