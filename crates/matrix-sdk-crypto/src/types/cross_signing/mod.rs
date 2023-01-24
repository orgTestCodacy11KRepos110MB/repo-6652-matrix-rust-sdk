// Copyright 2021 Devin Ragotzy.
// Copyright 2021 Timo Kösters.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

mod common;
mod master;
mod self_signing;
mod user_signing;

pub use common::*;
pub use master::*;
pub use self_signing::*;
pub use user_signing::*;

#[cfg(test)]
mod tests {
    use ruma::user_id;
    use serde_json::json;

    use super::CrossSigningKey;

    #[test]
    fn serialization() {
        let json = json!({
            "user_id": "@example:localhost",
              "usage": [
                "master"
              ],
              "keys": {
                "ed25519:rJ2TAGkEOP6dX41Ksll6cl8K3J48l8s/59zaXyvl2p0": "rJ2TAGkEOP6dX41Ksll6cl8K3J48l8s/59zaXyvl2p0"
              },
              "signatures": {
                "@example:localhost": {
                  "ed25519:WSKKLTJZCL": "ZzJp1wtmRdykXAUEItEjNiFlBrxx8L6/Vaen9am8AuGwlxxJtOkuY4m+4MPLvDPOgavKHLsrRuNLAfCeakMlCQ"
                }
              },
              "other_data": "other"
        });

        let key: CrossSigningKey =
            serde_json::from_value(json.clone()).expect("Can't deserialize cross signing key");

        assert_eq!(key.user_id, user_id!("@example:localhost"));

        let serialized = serde_json::to_value(key).expect("Can't reserialize cross signing key");

        assert_eq!(json, serialized);
    }
}
