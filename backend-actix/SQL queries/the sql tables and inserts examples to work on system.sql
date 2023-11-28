
	-- Chatting form drop tables:
DROP TABLE IF EXISTS chatting_form_messages_publishing_control;
DROP TABLE IF EXISTS chatting_form_messages_message_info;
DROP TABLE IF EXISTS chatting_form_messages_has_image_information;
DROP TABLE IF EXISTS chatting_form_messages_image_counter;
DROP TABLE IF EXISTS chatting_form_messages_image_information;
DROP TABLE IF EXISTS chatting_form_messages_image_time_infos;
DROP TABLE IF EXISTS chatting_form_messages_image_how_many_times_answered;
DROP TABLE IF EXISTS chatting_form_messages_image_like_dislake_founded_funny;
DROP TABLE IF EXISTS chatting_form_messages_like_dislake_information;
DROP TABLE IF EXISTS chatting_form_messages_message_time_info;
DROP TABLE IF EXISTS chatting_form_messages_answered_to_node;
DROP TABLE IF EXISTS chatting_form_answered_messages_info;

	-- astrology form drop tables:

DROP TABLE IF EXISTS astrology_form_messages_answered_counter;
DROP TABLE IF EXISTS astrology_form_messages_publishing_control;
DROP TABLE IF EXISTS astrology_form_messages_message_info;
DROP TABLE IF EXISTS astrology_form_messages_has_image_information;
DROP TABLE IF EXISTS astrology_form_messages_image_counter;
DROP TABLE IF EXISTS astrology_form_messages_image_information;
DROP TABLE IF EXISTS astrology_form_messages_image_time_infos;
DROP TABLE IF EXISTS astrology_form_messages_image_how_many_times_answered;
DROP TABLE IF EXISTS astrology_form_messages_image_like_dislake_founded_funny;
DROP TABLE IF EXISTS astrology_form_messages_like_dislake_information;
DROP TABLE IF EXISTS astrology_form_messages_message_time_info;
DROP TABLE IF EXISTS astrology_form_messages_answered_to_node;
DROP TABLE IF EXISTS astrology_form_answered_messages_info;
DROP TABLE IF EXISTS astrology_form_messages_random_string;
    
	-- game form drop tables:

DROP TABLE IF EXISTS game_form_messages_answered_counter;
DROP TABLE IF EXISTS game_form_messages_publishing_control;
DROP TABLE IF EXISTS game_form_messages_message_info;
DROP TABLE IF EXISTS game_form_messages_has_image_information;
DROP TABLE IF EXISTS game_form_messages_image_counter;
DROP TABLE IF EXISTS game_form_messages_image_information;
DROP TABLE IF EXISTS game_form_messages_image_time_infos;
DROP TABLE IF EXISTS game_form_messages_image_how_many_times_answered;
DROP TABLE IF EXISTS game_form_messages_image_like_dislake_founded_funny;
DROP TABLE IF EXISTS game_form_messages_like_dislake_information;
DROP TABLE IF EXISTS game_form_messages_message_time_info;
DROP TABLE IF EXISTS game_form_messages_answered_to_node;
DROP TABLE IF EXISTS game_form_answered_messages_info;
DROP TABLE IF EXISTS game_form_messages_random_string;
    
DROP TABLE IF EXISTS sport_form_answered_messages_info;
	-- sport form drop tables:

DROP TABLE IF EXISTS sport_form_messages_answered_counter;
DROP TABLE IF EXISTS sport_form_messages_publishing_control;
DROP TABLE IF EXISTS sport_form_messages_message_info;
DROP TABLE IF EXISTS sport_form_messages_has_image_information;
DROP TABLE IF EXISTS sport_form_messages_image_counter;
DROP TABLE IF EXISTS sport_form_messages_image_information;
DROP TABLE IF EXISTS sport_form_messages_image_time_infos;
DROP TABLE IF EXISTS sport_form_messages_image_how_many_times_answered;
DROP TABLE IF EXISTS sport_form_messages_image_like_dislake_founded_funny;
DROP TABLE IF EXISTS sport_form_messages_like_dislake_information;
DROP TABLE IF EXISTS sport_form_messages_message_time_info;
DROP TABLE IF EXISTS sport_form_messages_answered_to_node;
DROP TABLE IF EXISTS sport_form_answered_messages_info;
DROP TABLE IF EXISTS sport_form_messages_random_string;

	-- software form drop tables:

DROP TABLE IF EXISTS software_form_messages_answered_counter;
DROP TABLE IF EXISTS software_form_messages_publishing_control;
DROP TABLE IF EXISTS software_form_messages_message_info;
DROP TABLE IF EXISTS software_form_messages_has_image_information;
DROP TABLE IF EXISTS software_form_messages_image_counter;
DROP TABLE IF EXISTS software_form_messages_image_information;
DROP TABLE IF EXISTS software_form_messages_image_time_infos;
DROP TABLE IF EXISTS software_form_messages_image_how_many_times_answered;
DROP TABLE IF EXISTS software_form_messages_image_like_dislake_founded_funny;
DROP TABLE IF EXISTS software_form_messages_like_dislake_information;
DROP TABLE IF EXISTS software_form_messages_message_time_info;
DROP TABLE IF EXISTS software_form_messages_answered_to_node;
DROP TABLE IF EXISTS software_form_answered_messages_info;
DROP TABLE IF EXISTS software_form_messages_random_string;

	-- anime form drop tables:

DROP TABLE IF EXISTS anime_form_messages_answered_counter;
DROP TABLE IF EXISTS anime_form_messages_publishing_control;
DROP TABLE IF EXISTS anime_form_messages_message_info;
DROP TABLE IF EXISTS anime_form_messages_has_image_information;
DROP TABLE IF EXISTS anime_form_messages_image_counter;
DROP TABLE IF EXISTS anime_form_messages_image_information;
DROP TABLE IF EXISTS anime_form_messages_image_time_infos;
DROP TABLE IF EXISTS anime_form_messages_image_how_many_times_answered;
DROP TABLE IF EXISTS anime_form_messages_image_like_dislake_founded_funny;
DROP TABLE IF EXISTS anime_form_messages_like_dislake_information;
DROP TABLE IF EXISTS anime_form_messages_message_time_info;
DROP TABLE IF EXISTS anime_form_messages_answered_to_node;
DROP TABLE IF EXISTS anime_form_answered_messages_info;
DROP TABLE IF EXISTS anime_form_messages_random_string;

	-- main table drop table
