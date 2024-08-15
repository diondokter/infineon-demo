#[doc = "Register `SW_AMUXBUF_SEL` reader"]
pub type R = crate::R<SwAmuxbufSelSpec>;
#[doc = "Register `SW_AMUXBUF_SEL` writer"]
pub type W = crate::W<SwAmuxbufSelSpec>;
#[doc = "Field `SW_IRBY` reader - Set corresponding switch"]
pub type SwIrbyR = crate::BitReader;
#[doc = "Field `SW_IRBY` writer - Set corresponding switch"]
pub type SwIrbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_IRLB` reader - Set corresponding switch"]
pub type SwIrlbR = crate::BitReader;
#[doc = "Field `SW_IRLB` writer - Set corresponding switch"]
pub type SwIrlbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_ICA` reader - Set corresponding switch"]
pub type SwIcaR = crate::BitReader;
#[doc = "Field `SW_ICA` writer - Set corresponding switch"]
pub type SwIcaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_ICB` reader - Select waveform for corresponding switch"]
pub type SwIcbR = crate::FieldReader;
#[doc = "Field `SW_ICB` writer - Select waveform for corresponding switch"]
pub type SwIcbW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_IRLI` reader - Set corresponding switch"]
pub type SwIrliR = crate::BitReader;
#[doc = "Field `SW_IRLI` writer - Set corresponding switch"]
pub type SwIrliW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_IRH` reader - Set corresponding switch"]
pub type SwIrhR = crate::BitReader;
#[doc = "Field `SW_IRH` writer - Set corresponding switch"]
pub type SwIrhW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_IRL` reader - Set corresponding switch"]
pub type SwIrlR = crate::BitReader;
#[doc = "Field `SW_IRL` writer - Set corresponding switch"]
pub type SwIrlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irby(&self) -> SwIrbyR {
        SwIrbyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irlb(&self) -> SwIrlbR {
        SwIrlbR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_ica(&self) -> SwIcaR {
        SwIcaR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_icb(&self) -> SwIcbR {
        SwIcbR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irli(&self) -> SwIrliR {
        SwIrliR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irh(&self) -> SwIrhR {
        SwIrhR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_irl(&self) -> SwIrlR {
        SwIrlR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_irby(&mut self) -> SwIrbyW<SwAmuxbufSelSpec> {
        SwIrbyW::new(self, 4)
    }
    #[doc = "Bit 8 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_irlb(&mut self) -> SwIrlbW<SwAmuxbufSelSpec> {
        SwIrlbW::new(self, 8)
    }
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_ica(&mut self) -> SwIcaW<SwAmuxbufSelSpec> {
        SwIcaW::new(self, 12)
    }
    #[doc = "Bits 16:18 - Select waveform for corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_icb(&mut self) -> SwIcbW<SwAmuxbufSelSpec> {
        SwIcbW::new(self, 16)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_irli(&mut self) -> SwIrliW<SwAmuxbufSelSpec> {
        SwIrliW::new(self, 20)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_irh(&mut self) -> SwIrhW<SwAmuxbufSelSpec> {
        SwIrhW::new(self, 24)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_irl(&mut self) -> SwIrlW<SwAmuxbufSelSpec> {
        SwIrlW::new(self, 28)
    }
}
#[doc = "Amuxbuffer switches Waveform selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_amuxbuf_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_amuxbuf_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwAmuxbufSelSpec;
impl crate::RegisterSpec for SwAmuxbufSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_amuxbuf_sel::R`](R) reader structure"]
impl crate::Readable for SwAmuxbufSelSpec {}
#[doc = "`write(|w| ..)` method takes [`sw_amuxbuf_sel::W`](W) writer structure"]
impl crate::Writable for SwAmuxbufSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SW_AMUXBUF_SEL to value 0"]
impl crate::Resettable for SwAmuxbufSelSpec {
    const RESET_VALUE: u32 = 0;
}
