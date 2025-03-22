use anyhow::{Context, Error};
use sqlx::{PgPool, Row};
use testcontainers_modules::{postgres::Postgres, testcontainers::runners::AsyncRunner};
use chrono::{Utc, NaiveDate, NaiveTime};
use uuid::Uuid;
use rust_decimal::Decimal;
use ipnetwork::IpNetwork;
use mac_address::MacAddress;

use crate::migrate::POSTGRES_MIGRATOR;

mod autostructs;

pub async fn test_integration() -> Result<(), Error> {
    // startup the module
    let node = Postgres::default()
        .start()
        .await
        .context("postgres container did not start up ok")?;

    let port = node.get_host_port_ipv4(5432).await.context(
        "port that the postgres docker image is listening is not available or discoverable",
    )?;

    // prepare connection string
    let url = &format!("postgres://postgres:postgres@127.0.0.1:{port}/postgres");

    println!("connecting to postgres at {url}");

    let pool = PgPool::connect(url)
        .await
        .context("postgres connection pool failed to establish connection with test container")?;

    POSTGRES_MIGRATOR
        .run(&pool)
        .await
        .context("migrations used for testing purposes failed to apply")?;

    // Test all tables
    test_basic_types(&pool).await?;
    test_character_types(&pool).await?;
    test_binary_types(&pool).await?;
    test_date_time_types(&pool).await?;
    test_boolean_type(&pool).await?;
    test_uuid_type(&pool).await?;
    test_network_address_types(&pool).await?;
    test_bit_string_types(&pool).await?;
    test_text_search_types(&pool).await?;
    test_xml_type(&pool).await?;
    test_json_types(&pool).await?;
    test_range_types(&pool).await?;
    test_geometric_types(&pool).await?;
    test_array_types(&pool).await?;
    test_composite_type(&pool).await?;
    test_enum_type(&pool).await?;
    test_fdw(&pool).await?;
    test_oid_types(&pool).await?;
    test_special_types(&pool).await?;
    test_foreign_keys(&pool).await?;

    Ok(())
}

async fn test_basic_types(pool: &PgPool) -> Result<(), Error> {
    // Insert test data
    sqlx::query(
        "INSERT INTO table_basic_types (integer_column, smallint_column, bigint_column, numeric_column, 
         real_column, double_precision_column, money_column) 
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
    )
    .bind(42)
    .bind(Some(16i16))
    .bind(9999i64)
    .bind(Some(Decimal::new(314, 2)))
    .bind(Some(3.14f32))
    .bind(3.14f64)
    .bind(Some(Decimal::new(1000, 2)))
    .execute(pool)
    .await
    .context("Failed to insert test data")?;

    // Query and verify using generated struct
    let result = sqlx::query_as::<_, autostructs::TableBasicType>(
        "SELECT * FROM table_basic_types WHERE integer_column = $1",
    )
    .bind(42)
    .fetch_one(pool)
    .await
    .context("Failed to query data")?;

    assert_eq!(result.integer_column, 42);
    assert_eq!(result.smallint_column, Some(16i16));
    assert_eq!(result.bigint_column, 9999i64);
    assert_eq!(result.numeric_column, Some(Decimal::new(314, 2)));
    assert!((result.real_column.unwrap() - 3.14f32).abs() < f32::EPSILON);
    assert!((result.double_precision_column - 3.14f64).abs() < f64::EPSILON);
    assert_eq!(result.money_column, Some(Decimal::new(1000, 2)));

    Ok(())
}

async fn test_character_types(pool: &PgPool) -> Result<(), Error> {
    // Insert test data
    sqlx::query(
        "INSERT INTO table_character_types (varchar_column, char_column, text_column, citext_column) 
         VALUES ($1, $2, $3, $4)",
    )
    .bind("test varchar")
    .bind(Some("fixed char"))
    .bind(Some("test text"))
    .bind(Some("test citext"))
    .execute(pool)
    .await
    .context("Failed to insert test data")?;

    // Query and verify using generated struct
    let result = sqlx::query_as::<_, autostructs::TableCharacterType>(
        "SELECT * FROM table_character_types WHERE varchar_column = $1",        
    )
    .bind("test varchar")
    .fetch_one(pool)
    .await
    .context("Failed to query data")?;

    assert_eq!(result.varchar_column, "test varchar");
    assert_eq!(result.char_column, Some("fixed char".to_string()));
    assert_eq!(result.text_column, Some("test text".to_string()));
    assert_eq!(result.citext_column, Some("test citext".to_string()));

    Ok(())
}

