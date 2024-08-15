#[doc = "Register `SW_FW_MOD_SEL` reader"]
pub type R = crate::R<SwFwModSelSpec>;
#[doc = "Register `SW_FW_MOD_SEL` writer"]
pub type W = crate::W<SwFwModSelSpec>;
#[doc = "Field `SW_F1PM` reader - Set corresponding switch"]
pub type SwF1pmR = crate::BitReader;
#[doc = "Field `SW_F1PM` writer - Set corresponding switch"]
pub type SwF1pmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_F1MA` reader - Select waveform for corresponding switch"]
pub type SwF1maR = crate::FieldReader;
#[doc = "Field `SW_F1MA` writer - Select waveform for corresponding switch"]
pub type SwF1maW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_F1CA` reader - Select waveform for corresponding switch"]
pub type SwF1caR = crate::FieldReader;
#[doc = "Field `SW_F1CA` writer - Select waveform for corresponding switch"]
pub type SwF1caW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_C1CC` reader - Set corresponding switch"]
pub type SwC1ccR = crate::BitReader;
#[doc = "Field `SW_C1CC` writer - Set corresponding switch"]
pub type SwC1ccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_C1CD` reader - Set corresponding switch"]
pub type SwC1cdR = crate::BitReader;
#[doc = "Field `SW_C1CD` writer - Set corresponding switch"]
pub type SwC1cdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_C1F1` reader - Set corresponding switch"]
pub type SwC1f1R = crate::BitReader;
#[doc = "Field `SW_C1F1` writer - Set corresponding switch"]
pub type SwC1f1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_f1pm(&self) -> SwF1pmR {
        SwF1pmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:10 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f1ma(&self) -> SwF1maR {
        SwF1maR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f1ca(&self) -> SwF1caR {
        SwF1caR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c1cc(&self) -> SwC1ccR {
        SwC1ccR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c1cd(&self) -> SwC1cdR {
        SwC1cdR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c1f1(&self) -> SwC1f1R {
        SwC1f1R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_f1pm(&mut self) -> SwF1pmW<SwFwModSelSpec> {
        SwF1pmW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Select waveform for corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_f1ma(&mut self) -> SwF1maW<SwFwModSelSpec> {
        SwF1maW::new(self, 8)
    }
    #[doc = "Bits 16:18 - Select waveform for corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_f1ca(&mut self) -> SwF1caW<SwFwModSelSpec> {
        SwF1caW::new(self, 16)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_c1cc(&mut self) -> SwC1ccW<SwFwModSelSpec> {
        SwC1ccW::new(self, 20)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_c1cd(&mut self) -> SwC1cdW<SwFwModSelSpec> {
        SwC1cdW::new(self, 24)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_c1f1(&mut self) -> SwC1f1W<SwFwModSelSpec> {
        SwC1f1W::new(self, 28)
    }
}
#[doc = "Full Wave Cmod Switch Waveform selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_fw_mod_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_fw_mod_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwFwModSelSpec;
impl crate::RegisterSpec for SwFwModSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_fw_mod_sel::R`](R) reader structure"]
impl crate::Readable for SwFwModSelSpec {}
#[doc = "`write(|w| ..)` method takes [`sw_fw_mod_sel::W`](W) writer structure"]
impl crate::Writable for SwFwModSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SW_FW_MOD_SEL to value 0"]
impl crate::Resettable for SwFwModSelSpec {
    const RESET_VALUE: u32 = 0;
}
