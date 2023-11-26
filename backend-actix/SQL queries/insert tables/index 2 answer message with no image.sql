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
