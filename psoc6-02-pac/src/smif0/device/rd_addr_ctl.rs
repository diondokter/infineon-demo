#[doc = "Register `RD_ADDR_CTL` reader"]
pub type R = crate::R<RdAddrCtlSpec>;
#[doc = "Register `RD_ADDR_CTL` writer"]
pub type W = crate::W<RdAddrCtlSpec>;
#[doc = "Field `WIDTH` reader - Width of transfer."]
pub type WidthR = crate::FieldReader;
#[doc = "Field `WIDTH` writer - Width of transfer."]
pub type WidthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 16:17 - Width of transfer."]
    #[inline(always)]
    pub fn width(&self) -> WidthR {
        WidthR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17 - Width of transfer."]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WidthW<RdAddrCtlSpec> {
        WidthW::new(self, 16)
    }
}
#[doc = "Read address control\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_addr_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_addr_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdAddrCtlSpec;
impl crate::RegisterSpec for RdAddrCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_addr_ctl::R`](R) reader structure"]
impl crate::Readable for RdAddrCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`rd_addr_ctl::W`](W) writer structure"]
impl crate::Writable for RdAddrCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RD_ADDR_CTL to value 0"]
impl crate::Resettable for RdAddrCtlSpec {
    const RESET_VALUE: u32 = 0;
}
