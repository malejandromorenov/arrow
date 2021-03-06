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

#include "benchmark/benchmark.h"

#include "arrow/array.h"
#include "arrow/testing/random.h"

#include "parquet/column_reader.h"
#include "parquet/column_writer.h"
#include "parquet/file_reader.h"
#include "parquet/metadata.h"
#include "parquet/platform.h"
#include "parquet/thrift.h"

namespace parquet {

using schema::PrimitiveNode;

namespace benchmark {

std::shared_ptr<Int64Writer> BuildWriter(int64_t output_size,
                                         const std::shared_ptr<ArrowOutputStream>& dst,
                                         ColumnChunkMetaDataBuilder* metadata,
                                         ColumnDescriptor* schema,
                                         const WriterProperties* properties) {
  std::unique_ptr<PageWriter> pager =
      PageWriter::Open(dst, Compression::UNCOMPRESSED, metadata);
  std::shared_ptr<ColumnWriter> writer =
      ColumnWriter::Make(metadata, std::move(pager), properties);
  return std::static_pointer_cast<Int64Writer>(writer);
}

std::shared_ptr<ColumnDescriptor> Int64Schema(Repetition::type repetition) {
  auto node = PrimitiveNode::Make("int64", repetition, Type::INT64);
  return std::make_shared<ColumnDescriptor>(node, repetition != Repetition::REQUIRED,
                                            repetition == Repetition::REPEATED);
}

void SetBytesProcessed(::benchmark::State& state, Repetition::type repetition) {
  int64_t bytes_processed = state.iterations() * state.range(0) * sizeof(int64_t);
  if (repetition != Repetition::REQUIRED) {
    bytes_processed += state.iterations() * state.range(0) * sizeof(int16_t);
  }
  if (repetition == Repetition::REPEATED) {
    bytes_processed += state.iterations() * state.range(0) * sizeof(int16_t);
  }
  state.SetBytesProcessed(state.iterations() * state.range(0) * sizeof(int16_t));
}

template <Repetition::type repetition,
          Compression::type codec = Compression::UNCOMPRESSED>
static void BM_WriteInt64Column(::benchmark::State& state) {
  format::ColumnChunk thrift_metadata;

  ::arrow::random::RandomArrayGenerator rgen(1337);
  auto values = rgen.Int64(state.range(0), 0, 1000000, 0);
  const auto& i8_values = static_cast<const ::arrow::Int64Array&>(*values);

  std::vector<int16_t> definition_levels(state.range(0), 1);
  std::vector<int16_t> repetition_levels(state.range(0), 0);
  std::shared_ptr<ColumnDescriptor> schema = Int64Schema(repetition);
  std::shared_ptr<WriterProperties> properties = WriterProperties::Builder()
                                                     .compression(codec)
                                                     ->encoding(Encoding::PLAIN)
                                                     ->disable_dictionary()
                                                     ->build();
  auto metadata = ColumnChunkMetaDataBuilder::Make(
      properties, schema.get(), reinterpret_cast<uint8_t*>(&thrift_metadata));

  while (state.KeepRunning()) {
    auto stream = CreateOutputStream();
    std::shared_ptr<Int64Writer> writer = BuildWriter(
        state.range(0), stream, metadata.get(), schema.get(), properties.get());
    writer->WriteBatch(i8_values.length(), definition_levels.data(),
                       repetition_levels.data(), i8_values.raw_values());
    writer->Close();
  }
  SetBytesProcessed(state, repetition);
}

BENCHMARK_TEMPLATE(BM_WriteInt64Column, Repetition::REQUIRED)->Arg(1 << 20);
BENCHMARK_TEMPLATE(BM_WriteInt64Column, Repetition::OPTIONAL)->Arg(1 << 20);
BENCHMARK_TEMPLATE(BM_WriteInt64Column, Repetition::REPEATED)->Arg(1 << 20);
BENCHMARK_TEMPLATE(BM_WriteInt64Column, Repetition::REQUIRED, Compression::SNAPPY)
    ->Arg(1 << 20);
BENCHMARK_TEMPLATE(BM_WriteInt64Column, Repetition::OPTIONAL, Compression::SNAPPY)
    ->Arg(1 << 20);
BENCHMARK_TEMPLATE(BM_WriteInt64Column, Repetition::REPEATED, Compression::SNAPPY)
    ->Arg(1 << 20);

BENCHMARK_TEMPLATE(BM_WriteInt64Column, Repetition::REQUIRED, Compression::LZ4)
    ->Arg(1 << 20);
BENCHMARK_TEMPLATE(BM_WriteInt64Column, Repetition::OPTIONAL, Compression::LZ4)
    ->Arg(1 << 20);
BENCHMARK_TEMPLATE(BM_WriteInt64Column, Repetition::REPEATED, Compression::LZ4)
    ->Arg(1 << 20);

BENCHMARK_TEMPLATE(BM_WriteInt64Column, Repetition::REQUIRED, Compression::ZSTD)
    ->Arg(1 << 20);
BENCHMARK_TEMPLATE(BM_WriteInt64Column, Repetition::OPTIONAL, Compression::ZSTD)
    ->Arg(1 << 20);
BENCHMARK_TEMPLATE(BM_WriteInt64Column, Repetition::REPEATED, Compression::ZSTD)
    ->Arg(1 << 20);

std::shared_ptr<Int64Reader> BuildReader(std::shared_ptr<Buffer>& buffer,
                                         int64_t num_values, ColumnDescriptor* schema) {
  auto source = std::make_shared<::arrow::io::BufferReader>(buffer);
  std::unique_ptr<PageReader> page_reader =
      PageReader::Open(source, num_values, Compression::UNCOMPRESSED);
  return std::static_pointer_cast<Int64Reader>(
      ColumnReader::Make(schema, std::move(page_reader)));
}

template <Repetition::type repetition,
          Compression::type codec = Compression::UNCOMPRESSED>
static void BM_ReadInt64Column(::benchmark::State& state) {
  format::ColumnChunk thrift_metadata;
  std::vector<int64_t> values(state.range(0), 128);
  std::vector<int16_t> definition_levels(state.range(0), 1);
  std::vector<int16_t> repetition_levels(state.range(0), 0);
  std::shared_ptr<ColumnDescriptor> schema = Int64Schema(repetition);
  std::shared_ptr<WriterProperties> properties = WriterProperties::Builder()
                                                     .compression(codec)
                                                     ->encoding(Encoding::PLAIN)
                                                     ->disable_dictionary()
                                                     ->build();

  auto metadata = ColumnChunkMetaDataBuilder::Make(
      properties, schema.get(), reinterpret_cast<uint8_t*>(&thrift_metadata));

  auto stream = CreateOutputStream();
  std::shared_ptr<Int64Writer> writer =
      BuildWriter(state.range(0), stream, metadata.get(), schema.get(), properties.get());
  writer->WriteBatch(values.size(), definition_levels.data(), repetition_levels.data(),
                     values.data());
  writer->Close();

  std::shared_ptr<Buffer> src;
  PARQUET_THROW_NOT_OK(stream->Finish(&src));
  std::vector<int64_t> values_out(state.range(1));
  std::vector<int16_t> definition_levels_out(state.range(1));
  std::vector<int16_t> repetition_levels_out(state.range(1));
  while (state.KeepRunning()) {
    std::shared_ptr<Int64Reader> reader = BuildReader(src, state.range(1), schema.get());
    int64_t values_read = 0;
    for (size_t i = 0; i < values.size(); i += values_read) {
      reader->ReadBatch(values_out.size(), definition_levels_out.data(),
                        repetition_levels_out.data(), values_out.data(), &values_read);
    }
  }
  SetBytesProcessed(state, repetition);
}

BENCHMARK_TEMPLATE(BM_ReadInt64Column, Repetition::REQUIRED)
    ->RangePair(1024, 65536, 1, 1024);

BENCHMARK_TEMPLATE(BM_ReadInt64Column, Repetition::OPTIONAL)
    ->RangePair(1024, 65536, 1, 1024);

BENCHMARK_TEMPLATE(BM_ReadInt64Column, Repetition::REPEATED)
    ->RangePair(1024, 65536, 1, 1024);

BENCHMARK_TEMPLATE(BM_ReadInt64Column, Repetition::REQUIRED, Compression::SNAPPY)
    ->RangePair(1024, 65536, 1, 1024);
BENCHMARK_TEMPLATE(BM_ReadInt64Column, Repetition::OPTIONAL, Compression::SNAPPY)
    ->RangePair(1024, 65536, 1, 1024);
BENCHMARK_TEMPLATE(BM_ReadInt64Column, Repetition::REPEATED, Compression::SNAPPY)
    ->RangePair(1024, 65536, 1, 1024);

BENCHMARK_TEMPLATE(BM_ReadInt64Column, Repetition::REQUIRED, Compression::LZ4)
    ->RangePair(1024, 65536, 1, 1024);
BENCHMARK_TEMPLATE(BM_ReadInt64Column, Repetition::OPTIONAL, Compression::LZ4)
    ->RangePair(1024, 65536, 1, 1024);
BENCHMARK_TEMPLATE(BM_ReadInt64Column, Repetition::REPEATED, Compression::LZ4)
    ->RangePair(1024, 65536, 1, 1024);

BENCHMARK_TEMPLATE(BM_ReadInt64Column, Repetition::REQUIRED, Compression::ZSTD)
    ->RangePair(1024, 65536, 1, 1024);
BENCHMARK_TEMPLATE(BM_ReadInt64Column, Repetition::OPTIONAL, Compression::ZSTD)
    ->RangePair(1024, 65536, 1, 1024);
BENCHMARK_TEMPLATE(BM_ReadInt64Column, Repetition::REPEATED, Compression::ZSTD)
    ->RangePair(1024, 65536, 1, 1024);

static void BM_RleEncoding(::benchmark::State& state) {
  std::vector<int16_t> levels(state.range(0), 0);
  int64_t n = 0;
  std::generate(levels.begin(), levels.end(),
                [&state, &n] { return (n++ % state.range(1)) == 0; });
  int16_t max_level = 1;
  int64_t rle_size = LevelEncoder::MaxBufferSize(Encoding::RLE, max_level,
                                                 static_cast<int>(levels.size()));
  auto buffer_rle = AllocateBuffer();
  PARQUET_THROW_NOT_OK(buffer_rle->Resize(rle_size));

  while (state.KeepRunning()) {
    LevelEncoder level_encoder;
    level_encoder.Init(Encoding::RLE, max_level, static_cast<int>(levels.size()),
                       buffer_rle->mutable_data(), static_cast<int>(buffer_rle->size()));
    level_encoder.Encode(static_cast<int>(levels.size()), levels.data());
  }
  state.SetBytesProcessed(state.iterations() * state.range(0) * sizeof(int16_t));
  state.SetItemsProcessed(state.iterations() * state.range(0));
}

BENCHMARK(BM_RleEncoding)->RangePair(1024, 65536, 1, 16);

static void BM_RleDecoding(::benchmark::State& state) {
  LevelEncoder level_encoder;
  std::vector<int16_t> levels(state.range(0), 0);
  int64_t n = 0;
  std::generate(levels.begin(), levels.end(),
                [&state, &n] { return (n++ % state.range(1)) == 0; });
  int16_t max_level = 1;
  int rle_size = LevelEncoder::MaxBufferSize(Encoding::RLE, max_level,
                                             static_cast<int>(levels.size()));
  auto buffer_rle = AllocateBuffer();
  PARQUET_THROW_NOT_OK(buffer_rle->Resize(rle_size + sizeof(int32_t)));
  level_encoder.Init(Encoding::RLE, max_level, static_cast<int>(levels.size()),
                     buffer_rle->mutable_data() + sizeof(int32_t), rle_size);
  level_encoder.Encode(static_cast<int>(levels.size()), levels.data());
  reinterpret_cast<int32_t*>(buffer_rle->mutable_data())[0] = level_encoder.len();

  while (state.KeepRunning()) {
    LevelDecoder level_decoder;
    level_decoder.SetData(Encoding::RLE, max_level, static_cast<int>(levels.size()),
                          buffer_rle->data());
    level_decoder.Decode(static_cast<int>(state.range(0)), levels.data());
  }

  state.SetBytesProcessed(state.iterations() * state.range(0) * sizeof(int16_t));
  state.SetItemsProcessed(state.iterations() * state.range(0));
}

BENCHMARK(BM_RleDecoding)->RangePair(1024, 65536, 1, 16);

}  // namespace benchmark

}  // namespace parquet