async fn test_binary_types(pool: &PgPool) -> Result<(), Error> {
    let test_bytes = vec![1, 2, 3, 4];
    
    // Insert test data
    sqlx::query(
        "INSERT INTO table_binary_types (bytea_column) VALUES ($1)",
    )
    .bind(&test_bytes)
    .execute(pool)
    .await
    .context("Failed to insert test data")?;

    // Query and verify using generated struct
    let result = sqlx::query_as::<_, autostructs::TableBinaryType>(
        "SELECT * FROM table_binary_types WHERE bytea_column = $1",
    )
    .bind(&test_bytes)
    .fetch_one(pool)
    .await
    .context("Failed to query data")?;

    assert_eq!(result.bytea_column, test_bytes);

    Ok(())
}

async fn test_date_time_types(pool: &PgPool) -> Result<(), Error> {
    let now = Utc::now();
    let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
    let time = NaiveTime::from_hms_opt(12, 0, 0).unwrap();

    // Insert test data
    sqlx::query(
        "INSERT INTO table_date_time_types (timestamp_column, timestamp_tz_column, date_column, 
         time_column, time_tz_column, interval_column) 
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(now.naive_utc())
    .bind(Some(now))
    .bind(date)
    .bind(Some(time))
    .bind(time)
    .bind(Some("1 day"::sqlx::types::PgInterval))
    .execute(pool)
    .await
    .context("Failed to insert test data")?;

    // Query and verify using generated struct
    let result = sqlx::query_as::<_, autostructs::TableDateTimeType>(
        "SELECT * FROM table_date_time_types WHERE timestamp_column = $1",
    )
    .bind(now.naive_utc())
    .fetch_one(pool)
    .await
    .context("Failed to query data")?;

    assert_eq!(result.timestamp_column, now.naive_utc());
    assert_eq!(result.timestamp_tz_column, Some(now));
    assert_eq!(result.date_column, date);
    assert_eq!(result.time_column, Some(time));
    assert_eq!(result.time_tz_column, time);
    assert!(result.interval_column.is_some());

    Ok(())
}

async fn test_boolean_type(pool: &PgPool) -> Result<(), Error> {
    // Insert test data
    sqlx::query(
        "INSERT INTO table_boolean_type (boolean_column) VALUES ($1)",
    )
    .bind(true)
    .execute(pool)
    .await
    .context("Failed to insert test data")?;

    // Query and verify using generated struct
    let result = sqlx::query_as!(
        autostructs::TableBooleanType,
        "SELECT * FROM table_boolean_type WHERE boolean_column = $1",
        true
    )
    .fetch_one(pool)
    .await
    .context("Failed to query data")?;

    assert!(result.boolean_column);

    Ok(())
}

async fn test_uuid_type(pool: &PgPool) -> Result<(), Error> {
    let uuid = Uuid::new_v4();

    // Insert test data
    sqlx::query(
        "INSERT INTO table_uuid_type (id) VALUES ($1)",
    )
    .bind(uuid)
    .execute(pool)
    .await
    .context("Failed to insert test data")?;

    // Query and verify using generated struct
    let result = sqlx::query_as::<_, autostructs::TableUuidType>
        "SELECT * FROM table_uuid_type WHERE id = $1",
        uuid
    )
    .fetch_one(pool)
    .await
    .context("Failed to query data")?;

    assert_eq!(result.id, uuid);

    Ok(())
}

async fn test_network_address_types(pool: &PgPool) -> Result<(), Error> {
    let ip = IpNetwork::V4("192.168.1.1/32".parse().unwrap());
    let mac = MacAddress::new([0x00, 0x11, 0x22, 0x33, 0x44, 0x55]);

    // Insert test data
    sqlx::query(
        "INSERT INTO table_network_address_types (inet_column, cidr_column, macaddr_column, macaddr8_column) 
         VALUES ($1, $2, $3, $4)",
    )
    .bind(ip.to_string())
    .bind(Some("192.168.1.0/24"))
    .bind(Some(mac))
    .bind(Some(mac))
    .execute(pool)
    .await
    .context("Failed to insert test data")?;

    // Query and verify using generated struct
    let result = sqlx::query_as!(
        autostructs::TableNetworkAddressType,
        "SELECT * FROM table_network_address_types WHERE inet_column = $1",
        ip.to_string()
    )
    .fetch_one(pool)
    .await
    .context("Failed to query data")?;

    assert_eq!(result.inet_column, ip.to_string());
    assert_eq!(result.cidr_column, Some("192.168.1.0/24"));
    assert_eq!(result.macaddr_column, Some(mac));
    assert_eq!(result.macaddr8_column, Some(mac));

    Ok(())
}