DROP TABLE IF EXISTS track_messages_main_table;


DROP TABLE IF EXISTS id_to_username;
DROP TABLE IF EXISTS user_security_model_question; 
DROP TABLE IF EXISTS user_security_model_telephone_number; 
DROP TABLE IF EXISTS user_security_model_saving_mail; 
DROP TABLE IF EXISTS user_model_socials; 
DROP TABLE IF EXISTS users_info;
DROP TABLE IF EXISTS users;


    -- Big table on the top all of them
CREATE TABLE IF NOT EXISTS track_messages_main_table (
    counter_of_form_messages INT AUTO_INCREMENT PRIMARY KEY,
    main_table_all_random_string_identifiers VARCHAR(255) NOT NULL,
    user_id INT NOT NULL,
    is_answer bool DEFAULT FALSE,
    form_type VARCHAR(20) NOT NULL
);

	-- chatting form tables:

-- chatting form get random string table top table which has id and random string from code.
CREATE TABLE IF NOT EXISTS chatting_form_messages_random_string (
    id INT AUTO_INCREMENT PRIMARY KEY,
    random_string_to_get_id_after_create VARCHAR(255) NOT NULL UNIQUE
);

-- chatting form answered counter
CREATE TABLE IF NOT EXISTS chatting_form_messages_answered_counter (
    random_string_identifier VARCHAR(255) NOT NULL,
	answered_count INT NOT NULL DEFAULT 0, -- how many times answered info
    last_answered_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
	FOREIGN KEY (random_string_identifier) REFERENCES chatting_form_messages_random_string(random_string_to_get_id_after_create)
);

-- chatting form publishing control table
CREATE TABLE IF NOT EXISTS chatting_form_messages_publishing_control (
    random_string_identifier VARCHAR(255) NOT NULL,
    published BOOLEAN DEFAULT TRUE,
    publishing_detailes_changed_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
	FOREIGN KEY (random_string_identifier) REFERENCES chatting_form_messages_random_string(random_string_to_get_id_after_create)
);


-- chatting form message info
CREATE TABLE IF NOT EXISTS chatting_form_messages_message_info (
    random_string_identifier VARCHAR(255) NOT NULL,
    sender_user_id INT NOT NULL,
    title VARCHAR(255) DEFAULT NULL,
    content TEXT NOT NULL,
	FOREIGN KEY (random_string_identifier) REFERENCES chatting_form_messages_random_string(random_string_to_get_id_after_create)
);

-- chatting form has image info
CREATE TABLE IF NOT EXISTS chatting_form_messages_has_image_information (
    random_string_identifier VARCHAR(255) NOT NULL,
    has_image bool DEFAULT FALSE,
	FOREIGN KEY (random_string_identifier) REFERENCES chatting_form_messages_random_string(random_string_to_get_id_after_create)
);

-- chatting image counter
CREATE TABLE IF NOT EXISTS chatting_form_messages_image_counter (
	counter_of_image INT AUTO_INCREMENT PRIMARY KEY,
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	FOREIGN KEY (random_string_identifier) REFERENCES chatting_form_messages_random_string(random_string_to_get_id_after_create)
);


-- INSERT INTO images (image_data, image_name) VALUES (LOAD_FILE('/path/to/your/image.jpg'), 'image.jpg');
-- chatting image informations
CREATE TABLE IF NOT EXISTS chatting_form_messages_image_information (
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
    image_data MEDIUMBLOB NOT NULL,
    image_name VARCHAR(255) NOT NULL,
    image_sender_username VARCHAR(255) NOT NULL,
	FOREIGN KEY (random_string_identifier) REFERENCES chatting_form_messages_random_string(random_string_to_get_id_after_create)
);

-- chatting image time infos
CREATE TABLE IF NOT EXISTS chatting_form_messages_image_time_infos (
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    changed_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
	FOREIGN KEY (random_string_identifier) REFERENCES chatting_form_messages_random_string(random_string_to_get_id_after_create)
);

-- chatting image how many times answered table
CREATE TABLE IF NOT EXISTS chatting_form_messages_image_how_many_times_answered (
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	answered_count INT NOT NULL DEFAULT 0, -- how many times answered info
    last_answer_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,    
	FOREIGN KEY (random_string_identifier) REFERENCES chatting_form_messages_random_string(random_string_to_get_id_after_create)
);

-- chatting image like and dislake counter
CREATE TABLE IF NOT EXISTS chatting_form_messages_image_like_dislake_founded_funny (
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	image_liked_count INT NOT NULL DEFAULT 0, -- how many times answered info
	image_dislaked_count INT NOT NULL DEFAULT 0, -- how many times answered info
	image_founded_funny_count INT NOT NULL DEFAULT 0, -- how many times answered info    
	FOREIGN KEY (random_string_identifier) REFERENCES chatting_form_messages_random_string(random_string_to_get_id_after_create)
);

-- chatting message like and dislake counter
CREATE TABLE IF NOT EXISTS chatting_form_messages_like_dislake_information (
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	liked_count INT NOT NULL DEFAULT 0, -- how many times liked info
	dislaked_count INT NOT NULL DEFAULT 0, -- how many times unliked info
    founded_funny INT NOT NULL DEFAULT 0, -- how many times founded funny info
	FOREIGN KEY (random_string_identifier) REFERENCES chatting_form_messages_random_string(random_string_to_get_id_after_create)
);

-- chatting form time info
CREATE TABLE IF NOT EXISTS chatting_form_messages_message_time_info (
    random_string_identifier VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
	FOREIGN KEY (random_string_identifier) REFERENCES chatting_form_messages_random_string(random_string_to_get_id_after_create)
);

-- chatting form answers location 
CREATE TABLE IF NOT EXISTS chatting_form_messages_answered_to_node (
    random_string_identifier VARCHAR(255) NOT NULL, -- first node to show answering message
    answered_messages_string_value VARCHAR(255) NOT NULL,  -- second node to show answered message
	FOREIGN KEY (random_string_identifier) REFERENCES chatting_form_messages_random_string(random_string_to_get_id_after_create)
);

