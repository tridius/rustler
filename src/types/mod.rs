use ::{
    NifEnv,
    NifError,
    NifTerm,
    NifResult,
};

#[macro_use]
pub mod atom;
pub mod binary;
pub mod list;
pub mod map;
pub mod primitive;
pub mod string;
pub mod tuple;
pub mod pid;

pub mod elixir_struct;

pub trait NifEncoder {
    fn encode<'a>(&self, env: NifEnv<'a>) -> NifTerm<'a>;
}
pub trait NifDecoder<'a>: Sized+'a {
    fn decode(term: NifTerm<'a>) -> NifResult<Self>;
}

impl<'a> NifEncoder for NifTerm<'a> {
    fn encode<'b>(&self, env: NifEnv<'b>) -> NifTerm<'b> {
        self.in_env(env)
    }
}
impl<'a> NifDecoder<'a> for NifTerm<'a> {
    fn decode(term: NifTerm<'a>) -> NifResult<Self> {
        println!("testing");
        Ok(term)
    }
}

impl<'a, T> NifEncoder for &'a T where T: NifEncoder {
    fn encode<'c>(&self, env: NifEnv<'c>) -> NifTerm<'c> {
        <T as NifEncoder>::encode(self, env)
    }
}

impl<T> NifEncoder for Option<T> where T: NifEncoder {
    fn encode<'c>(&self, env: NifEnv<'c>) -> NifTerm<'c> {
        match *self {
            Some(ref value) => value.encode(env),
            None => atom::nil().encode(env),
        }
    }
}

impl<'a, T> NifDecoder<'a> for Option<T> where T: NifDecoder<'a> {
    fn decode(term: NifTerm<'a>) -> NifResult<Self> {
        println!("another test");
        if let Ok(term) = term.decode::<T>() {
            Ok(Some(term))
        } else {
            let decoded_atom: atom::NifAtom = term.decode()?;
            if decoded_atom == atom::nil() {
                Ok(None)
            } else {
                Err(NifError::BadArg)
            }
        }
    }
}

impl<T, E> NifEncoder for Result<T, E> where T: NifEncoder, E: NifEncoder {
    fn encode<'c>(&self, env: NifEnv<'c>) -> NifTerm<'c> {
        match *self {
            Ok(ref value) => (atom::ok().encode(env), value.encode(env)).encode(env),
            Err(ref err) => (atom::error().encode(env), err.encode(env)).encode(env),
        }
    }
}

impl<'a, T, E> NifDecoder<'a> for Result<T, E> where T: NifDecoder<'a>, E: NifDecoder<'a> {
    fn decode(term: NifTerm<'a>) -> NifResult<Self> {
        println!("third test");
        let (decoded_atom, inner_term): (atom::NifAtom, NifTerm) = term.decode()?;
        if decoded_atom == atom::ok() {
            let ok_value: T = inner_term.decode()?;
            Ok(Ok(ok_value))
        } else if decoded_atom == atom::error() {
            let err_value: E = inner_term.decode()?;
            Ok(Err(err_value))
        } else {
            Err(NifError::BadArg)
        }
    }
}
