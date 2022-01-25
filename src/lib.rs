pub mod runner;
pub mod game;
pub mod database;
pub mod time;

#[derive(Debug)]
#[allow(unused)]
pub struct ApplicationError {
    kind: String,
    message: String,
    cause: Option<Box<ApplicationError>>
}

impl ApplicationError {
    pub fn new(kind: &str, message: &str, cause: Option<Box<ApplicationError>>) -> Self {
        Self {
            kind: kind.into(),
            message: message.into(),
            cause
        }
    }
}

pub type AppResult<T> = Result<T, ApplicationError>;

#[macro_export]
macro_rules! trait_enum {
	($trait:ident, $enum:ident, $( $item:ident ) , *) => {
		pub enum $enum {
			$(
				$item($item),
			)*
		}

		impl std::ops::Deref for $enum {
			type Target = dyn $trait;

			fn deref(&self) -> &Self::Target {
				match self {
					$(
						$enum::$item(x) => x,
					)*
				}
			}
		}

        impl std::ops::DerefMut for $enum {
            fn deref_mut(&mut self) -> &mut Self::Target {
                match self {
					$(
						$enum::$item(x) => x,
					)*
				}
            }
        }

		impl From<$enum> for Box<dyn $trait> {
			fn from(input: $enum) -> Self {
				match input {
					$(
						$enum::$item(x) => Box::new(x),
					)*
				}
			}
		}

		impl<'a> From<&'a $enum> for &'a dyn $trait {
			fn from(input: &'a $enum) -> Self {
				&**input
			}
		}

		impl<'a> AsRef<dyn $trait + 'a> for $enum {
			fn as_ref(&self) -> &(dyn $trait + 'a) {
				&**self
			}
		}

		impl<'a> std::borrow::Borrow<dyn $trait + 'a> for $enum {
			fn borrow(&self) -> &(dyn $trait + 'a) {
				&**self
			}
		}

		$(
			impl From<$item> for $enum {
				fn from(input: $item) -> Self {
					$enum::$item(input)
				}
			}
		)*
	}
}
