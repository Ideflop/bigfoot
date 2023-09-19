const SIZE_VEC_RESERVED: usize = 1000;

#[derive(Debug, Clone)]
pub struct BigFoot {
    is_signed: bool,
    is_negative: bool,
    number: Vec<u8>,
}

impl BigFoot {
    // TODO: should only accept digit, and one dot else error
    // Default -> Creation of Struct BigFoot with 0
    pub fn new<T>(is_signed: bool, value: T) -> BigFoot
    where
        T: Into<String>,
    {
        let string_value: String = value.into();
        if string_value.is_empty() {
            create_bigfoot(false, false, String::from("0"), SIZE_VEC_RESERVED);
        }
        let size = if string_value.len() > SIZE_VEC_RESERVED {
            string_value.len().saturating_mul(10)
        } else {
            SIZE_VEC_RESERVED
        };
        if is_signed {
            from_i_to_big(string_value)
        } else {
            create_bigfoot(is_signed, false, string_value, size)
        }

    }

    pub fn from_unsigned<T: Into<u128>>(value: T) -> BigFoot {
        create_bigfoot(
            false,
            false,
            Into::<u128>::into(value).to_string(),
            SIZE_VEC_RESERVED,
        )
    }

    pub fn from_usize(value: usize) -> BigFoot {
        create_bigfoot(false, false, value.to_string(), SIZE_VEC_RESERVED)
    }

    pub fn from_signed<T: Into<i128>>(value: T) -> BigFoot {
        from_i_to_big(Into::<i128>::into(value).to_string())
    }

    pub fn from_isize(value: isize) -> BigFoot {
        from_i_to_big(value.to_string())
    }

    // TODO: the user must give the size of the number
    // but the number are chosen randomly
    // ex : 3 => 000
    //      10 => 0000000000
    fn new_random() -> Self {
        todo!()
    }
}

fn from_i_to_big(mut value: String) -> BigFoot {
    let mut is_negative = false;
    if value.contains("-") {
        is_negative = true;
        value.retain(|c| c != '-');
    }
    create_bigfoot(true, is_negative, value, SIZE_VEC_RESERVED)
}

// do the creation of the BigFoot struct
fn create_bigfoot(is_signed: bool, is_negative: bool, number: String, size: usize) -> BigFoot {
    let mut number_as_vec = number.as_bytes().to_vec();
    number_as_vec.reserve(size);

    BigFoot {
        is_signed,
        is_negative,
        number: number_as_vec,
    }
}

//trait Addition {
//    fn add(&self, other: BigFoot) -> BigFoot;
//}

impl std::fmt::Display for BigFoot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_signed && self.is_negative {
            write!(f, "-")?;
        };
        for n in self.number.iter() {
            // TODO: check if checked_sub is the right function to do it
            // maybe there is a faster one or a better one ?
            if let Some(digit) = n.checked_sub(b'0') {
                write!(f, "{}", digit)?;
            }
            if *n == b'.' {
                write!(f, ".")?;
            }
        }
        Ok(())
    }
}


//TODO: implement to_string for struct BigFoot
