use diesel::{ExpressionMethods, RunQueryDsl};
use diesel::{QueryDsl, SelectableHelper, SqliteConnection};
use rust_xlsxwriter::{ExcelDateTime, Format, Workbook, Worksheet, XlsxError};

use crate::database::{
    models::{Answer, Session, Step},
    schema::{answers, sessions, steps},
};
use std::collections::HashMap;

const BATCH_SIZE: i64 = 10000;
const TOKEN_LIST: &str = include_str!("../../data/tokens/list.json");

pub fn export_project_data(
    connection: &mut SqliteConnection,
    filepath: String,
    project_key: String,
) -> Result<(), String> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    match write_headers(worksheet) {
        Ok(_) => (),
        Err(err) => return Err(err.to_string()),
    }

    let count: i64 = answers::table
        .inner_join(steps::table.inner_join(sessions::table))
        .filter(sessions::project_key.eq(&project_key))
        .count()
        .get_result(connection)
        .unwrap();
    let mut page = 0;

    while page * BATCH_SIZE < count {
        match fetch_batch_and_write(connection, worksheet, &project_key, page * BATCH_SIZE) {
            Ok(()) => page += 1,
            Err(err) => return Err(err.to_string()),
        }
    }

    // Save the file to disk.
    workbook.save(filepath).unwrap();

    Ok(())
}

fn tokens() -> HashMap<String, String> {
    serde_json::from_str(TOKEN_LIST).unwrap()
}

fn get_token_type_from_key(input: &str) -> String {
    match tokens().get(input) {
        Some(counterpart) => counterpart.clone(),
        None => input.to_string(),
    }
}

fn fetch_batch_and_write(
    connection: &mut SqliteConnection,
    worksheet: &mut Worksheet,
    project_key: &str,
    offset: i64,
) -> Result<(), XlsxError> {
    let results: Vec<(Answer, Step, Session)> = answers::table
        .inner_join(steps::table.inner_join(sessions::table))
        .filter(sessions::project_key.eq(project_key))
        .limit(BATCH_SIZE)
        .offset(offset)
        .select((Answer::as_select(), Step::as_select(), Session::as_select()))
        .order(answers::id.asc())
        .load::<(Answer, Step, Session)>(connection)
        .unwrap();

    let format = Format::new().set_num_format("yyyy-mm-dd hh::mm:ss");

    for (row, (answer, step, session)) in results.iter().enumerate() {
        let row = 1 + row as u32;
        let created_at =
            ExcelDateTime::from_timestamp(step.created_at.assume_utc().unix_timestamp()).unwrap();
        worksheet.write(row, 0, &session.project_key)?;
        worksheet.write(row, 1, session.id)?;
        worksheet.write(row, 2, &session.theme_key)?;
        worksheet.write(row, 3, &step.question_key)?;
        worksheet.write_with_format(row, 4, created_at, &format)?;
        worksheet.write(row, 5, &answer.token_key)?;
        worksheet.write(row, 6, get_token_type_from_key(&answer.token_key))?;
        worksheet.write(row, 7, &answer.option_key)?;
    }

    Ok(())
}

fn write_headers(worksheet: &mut Worksheet) -> Result<(), XlsxError> {
    worksheet.write(0, 0, "Project key")?;
    worksheet.write(0, 1, "Session ID")?;
    worksheet.write(0, 2, "Theme key")?;
    worksheet.write(0, 3, "Question key")?;
    worksheet.write(0, 4, "Timestamp (UTC)")?;
    worksheet.write(0, 5, "RFID Token")?;
    worksheet.write(0, 6, "Token Identifier")?;
    worksheet.write(0, 7, "Option")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_token_list() {
        // NOTE: We simply call this function to assert that this doesn't panic
        tokens();
    }

    #[test]
    fn should_find_token_from_key() {
        let value = get_token_type_from_key("E2004702E4E16828021E62FE");

        assert_eq!(value, "bl_bl");
    }
}
