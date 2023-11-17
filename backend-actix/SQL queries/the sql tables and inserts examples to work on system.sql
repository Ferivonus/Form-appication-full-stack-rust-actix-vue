
    DROP TABLE IF EXISTS chatting_form_messages;
    DROP TABLE IF EXISTS astrology_form_messages;
    DROP TABLE IF EXISTS game_form_messages;
    DROP TABLE IF EXISTS sport_form_messages;
    DROP TABLE IF EXISTS software_form_messages;
    DROP TABLE IF EXISTS anime_form_messages;
    DROP TABLE IF EXISTS form_message_answers;
    DROP TABLE IF EXISTS id_to_username;

    DROP TABLE IF EXISTS form_types;

	DROP TABLE IF EXISTS user_security_model_question; 
	DROP TABLE IF EXISTS user_security_model_telephone_number; 
	DROP TABLE IF EXISTS user_security_model_saving_mail; 
	DROP TABLE IF EXISTS user_model_socials; 
    DROP TABLE IF EXISTS users; 
	

    -- Big table on the top all of them
CREATE TABLE IF NOT EXISTS form_types (
    counter_of_form_messages INT AUTO_INCREMENT PRIMARY KEY,
    message_id_on_their_channel INT NOT NULL,
    user_id INT NOT NULL,
    is_answer bool DEFAULT FALSE,
    form_type VARCHAR(20) NOT NULL
);

    -- Chatting form
