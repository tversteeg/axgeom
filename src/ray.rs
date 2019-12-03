use crate::*;
use core::convert::TryFrom;
use core::cmp::Ordering;

///A Ray.
#[derive(Debug, Copy, Clone)]
pub struct Ray<N> {
    pub point: Vec2<N>,
    pub dir: Vec2<N>,
}

impl<N:Copy+core::ops::Add<Output=N>+core::ops::Mul<Output=N>> Ray<N>{

    #[inline(always)]
    pub fn point_at_tval(&self,tval:N)->Vec2<N>{
        self.point+self.dir*tval
    }
}
impl<N> Ray<N>{
    

    #[inline(always)]
    pub fn inner_into<B:From<N>>(self)->Ray<B>{
        let point=self.point.inner_into();
        let dir=self.dir.inner_into();
        Ray{point,dir}
    }
    #[inline(always)]
    pub fn inner_try_into<B:TryFrom<N>>(self)->Result<Ray<B>,B::Error>{
        let point=self.point.inner_try_into();
        let dir=self.dir.inner_try_into();
        match(point,dir){
            (Ok(point),Ok(dir))=>{
                Ok(Ray{point,dir})
            },
            (Err(e),Ok(_))=>{
                Err(e)
            },
            (Ok(_),Err(e))=>{
                Err(e)
            },
            (Err(e),Err(_))=>{
                Err(e)
            }
        }
    }
}

impl<N:PartialOrd + Copy> Ray<N>{
    pub fn range_side(&self,axis:impl Axis,range:&Range<N>)->Ordering{
        
        let v=if axis.is_xaxis(){
            self.point.x
        }else{
            self.point.y
        };

        range.contains_ext(v)
    }
}



///Describes if a ray hit a rectangle.
#[derive(Copy, Clone, Debug)]
pub enum CastResult<N> {
    Hit(N),
    Inside,
    NoHit,
}


use roots;
use roots::*;
impl<N:num_traits::Float + roots::FloatType> Ray<N>{
    ///Checks if a ray intersects a circle.
    pub fn cast_to_circle(
        &self,
        center: Vec2<N>,
        radius: N,
    ) -> CastResult<N> {
        //https://math.stackexchange.com/questions/311921/get-location-of-vector-circle-intersection
        //circle
        //(x-center.x)^2+(y-center.y)^2=r2
        //ray
        //x(t)=ray.dir.x*t+ray.point.x
        //y(t)=ray.dir.y*t+ray.point.y
        //
        //solve for t.
        //
        //
        //we get:
        //
        //𝑎𝑡^2+𝑏𝑡+𝑐=0
        //
        //
        //
        //
        let ray=self;
        let zz = <N as FloatType>::zero();
        let two = <N as FloatType>::two();

        let a = ray.dir.x.powi(2) + ray.dir.y.powi(2);
        let b = two * ray.dir.x * (ray.point.x - center.x) + two * ray.dir.y * (ray.point.y - center.y);
        let c = (ray.point.x - center.x).powi(2) + (ray.point.y - center.y).powi(2) - radius.powi(2);

        match find_roots_quadratic(a, b, c) {
            Roots::No(_) => CastResult::NoHit,
            Roots::One([a]) => {
                if a < zz {
                    CastResult::NoHit
                } else {
                    CastResult::Hit(a)
                }
            }
            Roots::Two([a, b]) => {
                let (closer, further) = if a < b { (a, b) } else { (b, a) };

                if closer < zz && further < zz {
                    CastResult::NoHit
                } else if closer < zz && further > zz {
                    CastResult::Inside
                } else {
                    CastResult::Hit(closer)
                }
            }
            _ => unreachable!(),
        }
    }
}




impl<N: num_traits::Num + num_traits::Signed + PartialOrd + Copy + Ord+ core::fmt::Debug> Ray<N>{
    ///Returns if a ray intersects a box.
    pub fn cast_to_rect(
        &self,
        rect: &Rect<N>,
    ) -> CastResult<N> {
        let ray=self;

        let next_grid_pos={

            vec2(if ray.dir.x<N::zero(){
                rect.x.end
            }else if ray.dir.x>N::zero(){
                rect.x.start
            }else{
                if rect.x.contains(ray.point.x){
                    let diff=rect.y.difference_to_point(ray.point.y);
                    match diff{
                        Some(diff)=>{
                            if diff.signum()==-ray.dir.y.signum(){
                                return CastResult::Hit(diff.abs())
                            }else{
                                return CastResult::NoHit
                            }
                        },
                        None=>{
                            return CastResult::Inside
                        }
                    }
                }else{
                    return CastResult::NoHit;
                }
            },
            if ray.dir.y<N::zero(){
                rect.y.end
            }else if ray.dir.y>N::zero(){
                rect.y.start
            }else{
                if rect.y.contains(ray.point.y){
                    let diff=rect.x.difference_to_point(ray.point.x);
                    match diff{
                        Some(diff)=>{
                            if diff.signum()==-ray.dir.x.signum(){
                                return CastResult::Hit(diff.abs())
                            }else{
                                return CastResult::NoHit
                            }
                        },
                        None=>{
                            return CastResult::Inside
                        }
                    }

                  
                }else{
                    return CastResult::NoHit;
                }
                
            })
        };


        let tvalx=(next_grid_pos.x-ray.point.x)/ray.dir.x;
        let tvaly=(next_grid_pos.y-ray.point.y)/ray.dir.y;

        let tvalx=if tvalx>N::zero(){Some(tvalx)}else{None};
        let tvaly=if tvaly>N::zero(){Some(tvaly)}else{None};
        
        match (tvalx,tvaly){
            (Some(x),Some(y))=>{
                
                let x = if rect.y.contains(ray.point_at_tval(x).y){
                    Some(x)
                }else{
                    None
                };

                let y = if rect.x.contains(ray.point_at_tval(y).x){
                    Some(y)
                }else{
                    None
                };


                match (x,y){
                    (Some(x),Some(y))=>{
                        CastResult::Hit(x.min(y))
                    },
                    (Some(x),None)=>{
                        CastResult::Hit(x)
                    },
                    (None,Some(y))=>{
                        CastResult::Hit(y)
                    },
                    (None,None)=>{
                        CastResult::NoHit
                    }

                }
                
            },
            (Some(x),None)=>{
                if rect.y.contains(ray.point_at_tval(x).y){
                    CastResult::Hit(x)
                }else{
                    CastResult::NoHit
                }
            },
            (None,Some(y))=>{
                if rect.x.contains(ray.point_at_tval(y).x){
                    CastResult::Hit(y)
                }else{
                    CastResult::NoHit
                }
            },
            (None,None)=>{
                if rect.contains_point(ray.point){
                    CastResult::Inside
                }else{
                    CastResult::NoHit
                }
            }
        }
    }
}