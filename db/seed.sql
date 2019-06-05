-- Roles
INSERT INTO person_role (label) VALUES ('Admin');
INSERT INTO person_role (label) VALUES ('User');

INSERT INTO employee_role (label) VALUES ('Manager');
INSERT INTO employee_role (label) VALUES ('ReadOrder');
INSERT INTO employee_role (label) VALUES ('EditOrder');
INSERT INTO employee_role (label) VALUES ('ReadProduct');
INSERT INTO employee_role (label) VALUES ('EditProduct');

-- User
INSERT INTO person_profile (firstname, lastname, email, phone_number) VALUES('Mathieu', 'Delacroix', 'mathieu@cymer.com', '+33643085503');
INSERT INTO person (person_role_id, email, password, created_at, person_profile_id) VALUES (1, 'mathieu@cymer.com', '$2y$04$Xo.GrhZJtpgU4KxDXcJkVud8WGDmHmS5FFy3izciN6pr2SsWrHYHG', NOW(), 1);
INSERT INTO company (label, created_by_id, created_at, is_disabled) VALUES ('DLACREME', 1, NOW(), 'f');
INSERT INTO employee (person_id, company_id, is_disabled) VALUES (1, 1, 'f');
INSERT INTO employee_access (employee_id, employee_role_id) VALUES (1, 1);
INSERT INTO employee_access (employee_id, employee_role_id) VALUES (1, 2);
INSERT INTO employee_access (employee_id, employee_role_id) VALUES (1, 3);
INSERT INTO employee_access (employee_id, employee_role_id) VALUES (1, 4);