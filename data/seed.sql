INSERT INTO ping_targets(cron_url,is_activated,is_trial,expires_at) 
VALUES 
-- cases to ping
('https://yahoo.com',1,0,'2030-11-18 15:52:15'),
('https://example.com/344',1,0,'2030-11-18 15:52:15'),
('https://notexistingabcde.com',1,0,'2030-11-18 15:52:15'),
-- cases not to ping
('https://example.com/not_activated',0,0,'2030-11-18 15:52:15'),
('https://example.com/expired',1,0,'2019-11-17 15:52:15'),
('https://example.com/is_trial',1,1,'2030-11-18 15:52:15');