-- chatting form getting content from random string
CREATE TABLE IF NOT EXISTS chatting_form_answered_messages_info (
    random_string_identifier VARCHAR(255) NOT NULL, -- random string created by rust random function
    title_of_answered_message VARCHAR(255) NOT NULL,
	content_of_answered_message VARCHAR(255) NOT NULL,  -- first node to show answering message
	FOREIGN KEY (random_string_identifier) REFERENCES chatting_form_messages_random_string(random_string_to_get_id_after_create)
);


	-- astrology form tables:

-- astrology form get random string table top table which has id and random string from code.
CREATE TABLE IF NOT EXISTS astrology_form_messages_random_string (
    id INT AUTO_INCREMENT PRIMARY KEY,
    random_string_to_get_id_after_create VARCHAR(255) NOT NULL UNIQUE
);

-- astrology form answered counter
CREATE TABLE IF NOT EXISTS astrology_form_messages_answered_counter (
    random_string_identifier VARCHAR(255) NOT NULL,
	answered_count INT NOT NULL DEFAULT 0, -- how many times answered info
    last_answered_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
	FOREIGN KEY (random_string_identifier) REFERENCES astrology_form_messages_random_string(random_string_to_get_id_after_create)
);

-- astrology form publishing control table
CREATE TABLE IF NOT EXISTS astrology_form_messages_publishing_control (
    random_string_identifier VARCHAR(255) NOT NULL,
    published BOOLEAN DEFAULT TRUE,
    publishing_detailes_changed_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
	FOREIGN KEY (random_string_identifier) REFERENCES astrology_form_messages_random_string(random_string_to_get_id_after_create)
);


-- astrology form message info
CREATE TABLE IF NOT EXISTS astrology_form_messages_message_info (
    random_string_identifier VARCHAR(255) NOT NULL,
    sender_user_id INT NOT NULL,
    title VARCHAR(255) DEFAULT NULL,
    content TEXT NOT NULL,
	FOREIGN KEY (random_string_identifier) REFERENCES astrology_form_messages_random_string(random_string_to_get_id_after_create)
);

-- astrology form has image info
CREATE TABLE IF NOT EXISTS astrology_form_messages_has_image_information (
    random_string_identifier VARCHAR(255) NOT NULL,
    has_image bool DEFAULT FALSE,
	FOREIGN KEY (random_string_identifier) REFERENCES astrology_form_messages_random_string(random_string_to_get_id_after_create)
);

-- astrology image counter
CREATE TABLE IF NOT EXISTS astrology_form_messages_image_counter (
	counter_of_image INT AUTO_INCREMENT PRIMARY KEY,
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	FOREIGN KEY (random_string_identifier) REFERENCES astrology_form_messages_random_string(random_string_to_get_id_after_create)
);


-- INSERT INTO images (image_data, image_name) VALUES (LOAD_FILE('/path/to/your/image.jpg'), 'image.jpg');
-- astrology image informations
CREATE TABLE IF NOT EXISTS astrology_form_messages_image_information (
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
    image_data MEDIUMBLOB NOT NULL,
    image_name VARCHAR(255) NOT NULL,
    image_sender_username VARCHAR(255) NOT NULL,
	FOREIGN KEY (random_string_identifier) REFERENCES astrology_form_messages_random_string(random_string_to_get_id_after_create)
);

-- astrology image time infos
CREATE TABLE IF NOT EXISTS astrology_form_messages_image_time_infos (
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    changed_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
	FOREIGN KEY (random_string_identifier) REFERENCES astrology_form_messages_random_string(random_string_to_get_id_after_create)
);

-- astrology image how many times answered table
CREATE TABLE IF NOT EXISTS astrology_form_messages_image_how_many_times_answered (
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	answered_count INT NOT NULL DEFAULT 0, -- how many times answered info
    last_answer_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,    
	FOREIGN KEY (random_string_identifier) REFERENCES astrology_form_messages_random_string(random_string_to_get_id_after_create)
);

-- astrology image like and dislake counter
CREATE TABLE IF NOT EXISTS astrology_form_messages_image_like_dislake_founded_funny (
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	image_liked_count INT NOT NULL DEFAULT 0, -- how many times answered info
	image_dislaked_count INT NOT NULL DEFAULT 0, -- how many times answered info
	image_founded_funny_count INT NOT NULL DEFAULT 0, -- how many times answered info    
	FOREIGN KEY (random_string_identifier) REFERENCES astrology_form_messages_random_string(random_string_to_get_id_after_create)
);

-- astrology message like and dislake counter
CREATE TABLE IF NOT EXISTS astrology_form_messages_like_dislake_information (
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	liked_count INT NOT NULL DEFAULT 0, -- how many times answered info
	dislaked_count INT NOT NULL DEFAULT 0, -- how many times answered info
	founded_funny INT NOT NULL DEFAULT 0, -- how many times founded funny info
	FOREIGN KEY (random_string_identifier) REFERENCES astrology_form_messages_random_string(random_string_to_get_id_after_create)
);

-- astrology form time info
CREATE TABLE IF NOT EXISTS astrology_form_messages_message_time_info (
    random_string_identifier VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
	FOREIGN KEY (random_string_identifier) REFERENCES astrology_form_messages_random_string(random_string_to_get_id_after_create)
);

-- astrology form answers location 
CREATE TABLE IF NOT EXISTS astrology_form_messages_answered_to_node (
    random_string_identifier VARCHAR(255) NOT NULL, -- first node to show answering message
    answered_messages_string_value VARCHAR(255) NOT NULL,  -- second node to show answered message
	FOREIGN KEY (random_string_identifier) REFERENCES astrology_form_messages_random_string(random_string_to_get_id_after_create)
);

-- astrology form getting content from random string
CREATE TABLE IF NOT EXISTS astrology_form_answered_messages_info (
    random_string_identifier VARCHAR(255) NOT NULL, -- random string created by rust random function
    title_of_answered_message VARCHAR(255) NOT NULL,
	content_of_answered_message VARCHAR(255) NOT NULL,  -- first node to show answering message
	FOREIGN KEY (random_string_identifier) REFERENCES astrology_form_messages_random_string(random_string_to_get_id_after_create)
);


-- game form tables:

