use fastnum::{
    decimal::{Context, Decimal, UnsignedDecimal},
    int::{Int, UInt},
};

use crate::{InputValueError, InputValueResult, Scalar, ScalarType, Value};

#[Scalar(internal, name = "Decimal")]
impl<const N: usize> ScalarType for Decimal<N> {
    fn parse(value: Value) -> InputValueResult<Self> {
        match &value {
            Value::String(s) => Ok(Decimal::from_str(s, Context::default())?),
            Value::Number(n) => {
                if let Some(f) = n.as_f64() {
                    return Decimal::try_from(f).map_err(InputValueError::custom);
                }

                if let Some(f) = n.as_i64() {
                    return Ok(Decimal::from(f));
                }

                // unwrap safe here, because we have checked the other possibility
                Ok(Decimal::from(n.as_u64().unwrap()))
            }
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.to_string())
    }
}

#[Scalar(internal, name = "UnsignedDecimal")]
impl<const N: usize> ScalarType for UnsignedDecimal<N> {
    fn parse(value: Value) -> InputValueResult<Self> {
        match &value {
            Value::String(s) => Ok(UnsignedDecimal::from_str(s, Context::default())?),
            Value::Number(n) => {
                if let Some(f) = n.as_f64() {
                    return UnsignedDecimal::try_from(f).map_err(InputValueError::custom);
                }

                if let Some(f) = n.as_u64() {
                    return Ok(UnsignedDecimal::from(f));
                }

                Err(InputValueError::expected_type(value))
            }
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.to_string())
    }
}

#[Scalar(internal, name = "Integer")]
impl<const N: usize> ScalarType for Int<N> {
    fn parse(value: Value) -> InputValueResult<Self> {
        match &value {
            Value::Number(n) => {
                if let Some(f) = n.as_i64() {
                    return Ok(Int::from(f));
                }

                if let Some(f) = n.as_u64() {
                    return Ok(Int::try_from(f)?);
                }

                // a float
                Err(InputValueError::expected_type(value))
            }
            Value::String(s) => Ok(Int::from_str_radix(s, 10)?),
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.to_string())
    }
}

#[Scalar(internal, name = "UnsignedInteger")]
impl<const N: usize> ScalarType for UInt<N> {
    fn parse(value: Value) -> InputValueResult<Self> {
        match &value {
            Value::Number(n) => {
                if let Some(f) = n.as_i64() {
                    return Ok(UInt::try_from(f)?);
                }

                if let Some(f) = n.as_u64() {
                    return Ok(UInt::from(f));
                }

                // a float
                Err(InputValueError::expected_type(value))
            }
            Value::String(s) => Ok(UInt::from_str_radix(s, 10)?),
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.to_string())
    }
}

#[cfg(test)]
mod test {
    use fastnum::{D128, I128, U128, UD128};

    use crate::*;

    #[tokio::test]
    async fn test_fastnum() {
        struct Query;

        #[Object(internal)]
        impl Query {
            async fn decimal(&self, n: D128) -> D128 {
                n
            }
            async fn unsigned_decimal(&self, n: UD128) -> UD128 {
                n
            }
            async fn integer(&self, n: I128) -> I128 {
                n
            }
            async fn unsigned_integer(&self, n: U128) -> U128 {
                n
            }
        }

        let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
        assert_eq!(
            schema
                .execute(
                    r#"{
                    decimal1: decimal(n: "100")
                    decimal2: decimal(n: "108446744073709999999")
                    decimal3: decimal(n: "0")
                    decimal4: decimal(n: "1")
                    decimal5: decimal(n: "100.5")
                    decimal6: decimal(n: "-100.5")
                    decimal7: decimal(n: "0.5")
                    decimal8: decimal(n: "1.5")
                    
                    unsignedDecimal1: unsignedDecimal(n: "100")
                    unsignedDecimal2: unsignedDecimal(n: "10844674407370.9999999")
                    unsignedDecimal3: unsignedDecimal(n: "0")
                    unsignedDecimal4: unsignedDecimal(n: "1")
                    unsignedDecimal5: unsignedDecimal(n: "100.5")
                    unsignedDecimal6: unsignedDecimal(n: "0.5")
                    unsignedDecimal7: unsignedDecimal(n: "1.5")
                    
                    integer1: integer(n: "100")
                    integer2: integer(n: "-100")
                    integer3: integer(n: "0")
                    integer4: integer(n: "1")
                    
                    unsignedInteger1: unsignedInteger(n: "100")
                    unsignedInteger3: unsignedInteger(n: "0")
                }"#
                )
                .await
                .into_result()
                .unwrap()
                .data,
            value!({
                "decimal1": "100",
                "decimal2": "108446744073709999999",
                "decimal3": "0",
                "decimal4": "1",
                "decimal5": "100.5",
                "decimal6": "-100.5",
                "decimal7": "0.5",
                "decimal8": "1.5",

                "unsignedDecimal1": "100",
                "unsignedDecimal2": "10844674407370.9999999",
                "unsignedDecimal3": "0",
                "unsignedDecimal4": "1",
                "unsignedDecimal5": "100.5",
                "unsignedDecimal6": "0.5",
                "unsignedDecimal7": "1.5",

                "integer1": "100",
                "integer2": "-100",
                "integer3": "0",
                "integer4": "1",

                "unsignedInteger1": "100",
                "unsignedInteger3": "0",
            })
        );
    }
}
