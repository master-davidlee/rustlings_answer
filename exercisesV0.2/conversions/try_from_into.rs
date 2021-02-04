// TryFrom is a simple and safe type conversion that may fail in a controlled way under some circumstances.
// Basically, this is the same as From. The main difference is that this should return a Result type
// instead of the target type itself.
// You can read more about it at https://doc.rust-lang.org/std/convert/trait.TryFrom.html
use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}



// Your task is to complete this implementation
// and return an Ok result of inner type Color.
// You need to create an implementation for a tuple of three integers,
// an array of three integers and a slice of integers.
//
// Note that the implementation for tuple and array will be checked at compile time,
// but the slice implementation needs to check the slice length!
// Also note that correct RGB color values must be integers in the 0..=255 range.

// Tuple implementation
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = String;
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        if tuple.0 > 255 || tuple.1 > 255 || tuple.2 > 255 || tuple.0 < 0 || tuple.1 < 0 || tuple.2 < 0{
            return Err(String::from("should between 0-255"));
        }
         Ok(Color{
            red: tuple.0 as u8,
            green: tuple.1 as u8,
            blue: tuple.2 as u8,
        })
    }
}

// Array implementation
impl TryFrom<[i16; 3]> for Color {
    type Error = String;
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        
            if arr[0] < 0 || arr[1] < 0 || arr[2] < 0 || arr[0]>255 || arr[1]>255 || arr[2]>255 {
                return Err(String::from("should be between 0-255"));
            }
            Ok(
            Color{
                red: arr[0] as u8,
                green: arr[1] as u8,
                blue: arr[2] as u8
            }
            )
    }
}

// Slice implementation
impl TryFrom<&[i16]> for Color {
    type Error = String;
    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() !=3||slice[0] > 255 || slice[1] > 255 || slice[2] > 255 || slice[0] < 0 || slice[1] < 0|| slice[2] < 0{
            return Err(String::from("should be between 0-255"));
        }
        Ok(
            Color{
                red: slice[0] as u8,
                green: slice[1] as u8,
                blue: slice[2] as u8
            }
        )
    }
    
}

fn main() {
    // Use the `from` function
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1);

    // Since From is implemented for Color, we should be able to use Into
    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2);

    let v = vec![183, 65, 14];
    // With slice we should use `from` function
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3);
    // or take slice within round brackets and use Into
    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_out_of_range_positive() {
        assert!(Color::try_from((256, 1000, 10000)).is_err());
    }
    #[test]
    fn test_tuple_out_of_range_negative() {
        assert!(Color::try_from((-1, -10, -256)).is_err());
    }
    #[test]
    fn test_tuple_sum() {
        assert!(Color::try_from((-1, 255, 255)).is_err());
    }
    #[test]
    fn test_tuple_correct() {
        let c: Result<Color, String> = (183, 65, 14).try_into();
        assert_eq!(
            c,
            Ok(Color {
                red: 183,
                green: 65,
                blue: 14
            })
        );
    }
    #[test]
    fn test_array_out_of_range_positive() {
        let c: Result<Color, String> = [1000, 10000, 256].try_into();
        assert!(c.is_err());
    }
    #[test]
    fn test_array_out_of_range_negative() {
        let c: Result<Color, String> = [-10, -256, -1].try_into();
        assert!(c.is_err());
    }
    #[test]
    fn test_array_sum() {
        let c: Result<Color, String> = [-1, 255, 255].try_into();
        assert!(c.is_err());
    }
    #[test]
    fn test_array_correct() {
        let c: Result<Color, String> = [183, 65, 14].try_into();
        assert_eq!(
            c,
            Ok(Color {
                red: 183,
                green: 65,
                blue: 14
            })
        );
    }
    #[test]
    fn test_slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        assert!(Color::try_from(&arr[..]).is_err());
    }
    #[test]
    fn test_slice_out_of_range_negative() {
        let arr = [-256, -1, -10];
        assert!(Color::try_from(&arr[..]).is_err());
    }
    #[test]
    fn test_slice_sum() {
        let arr = [-1, 255, 255];
        assert!(Color::try_from(&arr[..]).is_err());
    }
    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c: Result<Color, String> = Color::try_from(&v[..]);
        assert_eq!(
            c,
            Ok(Color {
                red: 183,
                green: 65,
                blue: 14
            })
        );
    }
    #[test]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        assert!(Color::try_from(&v[..]).is_err());
    }
    #[test]
    fn test_slice_insufficient_length() {
        let v = vec![0, 0];
        assert!(Color::try_from(&v[..]).is_err());
    }
}
