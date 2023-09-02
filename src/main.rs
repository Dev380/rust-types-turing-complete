#![recursion_limit = "512"]

use std::marker::PhantomData;
use std::fmt::Debug;

trait Function<Argument> {
	type Output;
}

#[derive(Debug)]
struct Identity;

impl<Argument> Function<Argument> for Identity {
	type Output = Argument;
}

#[derive(Debug)]
struct Kilometre;

impl<Argument> Function<Argument> for Kilometre {
	type Output = AlwaysReturnConstant<Argument>;
}

#[derive(Debug)]
struct AlwaysReturnConstant<Constant>(PhantomData<Constant>);

impl<Constant, Argument> Function<Argument> for AlwaysReturnConstant<Constant> {
	type Output = Constant;
}

#[derive(Debug)]
struct Substitute;

impl<Argument> Function<Argument> for Substitute {
	type Output = SubstituteAfterFirstArgument<Argument>;
}

#[derive(Debug)]
struct SubstituteAfterFirstArgument<Arg1>(PhantomData<Arg1>);

impl<Arg1, Argument> Function<Argument> for SubstituteAfterFirstArgument<Arg1> {
	type Output = SubstituteAfterSecondArgument<Arg1, Argument>;
}

#[derive(Debug)]
struct SubstituteAfterSecondArgument<Arg1, Arg2>(PhantomData<Arg1>, PhantomData<Arg2>);

impl<Arg1: Function<Arg3>, Arg2: Function<Arg3>, Arg3> Function<Arg3> for SubstituteAfterSecondArgument<Arg1, Arg2>
where <Arg1 as Function<Arg3>>::Output: Function<<Arg2 as Function<Arg3>>::Output>
{
	type Output = <<Arg1 as Function<Arg3>>::Output as Function<<Arg2 as Function<Arg3>>::Output>>::Output;
}


// Boring stuff we have to do.  If we don't do this, then
// we'll have
// let result = Type; // since Output is an associated type
// which isn't allowed, so we need to convert it into an actual
// struct at runtime.

trait Construct {
	fn construct() -> Self;
}

impl Construct for Identity {
	fn construct() -> Self {
		Identity
	}
}

impl Construct for Kilometre {
	fn construct() -> Self {
		Kilometre
	}
}

impl Construct for Substitute {
	fn construct() -> Self {
		Substitute
	}
}

impl<T> Construct for AlwaysReturnConstant<T> {
	fn construct() -> Self {
		AlwaysReturnConstant(PhantomData)
	}
}

impl<T> Construct for SubstituteAfterFirstArgument<T> {
	fn construct() -> Self {
		SubstituteAfterFirstArgument(PhantomData)
	}
}

impl<T, U> Construct for SubstituteAfterSecondArgument<T, U> {
	fn construct() -> Self {
		SubstituteAfterSecondArgument(PhantomData, PhantomData)
	}
}

type Call<Func, Argument> = <Func as Function<Argument>>::Output;

fn main() {
	// I(K) = K
	let result = Call::<Identity, Kilometre>::construct();
	println!("IK = {result:?}");

	// SKSK = KKSK = K
	// Order of operations is right-to-left in SKI!
	let result = Call::<Call::<Call::<Substitute, Kilometre> ,Substitute>, Kilometre>::construct();
	println!("SKSK = {result:?}");

	// Infinite recursion! SII(anything) = (anything)(anything), similar to
	// (x.(x)(x)) in lambda calculus
	#[allow(dead_code)]
	type OmegaCombinator = Call::<Call::<Substitute, Identity>, Identity>;

	// Uncomment this to get an overflow error at compile time: E0275
	// println!("Infinite loop = {:?}", Call::<OmegaCombinator, OmegaCombinator>::construct());

	// Church numerals! We get integers now! It's kind of like peano but in lambda calculus.
	// If you need compile time numbers and aren't trolling like me, use typenum which uses
	// binary to speed it up. But Church numerals are nicer so I'm going to use that!

	// zero = f(x,y)(y) = f(x)(f(y)(y)) = f(x)(Identity) = KI
	// (because whatever the second arg is it'll return Identity. K acts like a "constant function factory")
	type Zero = Call::<Kilometre, Identity>;

	// need to define the succesor function:
	// succ(n) = f(x)(xn)
	// we can get f(x) = f(y)(xy), and invert it with S(K(SI))(K)
	type Invert = Call::<Substitute, Call::<Call::<Kilometre, Call::<Substitute, Identity>>, Kilometre>>;
	type Succ<Previous> = Call::<Invert, Previous>;

	// Numbers!
	type One = Succ<Zero>;
	type Two = Succ<One>;
	type Three = Succ<Two>;
	type Twelve = Succ<Succ<Succ<Succ<Succ<Succ<Succ<Succ<Succ<Succ<Succ<Succ<Zero>>>>>>>>>>>>;

	// Calling a number as a function with a function as an argument returns that function composing
	// over itself that many times, e.g. calling three(function) produces f(x)=function(function(function(x)))
	// So by calling any of them with arguments identity, kilometre it should return kilometre since identity of kilometre
	// any amount of times is still kilometre

	println!();
	println!("Zero: {:?}", Call::<Call::<Zero, Identity>, Kilometre>::construct());
	println!("One: {:?}", Call::<Call::<One, Identity>, Kilometre>::construct());
	println!("Two: {:?}", Call::<Call::<Two, Identity>, Kilometre>::construct());
	println!("Three: {:?}", Call::<Call::<Three, Identity>, Kilometre>::construct());
	println!("Twelve: {:?}", Call::<Call::<Twelve, Identity>, Kilometre>::construct());

	// That's it folks, my PC cannot handle any arithmetic because this is painfully inefficient
	// Also I'm 50% sure my implementation is wrong but oh well
}
