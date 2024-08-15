#[doc = "Register `SW_FW_TANK_SEL` reader"]
pub type R = crate::R<SwFwTankSelSpec>;
#[doc = "Register `SW_FW_TANK_SEL` writer"]
pub type W = crate::W<SwFwTankSelSpec>;
#[doc = "Field `SW_F2PT` reader - Set corresponding switch"]
pub type SwF2ptR = crate::BitReader;
#[doc = "Field `SW_F2PT` writer - Set corresponding switch"]
pub type SwF2ptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_F2MA` reader - Select waveform for corresponding switch"]
pub type SwF2maR = crate::FieldReader;
#[doc = "Field `SW_F2MA` writer - Select waveform for corresponding switch"]
pub type SwF2maW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_F2CA` reader - Select waveform for corresponding switch"]
pub type SwF2caR = crate::FieldReader;
#[doc = "Field `SW_F2CA` writer - Select waveform for corresponding switch"]
pub type SwF2caW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_F2CB` reader - Select waveform for corresponding switch"]
pub type SwF2cbR = crate::FieldReader;
#[doc = "Field `SW_F2CB` writer - Select waveform for corresponding switch"]
pub type SwF2cbW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SW_C2CC` reader - Set corresponding switch"]
pub type SwC2ccR = crate::BitReader;
#[doc = "Field `SW_C2CC` writer - Set corresponding switch"]
pub type SwC2ccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_C2CD` reader - Set corresponding switch"]
pub type SwC2cdR = crate::BitReader;
#[doc = "Field `SW_C2CD` writer - Set corresponding switch"]
pub type SwC2cdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_C2F2` reader - Set corresponding switch"]
pub type SwC2f2R = crate::BitReader;
#[doc = "Field `SW_C2F2` writer - Set corresponding switch"]
pub type SwC2f2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_f2pt(&self) -> SwF2ptR {
        SwF2ptR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f2ma(&self) -> SwF2maR {
        SwF2maR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f2ca(&self) -> SwF2caR {
        SwF2caR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Select waveform for corresponding switch"]
    #[inline(always)]
    pub fn sw_f2cb(&self) -> SwF2cbR {
        SwF2cbR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c2cc(&self) -> SwC2ccR {
        SwC2ccR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c2cd(&self) -> SwC2cdR {
        SwC2cdR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    pub fn sw_c2f2(&self) -> SwC2f2R {
        SwC2f2R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_f2pt(&mut self) -> SwF2ptW<SwFwTankSelSpec> {
        SwF2ptW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Select waveform for corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_f2ma(&mut self) -> SwF2maW<SwFwTankSelSpec> {
        SwF2maW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Select waveform for corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_f2ca(&mut self) -> SwF2caW<SwFwTankSelSpec> {
        SwF2caW::new(self, 12)
    }
    #[doc = "Bits 16:18 - Select waveform for corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_f2cb(&mut self) -> SwF2cbW<SwFwTankSelSpec> {
        SwF2cbW::new(self, 16)
    }
    #[doc = "Bit 20 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_c2cc(&mut self) -> SwC2ccW<SwFwTankSelSpec> {
        SwC2ccW::new(self, 20)
    }
    #[doc = "Bit 24 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_c2cd(&mut self) -> SwC2cdW<SwFwTankSelSpec> {
        SwC2cdW::new(self, 24)
    }
    #[doc = "Bit 28 - Set corresponding switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw_c2f2(&mut self) -> SwC2f2W<SwFwTankSelSpec> {
        SwC2f2W::new(self, 28)
    }
}
#[doc = "Full Wave Csh_tank Switch Waveform selection\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_fw_tank_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sw_fw_tank_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwFwTankSelSpec;
impl crate::RegisterSpec for SwFwTankSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_fw_tank_sel::R`](R) reader structure"]
impl crate::Readable for SwFwTankSelSpec {}
#[doc = "`write(|w| ..)` method takes [`sw_fw_tank_sel::W`](W) writer structure"]
impl crate::Writable for SwFwTankSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SW_FW_TANK_SEL to value 0"]
impl crate::Resettable for SwFwTankSelSpec {
    const RESET_VALUE: u32 = 0;
}
