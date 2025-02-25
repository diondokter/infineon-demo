#[doc = "Register `IN` reader"]
pub type R = crate::R<InSpec>;
#[doc = "Field `IN0` reader - IO pin state for pin 0 '0': Low logic level present on pin. '1': High logic level present on pin. On reset assertion , IN register will get reset. The Pad value takes 2 clock cycles to be reflected into IN Register. It's value then depends on the external pin value."]
pub type In0R = crate::BitReader;
#[doc = "Field `IN1` reader - IO pin state for pin 1"]
pub type In1R = crate::BitReader;
#[doc = "Field `IN2` reader - IO pin state for pin 2"]
pub type In2R = crate::BitReader;
#[doc = "Field `IN3` reader - IO pin state for pin 3"]
pub type In3R = crate::BitReader;
#[doc = "Field `IN4` reader - IO pin state for pin 4"]
pub type In4R = crate::BitReader;
#[doc = "Field `IN5` reader - IO pin state for pin 5"]
pub type In5R = crate::BitReader;
#[doc = "Field `IN6` reader - IO pin state for pin 6"]
pub type In6R = crate::BitReader;
#[doc = "Field `IN7` reader - IO pin state for pin 7"]
pub type In7R = crate::BitReader;
#[doc = "Field `FLT_IN` reader - Reads of this register return the logical state of the filtered pin as selected in the INTR_CFG.FLT_SEL register."]
pub type FltInR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - IO pin state for pin 0 '0': Low logic level present on pin. '1': High logic level present on pin. On reset assertion , IN register will get reset. The Pad value takes 2 clock cycles to be reflected into IN Register. It's value then depends on the external pin value."]
    #[inline(always)]
    pub fn in0(&self) -> In0R {
        In0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO pin state for pin 1"]
    #[inline(always)]
    pub fn in1(&self) -> In1R {
        In1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO pin state for pin 2"]
    #[inline(always)]
    pub fn in2(&self) -> In2R {
        In2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO pin state for pin 3"]
    #[inline(always)]
    pub fn in3(&self) -> In3R {
        In3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO pin state for pin 4"]
    #[inline(always)]
    pub fn in4(&self) -> In4R {
        In4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO pin state for pin 5"]
    #[inline(always)]
    pub fn in5(&self) -> In5R {
        In5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO pin state for pin 6"]
    #[inline(always)]
    pub fn in6(&self) -> In6R {
        In6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO pin state for pin 7"]
    #[inline(always)]
    pub fn in7(&self) -> In7R {
        In7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reads of this register return the logical state of the filtered pin as selected in the INTR_CFG.FLT_SEL register."]
    #[inline(always)]
    pub fn flt_in(&self) -> FltInR {
        FltInR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Port input state register\n\nYou can [`read`](crate::Reg::read) this register and get [`in_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InSpec;
impl crate::RegisterSpec for InSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_::R`](R) reader structure"]
impl crate::Readable for InSpec {}
#[doc = "`reset()` method sets IN to value 0"]
impl crate::Resettable for InSpec {
    const RESET_VALUE: u32 = 0;
}
