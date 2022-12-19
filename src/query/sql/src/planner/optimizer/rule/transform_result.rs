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

use crate::optimizer::s_expr::SExpr;

#[derive(Clone, Default)]
pub struct TransformResult {
    results: Vec<SExpr>,
}

impl TransformResult {
    pub fn new() -> Self {
        TransformResult { results: vec![] }
    }

    pub fn add_result(&mut self, result: SExpr) {
        self.results.push(result);
    }

    pub fn results(&self) -> &[SExpr] {
        &self.results
    }
}