-- game form get random string table top table which has id and random string from code.
CREATE TABLE IF NOT EXISTS game_form_messages_random_string (
    id INT AUTO_INCREMENT PRIMARY KEY,
    random_string_to_get_id_after_create VARCHAR(255) NOT NULL UNIQUE
);

-- game form answered counter
CREATE TABLE IF NOT EXISTS game_form_messages_answered_counter (
    random_string_identifier VARCHAR(255) NOT NULL,
	answered_count INT NOT NULL DEFAULT 0, -- how many times answered info
    last_answered_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
	FOREIGN KEY (random_string_identifier) REFERENCES game_form_messages_random_string(random_string_to_get_id_after_create)
);
   
-- game form publishing control table
CREATE TABLE IF NOT EXISTS game_form_messages_publishing_control (
    random_string_identifier VARCHAR(255) NOT NULL,
    published BOOLEAN DEFAULT TRUE,
    publishing_detailes_changed_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
	FOREIGN KEY (random_string_identifier) REFERENCES game_form_messages_random_string(random_string_to_get_id_after_create)
);


-- game form message info
CREATE TABLE IF NOT EXISTS game_form_messages_message_info (
    random_string_identifier VARCHAR(255) NOT NULL,
    sender_user_id INT NOT NULL,
    title VARCHAR(255) DEFAULT NULL,
    content TEXT NOT NULL,
	FOREIGN KEY (random_string_identifier) REFERENCES game_form_messages_random_string(random_string_to_get_id_after_create)
);

-- game form has image info
CREATE TABLE IF NOT EXISTS game_form_messages_has_image_information (
    random_string_identifier VARCHAR(255) NOT NULL,
    has_image bool DEFAULT FALSE,
	FOREIGN KEY (random_string_identifier) REFERENCES game_form_messages_random_string(random_string_to_get_id_after_create)
);

-- game image counter
CREATE TABLE IF NOT EXISTS game_form_messages_image_counter (
	counter_of_image INT AUTO_INCREMENT PRIMARY KEY,
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	FOREIGN KEY (random_string_identifier) REFERENCES game_form_messages_random_string(random_string_to_get_id_after_create)
);


-- INSERT INTO images (image_data, image_name) VALUES (LOAD_FILE('/path/to/your/image.jpg'), 'image.jpg');
-- game image informations
CREATE TABLE IF NOT EXISTS game_form_messages_image_information (
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
    image_data MEDIUMBLOB NOT NULL,
    image_name VARCHAR(255) NOT NULL,
    image_sender_username VARCHAR(255) NOT NULL,
	FOREIGN KEY (random_string_identifier) REFERENCES game_form_messages_random_string(random_string_to_get_id_after_create)
);

-- game image time infos
CREATE TABLE IF NOT EXISTS game_form_messages_image_time_infos (
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    changed_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
	FOREIGN KEY (random_string_identifier) REFERENCES game_form_messages_random_string(random_string_to_get_id_after_create)
);

-- game image how many times answered table
CREATE TABLE IF NOT EXISTS game_form_messages_image_how_many_times_answered (
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	answered_count INT NOT NULL DEFAULT 0, -- how many times answered info
    last_answer_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,    
	FOREIGN KEY (random_string_identifier) REFERENCES game_form_messages_random_string(random_string_to_get_id_after_create)
);

-- game image like and dislake counter
CREATE TABLE IF NOT EXISTS game_form_messages_image_like_dislake_founded_funny (
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	image_liked_count INT NOT NULL DEFAULT 0, -- how many times answered info
	image_dislaked_count INT NOT NULL DEFAULT 0, -- how many times answered info
	image_founded_funny_count INT NOT NULL DEFAULT 0, -- how many times answered info    
	FOREIGN KEY (random_string_identifier) REFERENCES game_form_messages_random_string(random_string_to_get_id_after_create)
);

-- game message like and dislake counter
CREATE TABLE IF NOT EXISTS game_form_messages_like_dislake_information (
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	liked_count INT NOT NULL DEFAULT 0, -- how many times answered info
	dislaked_count INT NOT NULL DEFAULT 0, -- how many times answered info
    founded_funny INT NOT NULL DEFAULT 0, -- how many times founded funny info
	FOREIGN KEY (random_string_identifier) REFERENCES game_form_messages_random_string(random_string_to_get_id_after_create)
);

-- game form time info
CREATE TABLE IF NOT EXISTS game_form_messages_message_time_info (
    random_string_identifier VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
	FOREIGN KEY (random_string_identifier) REFERENCES game_form_messages_random_string(random_string_to_get_id_after_create)
);

-- game form answers location 
CREATE TABLE IF NOT EXISTS game_form_messages_answered_to_node (
    random_string_identifier VARCHAR(255) NOT NULL, -- first node to show answering message
    answered_messages_string_value VARCHAR(255) NOT NULL,  -- second node to show answered message
	FOREIGN KEY (random_string_identifier) REFERENCES game_form_messages_random_string(random_string_to_get_id_after_create)
);

-- game form getting content from random string
CREATE TABLE IF NOT EXISTS game_form_answered_messages_info (
    random_string_identifier VARCHAR(255) NOT NULL, -- random string created by rust random function
    title_of_answered_message VARCHAR(255) NOT NULL,
	content_of_answered_message VARCHAR(255) NOT NULL,  -- first node to show answering message
	FOREIGN KEY (random_string_identifier) REFERENCES game_form_messages_random_string(random_string_to_get_id_after_create)
);


-- sport form tables:

-- sport form get random string table top table which has id and random string from code.
CREATE TABLE IF NOT EXISTS sport_form_messages_random_string (
    id INT AUTO_INCREMENT PRIMARY KEY,
    random_string_to_get_id_after_create VARCHAR(255) NOT NULL UNIQUE
);

-- sport form answered counter
CREATE TABLE IF NOT EXISTS sport_form_messages_answered_counter (
    random_string_identifier VARCHAR(255) NOT NULL,
	answered_count INT NOT NULL DEFAULT 0, -- how many times answered info
    last_answered_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
	FOREIGN KEY (random_string_identifier) REFERENCES sport_form_messages_random_string(random_string_to_get_id_after_create)
);

-- sport form publishing control table
CREATE TABLE IF NOT EXISTS sport_form_messages_publishing_control (
    random_string_identifier VARCHAR(255) NOT NULL,
    published BOOLEAN DEFAULT TRUE,
    publishing_detailes_changed_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
	FOREIGN KEY (random_string_identifier) REFERENCES sport_form_messages_random_string(random_string_to_get_id_after_create)
);