CREATE TABLE IF NOT EXISTS chatting_form_messages (
    id INT AUTO_INCREMENT PRIMARY KEY,
    random_string_to_get_id_after_create VARCHAR(255) NOT NULL,
    user_id INT NOT NULL,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    published BOOLEAN NOT NULL,
	answered_count INT NOT NULL DEFAULT 0,
    is_answer bool DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- chatting form answers location saver
CREATE TABLE IF NOT EXISTS chatting_form_messages_answer_list (
   chatting_form_message_id INT NOT NULL,
   answer_to INT NOT NULL
);


-- Astrology form
CREATE TABLE IF NOT EXISTS astrology_form_messages (
    id INT AUTO_INCREMENT PRIMARY KEY,
    random_string_to_get_id_after_create VARCHAR(255) NOT NULL,
    user_id INT NOT NULL,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    published BOOLEAN NOT NULL,
	answered_count INT NOT NULL DEFAULT 0,
    is_answer bool DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- astrology form answers location saver
CREATE TABLE IF NOT EXISTS astrology_form_messages_answer_list (
   astrology_form_message_id INT NOT NULL,
   answer_to INT NOT NULL
);


-- Game form
CREATE TABLE IF NOT EXISTS game_form_messages (
    id INT AUTO_INCREMENT PRIMARY KEY,
    random_string_to_get_id_after_create VARCHAR(255) NOT NULL,
    user_id INT NOT NULL,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    published BOOLEAN NOT NULL,
	answered_count INT NOT NULL DEFAULT 0,
    is_answer bool DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

--  game form answers location saver
CREATE TABLE IF NOT EXISTS  game_form_messages_answer_list (
	game_form_message_id INT NOT NULL,
	answer_to INT NOT NULL
);

-- Sport form
CREATE TABLE IF NOT EXISTS sport_form_messages (
    id INT AUTO_INCREMENT PRIMARY KEY,
    random_string_to_get_id_after_create VARCHAR(255) NOT NULL,
	user_id INT NOT NULL,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    published BOOLEAN NOT NULL,
	answered_count INT NOT NULL DEFAULT 0,
    is_answer bool DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);


-- sport form answers location saver
CREATE TABLE IF NOT EXISTS sport_form_messages_answer_list (
   sport_form_message_id INT NOT NULL,
   answer_to INT NOT NULL
);


-- Software form
CREATE TABLE IF NOT EXISTS software_form_messages (
    id INT AUTO_INCREMENT PRIMARY KEY,
    random_string_to_get_id_after_create VARCHAR(255) NOT NULL,
    user_id INT NOT NULL,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    published BOOLEAN DEFAULT TRUE,
	answered_count INT NOT NULL DEFAULT 0,
    is_answer bool DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Software form answers location saver
CREATE TABLE IF NOT EXISTS software_form_messages_answer_list (
   Software_form_message_id INT NOT NULL,
   answer_to INT NOT NULL
);


-- Anime form
CREATE TABLE IF NOT EXISTS anime_form_messages (
    id INT AUTO_INCREMENT PRIMARY KEY,
    random_string_to_get_id_after_create VARCHAR(255) NOT NULL,
    user_id INT NOT NULL,
    title VARCHAR(255) DEFAULT NULL,
    content TEXT NOT NULL,
    published BOOLEAN DEFAULT TRUE,
	answered_count INT NOT NULL DEFAULT 0,
    is_answer bool DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Anime form answers location saver
CREATE TABLE IF NOT EXISTS anime_form_messages_answer_list (
    anime_form_message_id INT NOT NULL,
    answer_to INT NOT NULL
);



ALTER TABLE form_types AUTO_INCREMENT = 1;
ALTER TABLE chatting_form_messages AUTO_INCREMENT = 1;
ALTER TABLE astrology_form_messages AUTO_INCREMENT = 1;
ALTER TABLE game_form_messages AUTO_INCREMENT = 1;
ALTER TABLE sport_form_messages AUTO_INCREMENT = 1;
ALTER TABLE software_form_messages AUTO_INCREMENT = 1;
ALTER TABLE anime_form_messages AUTO_INCREMENT = 1;


-- Inserting messages for the 'chatting' form

-- A user with user_id = 1 sends a form of type 'chatting' asking how others' days went
INSERT INTO form_types (message_id_on_their_channel, user_id, is_answer, form_type) VALUES (1, 1, FALSE, 'chatting');
INSERT INTO chatting_form_messages (random_string_to_get_id_after_create, user_id, title, content, published, answered_count, is_answer) VALUES ('random_string_1', 1, 'How was Your Day?', 'Share how your day went!', TRUE, 0, FALSE);

-- Another user responds to the 'How was Your Day?' form in the 'chatting' category
INSERT INTO form_types (message_id_on_their_channel, user_id, is_answer, form_type) VALUES (2, 2, TRUE, 'chatting');
INSERT INTO chatting_form_messages (random_string_to_get_id_after_create, user_id, title, content, published, answered_count, is_answer) VALUES ('random_string_2', 2, 'How was Your Day?', 'My day was great, thanks!', TRUE, 0, TRUE);
INSERT INTO chatting_form_messages_answer_list (chatting_form_message_id, answer_to) VALUES ('2', '1');
UPDATE chatting_form_messages SET answered_count = answered_count + 1 WHERE id = 1;

-- Another user responds to the previous answer in the 'How was Your Day?' form in the 'chatting' category
INSERT INTO form_types (message_id_on_their_channel, user_id, is_answer, form_type) VALUES (3, 3, TRUE, 'chatting');
INSERT INTO chatting_form_messages (random_string_to_get_id_after_create, user_id, title, content, published, answered_count, is_answer) VALUES ('random_string_3', 1, 'How was Your Day?', 'That sounds awesome!', TRUE, 0, TRUE);
INSERT INTO chatting_form_messages_answer_list (chatting_form_message_id, answer_to) VALUES ('3', '2');
UPDATE chatting_form_messages SET answered_count = answered_count + 1 WHERE id = 2;
-- Inserting messages for the 'game' form

-- Game Recommendation
-- A user with user_id = 1 sends a form of type 'game' asking for game recommendations
INSERT INTO form_types (message_id_on_their_channel, user_id, is_answer, form_type) VALUES (1, 4, FALSE, 'game');
INSERT INTO game_form_messages (random_string_to_get_id_after_create, user_id, title, content, published, answered_count, is_answer) VALUES ('random_string_4', 1, 'Game Recommendation', 'Any recommendations for good games to play?', TRUE, 0, FALSE);

-- Another user responds to the 'Game Recommendation' form in the 'game' category
INSERT INTO form_types (message_id_on_their_channel, user_id, is_answer, form_type) VALUES (2, 5, TRUE, 'game');
INSERT INTO game_form_messages (random_string_to_get_id_after_create, user_id, title, content, published, answered_count, is_answer) VALUES ('random_string_5', 1, 'Game Recommendation', 'You can play Counter-Strike: Global Offensive 2!', TRUE, 0, TRUE);
INSERT INTO game_form_messages_answer_list (game_form_message_id, answer_to) VALUES ('2', '1');
UPDATE game_form_messages SET answered_count = answered_count + 1 WHERE id = 1;

-- Another user responds to the 'Game Recommendation' form in the 'game' category with a different opinion
INSERT INTO form_types (message_id_on_their_channel, user_id, is_answer, form_type) VALUES (3, 4, TRUE, 'game');
INSERT INTO game_form_messages (random_string_to_get_id_after_create, user_id, title, content, published, answered_count, is_answer) VALUES ('random_string_6', 1, 'Game Recommendation', 'I think Counter-Strike is not that great, it sucks!', TRUE, 0, TRUE);
INSERT INTO game_form_messages_answer_list (game_form_message_id, answer_to) VALUES ('3', '2');
UPDATE game_form_messages SET answered_count = answered_count + 1 WHERE id = 2;


CREATE TABLE IF NOT EXISTS id_to_username (
    user_id INT NOT NULL,
    username VARCHAR(50) NOT NULL UNIQUE
);

    -- Users table:
CREATE TABLE IF NOT EXISTS users (
    id INT PRIMARY KEY AUTO_INCREMENT,
    username VARCHAR(50) NOT NULL UNIQUE,
    name VARCHAR(50) NOT NULL,
    surname VARCHAR(50) NOT NULL,
    email VARCHAR(100) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    sex VARCHAR(30) NOT NULL,
    favorite_anime_girl VARCHAR(255),
    last_login_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    registration_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_account_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- UserSecurityModelQuestion table:
CREATE TABLE IF NOT EXISTS user_security_model_question (
    user_id INT PRIMARY KEY,
    user_using_question BOOLEAN NOT NULL,
    security_question VARCHAR(255),
    security_answer VARCHAR(255),
    FOREIGN KEY (user_id) REFERENCES users(id),
	updated_question_security_model_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- UserSecurityModelNumber table:
CREATE TABLE IF NOT EXISTS user_security_model_telephone_number (
    user_id INT PRIMARY KEY,
    user_using_number BOOLEAN NOT NULL,
    tel_number BIGINT,
    FOREIGN KEY (user_id) REFERENCES users(id),
	updated_telephone_number_security_model_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- UserSecurityModelSavingMail table:
CREATE TABLE IF NOT EXISTS user_security_model_saving_mail (
    user_id INT PRIMARY KEY,
    user_using_saving_mail BOOLEAN NOT NULL,
    extra_mail VARCHAR(100),
    FOREIGN KEY (user_id) REFERENCES users(id),
	updated_saving_mail_security_model_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- UserModelSocials table:
CREATE TABLE IF NOT EXISTS user_model_socials (
    user_id INT PRIMARY KEY,
    facebook VARCHAR(255),
    twitter VARCHAR(255),
    instagram VARCHAR(255),
    linkedin VARCHAR(255),
    personal_website VARCHAR(255),
    FOREIGN KEY (user_id) REFERENCES users(id),
	updated_user_model_socials_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Inserting data into 'users' table
INSERT INTO users (username, name, surname, email, password_hash, sex, favorite_anime_girl)
VALUES ('alice_smith', 'Alice', 'Smith', 'alice.smith@example.com', 'hashed_password_456', 'Female', 'Saber'),
       ('bob_jones', 'Bob', 'Jones', 'bob.jones@example.com', 'hashed_password_789', 'Male', 'Rem'),
       ('eva_williams', 'Eva', 'Williams', 'eva.williams@example.com', 'hashed_password_012', 'Female', 'Luffy');

-- Inserting data into 'user_security_model_question' table
INSERT INTO user_security_model_question (user_id, user_using_question, security_question, security_answer)
VALUES (1, true, 'What is your favorite color?', 'Blue'),
       (2, true, 'In which city were you born?', 'New York'),
       (3, false, NULL, NULL);

-- Inserting data into 'user_security_model_telephone_number' table
INSERT INTO user_security_model_telephone_number (user_id, user_using_number, tel_number)
VALUES (1, true, 5557782939),
       (2, true, 5557782939),
       (3, false, NULL);

-- Inserting data into 'user_security_model_saving_mail' table
INSERT INTO user_security_model_saving_mail (user_id, user_using_saving_mail, extra_mail)
VALUES (1, true, 'backup_email@example.com'),
       (2, false, NULL),
       (3, true, 'secondary_email@example.com');

-- Inserting data into 'user_model_socials' table
INSERT INTO user_model_socials (user_id, facebook, twitter, instagram, linkedin, personal_website)
VALUES (1, 'facebook.com/alice', 'twitter.com/alice', 'instagram.com/alice', 'linkedin.com/in/alice', 'alice.com'),
       (2, 'facebook.com/bob', 'twitter.com/bob', 'instagram.com/bob', 'linkedin.com/in/bob', 'bob.com'),
       (3, NULL, NULL, NULL, NULL, NULL);

SELECT * FROM users;