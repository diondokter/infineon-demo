#[doc = "Register `TR_FIRO_CTL` reader"]
pub type R = crate::R<TrFiroCtlSpec>;
#[doc = "Register `TR_FIRO_CTL` writer"]
pub type W = crate::W<TrFiroCtlSpec>;
#[doc = "Field `POLYNOMIAL31` reader - Polynomial for programmable Fibonacci ring oscillator. The polynomial is represented WITHOUT the high order bit (this bit is always assumed '1'). The polynomial should be aligned such that the more significant bits (bit 30 and down) contain the polynomial and the less significant bits (bit 0 and up) contain padding '0's."]
pub type Polynomial31R = crate::FieldReader<u32>;
#[doc = "Field `POLYNOMIAL31` writer - Polynomial for programmable Fibonacci ring oscillator. The polynomial is represented WITHOUT the high order bit (this bit is always assumed '1'). The polynomial should be aligned such that the more significant bits (bit 30 and down) contain the polynomial and the less significant bits (bit 0 and up) contain padding '0's."]
pub type Polynomial31W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30 - Polynomial for programmable Fibonacci ring oscillator. The polynomial is represented WITHOUT the high order bit (this bit is always assumed '1'). The polynomial should be aligned such that the more significant bits (bit 30 and down) contain the polynomial and the less significant bits (bit 0 and up) contain padding '0's."]
    #[inline(always)]
    pub fn polynomial31(&self) -> Polynomial31R {
        Polynomial31R::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30 - Polynomial for programmable Fibonacci ring oscillator. The polynomial is represented WITHOUT the high order bit (this bit is always assumed '1'). The polynomial should be aligned such that the more significant bits (bit 30 and down) contain the polynomial and the less significant bits (bit 0 and up) contain padding '0's."]
    #[inline(always)]
    #[must_use]
    pub fn polynomial31(&mut self) -> Polynomial31W<TrFiroCtlSpec> {
        Polynomial31W::new(self, 0)
    }
}
#[doc = "True random FIRO control\n\nYou can [`read`](crate::Reg::read) this register and get [`tr_firo_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr_firo_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrFiroCtlSpec;
impl crate::RegisterSpec for TrFiroCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tr_firo_ctl::R`](R) reader structure"]
impl crate::Readable for TrFiroCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`tr_firo_ctl::W`](W) writer structure"]
impl crate::Writable for TrFiroCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TR_FIRO_CTL to value 0"]
impl crate::Resettable for TrFiroCtlSpec {
    const RESET_VALUE: u32 = 0;
}
