#[doc = "Register `CHAN_EN` reader"]
pub type R = crate::R<ChanEnSpec>;
#[doc = "Register `CHAN_EN` writer"]
pub type W = crate::W<ChanEnSpec>;
#[doc = "Field `CHAN_EN` reader - Channel enable. - 0: the corresponding channel is disabled. - 1: the corresponding channel is enabled, it will be included in the next scan."]
pub type ChanEnR = crate::FieldReader<u16>;
#[doc = "Field `CHAN_EN` writer - Channel enable. - 0: the corresponding channel is disabled. - 1: the corresponding channel is enabled, it will be included in the next scan."]
pub type ChanEnW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Channel enable. - 0: the corresponding channel is disabled. - 1: the corresponding channel is enabled, it will be included in the next scan."]
    #[inline(always)]
    pub fn chan_en(&self) -> ChanEnR {
        ChanEnR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Channel enable. - 0: the corresponding channel is disabled. - 1: the corresponding channel is enabled, it will be included in the next scan."]
    #[inline(always)]
    #[must_use]
    pub fn chan_en(&mut self) -> ChanEnW<ChanEnSpec> {
        ChanEnW::new(self, 0)
    }
}
#[doc = "Enable bits for the channels\n\nYou can [`read`](crate::Reg::read) this register and get [`chan_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chan_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChanEnSpec;
impl crate::RegisterSpec for ChanEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chan_en::R`](R) reader structure"]
impl crate::Readable for ChanEnSpec {}
#[doc = "`write(|w| ..)` method takes [`chan_en::W`](W) writer structure"]
impl crate::Writable for ChanEnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHAN_EN to value 0"]
impl crate::Resettable for ChanEnSpec {
    const RESET_VALUE: u32 = 0;
}
