use std::collections::{HashMap, HashSet};

use tipb::ColumnInfo;
use tidb_query_datatype::{
    codec::{self, datum::DatumDecoder, table}, expr::EvalContext, prelude::*, FieldTypeTp
};


// /// Cuts a non-empty row in row format v1.
// fn cut_row_v1(data: Vec<u8>, cols: &HashSet<i64>) -> Result<table::RowColsDict> {
//     let meta_map = {
//         let mut meta_map = HashMap::with_capacity_and_hasher(cols.len(), Default::default());
//         let length = data.len();
//         let mut tmp_data: &[u8] = data.as_ref();
//         while !tmp_data.is_empty() && meta_map.len() < cols.len() {
//             let id = tmp_data.read_datum()?.i64();
//             let offset = length - tmp_data.len();
//             let (val, rem) = codec::datum::split_datum(tmp_data, false)?;
//             if cols.contains(&id) {
//                 meta_map.insert(id, table::RowColMeta::new(offset, val.len()));
//             }
//             tmp_data = rem;
//         }
//         meta_map
//     };
//     Ok(table::RowColsDict::new(meta_map, data))
// }

// fn to_hash_map(row: &table::RowColsDict) -> HashMap<i64, Vec<u8>> {
//     let mut data = HashMap::with_capacity_and_hasher(row.cols.len(), Default::default());
//     if row.is_empty() {
//         return data;
//     }
//     for (key, meta) in &row.cols {
//         data.insert(
//             *key,
//             row.value[meta.offset..(meta.offset + meta.length)].to_vec(),
//         );
//     }
//     data
// }

// fn cut_row_as_owned(bs: &[u8], col_id_set: &HashSet<i64>) -> HashMap<i64, Vec<u8>> {
//     let is_empty_row =
//         col_id_set.is_empty() || bs.is_empty() || (bs.len() == 1 && bs[0] == codec::datum::NIL_FLAG);
//     let res = if is_empty_row {
//         table::RowColsDict::new(HashMap::default(), bs.to_vec())
//     } else {
//         cut_row_v1(bs.to_vec(), col_id_set).unwrap()
//     };
//     to_hash_map(&res)
// }

fn main() {
    let mut duration_col = ColumnInfo::default();
    duration_col
        .as_mut_accessor()
        .set_tp(FieldTypeTp::Duration)
        .set_decimal(2);

    let mut cols = HashMap::from_iter([
        (1, FieldTypeTp::LongLong.into()),
        (2, FieldTypeTp::VarChar.into()),
        (3, FieldTypeTp::NewDecimal.into()),
        (5, FieldTypeTp::Json.into()),
        (6, duration_col),
    ]);

    let duration_row = codec::mysql::duration::Duration::parse(&mut EvalContext::default(), "23:23:23.666", 2).unwrap();

    let mut row = HashMap::from_iter([
        (1, codec::Datum::I64(100)),
        (2, codec::Datum::Bytes(b"abc".to_vec())),
        (3, codec::Datum::Dec(10.into())),
        (5, codec::Datum::Json(r#"{"name": "John"}"#.parse().unwrap())),
        (6, codec::Datum::Dur(duration_row)),
    ]);

    let mut ctx = EvalContext::default();
    let col_ids: Vec<_> = row.iter().map(|(&id, _)| id).collect();
    let col_values: Vec<_> = row.values().cloned().collect();
    let mut col_encoded: HashMap<_, _> = row
        .iter()
        .map(|(k, v)| {
            let f = table::flatten(&mut ctx, v.clone()).unwrap();
            (*k, codec::datum::encode_value(&mut ctx, &[f]).unwrap())
        })
        .collect();
    let mut col_id_set: HashSet<_> = col_ids.iter().cloned().collect();

    let bs = table::encode_row(&mut ctx, col_values, &col_ids).unwrap();
    println!("Encoded row: {:?}", hex::encode(&bs));
    assert!(!bs.is_empty());
    let mut ctx = EvalContext::default();
    let r = table::decode_row(&mut bs.as_slice(), &mut ctx, &cols).unwrap();
    assert_eq!(row, r);

    // let mut datums: HashMap<_, _> = cut_row_as_owned(&bs, &col_id_set);
    // assert_eq!(col_encoded, datums);

    cols.insert(4, FieldTypeTp::Float.into());
    let r = table::decode_row(&mut bs.as_slice(), &mut ctx, &cols).unwrap();
    assert_eq!(row, r);

    // col_id_set.insert(4);
    // datums = cut_row_as_owned(&bs, &col_id_set);
    // assert_eq!(col_encoded, datums);

    // cols.remove(&4);
    // cols.remove(&3);
    // let r = table::decode_row(&mut bs.as_slice(), &mut ctx, &cols).unwrap();
    // row.remove(&3);
    // assert_eq!(row, r);

    // col_id_set.remove(&3);
    // col_id_set.remove(&4);
    // datums = cut_row_as_owned(&bs, &col_id_set);
    // col_encoded.remove(&3);
    // assert_eq!(col_encoded, datums);

    let bs = table::encode_row(&mut ctx, vec![], &[]).unwrap();
    assert!(!bs.is_empty());
    println!("Encoded row: {:?}", hex::encode(&bs));
    assert!(table::decode_row(&mut bs.as_slice(), &mut ctx, &cols)
        .unwrap()
        .is_empty());
    // datums = cut_row_as_owned(&bs, &col_id_set);
    // assert!(datums.is_empty());
}
