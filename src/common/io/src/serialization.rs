// Copyright 2021 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use common_exception::Result;

/// bincode seralize_into wrap with optimized config
#[inline]
pub fn serialize_into_buf<W: std::io::Write, T: serde::Serialize>(
    writer: &mut W,
    value: &T,
) -> Result<()> {
    bincode::serde::encode_into_std_write(value, writer, bincode::config::standard())?;
    Ok(())
}

/// bincode deserialize_from wrap with optimized config
#[inline]
pub fn deserialize_from_slice<T: serde::de::DeserializeOwned>(slice: &mut &[u8]) -> Result<T> {
    let (value, bytes_read) =
        bincode::serde::decode_from_slice(slice, bincode::config::standard())?;
    *slice = &slice[bytes_read..];
    Ok(value)
}