-- sport form message info
CREATE TABLE IF NOT EXISTS sport_form_messages_message_info (
    random_string_identifier VARCHAR(255) NOT NULL,
    sender_user_id INT NOT NULL,
    title VARCHAR(255) DEFAULT NULL,
    content TEXT NOT NULL,
	FOREIGN KEY (random_string_identifier) REFERENCES sport_form_messages_random_string(random_string_to_get_id_after_create)
);

-- sport form has image info
CREATE TABLE IF NOT EXISTS sport_form_messages_has_image_information (
    random_string_identifier VARCHAR(255) NOT NULL,
    has_image bool DEFAULT FALSE,
	FOREIGN KEY (random_string_identifier) REFERENCES sport_form_messages_random_string(random_string_to_get_id_after_create)
);

-- sport image counter
CREATE TABLE IF NOT EXISTS sport_form_messages_image_counter (
	counter_of_image INT AUTO_INCREMENT PRIMARY KEY,
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	FOREIGN KEY (random_string_identifier) REFERENCES sport_form_messages_random_string(random_string_to_get_id_after_create)
);


-- INSERT INTO images (image_data, image_name) VALUES (LOAD_FILE('/path/to/your/image.jpg'), 'image.jpg');
-- sport image informations
CREATE TABLE IF NOT EXISTS sport_form_messages_image_information (
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
    image_data MEDIUMBLOB NOT NULL,
    image_name VARCHAR(255) NOT NULL,
    image_sender_username VARCHAR(255) NOT NULL,
	FOREIGN KEY (random_string_identifier) REFERENCES sport_form_messages_random_string(random_string_to_get_id_after_create)
);

-- sport image time infos
CREATE TABLE IF NOT EXISTS sport_form_messages_image_time_infos (
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    changed_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
	FOREIGN KEY (random_string_identifier) REFERENCES sport_form_messages_random_string(random_string_to_get_id_after_create)
);

-- sport image how many times answered table
CREATE TABLE IF NOT EXISTS sport_form_messages_image_how_many_times_answered (
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	answered_count INT NOT NULL DEFAULT 0, -- how many times answered info
    last_answer_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,    
	FOREIGN KEY (random_string_identifier) REFERENCES sport_form_messages_random_string(random_string_to_get_id_after_create)
);

-- sport image like and dislake counter
CREATE TABLE IF NOT EXISTS sport_form_messages_image_like_dislake_founded_funny (
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	image_liked_count INT NOT NULL DEFAULT 0, -- how many times answered info
	image_dislaked_count INT NOT NULL DEFAULT 0, -- how many times answered info
	image_founded_funny_count INT NOT NULL DEFAULT 0, -- how many times answered info    
	FOREIGN KEY (random_string_identifier) REFERENCES sport_form_messages_random_string(random_string_to_get_id_after_create)
);

-- sport message like and dislake counter
CREATE TABLE IF NOT EXISTS sport_form_messages_like_dislake_information (
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	liked_count INT NOT NULL DEFAULT 0, -- how many times answered info
	dislaked_count INT NOT NULL DEFAULT 0, -- how many times answered info
    founded_funny INT NOT NULL DEFAULT 0, -- how many times founded funny info
	FOREIGN KEY (random_string_identifier) REFERENCES sport_form_messages_random_string(random_string_to_get_id_after_create)
);

-- sport form time info
CREATE TABLE IF NOT EXISTS sport_form_messages_message_time_info (
    random_string_identifier VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
	FOREIGN KEY (random_string_identifier) REFERENCES sport_form_messages_random_string(random_string_to_get_id_after_create)
);

-- sport form answers location 
CREATE TABLE IF NOT EXISTS sport_form_messages_answered_to_node (
    random_string_identifier VARCHAR(255) NOT NULL, -- first node to show answering message
    answered_messages_string_value VARCHAR(255) NOT NULL,  -- second node to show answered message
	FOREIGN KEY (random_string_identifier) REFERENCES sport_form_messages_random_string(random_string_to_get_id_after_create)
);

-- sport form getting content from random string
CREATE TABLE IF NOT EXISTS sport_form_answered_messages_info (
    random_string_identifier VARCHAR(255) NOT NULL, -- random string created by rust random function
    title_of_answered_message VARCHAR(255) NOT NULL,
	content_of_answered_message VARCHAR(255) NOT NULL,  -- first node to show answering message
	FOREIGN KEY (random_string_identifier) REFERENCES sport_form_messages_random_string(random_string_to_get_id_after_create)
);

	-- software form tables:

-- software form get random string table top table which has id and random string from code.
CREATE TABLE IF NOT EXISTS software_form_messages_random_string (
    id INT AUTO_INCREMENT PRIMARY KEY,
    random_string_to_get_id_after_create VARCHAR(255) NOT NULL UNIQUE
);

-- software form answered counter
CREATE TABLE IF NOT EXISTS software_form_messages_answered_counter (
    random_string_identifier VARCHAR(255) NOT NULL,
	answered_count INT NOT NULL DEFAULT 0, -- how many times answered info
    last_answered_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
	FOREIGN KEY (random_string_identifier) REFERENCES software_form_messages_random_string(random_string_to_get_id_after_create)
);

-- software form publishing control table
CREATE TABLE IF NOT EXISTS software_form_messages_publishing_control (
    random_string_identifier VARCHAR(255) NOT NULL,
    published BOOLEAN DEFAULT TRUE,
    publishing_detailes_changed_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
	FOREIGN KEY (random_string_identifier) REFERENCES software_form_messages_random_string(random_string_to_get_id_after_create)
);


-- software form message info
CREATE TABLE IF NOT EXISTS software_form_messages_message_info (
    random_string_identifier VARCHAR(255) NOT NULL,
    sender_user_id INT NOT NULL,
    title VARCHAR(255) DEFAULT NULL,
    content TEXT NOT NULL,
	FOREIGN KEY (random_string_identifier) REFERENCES software_form_messages_random_string(random_string_to_get_id_after_create)
);

