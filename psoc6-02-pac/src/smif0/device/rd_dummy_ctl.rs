#[doc = "Register `RD_DUMMY_CTL` reader"]
pub type R = crate::R<RdDummyCtlSpec>;
#[doc = "Register `RD_DUMMY_CTL` writer"]
pub type W = crate::W<RdDummyCtlSpec>;
#[doc = "Field `SIZE5` reader - Number of dummy cycles (minus 1): '0': 1 cycles ... '31': 32 cycles. Note: this field specifies dummy cycles, not dummy Bytes!"]
pub type Size5R = crate::FieldReader;
#[doc = "Field `SIZE5` writer - Number of dummy cycles (minus 1): '0': 1 cycles ... '31': 32 cycles. Note: this field specifies dummy cycles, not dummy Bytes!"]
pub type Size5W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PRESENT` reader - Presence of dummy cycles: '0': not present '1': present"]
pub type PresentR = crate::BitReader;
#[doc = "Field `PRESENT` writer - Presence of dummy cycles: '0': not present '1': present"]
pub type PresentW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Number of dummy cycles (minus 1): '0': 1 cycles ... '31': 32 cycles. Note: this field specifies dummy cycles, not dummy Bytes!"]
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
    #[doc = "Bits 0:4 - Number of dummy cycles (minus 1): '0': 1 cycles ... '31': 32 cycles. Note: this field specifies dummy cycles, not dummy Bytes!"]
    #[inline(always)]
    #[must_use]
    pub fn size5(&mut self) -> Size5W<RdDummyCtlSpec> {
        Size5W::new(self, 0)
    }
    #[doc = "Bit 31 - Presence of dummy cycles: '0': not present '1': present"]
    #[inline(always)]
    #[must_use]
    pub fn present(&mut self) -> PresentW<RdDummyCtlSpec> {
        PresentW::new(self, 31)
    }
}
#[doc = "Read dummy control\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_dummy_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_dummy_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdDummyCtlSpec;
impl crate::RegisterSpec for RdDummyCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_dummy_ctl::R`](R) reader structure"]
impl crate::Readable for RdDummyCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`rd_dummy_ctl::W`](W) writer structure"]
impl crate::Writable for RdDummyCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RD_DUMMY_CTL to value 0"]
impl crate::Resettable for RdDummyCtlSpec {
    const RESET_VALUE: u32 = 0;
}
