#[doc = "Register `BUFF_CTL` reader"]
pub type R = crate::R<BuffCtlSpec>;
#[doc = "Register `BUFF_CTL` writer"]
pub type W = crate::W<BuffCtlSpec>;
#[doc = "Field `WRITE_BUFF` reader - Specifies if write transfer can be buffered in the bus infrastructure bridges: '0': Write transfers are not buffered, independent of the transfer's bufferable attribute. '1': Write transfers can be buffered, if the transfer's bufferable attribute indicates that the transfer is a bufferable/posted write."]
pub type WriteBuffR = crate::BitReader;
#[doc = "Field `WRITE_BUFF` writer - Specifies if write transfer can be buffered in the bus infrastructure bridges: '0': Write transfers are not buffered, independent of the transfer's bufferable attribute. '1': Write transfers can be buffered, if the transfer's bufferable attribute indicates that the transfer is a bufferable/posted write."]
pub type WriteBuffW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Specifies if write transfer can be buffered in the bus infrastructure bridges: '0': Write transfers are not buffered, independent of the transfer's bufferable attribute. '1': Write transfers can be buffered, if the transfer's bufferable attribute indicates that the transfer is a bufferable/posted write."]
    #[inline(always)]
    pub fn write_buff(&self) -> WriteBuffR {
        WriteBuffR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Specifies if write transfer can be buffered in the bus infrastructure bridges: '0': Write transfers are not buffered, independent of the transfer's bufferable attribute. '1': Write transfers can be buffered, if the transfer's bufferable attribute indicates that the transfer is a bufferable/posted write."]
    #[inline(always)]
    #[must_use]
    pub fn write_buff(&mut self) -> WriteBuffW<BuffCtlSpec> {
        WriteBuffW::new(self, 0)
    }
}
#[doc = "Buffer control\n\nYou can [`read`](crate::Reg::read) this register and get [`buff_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buff_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BuffCtlSpec;
impl crate::RegisterSpec for BuffCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buff_ctl::R`](R) reader structure"]
impl crate::Readable for BuffCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`buff_ctl::W`](W) writer structure"]
impl crate::Writable for BuffCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUFF_CTL to value 0x01"]
impl crate::Resettable for BuffCtlSpec {
    const RESET_VALUE: u32 = 0x01;
}
