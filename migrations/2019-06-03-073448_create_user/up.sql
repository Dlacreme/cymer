CREATE TABLE person (
  id SERIAL PRIMARY KEY,
  person_role_id INT NOT NULL DEFAULT 1,
  email VARCHAR(255) NOT NULL,
  password VARCHAR(255) NOT NULL,
  created_at TIMESTAMP NOT NULL,
  person_profile_id INT NOT NULL,
  active_company_id INT NULL,
  notif_counter INT NOT NULL DEFAULT 0,
  is_disabled bool NOT NULL DEFAULT 'f'
);

CREATE TABLE person_role (
  id SERIAL PRIMARY KEY NOT NULL ,
  label varchar(255) NOT NULL
);

CREATE TABLE company (
  id SERIAL NOT NULL ,
  label varchar(255) NOT NULL,
  created_by_id int NOT NULL ,
  created_at timestamp NOT NULL ,
  is_disabled bool NOT NULL DEFAULT 'f' ,
  PRIMARY KEY (id)
);

CREATE TABLE employee_role (
  id SERIAL NOT NULL ,
  label varchar(255) NOT NULL ,
  PRIMARY KEY (id)
);

CREATE TABLE employee (
  id  SERIAL NOT NULL ,
  person_id int NOT NULL ,
  company_id int NOT NULL ,
  is_disabled bool NOT NULL DEFAULT 'f' ,
  PRIMARY KEY (id)
);

CREATE TABLE employee_access (
  employee_id int NOT NULL,
  employee_role_id int NOT NULL,
  PRIMARY KEY (employee_id, employee_role_id)
);

CREATE TABLE person_profile (
  id  SERIAL NOT NULL ,
  firstname varchar(255) NOT NULL,
  lastname varchar(255) NOT NULL,
  email varchar(255) NOT NULL,
  phone_number varchar(50) NOT NULL,
  PRIMARY KEY (id)
);

ALTER TABLE person ADD FOREIGN KEY (person_role_id) REFERENCES person_role (id);
ALTER TABLE person ADD FOREIGN KEY (person_profile_id) REFERENCES person_profile (id);
ALTER TABLE person ADD FOREIGN KEY (active_company_id) REFERENCES company (id);
ALTER TABLE company ADD FOREIGN KEY (created_by_id) REFERENCES person (id);
ALTER TABLE employee ADD FOREIGN KEY (person_id) REFERENCES person (id);
ALTER TABLE employee ADD FOREIGN KEY (company_id) REFERENCES company (id);
ALTER TABLE employee_access ADD FOREIGN KEY (employee_id) REFERENCES employee (id);
ALTER TABLE employee_access ADD FOREIGN KEY (employee_role_id) REFERENCES employee_role (id);