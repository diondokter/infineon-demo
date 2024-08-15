#[doc = "Register `PORT_SEL1` reader"]
pub type R = crate::R<PortSel1Spec>;
#[doc = "Register `PORT_SEL1` writer"]
pub type W = crate::W<PortSel1Spec>;
#[doc = "Field `IO4_SEL` reader - Selects connection for IO pin 4 route. See PORT_SEL0 for connection details."]
pub type Io4SelR = crate::FieldReader;
#[doc = "Field `IO4_SEL` writer - Selects connection for IO pin 4 route. See PORT_SEL0 for connection details."]
pub type Io4SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IO5_SEL` reader - Selects connection for IO pin 5 route."]
pub type Io5SelR = crate::FieldReader;
#[doc = "Field `IO5_SEL` writer - Selects connection for IO pin 5 route."]
pub type Io5SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IO6_SEL` reader - Selects connection for IO pin 6 route."]
pub type Io6SelR = crate::FieldReader;
#[doc = "Field `IO6_SEL` writer - Selects connection for IO pin 6 route."]
pub type Io6SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IO7_SEL` reader - Selects connection for IO pin 7 route."]
pub type Io7SelR = crate::FieldReader;
#[doc = "Field `IO7_SEL` writer - Selects connection for IO pin 7 route."]
pub type Io7SelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Selects connection for IO pin 4 route. See PORT_SEL0 for connection details."]
    #[inline(always)]
    pub fn io4_sel(&self) -> Io4SelR {
        Io4SelR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Selects connection for IO pin 5 route."]
    #[inline(always)]
    pub fn io5_sel(&self) -> Io5SelR {
        Io5SelR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Selects connection for IO pin 6 route."]
    #[inline(always)]
    pub fn io6_sel(&self) -> Io6SelR {
        Io6SelR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Selects connection for IO pin 7 route."]
    #[inline(always)]
    pub fn io7_sel(&self) -> Io7SelR {
        Io7SelR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selects connection for IO pin 4 route. See PORT_SEL0 for connection details."]
    #[inline(always)]
    #[must_use]
    pub fn io4_sel(&mut self) -> Io4SelW<PortSel1Spec> {
        Io4SelW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Selects connection for IO pin 5 route."]
    #[inline(always)]
    #[must_use]
    pub fn io5_sel(&mut self) -> Io5SelW<PortSel1Spec> {
        Io5SelW::new(self, 8)
    }
    #[doc = "Bits 16:20 - Selects connection for IO pin 6 route."]
    #[inline(always)]
    #[must_use]
    pub fn io6_sel(&mut self) -> Io6SelW<PortSel1Spec> {
        Io6SelW::new(self, 16)
    }
    #[doc = "Bits 24:28 - Selects connection for IO pin 7 route."]
    #[inline(always)]
    #[must_use]
    pub fn io7_sel(&mut self) -> Io7SelW<PortSel1Spec> {
        Io7SelW::new(self, 24)
    }
}
#[doc = "Port selection 1\n\nYou can [`read`](crate::Reg::read) this register and get [`port_sel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`port_sel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PortSel1Spec;
impl crate::RegisterSpec for PortSel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`port_sel1::R`](R) reader structure"]
impl crate::Readable for PortSel1Spec {}
#[doc = "`write(|w| ..)` method takes [`port_sel1::W`](W) writer structure"]
impl crate::Writable for PortSel1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PORT_SEL1 to value 0"]
impl crate::Resettable for PortSel1Spec {
    const RESET_VALUE: u32 = 0;
}
