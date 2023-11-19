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