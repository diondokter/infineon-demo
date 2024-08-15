#[doc = "Register `FM_PL_DATA[%s]` reader"]
pub type R = crate::R<FmPlDataSpec>;
#[doc = "Register `FM_PL_DATA[%s]` writer"]
pub type W = crate::W<FmPlDataSpec>;
#[doc = "Field `DATA32` reader - Four page latch Bytes When reading the page latches it requires FM_CTL.IF_SEL to be '1' Note: the high Voltage page latches are readable for test mode functionality."]
pub type Data32R = crate::FieldReader<u32>;
#[doc = "Field `DATA32` writer - Four page latch Bytes When reading the page latches it requires FM_CTL.IF_SEL to be '1' Note: the high Voltage page latches are readable for test mode functionality."]
pub type Data32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Four page latch Bytes When reading the page latches it requires FM_CTL.IF_SEL to be '1' Note: the high Voltage page latches are readable for test mode functionality."]
    #[inline(always)]
    pub fn data32(&self) -> Data32R {
        Data32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Four page latch Bytes When reading the page latches it requires FM_CTL.IF_SEL to be '1' Note: the high Voltage page latches are readable for test mode functionality."]
    #[inline(always)]
    #[must_use]
    pub fn data32(&mut self) -> Data32W<FmPlDataSpec> {
        Data32W::new(self, 0)
    }
}
#[doc = "Flash macro Page Latches data\n\nYou can [`read`](crate::Reg::read) this register and get [`fm_pl_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fm_pl_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmPlDataSpec;
impl crate::RegisterSpec for FmPlDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fm_pl_data::R`](R) reader structure"]
impl crate::Readable for FmPlDataSpec {}
#[doc = "`write(|w| ..)` method takes [`fm_pl_data::W`](W) writer structure"]
impl crate::Writable for FmPlDataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FM_PL_DATA[%s]
to value 0"]
impl crate::Resettable for FmPlDataSpec {
    const RESET_VALUE: u32 = 0;
}
