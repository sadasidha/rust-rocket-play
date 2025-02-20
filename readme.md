# robota-aggregate

only few methods are implemented here so far


## build

    cargo build

## run 

    cargo run



## current apis

### v1.0/aggregate?<start>&<end>&<agg_type>&<group_id>&<parent_id_pre>

This api is written for aggregate api v1.7

    curl "localhost:7128/v1.0/aggregate?start=2024-01-01&end=2024-01-01&agg_type=monthly&group_id=25&parent_id_pre=001-"