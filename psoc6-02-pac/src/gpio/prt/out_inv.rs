#[doc = "Register `OUT_INV` reader"]
pub type R = crate::R<OutInvSpec>;
#[doc = "Register `OUT_INV` writer"]
pub type W = crate::W<OutInvSpec>;
#[doc = "Field `OUT0` reader - IO invert output for pin 0: '0': Output state not affected. '1': Output state inverted ('0' => '1', '1' => '0')."]
pub type Out0R = crate::BitReader;
#[doc = "Field `OUT0` writer - IO invert output for pin 0: '0': Output state not affected. '1': Output state inverted ('0' => '1', '1' => '0')."]
pub type Out0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT1` reader - IO invert output for pin 1"]
pub type Out1R = crate::BitReader;
#[doc = "Field `OUT1` writer - IO invert output for pin 1"]
pub type Out1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT2` reader - IO invert output for pin 2"]
pub type Out2R = crate::BitReader;
#[doc = "Field `OUT2` writer - IO invert output for pin 2"]
pub type Out2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT3` reader - IO invert output for pin 3"]
pub type Out3R = crate::BitReader;
#[doc = "Field `OUT3` writer - IO invert output for pin 3"]
pub type Out3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT4` reader - IO invert output for pin 4"]
pub type Out4R = crate::BitReader;
#[doc = "Field `OUT4` writer - IO invert output for pin 4"]
pub type Out4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT5` reader - IO invert output for pin 5"]
pub type Out5R = crate::BitReader;
#[doc = "Field `OUT5` writer - IO invert output for pin 5"]
pub type Out5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT6` reader - IO invert output for pin 6"]
pub type Out6R = crate::BitReader;
#[doc = "Field `OUT6` writer - IO invert output for pin 6"]
pub type Out6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT7` reader - IO invert output for pin 7"]
pub type Out7R = crate::BitReader;
#[doc = "Field `OUT7` writer - IO invert output for pin 7"]
pub type Out7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IO invert output for pin 0: '0': Output state not affected. '1': Output state inverted ('0' => '1', '1' => '0')."]
    #[inline(always)]
    pub fn out0(&self) -> Out0R {
        Out0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO invert output for pin 1"]
    #[inline(always)]
    pub fn out1(&self) -> Out1R {
        Out1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO invert output for pin 2"]
    #[inline(always)]
    pub fn out2(&self) -> Out2R {
        Out2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO invert output for pin 3"]
    #[inline(always)]
    pub fn out3(&self) -> Out3R {
        Out3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO invert output for pin 4"]
    #[inline(always)]
    pub fn out4(&self) -> Out4R {
        Out4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO invert output for pin 5"]
    #[inline(always)]
    pub fn out5(&self) -> Out5R {
        Out5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO invert output for pin 6"]
    #[inline(always)]
    pub fn out6(&self) -> Out6R {
        Out6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO invert output for pin 7"]
    #[inline(always)]
    pub fn out7(&self) -> Out7R {
        Out7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO invert output for pin 0: '0': Output state not affected. '1': Output state inverted ('0' => '1', '1' => '0')."]
    #[inline(always)]
    #[must_use]
    pub fn out0(&mut self) -> Out0W<OutInvSpec> {
        Out0W::new(self, 0)
    }
    #[doc = "Bit 1 - IO invert output for pin 1"]
    #[inline(always)]
    #[must_use]
    pub fn out1(&mut self) -> Out1W<OutInvSpec> {
        Out1W::new(self, 1)
    }
    #[doc = "Bit 2 - IO invert output for pin 2"]
    #[inline(always)]
    #[must_use]
    pub fn out2(&mut self) -> Out2W<OutInvSpec> {
        Out2W::new(self, 2)
    }
    #[doc = "Bit 3 - IO invert output for pin 3"]
    #[inline(always)]
    #[must_use]
    pub fn out3(&mut self) -> Out3W<OutInvSpec> {
        Out3W::new(self, 3)
    }
    #[doc = "Bit 4 - IO invert output for pin 4"]
    #[inline(always)]
    #[must_use]
    pub fn out4(&mut self) -> Out4W<OutInvSpec> {
        Out4W::new(self, 4)
    }
    #[doc = "Bit 5 - IO invert output for pin 5"]
    #[inline(always)]
    #[must_use]
    pub fn out5(&mut self) -> Out5W<OutInvSpec> {
        Out5W::new(self, 5)
    }
    #[doc = "Bit 6 - IO invert output for pin 6"]
    #[inline(always)]
    #[must_use]
    pub fn out6(&mut self) -> Out6W<OutInvSpec> {
        Out6W::new(self, 6)
    }
    #[doc = "Bit 7 - IO invert output for pin 7"]
    #[inline(always)]
    #[must_use]
    pub fn out7(&mut self) -> Out7W<OutInvSpec> {
        Out7W::new(self, 7)
    }
}
#[doc = "Port output data invert register\n\nYou can [`read`](crate::Reg::read) this register and get [`out_inv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_inv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutInvSpec;
impl crate::RegisterSpec for OutInvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_inv::R`](R) reader structure"]
impl crate::Readable for OutInvSpec {}
#[doc = "`write(|w| ..)` method takes [`out_inv::W`](W) writer structure"]
impl crate::Writable for OutInvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_INV to value 0"]
impl crate::Resettable for OutInvSpec {
    const RESET_VALUE: u32 = 0;
}
