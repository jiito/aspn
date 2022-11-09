-- This file should undo anything in `up.sql`
update table developers set project_id=0 where project_id is null
alter table developers alter column project_id INT NOT NULL;
