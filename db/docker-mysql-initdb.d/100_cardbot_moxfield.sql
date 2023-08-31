DROP TABLE IF EXISTS `moxfield_decks`;

CREATE TABLE `moxfield_decks` (
  `id` int(11) unsigned NOT NULL AUTO_INCREMENT PRIMARY KEY,
  `created_at` datetime DEFAULT NOW(),
  `moxfield_id` varchar(255) DEFAULT NULL,
  `deck_json` TEXT
) ENGINE=InnoDB;

CREATE UNIQUE INDEX deck_search_index ON moxfield_decks (created_at,moxfield_id);