async fn test_bit_string_types(pool: &PgPool) -> Result<(), Error> {
    // Insert test data
    sqlx::query(
        "INSERT INTO table_bit_string_types (bit_column, bit_varying_column) 
         VALUES ($1, $2)",
    )
    .bind("10101010")
    .bind(Some("1010"))
    .execute(pool)
    .await
    .context("Failed to insert test data")?;

    // Query and verify using generated struct
    let result = sqlx::query_as!(
        autostructs::TableBitStringType,
        "SELECT * FROM table_bit_string_types WHERE bit_column = $1",
        "10101010"
    )
    .fetch_one(pool)
    .await
    .context("Failed to query data")?;

    assert_eq!(result.bit_column, "10101010");
    assert_eq!(result.bit_varying_column, Some("1010"));

    Ok(())
}

async fn test_text_search_types(pool: &PgPool) -> Result<(), Error> {
    // Insert test data
    sqlx::query(
        "INSERT INTO table_text_search_types (tsvector_column, tsquery_column) 
         VALUES ($1, $2)",
    )
    .bind("test search")
    .bind(Some("test & search"))
    .execute(pool)
    .await
    .context("Failed to insert test data")?;

    // Query and verify using generated struct
    let result = sqlx::query_as::<_, autostructs::TableTextSearchType>(
        "SELECT * FROM table_text_search_types WHERE tsvector_column = $1",
    )
    .bind("test search")
    .fetch_one(pool)
    .await
    .context("Failed to query data")?;

    assert_eq!(result.tsvector_column, "test search");
    assert_eq!(result.tsquery_column, Some("test & search".to_string()));

    Ok(())
}

async fn test_xml_type(pool: &PgPool) -> Result<(), Error> {
    let xml_data = "<test>data</test>";

    // Insert test data
    sqlx::query(
        "INSERT INTO table_xml_type (xml_column) VALUES ($1)",
    )
    .bind(xml_data)
    .execute(pool)
    .await
    .context("Failed to insert test data")?;

    // Query and verify using generated struct
    let result = sqlx::query_as!(
        autostructs::TableXmlType,
        "SELECT * FROM table_xml_type WHERE xml_column = $1",
        xml_data
    )
    .fetch_one(pool)
    .await
    .context("Failed to query data")?;

    assert_eq!(result.xml_column, xml_data);

    Ok(())
}

async fn test_json_types(pool: &PgPool) -> Result<(), Error> {
    let json_data = serde_json::json!({"key": "value"});

    // Insert test data
    sqlx::query(
        "INSERT INTO table_json_types (json_column, jsonb_column) 
         VALUES ($1, $2)",
    )
    .bind(Some(json_data.clone()))
    .bind(json_data.clone())
    .execute(pool)
    .await
    .context("Failed to insert test data")?;

    // Query and verify using generated struct
    let result = sqlx::query_as!(
        autostructs::TableJsonType,
        "SELECT * FROM table_json_types WHERE jsonb_column = $1",
        json_data
    )
    .fetch_one(pool)
    .await
    .context("Failed to query data")?;

    assert_eq!(result.json_column, Some(json_data.clone()));
    assert_eq!(result.jsonb_column, json_data);

    Ok(())
}