-- software form has image info
CREATE TABLE IF NOT EXISTS software_form_messages_has_image_information (
    random_string_identifier VARCHAR(255) NOT NULL,
    has_image bool DEFAULT FALSE,
	FOREIGN KEY (random_string_identifier) REFERENCES software_form_messages_random_string(random_string_to_get_id_after_create)
);

-- software image counter
CREATE TABLE IF NOT EXISTS software_form_messages_image_counter (
	counter_of_image INT AUTO_INCREMENT PRIMARY KEY,
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	FOREIGN KEY (random_string_identifier) REFERENCES software_form_messages_random_string(random_string_to_get_id_after_create)
);


-- INSERT INTO images (image_data, image_name) VALUES (LOAD_FILE('/path/to/your/image.jpg'), 'image.jpg');
-- software image informations
CREATE TABLE IF NOT EXISTS software_form_messages_image_information (
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
    image_data MEDIUMBLOB NOT NULL,
    image_name VARCHAR(255) NOT NULL,
    image_sender_username VARCHAR(255) NOT NULL,
	FOREIGN KEY (random_string_identifier) REFERENCES software_form_messages_random_string(random_string_to_get_id_after_create)
);

-- software image time infos
CREATE TABLE IF NOT EXISTS software_form_messages_image_time_infos (
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    changed_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
	FOREIGN KEY (random_string_identifier) REFERENCES software_form_messages_random_string(random_string_to_get_id_after_create)
);

-- software image how many times answered table
CREATE TABLE IF NOT EXISTS software_form_messages_image_how_many_times_answered (
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	answered_count INT NOT NULL DEFAULT 0, -- how many times answered info
    last_answer_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,    
	FOREIGN KEY (random_string_identifier) REFERENCES software_form_messages_random_string(random_string_to_get_id_after_create)
);

-- software image like and dislake counter
CREATE TABLE IF NOT EXISTS software_form_messages_image_like_dislake_founded_funny (
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	image_liked_count INT NOT NULL DEFAULT 0, -- how many times answered info
	image_dislaked_count INT NOT NULL DEFAULT 0, -- how many times answered info
	image_founded_funny_count INT NOT NULL DEFAULT 0, -- how many times answered info    
	FOREIGN KEY (random_string_identifier) REFERENCES software_form_messages_random_string(random_string_to_get_id_after_create)
);

-- software message like and dislake counter
CREATE TABLE IF NOT EXISTS software_form_messages_like_dislake_information (
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	liked_count INT NOT NULL DEFAULT 0, -- how many times answered info
	dislaked_count INT NOT NULL DEFAULT 0, -- how many times answered info
    founded_funny INT NOT NULL DEFAULT 0, -- how many times founded funny info
	FOREIGN KEY (random_string_identifier) REFERENCES software_form_messages_random_string(random_string_to_get_id_after_create)
);

-- software form time info
CREATE TABLE IF NOT EXISTS software_form_messages_message_time_info (
    random_string_identifier VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
	FOREIGN KEY (random_string_identifier) REFERENCES software_form_messages_random_string(random_string_to_get_id_after_create)
);

-- software form answers location 
CREATE TABLE IF NOT EXISTS software_form_messages_answered_to_node (
    random_string_identifier VARCHAR(255) NOT NULL,        -- first node to show answering message
    answered_messages_string_value VARCHAR(255) NOT NULL,  -- second node to show answered message
	FOREIGN KEY (random_string_identifier) REFERENCES software_form_messages_random_string(random_string_to_get_id_after_create)
);

-- software form getting content from random string
CREATE TABLE IF NOT EXISTS software_form_answered_messages_info (
    random_string_identifier VARCHAR(255) NOT NULL, -- random string created by rust random function
    title_of_answered_message VARCHAR(255) NOT NULL,
	content_of_answered_message VARCHAR(255) NOT NULL,  -- first node to show answering message
	FOREIGN KEY (random_string_identifier) REFERENCES software_form_messages_random_string(random_string_to_get_id_after_create)
);

	-- anime form tables:

-- Anime form get random string table top table which has id and random string from code.
CREATE TABLE IF NOT EXISTS anime_form_messages_random_string (
    id INT AUTO_INCREMENT PRIMARY KEY,
    random_string_to_get_id_after_create VARCHAR(255) NOT NULL UNIQUE
);

-- Anime form answered counter
CREATE TABLE IF NOT EXISTS anime_form_messages_answered_counter (
    random_string_identifier VARCHAR(255) NOT NULL,
	answered_count INT NOT NULL DEFAULT 0, -- how many times answered info
    last_answered_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
	FOREIGN KEY (random_string_identifier) REFERENCES anime_form_messages_random_string(random_string_to_get_id_after_create)
);

-- Anime form publishing control table
CREATE TABLE IF NOT EXISTS anime_form_messages_publishing_control (
    random_string_identifier VARCHAR(255) NOT NULL,
    published BOOLEAN DEFAULT TRUE,
    publishing_detailes_changed_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
	FOREIGN KEY (random_string_identifier) REFERENCES anime_form_messages_random_string(random_string_to_get_id_after_create)
);


-- Anime form message info
CREATE TABLE IF NOT EXISTS anime_form_messages_message_info (
    random_string_identifier VARCHAR(255) NOT NULL,
    sender_user_id INT NOT NULL,
    title VARCHAR(255) DEFAULT NULL,
    content TEXT NOT NULL,
	FOREIGN KEY (random_string_identifier) REFERENCES anime_form_messages_random_string(random_string_to_get_id_after_create)
);

-- Anime form has image info
CREATE TABLE IF NOT EXISTS anime_form_messages_has_image_information (
    random_string_identifier VARCHAR(255) NOT NULL,
    has_image bool DEFAULT FALSE,
	FOREIGN KEY (random_string_identifier) REFERENCES anime_form_messages_random_string(random_string_to_get_id_after_create)
);

-- anime image counter
CREATE TABLE IF NOT EXISTS anime_form_messages_image_counter (
	counter_of_image INT AUTO_INCREMENT PRIMARY KEY,
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	FOREIGN KEY (random_string_identifier) REFERENCES anime_form_messages_random_string(random_string_to_get_id_after_create)
);


