create table functions (
       id serial PRIMARY KEY,
       ref_name VARCHAR UNIQUE NOT NULL,
       route VARCHAR NOT NULL,
       project_id INT NOT NULL,
       CONSTRAINT fk_project FOREIGN KEY(project_id) REFERENCES projects(id)
);

insert into functions (ref_name, route, project_id)
values('test_ref_name', '/test_route', 1);
