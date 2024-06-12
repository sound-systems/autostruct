-- Table 1: Basic Types
CREATE TABLE table_basic_types (
    id SERIAL PRIMARY KEY,
    integer_column INTEGER NOT NULL,
    smallint_column SMALLINT,
    bigint_column BIGINT NOT NULL,
    numeric_column NUMERIC(10, 2),
    real_column REAL,
    double_precision_column DOUBLE PRECISION NOT NULL,
    serial_column SERIAL,
    bigserial_column BIGSERIAL,
    money_column MONEY
);

-- Table 2: Character Types
CREATE TABLE table_character_types (
    id SERIAL PRIMARY KEY,
    varchar_column VARCHAR(255) NOT NULL,
    char_column CHAR(10),
    text_column TEXT,
    citext_column CITEXT,
    foreign_key_basic INT REFERENCES table_basic_types(id)
);

-- Table 3: Binary Types
CREATE TABLE table_binary_types (
    id SERIAL PRIMARY KEY,
    bytea_column BYTEA NOT NULL
);

-- Table 4: Date/Time Types
CREATE TABLE table_date_time_types (
    id SERIAL PRIMARY KEY,
    timestamp_column TIMESTAMP NOT NULL,
    timestamp_tz_column TIMESTAMP WITH TIME ZONE,
    date_column DATE NOT NULL,
    time_column TIME,
    time_tz_column TIME WITH TIME ZONE NOT NULL,
    interval_column INTERVAL
);

-- Table 5: Boolean Type
CREATE TABLE table_boolean_type (
    id SERIAL PRIMARY KEY,
    boolean_column BOOLEAN NOT NULL,
    foreign_key_datetime INT REFERENCES table_date_time_types(id)
);

-- Table 6: UUID Type
CREATE TABLE table_uuid_type (
    id UUID DEFAULT uuid_generate_v4() PRIMARY KEY,
    foreign_key_boolean INT REFERENCES table_boolean_type(id)
);

-- Table 7: Network Address Types
CREATE TABLE table_network_address_types (
    id SERIAL PRIMARY KEY,
    inet_column INET NOT NULL,
    cidr_column CIDR,
    macaddr_column MACADDR,
    macaddr8_column MACADDR8
);

-- Table 8: Bit String Types
CREATE TABLE table_bit_string_types (
    id SERIAL PRIMARY KEY,
    bit_column BIT(8) NOT NULL,
    bit_varying_column BIT VARYING(8)
);

-- Table 9: Text Search Types
CREATE TABLE table_text_search_types (
    id SERIAL PRIMARY KEY,
    tsvector_column TSVECTOR NOT NULL,
    tsquery_column TSQUERY
);

-- Table 10: XML Type
CREATE TABLE table_xml_type (
    id SERIAL PRIMARY KEY,
    xml_column XML NOT NULL
);

-- Table 11: JSON Types
CREATE TABLE table_json_types (
    id SERIAL PRIMARY KEY,
    json_column JSON,
    jsonb_column JSONB NOT NULL,
    foreign_key_character INT REFERENCES table_character_types(id)
);

-- Table 12: Range Types
CREATE TABLE table_range_types (
    id SERIAL PRIMARY KEY,
    int4range_column INT4RANGE NOT NULL,
    int8range_column INT8RANGE,
    numrange_column NUMRANGE NOT NULL,
    tsrange_column TSRANGE,
    tstzrange_column TSTZRANGE,
    daterange_column DATERANGE
);

-- Table 13: Geometric Types
CREATE TABLE table_geometric_types (
    id SERIAL PRIMARY KEY,
    point_column POINT NOT NULL,
    line_column LINE,
    lseg_column LSEG NOT NULL,
    box_column BOX,
    path_column PATH,
    polygon_column POLYGON NOT NULL,
    circle_column CIRCLE
);

-- Table 14: Arrays
CREATE TABLE table_array_types (
    id SERIAL PRIMARY KEY,
    integer_array_column INTEGER [],
    text_array_column TEXT [] NOT NULL
);

-- Table 15: Composite Types
CREATE TYPE address AS (
    street VARCHAR(255),
    city VARCHAR(255),
    zip_code VARCHAR(10)
);

CREATE TABLE table_composite_type (
    id SERIAL PRIMARY KEY,
    address_column address NOT NULL,
    foreign_key_network INT REFERENCES table_network_address_types(id)
);

-- Table 16: Enumerated Types
CREATE TYPE mood AS ENUM ('sad', 'ok', 'happy');

CREATE TABLE table_enum_type (
    id SERIAL PRIMARY KEY,
    mood_column mood NOT NULL
);

-- Table 17: Foreign Data Wrapper Types
CREATE TABLE table_fdw (
    id SERIAL PRIMARY KEY,
    foreign_data_column VARCHAR(255)
);

-- Table 18: Object Identifier Types
CREATE TABLE table_oid_types (
    id SERIAL PRIMARY KEY,
    oid_column OID NOT NULL
);

-- Table 19: Other Special Types
CREATE TABLE table_special_types (
    id SERIAL PRIMARY KEY,
    pg_lsn_column PG_LSN NOT NULL,
    txid_snapshot_column TXID_SNAPSHOT,
    uuid_column UUID
);

-- Table 20: Foreign Keys
CREATE TABLE table_foreign_keys (
    id SERIAL PRIMARY KEY,
    fk_basic INT REFERENCES table_basic_types(id),
    fk_char INT REFERENCES table_character_types(id),
    fk_binary INT REFERENCES table_binary_types(id),
    fk_datetime INT REFERENCES table_date_time_types(id),
    fk_boolean INT REFERENCES table_boolean_type(id),
    fk_uuid UUID REFERENCES table_uuid_type(id),
    fk_network INT REFERENCES table_network_address_types(id),
    fk_bit_string INT REFERENCES table_bit_string_types(id),
    fk_text_search INT REFERENCES table_text_search_types(id),
    fk_xml INT REFERENCES table_xml_type(id),
    fk_json INT REFERENCES table_json_types(id),
    fk_range INT REFERENCES table_range_types(id),
    fk_geometric INT REFERENCES table_geometric_types(id),
    fk_array INT REFERENCES table_array_types(id),
    fk_composite INT REFERENCES table_composite_type(id),
    fk_enum INT REFERENCES table_enum_type(id),
    fk_fdw INT REFERENCES table_fdw(id),
    fk_oid INT REFERENCES table_oid_types(id),
    fk_special INT REFERENCES table_special_types(id)
);