async fn test_range_types(pool: &PgPool) -> Result<(), Error> {
    // Insert test data
    sqlx::query(
        "INSERT INTO table_range_types (int4range_column, int8range_column, numrange_column, 
         tsrange_column, tstzrange_column, daterange_column) 
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind("[1,5)")
    .bind(Some("[1,10)"))
    .bind("[1.1,5.5)")
    .bind(Some("[2024-01-01,2024-12-31)"))
    .bind(Some("[2024-01-01 00:00:00+00,2024-12-31 23:59:59+00)"))
    .bind(Some("[2024-01-01,2024-12-31)"))
    .execute(pool)
    .await
    .context("Failed to insert test data")?;

    // Query and verify using generated struct
    let result = sqlx::query_as!(
        autostructs::TableRangeType,
        "SELECT * FROM table_range_types WHERE int4range_column = $1",
        "[1,5)"
    )
    .fetch_one(pool)
    .await
    .context("Failed to query data")?;

    assert_eq!(result.int4range_column, "[1,5)");
    assert_eq!(result.int8range_column, Some("[1,10)"));
    assert_eq!(result.numrange_column, "[1.1,5.5)");
    assert_eq!(result.tsrange_column, Some("[2024-01-01,2024-12-31)"));
    assert_eq!(result.tstzrange_column, Some("[2024-01-01 00:00:00+00,2024-12-31 23:59:59+00)"));
    assert_eq!(result.daterange_column, Some("[2024-01-01,2024-12-31)"));

    Ok(())
}

async fn test_geometric_types(pool: &PgPool) -> Result<(), Error> {
    // Insert test data
    sqlx::query(
        "INSERT INTO table_geometric_types (point_column, line_column, lseg_column, box_column, 
         path_column, polygon_column, circle_column) 
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
    )
    .bind("(1,1)")
    .bind(Some("{(1,1),(2,2)}"))
    .bind("[(1,1),(2,2)]")
    .bind(Some("((1,1),(2,2))"))
    .bind(Some("[(1,1),(2,2),(1,1)]"))
    .bind("((1,1),(2,1),(2,2),(1,2),(1,1))")
    .bind(Some("<(1,1),1>"))
    .execute(pool)
    .await
    .context("Failed to insert test data")?;

    // Query and verify using generated struct
    let result = sqlx::query_as!(
        autostructs::TableGeometricType,
        "SELECT * FROM table_geometric_types WHERE point_column = $1",
        "(1,1)"
    )
    .fetch_one(pool)
    .await
    .context("Failed to query data")?;

    assert_eq!(result.point_column, "(1,1)");
    assert_eq!(result.line_column, Some("{(1,1),(2,2)}"));
    assert_eq!(result.lseg_column, "[(1,1),(2,2)]");
    assert_eq!(result.box_column, Some("((1,1),(2,2))"));
    assert_eq!(result.path_column, Some("[(1,1),(2,2),(1,1)]"));
    assert_eq!(result.polygon_column, "((1,1),(2,1),(2,2),(1,2),(1,1))");
    assert_eq!(result.circle_column, Some("<(1,1),1>"));

    Ok(())
}

async fn test_array_types(pool: &PgPool) -> Result<(), Error> {
    // Insert test data
    sqlx::query(
        "INSERT INTO table_array_types (integer_array_column, text_array_column) 
         VALUES ($1, $2)",
    )
    .bind(Some(vec![1, 2, 3]))
    .bind(vec!["a", "b", "c"])
    .execute(pool)
    .await
    .context("Failed to insert test data")?;

    // Query and verify using generated struct
    let result = sqlx::query_as!(
        autostructs::TableArrayType,
        "SELECT * FROM table_array_types WHERE text_array_column = $1",
        vec!["a", "b", "c"]
    )
    .fetch_one(pool)
    .await
    .context("Failed to query data")?;

    assert_eq!(result.integer_array_column, Some(vec![1, 2, 3]));
    assert_eq!(result.text_array_column, vec!["a", "b", "c"]);

    Ok(())
}

async fn test_composite_type(pool: &PgPool) -> Result<(), Error> {
    // Insert test data
    sqlx::query(
        "INSERT INTO table_composite_type (address_column) 
         VALUES (ROW('123 Main St', 'City', '12345'))",
    )
    .execute(pool)
    .await
    .context("Failed to insert test data")?;

    // Query and verify using generated struct
    let result = sqlx::query_as!(
        autostructs::TableCompositeType,
        "SELECT * FROM table_composite_type WHERE address_column = ROW('123 Main St', 'City', '12345')"
    )
    .fetch_one(pool)
    .await
    .context("Failed to query data")?;

    assert_eq!(result.address_column.street, "123 Main St");
    assert_eq!(result.address_column.city, "City");
    assert_eq!(result.address_column.zip_code, "12345");

    Ok(())
}

