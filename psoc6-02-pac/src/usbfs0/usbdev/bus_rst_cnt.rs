#[doc = "Register `BUS_RST_CNT` reader"]
pub type R = crate::R<BusRstCntSpec>;
#[doc = "Register `BUS_RST_CNT` writer"]
pub type W = crate::W<BusRstCntSpec>;
#[doc = "Field `BUS_RST_CNT` reader - Bus Reset Count Length"]
pub type BusRstCntR = crate::FieldReader;
#[doc = "Field `BUS_RST_CNT` writer - Bus Reset Count Length"]
pub type BusRstCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Bus Reset Count Length"]
    #[inline(always)]
    pub fn bus_rst_cnt(&self) -> BusRstCntR {
        BusRstCntR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Bus Reset Count Length"]
    #[inline(always)]
    #[must_use]
    pub fn bus_rst_cnt(&mut self) -> BusRstCntW<BusRstCntSpec> {
        BusRstCntW::new(self, 0)
    }
}
#[doc = "Bus Reset Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bus_rst_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bus_rst_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BusRstCntSpec;
impl crate::RegisterSpec for BusRstCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_rst_cnt::R`](R) reader structure"]
impl crate::Readable for BusRstCntSpec {}
#[doc = "`write(|w| ..)` method takes [`bus_rst_cnt::W`](W) writer structure"]
impl crate::Writable for BusRstCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUS_RST_CNT to value 0x0a"]
impl crate::Resettable for BusRstCntSpec {
    const RESET_VALUE: u32 = 0x0a;
}
