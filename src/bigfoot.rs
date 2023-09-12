const SIZE_VEC_RESERVED: usize = 1000;

#[derive(Debug)]
pub struct BigFoot {
    is_signed: bool,
    number: Vec<u8>,
}

impl BigFoot {
    // TODO: should only accept digit and dot else error
    pub fn new<T>(is_signed: bool, value: T) -> BigFoot
    where 
        T: Into<String>,
    { 
        let string_value: String = value.into();
        let size = if string_value.len() > SIZE_VEC_RESERVED {
            string_value.len().saturating_mul(10)
        } else {
            SIZE_VEC_RESERVED
        };

        create_big(is_signed, string_value, size)
    }

    pub fn from_unsigned<T: Into<u128>>(value: T) -> BigFoot { 
        create_big(false, Into::<u128>::into(value).to_string() , SIZE_VEC_RESERVED)
    }

    pub fn from_usize(value: usize) -> BigFoot { 
        create_big(false, value.to_string(), SIZE_VEC_RESERVED)
    }

    pub fn from_signed<T: Into<i128>>(value: T) -> BigFoot { 
        from_i_to_big(Into::<i128>::into(value).to_string())
    }

    pub fn from_isize(value: isize) -> BigFoot { 
        from_i_to_big(value.to_string())
    }

    // TODO: the size and the number are chosen randomly
    // the user must give the size of the number
    // ex : 3 => 000
    //      10 => 0000000000
    fn new_random() -> Self {
        todo!()
    }

}

fn from_i_to_big(mut value: String) -> BigFoot {
    value.retain(|c| c != '-');
    create_big(true, value, SIZE_VEC_RESERVED)
}

// do the actual creation of the BigFoot struct
fn create_big(is_signed: bool, number: String, size: usize) -> BigFoot {
    let mut number_as_vec = number.as_bytes().to_vec();
    number_as_vec.reserve(size);

    BigFoot { 
        is_signed, 
        number: number_as_vec,  
    } 
}

//trait Addition {
//    fn add(&self, other: BigFoot) -> BigFoot;
//}


impl std::fmt::Display for BigFoot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.is_signed {
           write!(f,"-")?;
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
