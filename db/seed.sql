-- Roles
INSERT INTO person_role (label) VALUES ('Admin');
INSERT INTO person_role (label) VALUES ('User');

INSERT INTO employee_role (label) VALUES ('Admin');
INSERT INTO employee_role (label) VALUES ('Manager');
INSERT INTO employee_role (label) VALUES ('Employee');

-- User
INSERT INTO person_profile (firstname, lastname, email, phone_number, updated_at) VALUES('Mathieu', 'Delacroix', 'mathieu@cymer.com', '+33643085503', NOW());
INSERT INTO person (person_role_id, email, password, created_at, person_profile_id, updated_at) VALUES (1, 'mathieu@cymer.com', '$2y$04$Xo.GrhZJtpgU4KxDXcJkVud8WGDmHmS5FFy3izciN6pr2SsWrHYHG', NOW(), 1, NOW());
INSERT INTO company (label, created_by_id, created_at, is_disabled, updated_at) VALUES ('DLACREME', 1, NOW(), 'f', NOW());
INSERT INTO employee (person_id, company_id, is_disabled, created_at, updated_at) VALUES (1, 1, 'f', NOW(), NOW());
INSERT INTO employee_access (employee_id, employee_role_id) VALUES (1, 1);
INSERT INTO employee_access (employee_id, employee_role_id) VALUES (1, 2);
INSERT INTO employee_access (employee_id, employee_role_id) VALUES (1, 3);
INSERT INTO employee_access (employee_id, employee_role_id) VALUES (1, 4);
