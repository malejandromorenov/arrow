// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

// automatically generated by the FlatBuffers compiler, do not modify


#![allow(dead_code)]
#![allow(unused_imports)]


use crate::ipc::gen::Schema::*;

use std::cmp::Ordering;
use std::mem;

use flatbuffers::EndianScalar;

/// ----------------------------------------------------------------------
/// Data structures for dense tensors
/// Shape data for a single axis in a tensor
pub enum TensorDimOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct TensorDim<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for TensorDim<'a> {
    type Inner = TensorDim<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> TensorDim<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        TensorDim { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args TensorDimArgs<'args>,
    ) -> flatbuffers::WIPOffset<TensorDim<'bldr>> {
        let mut builder = TensorDimBuilder::new(_fbb);
        builder.add_size_(args.size_);
        if let Some(x) = args.name {
            builder.add_name(x);
        }
        builder.finish()
    }

    pub const VT_SIZE_: flatbuffers::VOffsetT = 4;
    pub const VT_NAME: flatbuffers::VOffsetT = 6;

    /// Length of dimension
    #[inline]
    pub fn size_(&self) -> i64 {
        self._tab.get::<i64>(TensorDim::VT_SIZE_, Some(0)).unwrap()
    }
    /// Name of the dimension, optional
    #[inline]
    pub fn name(&self) -> Option<&'a str> {
        self._tab
            .get::<flatbuffers::ForwardsUOffset<&str>>(TensorDim::VT_NAME, None)
    }
}

pub struct TensorDimArgs<'a> {
    pub size_: i64,
    pub name: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for TensorDimArgs<'a> {
    #[inline]
    fn default() -> Self {
        TensorDimArgs {
            size_: 0,
            name: None,
        }
    }
}
pub struct TensorDimBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TensorDimBuilder<'a, 'b> {
    #[inline]
    pub fn add_size_(&mut self, size_: i64) {
        self.fbb_.push_slot::<i64>(TensorDim::VT_SIZE_, size_, 0);
    }
    #[inline]
    pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b str>) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(TensorDim::VT_NAME, name);
    }
    #[inline]
    pub fn new(
        _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> TensorDimBuilder<'a, 'b> {
        let start = _fbb.start_table();
        TensorDimBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<TensorDim<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

pub enum TensorOffset {}
#[derive(Copy, Clone, Debug, PartialEq)]

pub struct Tensor<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for Tensor<'a> {
    type Inner = Tensor<'a>;
    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf: buf, loc: loc },
        }
    }
}

