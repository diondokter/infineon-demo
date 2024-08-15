#[doc = "Register `WR_DUMMY_CTL` reader"]
pub type R = crate::R<WrDummyCtlSpec>;
#[doc = "Register `WR_DUMMY_CTL` writer"]
pub type W = crate::W<WrDummyCtlSpec>;
#[doc = "Field `SIZE5` reader - Number of dummy cycles (minus 1): '0': 1 cycles ... '31': 32 cycles."]
pub type Size5R = crate::FieldReader;
#[doc = "Field `SIZE5` writer - Number of dummy cycles (minus 1): '0': 1 cycles ... '31': 32 cycles."]
pub type Size5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRESENT` reader - Presence of dummy cycles: '0': not present '1': present"]
pub type PresentR = crate::BitReader;
#[doc = "Field `PRESENT` writer - Presence of dummy cycles: '0': not present '1': present"]
pub type PresentW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Number of dummy cycles (minus 1): '0': 1 cycles ... '31': 32 cycles."]
    #[inline(always)]
    pub fn size5(&self) -> Size5R {
        Size5R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Presence of dummy cycles: '0': not present '1': present"]
    #[inline(always)]
    pub fn present(&self) -> PresentR {
        PresentR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of dummy cycles (minus 1): '0': 1 cycles ... '31': 32 cycles."]
    #[inline(always)]
    #[must_use]
    pub fn size5(&mut self) -> Size5W<WrDummyCtlSpec> {
        Size5W::new(self, 0)
    }
    #[doc = "Bit 31 - Presence of dummy cycles: '0': not present '1': present"]
    #[inline(always)]
    #[must_use]
    pub fn present(&mut self) -> PresentW<WrDummyCtlSpec> {
        PresentW::new(self, 31)
    }
}
#[doc = "Write dummy control\n\nYou can [`read`](crate::Reg::read) this register and get [`wr_dummy_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wr_dummy_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WrDummyCtlSpec;
impl crate::RegisterSpec for WrDummyCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wr_dummy_ctl::R`](R) reader structure"]
impl crate::Readable for WrDummyCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`wr_dummy_ctl::W`](W) writer structure"]
impl crate::Writable for WrDummyCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WR_DUMMY_CTL to value 0"]
impl crate::Resettable for WrDummyCtlSpec {
    const RESET_VALUE: u32 = 0;
}
