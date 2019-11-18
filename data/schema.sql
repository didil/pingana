CREATE TABLE IF NOT EXISTS ping_targets(
    `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
   
     
    `status` varchar(20) DEFAULT 'active',
    `is_activated` tinyint(1) DEFAULT 0,
    `is_auto` tinyint(1) DEFAULT 0,
    `is_trial` tinyint(1) DEFAULT 0,
     
     
    `name` varchar(255) DEFAULT NULL,
    `key` varchar(255) DEFAULT NULL,
    `domain` varchar(255) DEFAULT NULL,
     
     
    `remote_count_sources` bigint(10) DEFAULT 0,
    `remote_count_videos` bigint(10) DEFAULT 0,
     
    `cron_url` varchar(255) NOT NULL UNIQUE,
    `cron_auth` tinyint(1) DEFAULT 0,
    `cron_auth_user` varchar(255) DEFAULT '',
    `cron_auth_password` varchar(255) DEFAULT '',
     
    `cron_fails_count` integer DEFAULT 0,
    `cron_last_ping_status` TEXT ,
    `cron_last_pinged_at` datetime DEFAULT NULL,
   
    `created_at` datetime DEFAULT NULL,
    `activated_at` datetime DEFAULT NULL,
    `expires_at` datetime DEFAULT NULL,
    `cancelled_at` datetime DEFAULT NULL,

    PRIMARY KEY (id)
);