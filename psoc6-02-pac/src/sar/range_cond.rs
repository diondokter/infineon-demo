#[doc = "Register `RANGE_COND` reader"]
pub type R = crate::R<RangeCondSpec>;
#[doc = "Register `RANGE_COND` writer"]
pub type W = crate::W<RangeCondSpec>;
#[doc = "Range condition select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RangeCond {
    #[doc = "0: result &lt; RANGE_LOW"]
    Below = 0,
    #[doc = "1: RANGE_LOW &lt;= result &lt; RANGE_HIGH"]
    Inside = 1,
    #[doc = "2: RANGE_HIGH &lt;= result"]
    Above = 2,
    #[doc = "3: result &lt; RANGE_LOW || RANGE_HIGH &lt;= result"]
    Outside = 3,
}
impl From<RangeCond> for u8 {
    #[inline(always)]
    fn from(variant: RangeCond) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RangeCond {
    type Ux = u8;
}
impl crate::IsEnum for RangeCond {}
#[doc = "Field `RANGE_COND` reader - Range condition select."]
pub type RangeCondR = crate::FieldReader<RangeCond>;
impl RangeCondR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RangeCond {
        match self.bits {
            0 => RangeCond::Below,
            1 => RangeCond::Inside,
            2 => RangeCond::Above,
            3 => RangeCond::Outside,
            _ => unreachable!(),
        }
    }
    #[doc = "result &lt; RANGE_LOW"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == RangeCond::Below
    }
    #[doc = "RANGE_LOW &lt;= result &lt; RANGE_HIGH"]
    #[inline(always)]
    pub fn is_inside(&self) -> bool {
        *self == RangeCond::Inside
    }
    #[doc = "RANGE_HIGH &lt;= result"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == RangeCond::Above
    }
    #[doc = "result &lt; RANGE_LOW || RANGE_HIGH &lt;= result"]
    #[inline(always)]
    pub fn is_outside(&self) -> bool {
        *self == RangeCond::Outside
    }
}
#[doc = "Field `RANGE_COND` writer - Range condition select."]
pub type RangeCondW<'a, REG> = crate::FieldWriter<'a, REG, 2, RangeCond, crate::Safe>;
impl<'a, REG> RangeCondW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "result &lt; RANGE_LOW"]
    #[inline(always)]
    pub fn below(self) -> &'a mut crate::W<REG> {
        self.variant(RangeCond::Below)
    }
    #[doc = "RANGE_LOW &lt;= result &lt; RANGE_HIGH"]
    #[inline(always)]
    pub fn inside(self) -> &'a mut crate::W<REG> {
        self.variant(RangeCond::Inside)
    }
    #[doc = "RANGE_HIGH &lt;= result"]
    #[inline(always)]
    pub fn above(self) -> &'a mut crate::W<REG> {
        self.variant(RangeCond::Above)
    }
    #[doc = "result &lt; RANGE_LOW || RANGE_HIGH &lt;= result"]
    #[inline(always)]
    pub fn outside(self) -> &'a mut crate::W<REG> {
        self.variant(RangeCond::Outside)
    }
}
impl R {
    #[doc = "Bits 30:31 - Range condition select."]
    #[inline(always)]
    pub fn range_cond(&self) -> RangeCondR {
        RangeCondR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Range condition select."]
    #[inline(always)]
    #[must_use]
    pub fn range_cond(&mut self) -> RangeCondW<RangeCondSpec> {
        RangeCondW::new(self, 30)
    }
}
#[doc = "Global range detect mode register.\n\nYou can [`read`](crate::Reg::read) this register and get [`range_cond::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`range_cond::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RangeCondSpec;
impl crate::RegisterSpec for RangeCondSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`range_cond::R`](R) reader structure"]
impl crate::Readable for RangeCondSpec {}
#[doc = "`write(|w| ..)` method takes [`range_cond::W`](W) writer structure"]
impl crate::Writable for RangeCondSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RANGE_COND to value 0"]
impl crate::Resettable for RangeCondSpec {
    const RESET_VALUE: u32 = 0;
}
