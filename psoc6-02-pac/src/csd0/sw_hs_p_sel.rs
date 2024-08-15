#[doc = "Register `SW_HS_P_SEL` reader"]
pub type R = crate::R<SwHsPSelSpec>;
#[doc = "Register `SW_HS_P_SEL` writer"]
pub type W = crate::W<SwHsPSelSpec>;
#[doc = "Field `SW_HMPM` reader - Set HMPM switch 0: static open 1: static closed"]
pub type SwHmpmR = crate::BitReader;
#[doc = "Field `SW_HMPM` writer - Set HMPM switch 0: static open 1: static closed"]
pub type SwHmpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_HMPT` reader - Set corresponding switch"]
pub type SwHmptR = crate::BitReader;
#[doc = "Field `SW_HMPT` writer - Set corresponding switch"]
pub type SwHmptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_HMPS` reader - Set corresponding switch"]
pub type SwHmpsR = crate::BitReader;
#[doc = "Field `SW_HMPS` writer - Set corresponding switch"]
pub type SwHmpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_HMMA` reader - Set corresponding switch"]
pub type SwHmmaR = crate::BitReader;
#[doc = "Field `SW_HMMA` writer - Set corresponding switch"]
pub type SwHmmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_HMMB` reader - Set corresponding switch"]
pub type SwHmmbR = crate::BitReader;
#[doc = "Field `SW_HMMB` writer - Set corresponding switch"]
pub type SwHmmbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_HMCA` reader - Set corresponding switch"]
pub type SwHmcaR = crate::BitReader;
#[doc = "Field `SW_HMCA` writer - Set corresponding switch"]
pub type SwHmcaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_HMCB` reader - Set corresponding switch"]
pub type SwHmcbR = crate::BitReader;
#[doc = "Field `SW_HMCB` writer - Set corresponding switch"]
pub type SwHmcbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_HMRH` reader - Set corresponding switch"]
pub type SwHmrhR = crate::BitReader;
#[doc = "Field `SW_HMRH` writer - Set corresponding switch"]
pub type SwHmrhW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set HMPM switch 0: static open 1: static closed"]
    #[inline(always)]
    pub fn sw_hmpm(&self) -> SwHmpmR {
        SwHmpmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmpt(&self) -> SwHmptR {
        SwHmptR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmps(&self) -> SwHmpsR {
        SwHmpsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmma(&self) -> SwHmmaR {
        SwHmmaR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmmb(&self) -> SwHmmbR {
        SwHmmbR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmca(&self) -> SwHmcaR {
        SwHmcaR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmcb(&self) -> SwHmcbR {
        SwHmcbR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_hmrh(&self) -> SwHmrhR {
        SwHmrhR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set HMPM switch 0: static open 1: static closed"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hmpm(&mut self) -> SwHmpmW<SwHsPSelSpec> {
        SwHmpmW::new(self, 0)
    }
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hmpt(&mut self) -> SwHmptW<SwHsPSelSpec> {
        SwHmptW::new(self, 4)
    }
    #[doc = "Bit 8 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hmps(&mut self) -> SwHmpsW<SwHsPSelSpec> {
        SwHmpsW::new(self, 8)
    }
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hmma(&mut self) -> SwHmmaW<SwHsPSelSpec> {
        SwHmmaW::new(self, 12)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hmmb(&mut self) -> SwHmmbW<SwHsPSelSpec> {
        SwHmmbW::new(self, 16)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hmca(&mut self) -> SwHmcaW<SwHsPSelSpec> {
        SwHmcaW::new(self, 20)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hmcb(&mut self) -> SwHmcbW<SwHsPSelSpec> {
        SwHmcbW::new(self, 24)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_hmrh(&mut self) -> SwHmrhW<SwHsPSelSpec> {
        SwHmrhW::new(self, 28)
    }
}
#[doc = "HSCMP Pos input switch Waveform selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_hs_p_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_hs_p_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwHsPSelSpec;
impl crate::RegisterSpec for SwHsPSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_hs_p_sel::R`](R) reader structure"]
impl crate::Readable for SwHsPSelSpec {}
#[doc = "`write(|w| ..)` method takes [`sw_hs_p_sel::W`](W) writer structure"]
impl crate::Writable for SwHsPSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SW_HS_P_SEL to value 0"]
impl crate::Resettable for SwHsPSelSpec {
    const RESET_VALUE: u32 = 0;
}
