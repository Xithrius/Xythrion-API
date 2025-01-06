CREATE TABLE IF NOT EXISTS link_map_channels (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    server_id BIGINT NOT NULL,
    input_channel_id BIGINT NOT NULL,
    output_channel_id BIGINT NOT NULL
);

CREATE TABLE IF NOT EXISTS link_map_converters (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    from_link TEXT NOT NULL,
    to_link TEXT,
    xpath TEXT,
    CONSTRAINT check_xor_constraint CHECK (
        (to_link IS NOT NULL AND xpath IS NULL) OR (to_link IS NULL AND xpath IS NOT NULL)
    )
);

CREATE TABLE IF NOT EXISTS channel_converter_association (
    channel_id UUID REFERENCES link_map_channels(id) ON DELETE CASCADE,
    converter_id UUID REFERENCES link_map_converters(id) ON DELETE CASCADE,
    PRIMARY KEY (channel_id, converter_id)
);

CREATE TABLE IF NOT EXISTS trusted (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id BIGINT UNIQUE,
    at TIMESTAMP DEFAULT now()
);
