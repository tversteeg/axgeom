
use core::ops::*;
use ordered_float::NotNan;
use crate::AxisTrait;
use num_traits::float::Float;
use core::convert::TryFrom;
use num_traits::Zero;
use primitive_from::PrimitiveFrom;




///Convenience function to create a vector.
#[inline(always)]
pub const fn vec2<N>(x:N,y:N)->Vec2<N>{
	Vec2{x,y}
}

///Convenience function to create a vector where both component are the same.
#[inline(always)]
pub fn vec2same<N:Copy>(a:N)->Vec2<N>{
	Vec2{x:a,y:a}
}



impl<N:Float> AsRef<Vec2<N>> for Vec2<NotNan<N>>{
	#[inline(always)]
	fn as_ref(&self)->&Vec2<N>{
		unsafe{&*((self as *const Self) as *const Vec2<N>)}
	}

}

impl<N:Float> AsMut<Vec2<N>> for Vec2<NotNan<N>>{
	#[inline(always)]
	fn as_mut(&mut self)->&mut Vec2<N>{
		unsafe{&mut *((self as *mut Self) as *mut Vec2<N>)}
	}

}

///A 2D vector.
#[derive(Copy,Clone,Debug,PartialEq,Eq,Hash)]
pub struct Vec2<N>{
	pub x:N,
	pub y:N
}



impl<S:Mul<Output=S> + Add<Output=S> + Copy> Vec2<S>{
	#[inline(always)]
	pub fn magnitude2(&self)->S{
		self.x*self.x+self.y*self.y
	}
	#[inline(always)]
	pub fn dot(&self,other:Vec2<S>)->S{
		self.x*other.x+self.y*other.y
	}
}
impl<S:Float> Vec2<S>{

	#[inline(always)]
	pub fn normalize_to(&self,mag:S)->Vec2<S>{
		let l=self.magnitude2().sqrt();
		(*self)*(mag/l)
	}

	#[inline(always)]
	pub fn magnitude(&self)->S{
		self.magnitude2().sqrt()
	}
}






///Cast an array of 2 elements of primitive type to another primitive type using "as" on each element.
pub fn arr2_as<B:Copy,A:PrimitiveFrom<B>>(a:[B;2])->[A;2]{
	[PrimitiveFrom::from(a[0]),PrimitiveFrom::from(a[1])]
}

impl<B:Copy> Vec2<B>{
	pub fn inner_as<A:PrimitiveFrom<B>>(&self)->Vec2<A>{
		vec2(PrimitiveFrom::from(self.x),PrimitiveFrom::from(self.y))
	}
}

impl<B> Vec2<B>{
	

     ///Get the range of one axis.
    #[inline(always)]
    pub fn get_axis(&self,axis:impl AxisTrait)->&B{
        if axis.is_xaxis(){
            &self.x
        }else{
            &self.y
        }
    }
    
    ///Get the mutable range of one axis.
    #[inline(always)]
    pub fn get_axis_mut(&mut self,axis:impl AxisTrait)->&mut B{
        if axis.is_xaxis(){
            &mut self.x
        }else{
            &mut self.y
        }
    }



	#[inline(always)]
	pub fn inner_into<A:From<B>>(self)->Vec2<A>{
	    let x=A::from(self.x);
	    let y=A::from(self.y);
	    vec2(x,y)
	}

	#[inline(always)]
	pub fn inner_try_into<A:TryFrom<B>>(self)->Result<Vec2<A>,A::Error>{
	    let x=A::try_from(self.x);
	    let y=A::try_from(self.y);
	    match(x,y){
	        (Ok(x),Ok(y))=>{
	            Ok(vec2(x,y))
	        },
	        (Ok(_),Err(e))=>{
	            Err(e)
	        },
	        (Err(e),Ok(_))=>{
	            Err(e)
	        },
	        (Err(e),Err(_))=>{
	            Err(e)
	        }
	    }
	}
}

/*
impl<S: NumCast + Copy> Vec2<S> {
    /// Component-wise casting to another type.
    #[inline]
    pub fn cast<T: NumCast>(&self) -> Option<Vec2<S>> {
    	let a=NumCast::from(self.x);
    	let b=NumCast::from(self.y);
    	match(a,b){
    		(Some(x),Some(y))=>{
    			Some(Vec2{x,y})
    		},
    		_=>{
    			None
    		}
    	}
    }
}
*/


impl<S:Add<Output=S> + Copy> Add<Self> for Vec2<S>{
	type Output=Self;
	#[inline(always)]
	fn add(self,rhs:Self)->Self{
		vec2(self.x+rhs.x,self.y+rhs.y)
	}
}

impl<S:Sub<Output=S> + Copy> Sub<Self> for Vec2<S>{
	type Output=Self;
	#[inline(always)]
	fn sub(self,rhs:Self)->Self{
		vec2(self.x-rhs.x,self.y-rhs.y)
	}
}


impl<S:Mul<Output=S> + Copy> Mul<S> for Vec2<S>{
	type Output=Self;
	#[inline(always)]
	fn mul(self,rhs:S)->Self{
		vec2(self.x*rhs,self.y*rhs)
	}
}

impl<S:Div<Output=S> + Copy> Div<S> for Vec2<S>{
	#[inline(always)]
	type Output=Self;
	fn div(self,rhs:S)->Self{
		vec2(self.x/rhs,self.y/rhs)
	}
}


impl<S:DivAssign<S> + Copy> DivAssign<S> for Vec2<S>{
	#[inline(always)]
	fn div_assign(&mut self,scalar:S){
		self.x/=scalar;
		self.y/=scalar;
	}	
}
impl<S:MulAssign<S> + Copy> MulAssign<S> for Vec2<S>{
	#[inline(always)]
	fn mul_assign(&mut self,scalar:S){
		self.x*=scalar;
		self.y*=scalar;
	}	
}

impl<S:AddAssign<S> + Copy> AddAssign<Self> for Vec2<S>{
	#[inline(always)]
	fn add_assign(&mut self,rhs:Self){
		self.x+=rhs.x;
		self.y+=rhs.y;
	}	
}
impl<S:SubAssign<S> + Copy> SubAssign<Self> for Vec2<S>{
	#[inline(always)]
	fn sub_assign(&mut self,rhs:Self){
		self.x-=rhs.x;
		self.y-=rhs.y;
	}	
}



impl<S: Neg<Output = S>> Neg for Vec2<S> {
    type Output = Vec2<S>;

    #[inline]
    fn neg(self) -> Vec2<S> { vec2(-self.x,-self.y) }
}


impl<S: Zero + Eq + Copy> Zero for Vec2<S> {
    #[inline(always)]
    fn zero() -> Vec2<S> {
    	vec2(S::zero(),S::zero())
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        *self == Vec2::zero()
    }
}
