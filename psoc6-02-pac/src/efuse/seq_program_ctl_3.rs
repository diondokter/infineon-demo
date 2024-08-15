#[doc = "Register `SEQ_PROGRAM_CTL_3` reader"]
pub type R = crate::R<SeqProgramCtl3Spec>;
#[doc = "Register `SEQ_PROGRAM_CTL_3` writer"]
pub type W = crate::W<SeqProgramCtl3Spec>;
#[doc = "Field `CYCLES` reader - Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\]
IP clock cycles."]
pub type CyclesR = crate::FieldReader<u16>;
#[doc = "Field `CYCLES` writer - Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\]
IP clock cycles."]
pub type CyclesW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `STROBE_A` reader - Specifies value of eFUSE control signal strobe_a"]
pub type StrobeAR = crate::BitReader;
#[doc = "Field `STROBE_A` writer - Specifies value of eFUSE control signal strobe_a"]
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
#[doc = "Field `DONE` reader - When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
pub type DoneR = crate::BitReader;
#[doc = "Field `DONE` writer - When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\]
IP clock cycles."]
    #[inline(always)]
    pub fn cycles(&self) -> CyclesR {
        CyclesR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16 - Specifies value of eFUSE control signal strobe_a"]
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
    #[doc = "Bit 31 - When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\]
IP clock cycles."]
    #[inline(always)]
    #[must_use]
    pub fn cycles(&mut self) -> CyclesW<SeqProgramCtl3Spec> {
        CyclesW::new(self, 0)
    }
    #[doc = "Bit 16 - Specifies value of eFUSE control signal strobe_a"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_a(&mut self) -> StrobeAW<SeqProgramCtl3Spec> {
        StrobeAW::new(self, 16)
    }
    #[doc = "Bit 17 - Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_b(&mut self) -> StrobeBW<SeqProgramCtl3Spec> {
        StrobeBW::new(self, 17)
    }
    #[doc = "Bit 18 - Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_c(&mut self) -> StrobeCW<SeqProgramCtl3Spec> {
        StrobeCW::new(self, 18)
    }
    #[doc = "Bit 19 - Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_d(&mut self) -> StrobeDW<SeqProgramCtl3Spec> {
        StrobeDW::new(self, 19)
    }
    #[doc = "Bit 20 - Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_e(&mut self) -> StrobeEW<SeqProgramCtl3Spec> {
        StrobeEW::new(self, 20)
    }
    #[doc = "Bit 21 - Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_f(&mut self) -> StrobeFW<SeqProgramCtl3Spec> {
        StrobeFW::new(self, 21)
    }
    #[doc = "Bit 22 - Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    #[must_use]
    pub fn strobe_g(&mut self) -> StrobeGW<SeqProgramCtl3Spec> {
        StrobeGW::new(self, 22)
    }
    #[doc = "Bit 31 - When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DoneW<SeqProgramCtl3Spec> {
        DoneW::new(self, 31)
    }
}
#[doc = "Sequencer program control 3\n\nYou can [`read`](crate::Reg::read) this register and get [`seq_program_ctl_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seq_program_ctl_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeqProgramCtl3Spec;
impl crate::RegisterSpec for SeqProgramCtl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seq_program_ctl_3::R`](R) reader structure"]
impl crate::Readable for SeqProgramCtl3Spec {}
#[doc = "`write(|w| ..)` method takes [`seq_program_ctl_3::W`](W) writer structure"]
impl crate::Writable for SeqProgramCtl3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEQ_PROGRAM_CTL_3 to value 0x0031_0005"]
impl crate::Resettable for SeqProgramCtl3Spec {
    const RESET_VALUE: u32 = 0x0031_0005;
}
