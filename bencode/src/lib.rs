pub trait Bencode {
    fn bsize(&self) -> usize;
    fn try_decode(from: &str) -> Option<Self> 
        where Self: Sized;
}

impl Bencode for i64 {
    fn bsize(&self) -> usize {
        //todo: do it without heap allocation?
        self.to_string().len() + 2
    }
 
    fn try_decode(from: &str) -> Option<i64> {
        let ipos = from.find('i')?;
        let epos = from.find('e')?;
        from[ipos+1..epos].parse::<i64>().ok()
    }
}

impl Bencode for String {
    fn bsize(&self) -> usize {
        self.len() + self.len().to_string().len() + 1
    }

    fn try_decode(from: &str) -> Option<String> {
        let cpos = from.find(':')?;
        let size = from[..cpos].parse::<usize>().ok()?;
        if size > from[cpos+1..].len() {
            return None;
        }
        Some(String::from(&from[cpos+1..cpos+1+size]))
    }
}

pub fn deserialize<T: Bencode>(contents: &str) -> Option<T>  {
    T::try_decode(contents)
}

#[test]
fn test_try_decode_i64() {
    assert_eq!(None, i64::try_decode(""));
    assert_eq!(None, i64::try_decode("i"));
    assert_eq!(None, i64::try_decode("i-"));
    assert_eq!(None, i64::try_decode("i-123-11-"));
    assert_eq!(None, i64::try_decode("ie"));
    assert_eq!(None, i64::try_decode("i-e"));
    assert_eq!(Some(0), i64::try_decode("i0e"));
    assert_eq!(Some(123), i64::try_decode("i123e"));
    assert_eq!(Some(-23), i64::try_decode("i-23e"));
}

#[test]
fn test_try_decode_string() {
    assert_eq!(None, String::try_decode(""));
    assert_eq!(None, String::try_decode("5:abcd"));
    assert_eq!(None, String::try_decode("-4:abcd"));
    assert_eq!(Some("abcd".to_string()), String::try_decode("4:abcd"));
    assert_eq!(Some("".to_string()), String::try_decode("0:"));
    assert_eq!(Some("A".to_string()), String::try_decode("1:A"));
    assert_eq!(Some("A".to_string()), String::try_decode("1:ABCDE"));
}