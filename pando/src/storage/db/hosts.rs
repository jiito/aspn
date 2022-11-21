use crate::{models, storage::db::establish_connection};
use diesel::pg::PgConnection;
use diesel::prelude::*;

fn connect_host_to_function(host: &models::Host, func: &models::Function, conn: &mut PgConnection) {
    use crate::schema::hosts_functions;

    let new_host_function = models::NewHostFunctionIDs {
        host_id: host.id,
        function_id: func.id,
    };

    let host_function = diesel::insert_into(hosts_functions::table)
        .values(&new_host_function)
        .get_result::<models::HostsFunctions>(conn)
        .expect("Error saving function");
}

impl models::NewHost {
    fn save(&self, conn: &mut PgConnection) {
        use crate::schema::hosts;

        diesel::insert_into(hosts::table)
            .values(self)
            .get_result::<models::Host>(conn)
            .expect("Could not save host");
    }
}
