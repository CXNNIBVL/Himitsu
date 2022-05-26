#[macro_export]
/// Create a new secure array inline
macro_rules! array {

    // Default Array of type and length
    ($type: ty, $length: literal) => {

        { 
            use crate::util::secure::Array;
            Array::<$type, $length>::default() 
        }
    };

    // From default value and length
    ($default_value: literal; $length: literal) => {

        {
            let arr = [$default_value; $length];
            let sec = array!(arr);
            
            sec
        }

    };

    // From array identifier
    ($name: ident) => { 
        { 
            use crate::util::secure::Array;
            Array::from($name) 
        } 
    };

    // From defined list
    ( $( $x:expr ),* ) => {
        {
            let arr = [ $( $x, )* ];
            array!(arr)
        }
    };

}

#[cfg(test)]
mod tests {

    use crate::array;

    #[test]
    fn test_macro_default_array() {

        let arr = array![u8, 3];
        let expected = [0u8,0,0];

        assert_eq!(arr.as_ref(), expected.as_ref());
    }

    #[test]
    fn test_macro_default_value_array() {

        let arr = array!["Hello"; 2];
        let expected = ["Hello", "Hello"];

        assert_eq!(arr.as_ref(), expected.as_ref());
    }

    #[test]
    fn test_macro_array_from_ident() {

        let predef_arr = [0,1,2];
        let arr = array!(predef_arr);
        let expected = [0,1,2];

        assert_eq!(arr.as_ref(), expected.as_ref());
    }

    #[test]
    fn test_macro_array_from_list() {

        let arr = array![1, 2, 3];
        let expected = [1, 2, 3];

        assert_eq!(arr.as_ref(), expected.as_ref());
    }

}