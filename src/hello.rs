#[cfg(any(test, feature = "pg_test"))]
#[pgrx::pg_schema]
mod tests {
    use pgrx::prelude::*;

    #[pg_test]
    #[search_path(@extschema@)]
    fn plrust_basic() -> spi::Result<()> {
        let definition = r#"
            CREATE FUNCTION get_one() RETURNS INT
                LANGUAGE PLRUST AS
            $$
                Ok(Some(1))
            $$;
        "#;
        Spi::run(definition)?;

        let result = Spi::get_one::<i64>(
            r#"
            SELECT get_one($1);
        "#);
        assert_eq!(result, Ok(Some(())));
        Ok(())
    }

}
