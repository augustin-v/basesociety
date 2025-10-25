ALTER TABLE agents ADD COLUMN profile TEXT NOT NULL DEFAULT '{}';
UPDATE agents SET profile = '{"personality": "", "desires": "", "skills": []}';