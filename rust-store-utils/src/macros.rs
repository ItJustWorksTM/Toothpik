/*
 * Copyright 2020 ItJustWorksTM
 *
 *  Licensed under the Apache License, Version 2.0 (the "License");
 *  you may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *       http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS,
 *  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 */

#[macro_export]
macro_rules! static_def_regex {
    ($vec_ref:ident, $($var_name:ident: $re:expr),*) => {
        ::paste::paste! {
            ::lazy_static::lazy_static! {
                $(static ref [<$var_name:upper>]: ::regex::Regex = ::regex::Regex::new($re).unwrap();)*
            };
            $(let $var_name: &::regex::Regex = &[<$var_name:upper>];)*
            let $vec_ref = vec![$(&$var_name),*];
        };
    };
}
