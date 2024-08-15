#[doc = "Register `COUNTER` reader"]
pub type R = crate::R<CounterSpec>;
#[doc = "Register `COUNTER` writer"]
pub type W = crate::W<CounterSpec>;
#[doc = "Field `COUNTER` reader - 16-bit / 32-bit counter value. It is advised to not write to this field when the counter is running."]
pub type CounterR = crate::FieldReader<u32>;
#[doc = "Field `COUNTER` writer - 16-bit / 32-bit counter value. It is advised to not write to this field when the counter is running."]
pub type CounterW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 16-bit / 32-bit counter value. It is advised to not write to this field when the counter is running."]
    #[inline(always)]
    pub fn counter(&self) -> CounterR {
        CounterR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 16-bit / 32-bit counter value. It is advised to not write to this field when the counter is running."]
    #[inline(always)]
    #[must_use]
    pub fn counter(&mut self) -> CounterW<CounterSpec> {
        CounterW::new(self, 0)
    }
}
#[doc = "Counter count register\n\nYou can [`read`](crate::Reg::read) this register and get [`counter::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`counter::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CounterSpec;
impl crate::RegisterSpec for CounterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`counter::R`](R) reader structure"]
impl crate::Readable for CounterSpec {}
#[doc = "`write(|w| ..)` method takes [`counter::W`](W) writer structure"]
impl crate::Writable for CounterSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets COUNTER to value 0"]
impl crate::Resettable for CounterSpec {
    const RESET_VALUE: u32 = 0;
}
