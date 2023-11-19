-- Inserting messages to anime form, it has not any images:

INSERT INTO anime_form_messages_random_string (random_string_to_get_id_after_create) VALUES ('şifreli_mesaj_1'); -- where random_string_to_get_id_after_create = şifreli_mesaj get id
INSERT INTO track_messages_main_table (main_table_all_random_string_identifiers, user_id, is_answer, form_type) VALUES ('şifreli_mesaj_1', 1, FALSE, 'anime'); -- user id setted 1 as default
INSERT INTO anime_form_messages_message_info ( random_String_identifier, sender_user_id, title, content) VALUES ('şifreli_mesaj_1', 1,'Which anime I should watch', 'Do you guys recomant me that watch hunter x hunter?'); -- in same function that will asigned as step 2 action, it will take id.
INSERT INTO anime_form_messages_message_time_info( random_String_identifier ) VALUES ('şifreli_mesaj_1');
INSERT INTO anime_form_messages_has_image_information ( random_String_identifier, has_image ) VALUES ('şifreli_mesaj_1',FALSE);
INSERT INTO anime_form_messages_publishing_control (random_string_identifier, published) VALUES ('şifreli_mesaj_1', TRUE);

-- Setting defaults.
INSERT INTO anime_form_messages_like_dislake_information (random_string_identifier) VALUES ('şifreli_mesaj_1');
INSERT INTO anime_form_messages_answered_counter (random_string_identifier) VALUES ('şifreli_mesaj_1');