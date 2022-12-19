# RustHackathon2022

# Introduction

本次参赛作品是为databend集成[PA](https://github.com/sundy-li/pa)(一个无缝对接 apache/arrow 的高性能存储格式)以提升io性能.

[Databend](https://github.com/datafuselabs/databend) 是一款现代化的,高性能云原生分析型实时数据仓库. 它强大的分析能力来自于 apache/arrow -- 一款高性能向量化计算库. 而它核心的数据存储格式, 是 apache/parquet -- 一个兼具性能,高压缩比的开放式的数据存储格式.

尽管依靠这些优秀的第三方库,Databend 表现出了卓越的性能,但当我们深入了解之后依然发现,数据从 parquet 转换成 arrow 时,消耗了大量的 cpu 在 memcopy 上. 于是我们开始思考, 是否能够找到一种与 arrow 无缝对接的存储格式, 去掉中间 decode 的开销, 直接将原始的字节流拷贝到 arrow 的缓冲区中, 一步到位? 就这样, PA 诞生了.

# Quick Start

1. 构建&运行databend

```
git clone https://github.com/doki23/RustHackathon2022.git --depth=1

cd RustHackathon2022

cargo build --release

BUILD_PROFILE=release scripts/ci/deploy/databend-query-standalone-embedded-meta.sh
```

2. 准备查询客户端
- 使用mysql client
```
mysql -h127.0.0.1 -uroot -P3307
```

- 或者使用databend http query handler
```
curl -u root: --request POST '127.0.0.1:8000/v1/query/' --header 'Content-Type: application/json' --data-raw '{"sql": ${YourSqlStatement}}'
```

3. 建表

```sql
CREATE TABLE lineorder
(
    LO_ORDERKEY             INT,
    LO_LINENUMBER           SMALLINT,
    LO_CUSTKEY              INT,
    LO_PARTKEY              INT,
    LO_SUPPKEY              INT,
    LO_ORDERDATE            Date,
    LO_ORDERPRIORITY        VARCHAR,
    LO_SHIPPRIORITY         SMALLINT,
    LO_QUANTITY             SMALLINT,
    LO_EXTENDEDPRICE        INT,
    LO_ORDTOTALPRICE        INT,
    LO_DISCOUNT             SMALLINT,
    LO_REVENUE              INT,
    LO_SUPPLYCOST           INT,
    LO_TAX                  SMALLINT,
    LO_COMMITDATE           DATE,
    LO_SHIPMODE             VARCHAR
)
ENGINE=FUSE STORAGE_FORMAT='native';
```
STORAGE_FORMAT='native' 即切换到pa格式, 默认是parquet

下面建parquet表, 做对比用

```sql
CREATE TABLE lineorder_parquet
(
    LO_ORDERKEY             INT,
    LO_LINENUMBER           SMALLINT,
    LO_CUSTKEY              INT,
    LO_PARTKEY              INT,
    LO_SUPPKEY              INT,
    LO_ORDERDATE            Date,
    LO_ORDERPRIORITY        VARCHAR,
    LO_SHIPPRIORITY         SMALLINT,
    LO_QUANTITY             SMALLINT,
    LO_EXTENDEDPRICE        INT,
    LO_ORDTOTALPRICE        INT,
    LO_DISCOUNT             SMALLINT,
    LO_REVENUE              INT,
    LO_SUPPLYCOST           INT,
    LO_TAX                  SMALLINT,
    LO_COMMITDATE           DATE,
    LO_SHIPMODE             VARCHAR
)
ENGINE=FUSE;
```

4. 载入测试数据

```
git clone https://github.com/doki23/ssb-dbgen.git

cd ssb-dbgen

docker build . -t ssb-dbgen:latest

# -s 3 => scale = 3g
docker run -v $(pwd)/data:/data --rm ssb-dbgen:latest -s 3 -T l

time curl -XPUT 'http://root:@127.0.0.1:8000/v1/streaming_load' -H 'insert_sql: insert into lineorder format CSV' -F 'upload=@"./data/lineorder.tbl"'

time curl -XPUT 'http://root:@127.0.0.1:8000/v1/streaming_load' -H 'insert_sql: insert into lineorder_parquet format CSV' -F 'upload=@"./data/lineorder.tbl"'
```

5. 查询
```sql
## pa
curl -w 'Time: %{time_total}\n' http://root@localhost:8124\?max_storage_io_requests\=16 -d "select * from lineorder limit 10000000" 2>&1 | grep -e '^Time: ' | sed 's/Time: //'

## parquet
curl -w 'Time: %{time_total}\n' http://root@localhost:8124\?max_storage_io_requests\=16 -d "select * from lineorder_parquet limit 10000000" 2>&1 | grep -e '^Time: ' | sed 's/Time: //'
```
# Performance
TODO
