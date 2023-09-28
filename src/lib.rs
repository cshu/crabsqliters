use crabrs::*;

pub fn result_rows_empty(res: rusqlite::Result<rusqlite::Rows<'_>>) -> Result<bool, CustomErr> {
    let mut rows = res?;
    Ok(rows.next()?.is_none())
}

//you might be able to write this with generics but personally I prefer macro
macro_rules! query_n_collect_into_vec {
    ($fnnm: ident, $elemty: ty) => {
        pub fn $fnnm(res: rusqlite::Result<rusqlite::Rows<'_>>) -> CustRes<Vec<$elemty>> {
            let mut rows = res?;
            let mut retval: Vec<$elemty> = vec![];
            while let Some(row) = rows.next()? {
                retval.push(row.get(0)?);
            }
            Ok(retval)
        }
    };
}

query_n_collect_into_vec! {query_n_collect_into_vec_string, String}
query_n_collect_into_vec! {query_n_collect_into_vec_i64, i64}

macro_rules! exec_with_slice {
    ($fnnm: ident, $elemty: ty) => {
        pub fn $fnnm(db: &rusqlite::Connection, sqlstr: &str, paramsli: &[$elemty]) -> CustRes<()> {
            let mut cached_stmt = db.prepare_cached(sqlstr)?;
            for slielem in paramsli {
                let l_elem: $elemty = *slielem;
                cached_stmt.execute((l_elem,))?;
            }
            Ok(())
        }
    };
}

exec_with_slice! {exec_with_slice_i64, i64}
