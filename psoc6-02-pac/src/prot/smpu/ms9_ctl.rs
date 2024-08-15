#[doc = "Register `MS9_CTL` reader"]
pub type R = crate::R<Ms9CtlSpec>;
#[doc = "Register `MS9_CTL` writer"]
pub type W = crate::W<Ms9CtlSpec>;
#[doc = "Field `P` reader - See MS0_CTL.P."]
pub type PR = crate::BitReader;
#[doc = "Field `P` writer - See MS0_CTL.P."]
pub type PW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NS` reader - See MS0_CTL.NS."]
pub type NsR = crate::BitReader;
#[doc = "Field `NS` writer - See MS0_CTL.NS."]
pub type NsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRIO` reader - See MS0_CTL.PRIO"]
pub type PrioR = crate::FieldReader;
#[doc = "Field `PRIO` writer - See MS0_CTL.PRIO"]
pub type PrioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PC_MASK_0` reader - See MS0_CTL.PC_MASK_0."]
pub type PcMask0R = crate::BitReader;
#[doc = "Field `PC_MASK_15_TO_1` reader - See MS0_CTL.PC_MASK_15_TO_1."]
pub type PcMask15To1R = crate::FieldReader<u16>;
#[doc = "Field `PC_MASK_15_TO_1` writer - See MS0_CTL.PC_MASK_15_TO_1."]
pub type PcMask15To1W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bit 0 - See MS0_CTL.P."]
    #[inline(always)]
    pub fn p(&self) -> PR {
        PR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - See MS0_CTL.NS."]
    #[inline(always)]
    pub fn ns(&self) -> NsR {
        NsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:9 - See MS0_CTL.PRIO"]
    #[inline(always)]
    pub fn prio(&self) -> PrioR {
        PrioR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - See MS0_CTL.PC_MASK_0."]
    #[inline(always)]
    pub fn pc_mask_0(&self) -> PcMask0R {
        PcMask0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    pub fn pc_mask_15_to_1(&self) -> PcMask15To1R {
        PcMask15To1R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - See MS0_CTL.P."]
    #[inline(always)]
    #[must_use]
    pub fn p(&mut self) -> PW<Ms9CtlSpec> {
        PW::new(self, 0)
    }
    #[doc = "Bit 1 - See MS0_CTL.NS."]
    #[inline(always)]
    #[must_use]
    pub fn ns(&mut self) -> NsW<Ms9CtlSpec> {
        NsW::new(self, 1)
    }
    #[doc = "Bits 8:9 - See MS0_CTL.PRIO"]
    #[inline(always)]
    #[must_use]
    pub fn prio(&mut self) -> PrioW<Ms9CtlSpec> {
        PrioW::new(self, 8)
    }
    #[doc = "Bits 17:31 - See MS0_CTL.PC_MASK_15_TO_1."]
    #[inline(always)]
    #[must_use]
    pub fn pc_mask_15_to_1(&mut self) -> PcMask15To1W<Ms9CtlSpec> {
        PcMask15To1W::new(self, 17)
    }
}
#[doc = "Master 9 protection context control\n\nYou can [`read`](crate::Reg::read) this register and get [`ms9_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ms9_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ms9CtlSpec;
impl crate::RegisterSpec for Ms9CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ms9_ctl::R`](R) reader structure"]
impl crate::Readable for Ms9CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ms9_ctl::W`](W) writer structure"]
impl crate::Writable for Ms9CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MS9_CTL to value 0x0303"]
impl crate::Resettable for Ms9CtlSpec {
    const RESET_VALUE: u32 = 0x0303;
}
