CREATE TABLE people (
  id SERIAL PRIMARY KEY,
  role_id SMALLINT NOT NULL DEFAULT 1,
  email VARCHAR(255) NOT NULL,
  password VARCHAR(255) NULL,
  created_at TIMESTAMP NOT NULL,
  profile_id INT NOT NULL,
  active_company_id INT NULL,
  notif_counter SMALLINT NOT NULL DEFAULT 0
);

CREATE TABLE access (
  id  SERIAL PRIMARY KEY NOT NULL ,
  label varchar(255) NOT NULL
);

CREATE TABLE company (
  id  SERIAL NOT NULL ,
  label varchar(255) NOT NULL ,
  created_by_id int NOT NULL ,
  created_at timestamp NOT NULL ,
  is_disabled bool NOT NULL DEFAULT 'f' ,
  PRIMARY KEY (id)
);

CREATE TABLE employee_role (
  id  SERIAL NOT NULL ,
  label varchar(255) NOT NULL ,
  PRIMARY KEY (id)
);

CREATE TABLE employee (
  id  SERIAL NOT NULL ,
  people_id int NOT NULL ,
  company_id int NOT NULL ,
  employee_role_id smallint NOT NULL ,
  is_disabled bool NOT NULL DEFAULT 'f' ,
  PRIMARY KEY (id)
);

CREATE TABLE profile (
  id  SERIAL NOT NULL ,
  firstname varchar(255) ,
  lastname varchar(255) ,
  email varchar(255) ,
  phone_number varchar(50) ,
  PRIMARY KEY (id)
);

ALTER TABLE people ADD FOREIGN KEY (role_id) REFERENCES role (id);
ALTER TABLE people ADD FOREIGN KEY (profile_id) REFERENCES profile (id);
ALTER TABLE people ADD FOREIGN KEY (active_company_id) REFERENCES company (id);
ALTER TABLE company ADD FOREIGN KEY (created_by_id) REFERENCES people (id);
ALTER TABLE employee ADD FOREIGN KEY (people_id) REFERENCES people (id);
ALTER TABLE employee ADD FOREIGN KEY (company_id) REFERENCES company (id);
ALTER TABLE employee ADD FOREIGN KEY (employee_role_id) REFERENCES employee_role (id);
