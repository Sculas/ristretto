use crate::attributes::Attribute;
use crate::constant_pool::ConstantPool;
use crate::error::Result;
use crate::method_access_flags::MethodAccessFlags;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;

/// Method.
///
/// See: <https://docs.oracle.com/javase/specs/jvms/se22/html/jvms-4.html#jvms-4.6>
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Method {
    pub access_flags: MethodAccessFlags,
    pub name_index: u16,
    pub descriptor_index: u16,
    pub attributes: Vec<Attribute>,
}

impl Method {
    /// Deserialize the `Method` from bytes.
    ///
    /// # Errors
    /// Returns an error if the bytes do not represent a valid Method.
    pub fn from_bytes(constant_pool: &ConstantPool, bytes: &mut Cursor<Vec<u8>>) -> Result<Method> {
        let access_flags = MethodAccessFlags::from_bytes(bytes)?;
        let name_index = bytes.read_u16::<BigEndian>()?;
        let descriptor_index = bytes.read_u16::<BigEndian>()?;

        let attribute_count = bytes.read_u16::<BigEndian>()?;
        let mut attributes = Vec::with_capacity(attribute_count as usize);
        for _ in 0..attribute_count {
            let attribute = Attribute::from_bytes(constant_pool, bytes)?;
            attributes.push(attribute);
        }

        let method = Method {
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        };
        Ok(method)
    }

    /// Serialize the `Method` to bytes.
    ///
    /// # Errors
    /// If there are more than 65,535 attributes, an error is returned.
    pub fn to_bytes(&self, bytes: &mut Vec<u8>) -> Result<()> {
        self.access_flags.to_bytes(bytes)?;
        bytes.write_u16::<BigEndian>(self.name_index)?;
        bytes.write_u16::<BigEndian>(self.descriptor_index)?;

        let attributes_length = u16::try_from(self.attributes.len())?;
        bytes.write_u16::<BigEndian>(attributes_length)?;
        for attribute in &self.attributes {
            attribute.to_bytes(bytes)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::attributes::Attribute;
    use crate::constant::Constant;

    #[test]
    fn test_method() -> Result<()> {
        let mut constant_pool = ConstantPool::default();
        constant_pool.add(Constant::Utf8("ConstantValue".to_string()));
        let mut attribute_bytes = Cursor::new([0, 1, 0, 0, 0, 2, 4, 2].to_vec());
        let attribute = Attribute::from_bytes(&constant_pool, &mut attribute_bytes)?;
        let method = Method {
            access_flags: MethodAccessFlags::PUBLIC,
            name_index: 1,
            descriptor_index: 2,
            attributes: vec![attribute],
        };

        let mut bytes = Vec::new();
        method.to_bytes(&mut bytes)?;

        let mut bytes = Cursor::new(bytes);
        let result = Method::from_bytes(&constant_pool, &mut bytes)?;
        assert_eq!(result, method);
        Ok(())
    }
}
