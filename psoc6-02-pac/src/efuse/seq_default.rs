#[doc = "Register `SEQ_DEFAULT` reader"]
pub type R = crate::R<SeqDefaultSpec>;
#[doc = "Register `SEQ_DEFAULT` writer"]
pub type W = crate::W<SeqDefaultSpec>;
#[doc = "Field `STROBE_A` reader - Specifies value of eFUSE control signal strobe_f"]
pub type StrobeAR = crate::BitReader;
#[doc = "Field `STROBE_A` writer - Specifies value of eFUSE control signal strobe_f"]
pub type StrobeAW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STROBE_B` reader - Specifies value of eFUSEcontrol signal strobe_b"]
pub type StrobeBR = crate::BitReader;
#[doc = "Field `STROBE_B` writer - Specifies value of eFUSEcontrol signal strobe_b"]
pub type StrobeBW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STROBE_C` reader - Specifies value of eFUSE control signal strobe_c"]
pub type StrobeCR = crate::BitReader;
#[doc = "Field `STROBE_C` writer - Specifies value of eFUSE control signal strobe_c"]
pub type StrobeCW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STROBE_D` reader - Specifies value of eFUSE control signal strobe_d"]
pub type StrobeDR = crate::BitReader;
#[doc = "Field `STROBE_D` writer - Specifies value of eFUSE control signal strobe_d"]
pub type StrobeDW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STROBE_E` reader - Specifies value of eFUSE control signal strobe_e"]
pub type StrobeER = crate::BitReader;
#[doc = "Field `STROBE_E` writer - Specifies value of eFUSE control signal strobe_e"]
pub type StrobeEW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STROBE_F` reader - Specifies value of eFUSE control signal strobe_f"]
pub type StrobeFR = crate::BitReader;
#[doc = "Field `STROBE_F` writer - Specifies value of eFUSE control signal strobe_f"]
pub type StrobeFW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STROBE_G` reader - Specifies value of eFUSE control signal strobe_g"]
pub type StrobeGR = crate::BitReader;
#[doc = "Field `STROBE_G` writer - Specifies value of eFUSE control signal strobe_g"]
pub type StrobeGW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn strobe_a(&self) -> StrobeAR {
        StrobeAR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub fn strobe_b(&self) -> StrobeBR {
        StrobeBR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub fn strobe_c(&self) -> StrobeCR {
        StrobeCR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub fn strobe_d(&self) -> StrobeDR {
        StrobeDR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub fn strobe_e(&self) -> StrobeER {
        StrobeER::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn strobe_f(&self) -> StrobeFR {
        StrobeFR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub fn strobe_g(&self) -> StrobeGR {
        StrobeGR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_a(&mut self) -> StrobeAW<SeqDefaultSpec> {
        StrobeAW::new(self, 16)
    }
    #[doc = "Bit 17 - Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_b(&mut self) -> StrobeBW<SeqDefaultSpec> {
        StrobeBW::new(self, 17)
    }
    #[doc = "Bit 18 - Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_c(&mut self) -> StrobeCW<SeqDefaultSpec> {
        StrobeCW::new(self, 18)
    }
    #[doc = "Bit 19 - Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_d(&mut self) -> StrobeDW<SeqDefaultSpec> {
        StrobeDW::new(self, 19)
    }
    #[doc = "Bit 20 - Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_e(&mut self) -> StrobeEW<SeqDefaultSpec> {
        StrobeEW::new(self, 20)
    }
    #[doc = "Bit 21 - Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_f(&mut self) -> StrobeFW<SeqDefaultSpec> {
        StrobeFW::new(self, 21)
    }
    #[doc = "Bit 22 - Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_g(&mut self) -> StrobeGW<SeqDefaultSpec> {
        StrobeGW::new(self, 22)
    }
}
#[doc = "Sequencer Default value\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_default::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_default::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeqDefaultSpec;
impl crate::RegisterSpec for SeqDefaultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq_default::R`](R) reader structure"]
impl crate::Readable for SeqDefaultSpec {}
#[doc = "`write(|w| ..)` method takes [`seq_default::W`](W) writer structure"]
impl crate::Writable for SeqDefaultSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEQ_DEFAULT to value 0x001d_0000"]
impl crate::Resettable for SeqDefaultSpec {
    const RESET_VALUE: u32 = 0x001d_0000;
}
