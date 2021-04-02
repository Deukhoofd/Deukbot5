pub async fn initialise_tables() {
    let conn = super::get_connection().await;
    info!("Initialising tables");
    conn.client
        .batch_execute(
            "
CREATE TABLE IF NOT EXISTS permission_roles (
    server_id bigint NOT NULL,
    role_id bigint NOT NULL,
    permission_level smallint NOT NULL,
    PRIMARY KEY(server_id, role_id)
);

CREATE TABLE IF NOT EXISTS server_settings (
    server_id bigint NOT NULL,
    muted_role bigint NOT NULL,
    enabled_jokes varchar(255),
    PRIMARY KEY(server_id)
);

CREATE TABLE IF NOT EXISTS tags (
    server_id bigint NOT NULL,
    key varchar(25) NOT NULL,
    value varchar(255) NOT NULL,
    PRIMARY KEY(key)
);

CREATE TABLE IF NOT EXISTS warnings (
    id serial NOT NULL,
    serverId bigint NOT NULL,
    userId bigint NOT NULL,
    message varchar(255),
    PRIMARY KEY(id)
);
",
        )
        .await
        .expect("Failed to create table.");
    info!("Finished initialising tables.");
}
