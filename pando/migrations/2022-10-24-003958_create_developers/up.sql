CREATE TABLE developers (
       id serial PRIMARY KEY,
       name VARCHAR NOT NULL,
       ip_address INET NOT NULL,
       project_id INT NOT NULL,
       CONSTRAINT fk_project FOREIGN KEY(project_id) REFERENCES projects(id)
)
