CREATE TABLE person (
  id SERIAL PRIMARY KEY,
  access_id INT NOT NULL DEFAULT 1,
  email VARCHAR(255) NOT NULL,
  password VARCHAR(255) NOT NULL,
  created_at TIMESTAMP NOT NULL,
  person_profile_id INT NOT NULL,
  active_company_id INT NULL,
  notif_counter INT NOT NULL DEFAULT 0
);

CREATE TABLE access (
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

CREATE TABLE employee_access (
  id SERIAL NOT NULL ,
  label varchar(255) NOT NULL ,
  PRIMARY KEY (id)
);

CREATE TABLE employee (
  id  SERIAL NOT NULL ,
  person_id int NOT NULL ,
  company_id int NOT NULL ,
  employee_access_id INT NOT NULL ,
  is_disabled bool NOT NULL DEFAULT 'f' ,
  PRIMARY KEY (id)
);

CREATE TABLE person_profile (
  id  SERIAL NOT NULL ,
  firstname varchar(255) ,
  lastname varchar(255) ,
  email varchar(255) ,
  phone_number varchar(50) ,
  PRIMARY KEY (id)
);

ALTER TABLE person ADD FOREIGN KEY (access_id) REFERENCES access (id);
ALTER TABLE person ADD FOREIGN KEY (person_profile_id) REFERENCES person_profile (id);
ALTER TABLE person ADD FOREIGN KEY (active_company_id) REFERENCES company (id);
ALTER TABLE company ADD FOREIGN KEY (created_by_id) REFERENCES person (id);
ALTER TABLE employee ADD FOREIGN KEY (person_id) REFERENCES person (id);
ALTER TABLE employee ADD FOREIGN KEY (company_id) REFERENCES company (id);
ALTER TABLE employee ADD FOREIGN KEY (employee_access_id) REFERENCES employee_access (id);
