use crate::utils;
use crate::{models, storage::db::establish_connection};
use anyhow::Result;
use diesel::pg::PgConnection;
use diesel::prelude::*;

pub fn connect_host_to_function(
    conn: &mut PgConnection,
    host: &models::Host,
    func: &models::Function,
) -> Result<models::HostsFunctions> {
    use crate::schema::hosts_functions;

    let new_host_function = models::NewHostFunctionIDs {
        host_id: host.id,
        function_id: func.id,
    };

    let host_function = diesel::insert_into(hosts_functions::table)
        .values(&new_host_function)
        .get_result::<models::HostsFunctions>(conn)?;

    Ok(host_function)
}

pub fn find_host_by_token(conn: &mut PgConnection, token: &str) -> models::Host {
    use crate::schema::hosts::dsl;

    let query = dsl::hosts.filter(crate::schema::hosts::user_token.eq(token));

    let host = query
        .first::<models::Host>(conn)
        .expect("Could not find host");

    host
}

pub fn find_current_host() -> Result<models::Host> {
    let conn = &mut establish_connection();
    let config = utils::config::host::read_config();
    let host = find_host_by_token(conn, &config.host.unwrap().token);
    Ok(host)
}

impl models::Host {
    pub fn online(&self, conn: &mut PgConnection) -> Result<()> {
        diesel::update(self)
            .set(crate::schema::hosts::is_online.eq(true))
            .execute(conn);
        Ok(())
    }
    pub fn offline(&self, conn: &mut PgConnection) -> Result<()> {
        diesel::update(self)
            .set(crate::schema::hosts::is_online.eq(false))
            .execute(conn);
        Ok(())
    }
}

impl models::NewHost {
    pub fn save(&self, conn: &mut PgConnection) {
        use crate::schema::hosts;

        diesel::insert_into(hosts::table)
            .values(self)
            .get_result::<models::Host>(conn)
            .expect("Could not save host");
    }
}