-- INSERT INTO images (image_data, image_name) VALUES (LOAD_FILE('/path/to/your/image.jpg'), 'image.jpg');
-- anime image informations
CREATE TABLE IF NOT EXISTS anime_form_messages_image_information (
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
    image_data MEDIUMBLOB NOT NULL,
    image_name VARCHAR(255) NOT NULL,
    image_sender_username VARCHAR(255) NOT NULL,
	FOREIGN KEY (random_string_identifier) REFERENCES anime_form_messages_random_string(random_string_to_get_id_after_create)
);

-- anime image time infos
CREATE TABLE IF NOT EXISTS anime_form_messages_image_time_infos (
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    changed_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
	FOREIGN KEY (random_string_identifier) REFERENCES anime_form_messages_random_string(random_string_to_get_id_after_create)
);

-- anime image how many times answered table
CREATE TABLE IF NOT EXISTS anime_form_messages_image_how_many_times_answered (
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	answered_count INT NOT NULL DEFAULT 0, -- how many times answered info
    last_answer_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,    
	FOREIGN KEY (random_string_identifier) REFERENCES anime_form_messages_random_string(random_string_to_get_id_after_create)
);

-- anime image like and dislake counter
CREATE TABLE IF NOT EXISTS anime_form_messages_image_like_dislake_founded_funny (
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	image_liked_count INT NOT NULL DEFAULT 0, -- how many times answered info
	image_dislaked_count INT NOT NULL DEFAULT 0, -- how many times answered info
	image_founded_funny_count INT NOT NULL DEFAULT 0, -- how many times answered info    
	FOREIGN KEY (random_string_identifier) REFERENCES anime_form_messages_random_string(random_string_to_get_id_after_create)
);

-- anime message like and dislake counter
CREATE TABLE IF NOT EXISTS anime_form_messages_like_dislake_information (
	random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	liked_count INT NOT NULL DEFAULT 0, -- how many times answered info
	dislaked_count INT NOT NULL DEFAULT 0, -- how many times answered info
    founded_funny INT NOT NULL DEFAULT 0, -- how many times founded funny info
	FOREIGN KEY (random_string_identifier) REFERENCES anime_form_messages_random_string(random_string_to_get_id_after_create)
);

-- Anime form time info
CREATE TABLE IF NOT EXISTS anime_form_messages_message_time_info (
    random_string_identifier VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
	FOREIGN KEY (random_string_identifier) REFERENCES anime_form_messages_random_string(random_string_to_get_id_after_create)
);

-- Anime form answers location 
CREATE TABLE IF NOT EXISTS anime_form_messages_answered_to_node (
    random_string_identifier VARCHAR(255) NOT NULL, -- first node to show answering message
    answered_messages_string_value VARCHAR(255) NOT NULL,  -- second node to show answered message
	FOREIGN KEY (random_string_identifier) REFERENCES anime_form_messages_random_string(random_string_to_get_id_after_create)
);

-- anime form getting content from random string
CREATE TABLE IF NOT EXISTS anime_form_answered_messages_info (
    random_string_identifier VARCHAR(255) NOT NULL, -- random string created by rust random function
    title_of_answered_message VARCHAR(255) NOT NULL,
	content_of_answered_message VARCHAR(255) NOT NULL,  -- first node to show answering message
	FOREIGN KEY (random_string_identifier) REFERENCES anime_form_messages_random_string(random_string_to_get_id_after_create)
);




	-- making counter's default 1 for not dealing with errors.

-- chatting tables counters are asigned to 1
ALTER TABLE chatting_form_messages_image_counter AUTO_INCREMENT = 1;
ALTER TABLE chatting_form_messages_random_string AUTO_INCREMENT = 1;

-- astrology tables counters are asigned to 1
ALTER TABLE astrology_form_messages_image_counter AUTO_INCREMENT = 1;
ALTER TABLE astrology_form_messages_random_string AUTO_INCREMENT = 1;

-- game tables counters are asigned to 1
ALTER TABLE game_form_messages_image_counter AUTO_INCREMENT = 1;
ALTER TABLE game_form_messages_random_string AUTO_INCREMENT = 1;

-- sport tables counters are asigned to 1
ALTER TABLE sport_form_messages_image_counter AUTO_INCREMENT = 1;
ALTER TABLE sport_form_messages_random_string AUTO_INCREMENT = 1;

-- software tables counters are asigned to 1
ALTER TABLE software_form_messages_image_counter AUTO_INCREMENT = 1;
ALTER TABLE software_form_messages_random_string AUTO_INCREMENT = 1;

-- anime tables counters are asigned to 1
ALTER TABLE anime_form_messages_image_counter AUTO_INCREMENT = 1;
ALTER TABLE anime_form_messages_random_string AUTO_INCREMENT = 1;

-- Inserting messages to anime form, it has not any images: first code to e

INSERT INTO anime_form_messages_random_string (random_string_to_get_id_after_create) VALUES ('şifreli_mesaj_1'); -- where random_string_to_get_id_after_create = şifreli_mesaj get id
INSERT INTO track_messages_main_table (main_table_all_random_string_identifiers, user_id, is_answer, form_type) VALUES ('şifreli_mesaj_1', 1, FALSE, 'anime'); -- user id setted 1 as default
INSERT INTO anime_form_messages_message_info ( random_String_identifier, sender_user_id, title, content) VALUES ('şifreli_mesaj_1', 1,'Which anime I should watch', 'Do you guys recomant me that watch hunter x hunter?'); -- in same function that will asigned as step 2 action, it will take id.
INSERT INTO anime_form_messages_message_time_info( random_String_identifier ) VALUES ('şifreli_mesaj_1');
INSERT INTO anime_form_messages_has_image_information ( random_String_identifier, has_image ) VALUES ('şifreli_mesaj_1',FALSE);
INSERT INTO anime_form_messages_publishing_control (random_string_identifier, published) VALUES ('şifreli_mesaj_1', TRUE);

-- Setting defaults.
INSERT INTO anime_form_messages_like_dislake_information (random_string_identifier) VALUES ('şifreli_mesaj_1');
INSERT INTO anime_form_messages_answered_counter (random_string_identifier) VALUES ('şifreli_mesaj_1');



-- Inserting messages to anime form to answer first message, it has not any images : Ansering to message 1 which in normal form message with no image.sql

