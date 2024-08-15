#[doc = "Register `EP0_DR[%s]` reader"]
pub type R = crate::R<Ep0DrSpec>;
#[doc = "Register `EP0_DR[%s]` writer"]
pub type W = crate::W<Ep0DrSpec>;
#[doc = "Field `DATA_BYTE` reader - This register is shared for both transmit and receive. The count in the EP0_CNT register determines the number of bytes received or to be transferred."]
pub type DataByteR = crate::FieldReader;
#[doc = "Field `DATA_BYTE` writer - This register is shared for both transmit and receive. The count in the EP0_CNT register determines the number of bytes received or to be transferred."]
pub type DataByteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - This register is shared for both transmit and receive. The count in the EP0_CNT register determines the number of bytes received or to be transferred."]
    #[inline(always)]
    pub fn data_byte(&self) -> DataByteR {
        DataByteR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - This register is shared for both transmit and receive. The count in the EP0_CNT register determines the number of bytes received or to be transferred."]
    #[inline(always)]
    #[must_use]
    pub fn data_byte(&mut self) -> DataByteW<Ep0DrSpec> {
        DataByteW::new(self, 0)
    }
}
#[doc = "Control End point EP0 Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ep0_dr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ep0_dr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ep0DrSpec;
impl crate::RegisterSpec for Ep0DrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep0_dr::R`](R) reader structure"]
impl crate::Readable for Ep0DrSpec {}
#[doc = "`write(|w| ..)` method takes [`ep0_dr::W`](W) writer structure"]
impl crate::Writable for Ep0DrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EP0_DR[%s]
to value 0"]
impl crate::Resettable for Ep0DrSpec {
    const RESET_VALUE: u32 = 0;
}
