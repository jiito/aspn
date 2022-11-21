-- Your SQL goes here

CREATE TABLE hosts (
       id serial PRIMARY KEY,
       ip_address INET NOT NULL,
       user_token VARCHAR NOT NULL
);

CREATE TABLE hosts_functions (
       function_id INT NOT NULL,
       host_id INT NOT NULL,
       PRIMARY KEY (function_id, host_id),
       CONSTRAINT fk_host FOREIGN KEY(host_id) REFERENCES hosts(id),
       CONSTRAINT fk_function FOREIGN KEY(function_id) REFERENCES functions(id)
);
