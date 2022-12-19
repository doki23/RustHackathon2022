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

use std::f64::consts::PI;

use common_datavalues::prelude::*;
use common_exception::Result;

use crate::scalars::scalar_function_test::test_scalar_functions;
use crate::scalars::scalar_function_test::ScalarFunctionTest;

#[test]
fn test_degress_function() -> Result<()> {
    let tests = vec![ScalarFunctionTest {
        name: "degress-passed",
        columns: vec![Series::from_data([Some(PI), Some(PI / 2.0), None])],
        expect: Series::from_data([Some(180_f64), Some(90.0), None]),
        error: "",
    }];

    test_scalar_functions("degrees", &tests)
}

#[test]
fn test_radians_function() -> Result<()> {
    let tests = vec![ScalarFunctionTest {
        name: "radians-passed",
        columns: vec![Series::from_data([Some(180), None])],
        expect: Series::from_data([Some(PI), None]),
        error: "",
    }];

    test_scalar_functions("radians", &tests)
}
