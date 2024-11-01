#[macro_export]
macro_rules! impl_deserialize_uint_tags {
    ($type_label:expr, $enum_type:ty, $value:ty, { $($variant:ident => $data_ty:ty),* $(,)? }) => {
        impl<'de> serde::Deserialize<'de> for $value {
            fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
                let value = serde_json::Value::deserialize(d)?;
                let radd_type = value.get($type_label).unwrap();
                let radd_type = radd_type.as_u64().unwrap();

                let relationship_add_info = match num::FromPrimitive::from_u8(radd_type as u8).unwrap() {
                    $(
                        <$enum_type>::$variant => {
                            let data = <$data_ty>::deserialize(&value).map_err(D::Error::custom)?;
                            <$value>::$variant(data)
                        },
                    )*
                };

                Ok(relationship_add_info)
            }
        }
    }
}   

#[macro_export]
macro_rules! mapped_deserialize {
    ($($variant:ident => $channel_type:ident($data_type:ident)),* $(,)?) => {
        match channel_type {
            $(
                $variant => $channel_type(
                    $data_type::deserialize(value).map_err(D::Error::custom)?
                ),
            )*
        }
    }
}