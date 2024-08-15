#[doc = "Register `SW_CMP_P_SEL` reader"]
pub type R = crate::R<SwCmpPSelSpec>;
#[doc = "Register `SW_CMP_P_SEL` writer"]
pub type W = crate::W<SwCmpPSelSpec>;
#[doc = "Field `SW_SFPM` reader - Select waveform for corresponding switch"]
pub type SwSfpmR = crate::FieldReader;
#[doc = "Field `SW_SFPM` writer - Select waveform for corresponding switch"]
pub type SwSfpmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_SFPT` reader - Select waveform for corresponding switch"]
pub type SwSfptR = crate::FieldReader;
#[doc = "Field `SW_SFPT` writer - Select waveform for corresponding switch"]
pub type SwSfptW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_SFPS` reader - Select waveform for corresponding switch"]
pub type SwSfpsR = crate::FieldReader;
#[doc = "Field `SW_SFPS` writer - Select waveform for corresponding switch"]
pub type SwSfpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_SFMA` reader - Set corresponding switch"]
pub type SwSfmaR = crate::BitReader;
#[doc = "Field `SW_SFMA` writer - Set corresponding switch"]
pub type SwSfmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_SFMB` reader - Set corresponding switch"]
pub type SwSfmbR = crate::BitReader;
#[doc = "Field `SW_SFMB` writer - Set corresponding switch"]
pub type SwSfmbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_SFCA` reader - Set corresponding switch"]
pub type SwSfcaR = crate::BitReader;
#[doc = "Field `SW_SFCA` writer - Set corresponding switch"]
pub type SwSfcaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_SFCB` reader - Set corresponding switch"]
pub type SwSfcbR = crate::BitReader;
#[doc = "Field `SW_SFCB` writer - Set corresponding switch"]
pub type SwSfcbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_sfpm(&self) -> SwSfpmR {
        SwSfpmR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_sfpt(&self) -> SwSfptR {
        SwSfptR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_sfps(&self) -> SwSfpsR {
        SwSfpsR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfma(&self) -> SwSfmaR {
        SwSfmaR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfmb(&self) -> SwSfmbR {
        SwSfmbR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfca(&self) -> SwSfcaR {
        SwSfcaR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_sfcb(&self) -> SwSfcbR {
        SwSfcbR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Select waveform for corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_sfpm(&mut self) -> SwSfpmW<SwCmpPSelSpec> {
        SwSfpmW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Select waveform for corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_sfpt(&mut self) -> SwSfptW<SwCmpPSelSpec> {
        SwSfptW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Select waveform for corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_sfps(&mut self) -> SwSfpsW<SwCmpPSelSpec> {
        SwSfpsW::new(self, 8)
    }
    #[doc = "Bit 12 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_sfma(&mut self) -> SwSfmaW<SwCmpPSelSpec> {
        SwSfmaW::new(self, 12)
    }
    #[doc = "Bit 16 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_sfmb(&mut self) -> SwSfmbW<SwCmpPSelSpec> {
        SwSfmbW::new(self, 16)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_sfca(&mut self) -> SwSfcaW<SwCmpPSelSpec> {
        SwSfcaW::new(self, 20)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_sfcb(&mut self) -> SwSfcbW<SwCmpPSelSpec> {
        SwSfcbW::new(self, 24)
    }
}
#[doc = "CSDCMP Pos Switch Waveform selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_cmp_p_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_cmp_p_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwCmpPSelSpec;
impl crate::RegisterSpec for SwCmpPSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_cmp_p_sel::R`](R) reader structure"]
impl crate::Readable for SwCmpPSelSpec {}
#[doc = "`write(|w| ..)` method takes [`sw_cmp_p_sel::W`](W) writer structure"]
impl crate::Writable for SwCmpPSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SW_CMP_P_SEL to value 0"]
impl crate::Resettable for SwCmpPSelSpec {
    const RESET_VALUE: u32 = 0;
}