impl<'a> Tensor<'a> {
    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        Tensor { _tab: table }
    }
    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args TensorArgs<'args>,
    ) -> flatbuffers::WIPOffset<Tensor<'bldr>> {
        let mut builder = TensorBuilder::new(_fbb);
        if let Some(x) = args.data {
            builder.add_data(x);
        }
        if let Some(x) = args.strides {
            builder.add_strides(x);
        }
        if let Some(x) = args.shape {
            builder.add_shape(x);
        }
        if let Some(x) = args.type_ {
            builder.add_type_(x);
        }
        builder.add_type_type(args.type_type);
        builder.finish()
    }

    pub const VT_TYPE_TYPE: flatbuffers::VOffsetT = 4;
    pub const VT_TYPE_: flatbuffers::VOffsetT = 6;
    pub const VT_SHAPE: flatbuffers::VOffsetT = 8;
    pub const VT_STRIDES: flatbuffers::VOffsetT = 10;
    pub const VT_DATA: flatbuffers::VOffsetT = 12;

    #[inline]
    pub fn type_type(&self) -> Type {
        self._tab
            .get::<Type>(Tensor::VT_TYPE_TYPE, Some(Type::NONE))
            .unwrap()
    }
    /// The type of data contained in a value cell. Currently only fixed-width
    /// value types are supported, no strings or nested types
    #[inline]
    pub fn type_(&self) -> Option<flatbuffers::Table<'a>> {
        self._tab
            .get::<flatbuffers::ForwardsUOffset<flatbuffers::Table<'a>>>(
                Tensor::VT_TYPE_,
                None,
            )
    }
    /// The dimensions of the tensor, optionally named
    #[inline]
    pub fn shape(
        &self,
    ) -> Option<flatbuffers::Vector<flatbuffers::ForwardsUOffset<TensorDim<'a>>>> {
        self._tab.get::<flatbuffers::ForwardsUOffset<
            flatbuffers::Vector<flatbuffers::ForwardsUOffset<TensorDim<'a>>>,
        >>(Tensor::VT_SHAPE, None)
    }
    /// Non-negative byte offsets to advance one value cell along each dimension
    #[inline]
    pub fn strides(&self) -> Option<flatbuffers::Vector<'a, i64>> {
        self._tab
            .get::<flatbuffers::ForwardsUOffset<flatbuffers::Vector<'a, i64>>>(
                Tensor::VT_STRIDES,
                None,
            )
    }
    /// The location and size of the tensor's data
    #[inline]
    pub fn data(&self) -> Option<&'a Buffer> {
        self._tab.get::<Buffer>(Tensor::VT_DATA, None)
    }
    #[inline]
    #[allow(non_snake_case)]
    pub fn type__as_null(&'a self) -> Option<Null> {
        if self.type_type() == Type::Null {
            self.type_().map(|u| Null::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type__as_int(&'a self) -> Option<Int> {
        if self.type_type() == Type::Int {
            self.type_().map(|u| Int::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type__as_floating_point(&'a self) -> Option<FloatingPoint> {
        if self.type_type() == Type::FloatingPoint {
            self.type_().map(|u| FloatingPoint::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type__as_binary(&'a self) -> Option<Binary> {
        if self.type_type() == Type::Binary {
            self.type_().map(|u| Binary::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type__as_utf_8(&'a self) -> Option<Utf8> {
        if self.type_type() == Type::Utf8 {
            self.type_().map(|u| Utf8::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type__as_bool(&'a self) -> Option<Bool> {
        if self.type_type() == Type::Bool {
            self.type_().map(|u| Bool::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type__as_decimal(&'a self) -> Option<Decimal> {
        if self.type_type() == Type::Decimal {
            self.type_().map(|u| Decimal::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type__as_date(&'a self) -> Option<Date> {
        if self.type_type() == Type::Date {
            self.type_().map(|u| Date::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type__as_time(&'a self) -> Option<Time> {
        if self.type_type() == Type::Time {
            self.type_().map(|u| Time::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type__as_timestamp(&'a self) -> Option<Timestamp> {
        if self.type_type() == Type::Timestamp {
            self.type_().map(|u| Timestamp::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type__as_interval(&'a self) -> Option<Interval> {
        if self.type_type() == Type::Interval {
            self.type_().map(|u| Interval::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type__as_list(&'a self) -> Option<List> {
        if self.type_type() == Type::List {
            self.type_().map(|u| List::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type__as_struct_(&'a self) -> Option<Struct_> {
        if self.type_type() == Type::Struct_ {
            self.type_().map(|u| Struct_::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type__as_union(&'a self) -> Option<Union> {
        if self.type_type() == Type::Union {
            self.type_().map(|u| Union::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type__as_fixed_size_binary(&'a self) -> Option<FixedSizeBinary> {
        if self.type_type() == Type::FixedSizeBinary {
            self.type_().map(|u| FixedSizeBinary::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type__as_fixed_size_list(&'a self) -> Option<FixedSizeList> {
        if self.type_type() == Type::FixedSizeList {
            self.type_().map(|u| FixedSizeList::init_from_table(u))
        } else {
            None
        }
    }

    #[inline]
    #[allow(non_snake_case)]
    pub fn type__as_map(&'a self) -> Option<Map> {
        if self.type_type() == Type::Map {
            self.type_().map(|u| Map::init_from_table(u))
        } else {
            None
        }
    }
}

pub struct TensorArgs<'a> {
    pub type_type: Type,
    pub type_: Option<flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>>,
    pub shape: Option<
        flatbuffers::WIPOffset<
            flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<TensorDim<'a>>>,
        >,
    >,
    pub strides: Option<flatbuffers::WIPOffset<flatbuffers::Vector<'a, i64>>>,
    pub data: Option<&'a Buffer>,
}
impl<'a> Default for TensorArgs<'a> {
    #[inline]
    fn default() -> Self {
        TensorArgs {
            type_type: Type::NONE,
            type_: None,
            shape: None,
            strides: None,
            data: None,
        }
    }
}
pub struct TensorBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> TensorBuilder<'a, 'b> {
    #[inline]
    pub fn add_type_type(&mut self, type_type: Type) {
        self.fbb_
            .push_slot::<Type>(Tensor::VT_TYPE_TYPE, type_type, Type::NONE);
    }
    #[inline]
    pub fn add_type_(
        &mut self,
        type_: flatbuffers::WIPOffset<flatbuffers::UnionWIPOffset>,
    ) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(Tensor::VT_TYPE_, type_);
    }
    #[inline]
    pub fn add_shape(
        &mut self,
        shape: flatbuffers::WIPOffset<
            flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<TensorDim<'b>>>,
        >,
    ) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(Tensor::VT_SHAPE, shape);
    }
    #[inline]
    pub fn add_strides(
        &mut self,
        strides: flatbuffers::WIPOffset<flatbuffers::Vector<'b, i64>>,
    ) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(Tensor::VT_STRIDES, strides);
    }
    #[inline]
    pub fn add_data(&mut self, data: &'b Buffer) {
        self.fbb_.push_slot_always::<&Buffer>(Tensor::VT_DATA, data);
    }
    #[inline]
    pub fn new(
        _fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    ) -> TensorBuilder<'a, 'b> {
        let start = _fbb.start_table();
        TensorBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }
    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<Tensor<'a>> {
        let o = self.fbb_.end_table(self.start_);
        flatbuffers::WIPOffset::new(o.value())
    }
}

#[inline]
pub fn get_root_as_tensor<'a>(buf: &'a [u8]) -> Tensor<'a> {
    flatbuffers::get_root::<Tensor<'a>>(buf)
}

#[inline]
pub fn get_size_prefixed_root_as_tensor<'a>(buf: &'a [u8]) -> Tensor<'a> {
    flatbuffers::get_size_prefixed_root::<Tensor<'a>>(buf)
}

#[inline]
pub fn finish_tensor_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Tensor<'a>>,
) {
    fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_tensor_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<Tensor<'a>>,
) {
    fbb.finish_size_prefixed(root, None);
}