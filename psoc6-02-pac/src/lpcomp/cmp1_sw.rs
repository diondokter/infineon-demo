#[doc = "Register `CMP1_SW` reader"]
pub type R = crate::R<Cmp1SwSpec>;
#[doc = "Register `CMP1_SW` writer"]
pub type W = crate::W<Cmp1SwSpec>;
#[doc = "Field `CMP1_IP1` reader - Comparator 1 positive terminal isolation switch to GPIO"]
pub type Cmp1Ip1R = crate::BitReader;
#[doc = "Field `CMP1_IP1` writer - Comparator 1 positive terminal isolation switch to GPIO"]
pub type Cmp1Ip1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1_AP1` reader - Comparator 1 positive terminal switch to amuxbusA"]
pub type Cmp1Ap1R = crate::BitReader;
#[doc = "Field `CMP1_AP1` writer - Comparator 1 positive terminal switch to amuxbusA"]
pub type Cmp1Ap1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1_BP1` reader - Comparator 1 positive terminal switch to amuxbusB"]
pub type Cmp1Bp1R = crate::BitReader;
#[doc = "Field `CMP1_BP1` writer - Comparator 1 positive terminal switch to amuxbusB"]
pub type Cmp1Bp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1_IN1` reader - Comparator 1 negative terminal isolation switch to GPIO"]
pub type Cmp1In1R = crate::BitReader;
#[doc = "Field `CMP1_IN1` writer - Comparator 1 negative terminal isolation switch to GPIO"]
pub type Cmp1In1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1_AN1` reader - Comparator 1 negative terminal switch to amuxbusA"]
pub type Cmp1An1R = crate::BitReader;
#[doc = "Field `CMP1_AN1` writer - Comparator 1 negative terminal switch to amuxbusA"]
pub type Cmp1An1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1_BN1` reader - Comparator 1 negative terminal switch to amuxbusB"]
pub type Cmp1Bn1R = crate::BitReader;
#[doc = "Field `CMP1_BN1` writer - Comparator 1 negative terminal switch to amuxbusB"]
pub type Cmp1Bn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1_VN1` reader - Comparator 1 negative terminal switch to local Vref (LPREF_EN must be set)"]
pub type Cmp1Vn1R = crate::BitReader;
#[doc = "Field `CMP1_VN1` writer - Comparator 1 negative terminal switch to local Vref (LPREF_EN must be set)"]
pub type Cmp1Vn1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator 1 positive terminal isolation switch to GPIO"]
    #[inline(always)]
    pub fn cmp1_ip1(&self) -> Cmp1Ip1R {
        Cmp1Ip1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 positive terminal switch to amuxbusA"]
    #[inline(always)]
    pub fn cmp1_ap1(&self) -> Cmp1Ap1R {
        Cmp1Ap1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator 1 positive terminal switch to amuxbusB"]
    #[inline(always)]
    pub fn cmp1_bp1(&self) -> Cmp1Bp1R {
        Cmp1Bp1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Comparator 1 negative terminal isolation switch to GPIO"]
    #[inline(always)]
    pub fn cmp1_in1(&self) -> Cmp1In1R {
        Cmp1In1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comparator 1 negative terminal switch to amuxbusA"]
    #[inline(always)]
    pub fn cmp1_an1(&self) -> Cmp1An1R {
        Cmp1An1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Comparator 1 negative terminal switch to amuxbusB"]
    #[inline(always)]
    pub fn cmp1_bn1(&self) -> Cmp1Bn1R {
        Cmp1Bn1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Comparator 1 negative terminal switch to local Vref (LPREF_EN must be set)"]
    #[inline(always)]
    pub fn cmp1_vn1(&self) -> Cmp1Vn1R {
        Cmp1Vn1R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 1 positive terminal isolation switch to GPIO"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1_ip1(&mut self) -> Cmp1Ip1W<Cmp1SwSpec> {
        Cmp1Ip1W::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator 1 positive terminal switch to amuxbusA"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1_ap1(&mut self) -> Cmp1Ap1W<Cmp1SwSpec> {
        Cmp1Ap1W::new(self, 1)
    }
    #[doc = "Bit 2 - Comparator 1 positive terminal switch to amuxbusB"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1_bp1(&mut self) -> Cmp1Bp1W<Cmp1SwSpec> {
        Cmp1Bp1W::new(self, 2)
    }
    #[doc = "Bit 4 - Comparator 1 negative terminal isolation switch to GPIO"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1_in1(&mut self) -> Cmp1In1W<Cmp1SwSpec> {
        Cmp1In1W::new(self, 4)
    }
    #[doc = "Bit 5 - Comparator 1 negative terminal switch to amuxbusA"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1_an1(&mut self) -> Cmp1An1W<Cmp1SwSpec> {
        Cmp1An1W::new(self, 5)
    }
    #[doc = "Bit 6 - Comparator 1 negative terminal switch to amuxbusB"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1_bn1(&mut self) -> Cmp1Bn1W<Cmp1SwSpec> {
        Cmp1Bn1W::new(self, 6)
    }
    #[doc = "Bit 7 - Comparator 1 negative terminal switch to local Vref (LPREF_EN must be set)"]
    #[inline(always)]
    #[must_use]
    pub fn cmp1_vn1(&mut self) -> Cmp1Vn1W<Cmp1SwSpec> {
        Cmp1Vn1W::new(self, 7)
    }
}
#[doc = "Comparator 1 switch control\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp1_sw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1_sw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmp1SwSpec;
impl crate::RegisterSpec for Cmp1SwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp1_sw::R`](R) reader structure"]
impl crate::Readable for Cmp1SwSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp1_sw::W`](W) writer structure"]
impl crate::Writable for Cmp1SwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMP1_SW to value 0"]
impl crate::Resettable for Cmp1SwSpec {
    const RESET_VALUE: u32 = 0;
}