async fn test_enum_type(pool: &PgPool) -> Result<(), Error> {
    // Insert test data
    sqlx::query(
        "INSERT INTO table_enum_type (mood_column) VALUES ($1)",
    )
    .bind("happy")
    .execute(pool)
    .await
    .context("Failed to insert test data")?;

    // Query and verify using generated struct
    let result = sqlx::query_as!(
        autostructs::TableEnumType,
        "SELECT * FROM table_enum_type WHERE mood_column = $1",
        "happy"
    )
    .fetch_one(pool)
    .await
    .context("Failed to query data")?;

    assert_eq!(result.mood_column, "happy");

    Ok(())
}

async fn test_fdw(pool: &PgPool) -> Result<(), Error> {
    // Insert test data
    sqlx::query(
        "INSERT INTO table_fdw (foreign_data_column) VALUES ($1)",
    )
    .bind("test data")
    .execute(pool)
    .await
    .context("Failed to insert test data")?;

    // Query and verify using generated struct
    let result = sqlx::query_as!(
        autostructs::TableFdw,
        "SELECT * FROM table_fdw WHERE foreign_data_column = $1",
        "test data"
    )
    .fetch_one(pool)
    .await
    .context("Failed to query data")?;

    assert_eq!(result.foreign_data_column, "test data");

    Ok(())
}

async fn test_oid_types(pool: &PgPool) -> Result<(), Error> {
    // Insert test data
    sqlx::query(
        "INSERT INTO table_oid_types (oid_column) VALUES ($1)",
    )
    .bind(1234)
    .execute(pool)
    .await
    .context("Failed to insert test data")?;

    // Query and verify using generated struct
    let result = sqlx::query_as!(
        autostructs::TableOidType,
        "SELECT * FROM table_oid_types WHERE oid_column = $1",
        1234
    )
    .fetch_one(pool)
    .await
    .context("Failed to query data")?;

    assert_eq!(result.oid_column, 1234);

    Ok(())
}

async fn test_special_types(pool: &PgPool) -> Result<(), Error> {
    let uuid = Uuid::new_v4();

    // Insert test data
    sqlx::query(
        "INSERT INTO table_special_types (pg_lsn_column, uuid_column) 
         VALUES ($1, $2)",
    )
    .bind("0/16B7898")
    .bind(Some(uuid))
    .execute(pool)
    .await
    .context("Failed to insert test data")?;

    // Query and verify using generated struct
    let result = sqlx::query_as!(
        autostructs::TableSpecialType,
        "SELECT * FROM table_special_types WHERE pg_lsn_column = $1",
        "0/16B7898"
    )
    .fetch_one(pool)
    .await
    .context("Failed to query data")?;

    assert_eq!(result.pg_lsn_column, "0/16B7898");
    assert_eq!(result.uuid_column, Some(uuid));

    Ok(())
}

async fn test_foreign_keys(pool: &PgPool) -> Result<(), Error> {
    // First insert data into referenced tables
    let basic_id = sqlx::query!(
        "INSERT INTO table_basic_types (integer_column, bigint_column, double_precision_column) 
         VALUES ($1, $2, $3) RETURNING id",
        42, 9999i64, 3.14f64
    )
    .fetch_one(pool)
    .await
    .context("Failed to insert basic type data")?
    .id;

    let char_id = sqlx::query!(
        "INSERT INTO table_character_types (varchar_column) VALUES ($1) RETURNING id",
        "test varchar"
    )
    .fetch_one(pool)
    .await
    .context("Failed to insert character type data")?
    .id;

    let uuid = Uuid::new_v4();
    sqlx::query!(
        "INSERT INTO table_uuid_type (id) VALUES ($1)",
        uuid
    )
    .execute(pool)
    .await
    .context("Failed to insert uuid data")?;

    // Insert test data with foreign keys
    sqlx::query(
        "INSERT INTO table_foreign_keys (fk_basic, fk_char, fk_uuid) 
         VALUES ($1, $2, $3)",
    )
    .bind(basic_id)
    .bind(char_id)
    .bind(uuid)
    .execute(pool)
    .await
    .context("Failed to insert foreign key data")?;

    // Query and verify using generated struct
    let result = sqlx::query_as!(
        autostructs::TableForeignKey,
        "SELECT * FROM table_foreign_keys WHERE fk_basic = $1",
        basic_id
    )
    .fetch_one(pool)
    .await
    .context("Failed to query data")?;

    assert_eq!(result.fk_basic, Some(basic_id));
    assert_eq!(result.fk_char, Some(char_id));
    assert_eq!(result.fk_uuid, Some(uuid));

    Ok(())
}
