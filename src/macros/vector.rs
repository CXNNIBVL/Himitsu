#[macro_export]
/// Create a new secure Vector inline
macro_rules! vector {

    // Default Vector of type and length
    ($type: ty) => {

        { 
            use $crate::util::Vector;
            Vector::<$type>::default() 
        }
    };

    // From default value and length as literal
    ($default_value: literal; $length: literal) => {

        {
            use $crate::util::Vector;
            let v = vec![$default_value; $length];
            
            Vector::from(v)
        }
    };

    // From default value and length as ident (e.g constant)
    ($default_value: literal; $length: ident) => {

        {
            use $crate::util::Vector;
            let v = vec![$default_value; $length];
            
            Vector::from(v)
        }

    };

    // From defined list
    ( $( $x:expr ),* ) => {
        {   
            use $crate::util::Vector;
            let vec = vec![ $( $x, )* ];
            let sec = Vector::from(vec);
            sec
        }
    };

}

#[cfg(test)]
mod tests {

    use crate::vector;

    #[test]
    fn test_macro_default_vector() {

        let vec = vector!(u8);
        let expected = Vec::<u8>::new();
        
        let rvec: Vec<u8> = vec.into();
        assert_eq!(rvec, expected);
    }

    #[test]
    fn test_macro_default_value_vector() {

        let vec = vector![0; 2];
        let expected = vec![0;2];

        let rvec: Vec<u8> = vec.into();
        assert_eq!(rvec, expected);
    }

    #[test]
    fn test_macro_array_from_list() {

        let vec = vector![1, 2, 3];
        let expected = [1, 2, 3];

        assert_eq!(vec.as_ref(), expected.as_ref());
    }

}