INSERT INTO anime_form_messages_random_string (random_string_to_get_id_after_create) VALUES ('şifreli_mesaj_2'); -- where random_string_to_get_id_after_create = şifreli_mesaj get id
INSERT INTO track_messages_main_table (main_table_all_random_string_identifiers, user_id, is_answer, form_type) VALUES ('şifreli_mesaj_2', 2, TRUE, 'anime'); -- user id setted 2 as default
-- I need to get datas which are " where sender sending message and it's random_string to get data ".
INSERT INTO anime_form_messages_message_info ( random_String_identifier, sender_user_id, title, content) VALUES ('şifreli_mesaj_2', 2,'Which anime I should watch', 'Actually I dont like hunter x hunter, just watch mirai nikki'); -- in same function that will asigned as step 2 action, it will take id.
INSERT INTO anime_form_messages_message_time_info( random_String_identifier ) VALUES ('şifreli_mesaj_2');
INSERT INTO anime_form_messages_has_image_information ( random_String_identifier, has_image ) VALUES ('şifreli_mesaj_2',FALSE);
INSERT INTO anime_form_messages_publishing_control (random_string_identifier, published) VALUES ('şifreli_mesaj_2', TRUE);

-- Setting defaults.
INSERT INTO anime_form_messages_like_dislake_information (random_string_identifier) VALUES ('şifreli_mesaj_2');
INSERT INTO anime_form_messages_answered_counter (random_string_identifier) VALUES ('şifreli_mesaj_2');

-- update main message to set a new answered
UPDATE anime_form_messages_answered_counter SET answered_count = answered_count + 1 WHERE random_string_identifier = 'şifreli_mesaj_1';
-- update main message to set founded funny 
UPDATE anime_form_messages_like_dislake_information SET founded_funny = founded_funny + 1 WHERE random_string_identifier = 'şifreli_mesaj_1';

-- Connecting with answered message:

INSERT INTO anime_form_messages_answered_to_node (random_string_identifier, answered_messages_string_value) VALUES ('şifreli_mesaj_2','şifreli_mesaj_1');
INSERT INTO anime_form_answered_messages_info (random_string_identifier, title_of_answered_message, content_of_answered_message) VALUES ('şifreli_mesaj_2','Which anime I should watch', 'Do you guys recomant me that watch hunter x hunter?' );



    -- Users tables:
    
-- users table which is main table:
CREATE TABLE IF NOT EXISTS users (
    id INT PRIMARY KEY AUTO_INCREMENT,
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(100) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    registration_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_login_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- users_info table to save user infos:
CREATE TABLE IF NOT EXISTS users_info(
    username VARCHAR(50) NOT NULL UNIQUE,
    from_as_country VARCHAR(30) DEFAULT NULL,
    name VARCHAR(50) DEFAULT NULL,
    surname VARCHAR(50) DEFAULT NULL,
	sex VARCHAR(30) DEFAULT NULL,
    favorite_anime_girl VARCHAR(255) DEFAULT NULL,
	FOREIGN KEY (username) REFERENCES users(username),
	last_updated_users_info_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- UserSecurityModelQuestion table:
CREATE TABLE IF NOT EXISTS user_security_model_question (
    username VARCHAR(50) NOT NULL UNIQUE,
    user_using_question BOOLEAN NOT NULL,
    security_question VARCHAR(255),
    security_answer VARCHAR(255),
    FOREIGN KEY (username) REFERENCES users(username),
	updated_question_security_model_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- UserSecurityModelNumber table:
CREATE TABLE IF NOT EXISTS user_security_model_telephone_number (
    username VARCHAR(50) NOT NULL UNIQUE,
    user_using_number BOOLEAN NOT NULL,
    tel_number BIGINT,
	FOREIGN KEY (username) REFERENCES users(username),
	updated_telephone_number_security_model_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- UserSecurityModelSavingMail table:
CREATE TABLE IF NOT EXISTS user_security_model_saving_mail (
    username VARCHAR(50) NOT NULL UNIQUE,
    user_using_saving_mail BOOLEAN NOT NULL,
    extra_mail VARCHAR(100),
    FOREIGN KEY (username) REFERENCES users(username),
	updated_saving_mail_security_model_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- UserModelSocials table:
CREATE TABLE IF NOT EXISTS user_model_socials (
    username VARCHAR(50) NOT NULL UNIQUE,
    facebook VARCHAR(255),
    twitter VARCHAR(255),
    instagram VARCHAR(255),
    linkedin VARCHAR(255),
    personal_website VARCHAR(255),
    FOREIGN KEY (username) REFERENCES users(username),
	updated_user_model_socials_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Inserting data into 'users' table
INSERT INTO users (username, email, password_hash)
VALUES ('alice_smith', 'alice.smith@example.com', 'hashed_password_456'),
       ('bob_jones', 'bob.jones@example.com', 'hashed_password_789'),
       ('eva_williams', 'eva.williams@example.com', 'hashed_password_012');

-- Inserting data into 'user_security_model_question' table
INSERT INTO user_security_model_question (username, user_using_question, security_question, security_answer)
VALUES ('alice_smith', true, 'What is your favorite color?', 'Blue'),
       ('bob_jones', true, 'In which city were you born?', 'New York'),
       ('eva_williams', false, NULL, NULL);

-- Inserting data into 'user_security_model_telephone_number' table
INSERT INTO user_security_model_telephone_number (username, user_using_number, tel_number)
VALUES ('alice_smith', true, 5557782939),
       ('bob_jones', true, 5557782939),
       ('eva_williams', false, NULL);

-- Inserting data into 'user_security_model_saving_mail' table
INSERT INTO user_security_model_saving_mail (username, user_using_saving_mail, extra_mail)
VALUES ('alice_smith', true, 'backup_email@example.com'),
       ('bob_jones', false, NULL),
       ('eva_williams', true, 'secondary_email@example.com');






-- Inserting data into 'user_model_socials' table
INSERT INTO user_model_socials (username, facebook, twitter, instagram, linkedin, personal_website)
VALUES ('alice_smith', 'facebook.com/alice', 'twitter.com/alice', 'instagram.com/alice', 'linkedin.com/in/alice', 'alice.com'),
       ('bob_jones', 'facebook.com/bob', 'twitter.com/bob', 'instagram.com/bob', 'linkedin.com/in/bob', 'bob.com'),
       ('eva_williams', NULL, NULL, NULL, NULL, NULL);

SELECT * FROM users;